# Auto-generated BRK Python client
# Do not edit manually

from __future__ import annotations
from typing import TypeVar, Generic, Any, Optional, List, Literal, TypedDict, Final, Union, Protocol
import httpx

T = TypeVar('T')

# Type definitions

Address = str
Sats = int
TypeIndex = int
class AddressChainStats(TypedDict):
    funded_txo_count: int
    funded_txo_sum: Sats
    spent_txo_count: int
    spent_txo_sum: Sats
    tx_count: int
    type_index: TypeIndex

class AddressMempoolStats(TypedDict):
    funded_txo_count: int
    funded_txo_sum: Sats
    spent_txo_count: int
    spent_txo_sum: Sats
    tx_count: int

class AddressParam(TypedDict):
    address: Address

class AddressStats(TypedDict):
    address: Address
    chain_stats: AddressChainStats
    mempool_stats: Union[AddressMempoolStats, None]

Txid = str
class AddressTxidsParam(TypedDict):
    after_txid: Union[Txid, None]
    limit: int

class AddressValidation(TypedDict):
    address: Optional[str]
    isscript: Optional[bool]
    isvalid: bool
    iswitness: Optional[bool]
    scriptPubKey: Optional[str]
    witness_program: Optional[str]
    witness_version: Optional[int]

AnyAddressIndex = TypeIndex
Bitcoin = float
BlkPosition = int
class BlockCountParam(TypedDict):
    block_count: int

Height = int
Timestamp = int
class BlockFeesEntry(TypedDict):
    avgFees: Sats
    avgHeight: Height
    timestamp: Timestamp

BlockHash = str
class BlockHashParam(TypedDict):
    hash: BlockHash

TxIndex = int
class BlockHashStartIndex(TypedDict):
    hash: BlockHash
    start_index: TxIndex

class BlockHashTxIndex(TypedDict):
    hash: BlockHash
    index: TxIndex

Weight = int
class BlockInfo(TypedDict):
    difficulty: float
    height: Height
    id: BlockHash
    size: int
    timestamp: Timestamp
    tx_count: int
    weight: Weight

class BlockRewardsEntry(TypedDict):
    avgHeight: int
    avgRewards: int
    timestamp: int

class BlockSizeEntry(TypedDict):
    avgHeight: int
    avgSize: int
    timestamp: int

class BlockWeightEntry(TypedDict):
    avgHeight: int
    avgWeight: int
    timestamp: int

class BlockSizesWeights(TypedDict):
    sizes: List[BlockSizeEntry]
    weights: List[BlockWeightEntry]

class BlockStatus(TypedDict):
    height: Union[Height, None]
    in_best_chain: bool
    next_best: Union[BlockHash, None]

class BlockTimestamp(TypedDict):
    hash: BlockHash
    height: Height
    timestamp: str

Cents = int
Close = Cents
Format = Literal["json", "csv"]
class DataRangeFormat(TypedDict):
    count: Optional[int]
    format: Format
    from_: Optional[int]
    to: Optional[int]

Date = int
DateIndex = int
DecadeIndex = int
class DifficultyAdjustment(TypedDict):
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
    change_percent: float
    difficulty: float
    height: Height
    timestamp: Timestamp

class DifficultyEntry(TypedDict):
    difficulty: float
    height: Height
    timestamp: Timestamp

DifficultyEpoch = int
Dollars = float
class EmptyAddressData(TypedDict):
    funded_txo_count: int
    transfered: Sats
    tx_count: int

EmptyAddressIndex = TypeIndex
EmptyOutputIndex = TypeIndex
FeeRate = float
HalvingEpoch = int
class HashrateEntry(TypedDict):
    avgHashrate: int
    timestamp: Timestamp

class HashrateSummary(TypedDict):
    currentDifficulty: float
    currentHashrate: int
    difficulty: List[DifficultyEntry]
    hashrates: List[HashrateEntry]

class Health(TypedDict):
    service: str
    status: str
    timestamp: str

class HeightParam(TypedDict):
    height: Height

Hex = str
High = Cents
class IndexInfo(TypedDict):
    aliases: List[str]
    index: Index

Limit = int
class LimitParam(TypedDict):
    limit: Limit

class LoadedAddressData(TypedDict):
    funded_txo_count: int
    realized_cap: Dollars
    received: Sats
    sent: Sats
    spent_txo_count: int
    tx_count: int

LoadedAddressIndex = TypeIndex
Low = Cents
class MempoolBlock(TypedDict):
    blockSize: int
    blockVSize: float
    feeRange: List[FeeRate]
    medianFee: FeeRate
    nTx: int
    totalFees: Sats

VSize = int
class MempoolInfo(TypedDict):
    count: int
    total_fee: Sats
    vsize: VSize

Metric = str
class MetricCount(TypedDict):
    distinct_metrics: int
    lazy_endpoints: int
    stored_endpoints: int
    total_endpoints: int

class MetricParam(TypedDict):
    metric: Metric

Metrics = str
class MetricSelection(TypedDict):
    count: Optional[int]
    format: Format
    from_: Optional[int]
    index: Index
    metrics: Metrics
    to: Optional[int]

class MetricSelectionLegacy(TypedDict):
    count: Optional[int]
    format: Format
    from_: Optional[int]
    ids: Metrics
    index: Index
    to: Optional[int]

class MetricWithIndex(TypedDict):
    index: Index
    metric: Metric

MonthIndex = int
Open = Cents
class OHLCCents(TypedDict):
    close: Close
    high: High
    low: Low
    open: Open

class OHLCDollars(TypedDict):
    close: Close
    high: High
    low: Low
    open: Open

class OHLCSats(TypedDict):
    close: Close
    high: High
    low: Low
    open: Open

OpReturnIndex = TypeIndex
OutPoint = int
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
    current_page: int
    max_page: int
    metrics: List[str]

class Pagination(TypedDict):
    page: Optional[int]

class PoolBlockCounts(TypedDict):
    _1w: int
    _24h: int
    all: int

class PoolBlockShares(TypedDict):
    _1w: float
    _24h: float
    all: float

PoolSlug = Literal["unknown", "blockfills", "ultimuspool", "terrapool", "luxor", "onethash", "btccom", "bitfarms", "huobipool", "wayicn", "canoepool", "btctop", "bitcoincom", "pool175btc", "gbminers", "axbt", "asicminer", "bitminter", "bitcoinrussia", "btcserv", "simplecoinus", "btcguild", "eligius", "ozcoin", "eclipsemc", "maxbtc", "triplemining", "coinlab", "pool50btc", "ghashio", "stminingcorp", "bitparking", "mmpool", "polmine", "kncminer", "bitalo", "f2pool", "hhtt", "megabigpower", "mtred", "nmcbit", "yourbtcnet", "givemecoins", "braiinspool", "antpool", "multicoinco", "bcpoolio", "cointerra", "kanopool", "solock", "ckpool", "nicehash", "bitclub", "bitcoinaffiliatenetwork", "btcc", "bwpool", "exxbw", "bitsolo", "bitfury", "twentyoneinc", "digitalbtc", "eightbaochi", "mybtccoinpool", "tbdice", "hashpool", "nexious", "bravomining", "hotpool", "okexpool", "bcmonster", "onehash", "bixin", "tatmaspool", "viabtc", "connectbtc", "batpool", "waterhole", "dcexploration", "dcex", "btpool", "fiftyeightcoin", "bitcoinindia", "shawnp0wers", "phashio", "rigpool", "haozhuzhu", "sevenpool", "miningkings", "hashbx", "dpool", "rawpool", "haominer", "helix", "bitcoinukraine", "poolin", "secretsuperstar", "tigerpoolnet", "sigmapoolcom", "okpooltop", "hummerpool", "tangpool", "bytepool", "spiderpool", "novablock", "miningcity", "binancepool", "minerium", "lubiancom", "okkong", "aaopool", "emcdpool", "foundryusa", "sbicrypto", "arkpool", "purebtccom", "marapool", "kucoinpool", "entrustcharitypool", "okminer", "titan", "pegapool", "btcnuggets", "cloudhashing", "digitalxmintsy", "telco214", "btcpoolparty", "multipool", "transactioncoinmining", "btcdig", "trickysbtcpool", "btcmp", "eobot", "unomp", "patels", "gogreenlight", "ekanembtc", "canoe", "tiger", "onem1x", "zulupool", "secpool", "ocean", "whitepool", "wk057", "futurebitapollosolo", "carbonnegative", "portlandhodl", "phoenix", "neopool", "maxipool", "bitfufupool", "luckypool", "miningdutch", "publicpool", "miningsquared", "innopolistech", "btclab", "parasite"]
class PoolDetailInfo(TypedDict):
    addresses: List[str]
    id: int
    link: str
    name: str
    regexes: List[str]
    slug: PoolSlug

class PoolDetail(TypedDict):
    blockCount: PoolBlockCounts
    blockShare: PoolBlockShares
    estimatedHashrate: int
    pool: PoolDetailInfo
    reportedHashrate: Optional[int]

class PoolInfo(TypedDict):
    name: str
    slug: PoolSlug
    unique_id: int

class PoolSlugParam(TypedDict):
    slug: PoolSlug

class PoolStats(TypedDict):
    blockCount: int
    emptyBlocks: int
    link: str
    name: str
    poolId: int
    rank: int
    share: float
    slug: PoolSlug

class PoolsSummary(TypedDict):
    blockCount: int
    lastEstimatedHashrate: int
    pools: List[PoolStats]

QuarterIndex = int
RawLockTime = int
class RecommendedFees(TypedDict):
    economyFee: FeeRate
    fastestFee: FeeRate
    halfHourFee: FeeRate
    hourFee: FeeRate
    minimumFee: FeeRate

class RewardStats(TypedDict):
    endBlock: Height
    startBlock: Height
    totalFee: Sats
    totalReward: Sats
    totalTx: int

SemesterIndex = int
StoredBool = int
StoredF32 = float
StoredF64 = float
StoredI16 = int
StoredU16 = int
StoredU32 = int
StoredU64 = int
class SupplyState(TypedDict):
    utxo_count: int
    value: Sats

TimePeriod = Literal["24h", "3d", "1w", "1m", "3m", "6m", "1y", "2y", "3y"]
class TimePeriodParam(TypedDict):
    time_period: TimePeriod

class TimestampParam(TypedDict):
    timestamp: Timestamp

class TxOut(TypedDict):
    scriptpubkey: str
    value: Sats

Vout = int
class TxIn(TypedDict):
    inner_redeemscript_asm: Optional[str]
    is_coinbase: bool
    prevout: Union[TxOut, None]
    scriptsig: str
    scriptsig_asm: str
    sequence: int
    txid: Txid
    vout: Vout

class TxStatus(TypedDict):
    block_hash: Union[BlockHash, None]
    block_height: Union[Height, None]
    block_time: Union[Timestamp, None]
    confirmed: bool

TxVersion = int
class Transaction(TypedDict):
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
Vin = int
class TxOutspend(TypedDict):
    spent: bool
    status: Union[TxStatus, None]
    txid: Union[Txid, None]
    vin: Union[Vin, None]

class TxidParam(TypedDict):
    txid: Txid

class TxidVout(TypedDict):
    txid: Txid
    vout: Vout

UnknownOutputIndex = TypeIndex
class Utxo(TypedDict):
    status: TxStatus
    txid: Txid
    value: Sats
    vout: Vout

class ValidateAddressParam(TypedDict):
    address: str

WeekIndex = int
YearIndex = int
Index = Literal["dateindex", "decadeindex", "difficultyepoch", "emptyoutputindex", "halvingepoch", "height", "txinindex", "monthindex", "opreturnindex", "txoutindex", "p2aaddressindex", "p2msoutputindex", "p2pk33addressindex", "p2pk65addressindex", "p2pkhaddressindex", "p2shaddressindex", "p2traddressindex", "p2wpkhaddressindex", "p2wshaddressindex", "quarterindex", "semesterindex", "txindex", "unknownoutputindex", "weekindex", "yearindex", "loadedaddressindex", "emptyaddressindex"]
class MetricLeafWithSchema(TypedDict):
    indexes: List[Index]
    kind: str
    name: str
    type: str

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

    def by_dateindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'dateindex')

    def by_decadeindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'decadeindex')

    def by_difficultyepoch(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'difficultyepoch')

    def by_height(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'height')

    def by_monthindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'monthindex')

    def by_quarterindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'quarterindex')

    def by_semesterindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'semesterindex')

    def by_weekindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'weekindex')

    def by_yearindex(self) -> MetricEndpoint[T]:
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
        if index == 'dateindex': return self.by.by_dateindex()
        elif index == 'decadeindex': return self.by.by_decadeindex()
        elif index == 'difficultyepoch': return self.by.by_difficultyepoch()
        elif index == 'height': return self.by.by_height()
        elif index == 'monthindex': return self.by.by_monthindex()
        elif index == 'quarterindex': return self.by.by_quarterindex()
        elif index == 'semesterindex': return self.by.by_semesterindex()
        elif index == 'weekindex': return self.by.by_weekindex()
        elif index == 'yearindex': return self.by.by_yearindex()
        return None

class _MetricPattern2By(Generic[T]):
    """Index endpoint methods container."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def by_dateindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'dateindex')

    def by_decadeindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'decadeindex')

    def by_difficultyepoch(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'difficultyepoch')

    def by_monthindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'monthindex')

    def by_quarterindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'quarterindex')

    def by_semesterindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'semesterindex')

    def by_weekindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'weekindex')

    def by_yearindex(self) -> MetricEndpoint[T]:
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
        if index == 'dateindex': return self.by.by_dateindex()
        elif index == 'decadeindex': return self.by.by_decadeindex()
        elif index == 'difficultyepoch': return self.by.by_difficultyepoch()
        elif index == 'monthindex': return self.by.by_monthindex()
        elif index == 'quarterindex': return self.by.by_quarterindex()
        elif index == 'semesterindex': return self.by.by_semesterindex()
        elif index == 'weekindex': return self.by.by_weekindex()
        elif index == 'yearindex': return self.by.by_yearindex()
        return None

class _MetricPattern3By(Generic[T]):
    """Index endpoint methods container."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def by_dateindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'dateindex')

    def by_decadeindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'decadeindex')

    def by_height(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'height')

    def by_monthindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'monthindex')

    def by_quarterindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'quarterindex')

    def by_semesterindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'semesterindex')

    def by_weekindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'weekindex')

    def by_yearindex(self) -> MetricEndpoint[T]:
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
        if index == 'dateindex': return self.by.by_dateindex()
        elif index == 'decadeindex': return self.by.by_decadeindex()
        elif index == 'height': return self.by.by_height()
        elif index == 'monthindex': return self.by.by_monthindex()
        elif index == 'quarterindex': return self.by.by_quarterindex()
        elif index == 'semesterindex': return self.by.by_semesterindex()
        elif index == 'weekindex': return self.by.by_weekindex()
        elif index == 'yearindex': return self.by.by_yearindex()
        return None

class _MetricPattern4By(Generic[T]):
    """Index endpoint methods container."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def by_dateindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'dateindex')

    def by_decadeindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'decadeindex')

    def by_monthindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'monthindex')

    def by_quarterindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'quarterindex')

    def by_semesterindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'semesterindex')

    def by_weekindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'weekindex')

    def by_yearindex(self) -> MetricEndpoint[T]:
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
        if index == 'dateindex': return self.by.by_dateindex()
        elif index == 'decadeindex': return self.by.by_decadeindex()
        elif index == 'monthindex': return self.by.by_monthindex()
        elif index == 'quarterindex': return self.by.by_quarterindex()
        elif index == 'semesterindex': return self.by.by_semesterindex()
        elif index == 'weekindex': return self.by.by_weekindex()
        elif index == 'yearindex': return self.by.by_yearindex()
        return None

class _MetricPattern5By(Generic[T]):
    """Index endpoint methods container."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def by_decadeindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'decadeindex')

    def by_monthindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'monthindex')

    def by_quarterindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'quarterindex')

    def by_semesterindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'semesterindex')

    def by_weekindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'weekindex')

    def by_yearindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'yearindex')

class MetricPattern5(Generic[T]):
    """Index accessor for metrics with 6 indexes."""
    
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
        return ['decadeindex', 'monthindex', 'quarterindex', 'semesterindex', 'weekindex', 'yearindex']

    def get(self, index: str) -> Optional[MetricEndpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'decadeindex': return self.by.by_decadeindex()
        elif index == 'monthindex': return self.by.by_monthindex()
        elif index == 'quarterindex': return self.by.by_quarterindex()
        elif index == 'semesterindex': return self.by.by_semesterindex()
        elif index == 'weekindex': return self.by.by_weekindex()
        elif index == 'yearindex': return self.by.by_yearindex()
        return None

class _MetricPattern6By(Generic[T]):
    """Index endpoint methods container."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def by_dateindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'dateindex')

    def by_height(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'height')

class MetricPattern6(Generic[T]):
    """Index accessor for metrics with 2 indexes."""
    
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
        return ['dateindex', 'height']

    def get(self, index: str) -> Optional[MetricEndpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'dateindex': return self.by.by_dateindex()
        elif index == 'height': return self.by.by_height()
        return None

class _MetricPattern7By(Generic[T]):
    """Index endpoint methods container."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def by_dateindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'dateindex')

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
        return ['dateindex']

    def get(self, index: str) -> Optional[MetricEndpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'dateindex': return self.by.by_dateindex()
        return None

class _MetricPattern8By(Generic[T]):
    """Index endpoint methods container."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def by_decadeindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'decadeindex')

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
        return ['decadeindex']

    def get(self, index: str) -> Optional[MetricEndpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'decadeindex': return self.by.by_decadeindex()
        return None

class _MetricPattern9By(Generic[T]):
    """Index endpoint methods container."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def by_difficultyepoch(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'difficultyepoch')

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
        return ['difficultyepoch']

    def get(self, index: str) -> Optional[MetricEndpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'difficultyepoch': return self.by.by_difficultyepoch()
        return None

class _MetricPattern10By(Generic[T]):
    """Index endpoint methods container."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def by_emptyoutputindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'emptyoutputindex')

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
        return ['emptyoutputindex']

    def get(self, index: str) -> Optional[MetricEndpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'emptyoutputindex': return self.by.by_emptyoutputindex()
        return None

class _MetricPattern11By(Generic[T]):
    """Index endpoint methods container."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def by_halvingepoch(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'halvingepoch')

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
        return ['halvingepoch']

    def get(self, index: str) -> Optional[MetricEndpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'halvingepoch': return self.by.by_halvingepoch()
        return None

class _MetricPattern12By(Generic[T]):
    """Index endpoint methods container."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def by_height(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'height')

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
        return ['height']

    def get(self, index: str) -> Optional[MetricEndpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'height': return self.by.by_height()
        return None

class _MetricPattern13By(Generic[T]):
    """Index endpoint methods container."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def by_txinindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'txinindex')

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
        return ['txinindex']

    def get(self, index: str) -> Optional[MetricEndpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'txinindex': return self.by.by_txinindex()
        return None

class _MetricPattern14By(Generic[T]):
    """Index endpoint methods container."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def by_monthindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'monthindex')

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
        return ['monthindex']

    def get(self, index: str) -> Optional[MetricEndpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'monthindex': return self.by.by_monthindex()
        return None

class _MetricPattern15By(Generic[T]):
    """Index endpoint methods container."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def by_opreturnindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'opreturnindex')

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
        return ['opreturnindex']

    def get(self, index: str) -> Optional[MetricEndpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'opreturnindex': return self.by.by_opreturnindex()
        return None

class _MetricPattern16By(Generic[T]):
    """Index endpoint methods container."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def by_txoutindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'txoutindex')

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
        return ['txoutindex']

    def get(self, index: str) -> Optional[MetricEndpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'txoutindex': return self.by.by_txoutindex()
        return None

class _MetricPattern17By(Generic[T]):
    """Index endpoint methods container."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def by_p2aaddressindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'p2aaddressindex')

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
        return ['p2aaddressindex']

    def get(self, index: str) -> Optional[MetricEndpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'p2aaddressindex': return self.by.by_p2aaddressindex()
        return None

class _MetricPattern18By(Generic[T]):
    """Index endpoint methods container."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def by_p2msoutputindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'p2msoutputindex')

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
        return ['p2msoutputindex']

    def get(self, index: str) -> Optional[MetricEndpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'p2msoutputindex': return self.by.by_p2msoutputindex()
        return None

class _MetricPattern19By(Generic[T]):
    """Index endpoint methods container."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def by_p2pk33addressindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'p2pk33addressindex')

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
        return ['p2pk33addressindex']

    def get(self, index: str) -> Optional[MetricEndpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'p2pk33addressindex': return self.by.by_p2pk33addressindex()
        return None

class _MetricPattern20By(Generic[T]):
    """Index endpoint methods container."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def by_p2pk65addressindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'p2pk65addressindex')

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
        return ['p2pk65addressindex']

    def get(self, index: str) -> Optional[MetricEndpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'p2pk65addressindex': return self.by.by_p2pk65addressindex()
        return None

class _MetricPattern21By(Generic[T]):
    """Index endpoint methods container."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def by_p2pkhaddressindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'p2pkhaddressindex')

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
        return ['p2pkhaddressindex']

    def get(self, index: str) -> Optional[MetricEndpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'p2pkhaddressindex': return self.by.by_p2pkhaddressindex()
        return None

class _MetricPattern22By(Generic[T]):
    """Index endpoint methods container."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def by_p2shaddressindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'p2shaddressindex')

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
        return ['p2shaddressindex']

    def get(self, index: str) -> Optional[MetricEndpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'p2shaddressindex': return self.by.by_p2shaddressindex()
        return None

class _MetricPattern23By(Generic[T]):
    """Index endpoint methods container."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def by_p2traddressindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'p2traddressindex')

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
        return ['p2traddressindex']

    def get(self, index: str) -> Optional[MetricEndpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'p2traddressindex': return self.by.by_p2traddressindex()
        return None

class _MetricPattern24By(Generic[T]):
    """Index endpoint methods container."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def by_p2wpkhaddressindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'p2wpkhaddressindex')

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
        return ['p2wpkhaddressindex']

    def get(self, index: str) -> Optional[MetricEndpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'p2wpkhaddressindex': return self.by.by_p2wpkhaddressindex()
        return None

class _MetricPattern25By(Generic[T]):
    """Index endpoint methods container."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def by_p2wshaddressindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'p2wshaddressindex')

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
        return ['p2wshaddressindex']

    def get(self, index: str) -> Optional[MetricEndpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'p2wshaddressindex': return self.by.by_p2wshaddressindex()
        return None

class _MetricPattern26By(Generic[T]):
    """Index endpoint methods container."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def by_quarterindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'quarterindex')

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
        return ['quarterindex']

    def get(self, index: str) -> Optional[MetricEndpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'quarterindex': return self.by.by_quarterindex()
        return None

class _MetricPattern27By(Generic[T]):
    """Index endpoint methods container."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def by_semesterindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'semesterindex')

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
        return ['semesterindex']

    def get(self, index: str) -> Optional[MetricEndpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'semesterindex': return self.by.by_semesterindex()
        return None

class _MetricPattern28By(Generic[T]):
    """Index endpoint methods container."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def by_txindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'txindex')

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
        return ['txindex']

    def get(self, index: str) -> Optional[MetricEndpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'txindex': return self.by.by_txindex()
        return None

class _MetricPattern29By(Generic[T]):
    """Index endpoint methods container."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def by_unknownoutputindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'unknownoutputindex')

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
        return ['unknownoutputindex']

    def get(self, index: str) -> Optional[MetricEndpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'unknownoutputindex': return self.by.by_unknownoutputindex()
        return None

class _MetricPattern30By(Generic[T]):
    """Index endpoint methods container."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def by_weekindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'weekindex')

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
        return ['weekindex']

    def get(self, index: str) -> Optional[MetricEndpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'weekindex': return self.by.by_weekindex()
        return None

class _MetricPattern31By(Generic[T]):
    """Index endpoint methods container."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def by_yearindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'yearindex')

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
        return ['yearindex']

    def get(self, index: str) -> Optional[MetricEndpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'yearindex': return self.by.by_yearindex()
        return None

class _MetricPattern32By(Generic[T]):
    """Index endpoint methods container."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def by_loadedaddressindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'loadedaddressindex')

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
        return ['loadedaddressindex']

    def get(self, index: str) -> Optional[MetricEndpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'loadedaddressindex': return self.by.by_loadedaddressindex()
        return None

class _MetricPattern33By(Generic[T]):
    """Index endpoint methods container."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def by_emptyaddressindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'emptyaddressindex')

class MetricPattern33(Generic[T]):
    """Index accessor for metrics with 1 indexes."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name
        self.by: _MetricPattern33By[T] = _MetricPattern33By(client, name)

    @property
    def name(self) -> str:
        """Get the metric name."""
        return self._name

    def indexes(self) -> List[str]:
        """Get the list of available indexes."""
        return ['emptyaddressindex']

    def get(self, index: str) -> Optional[MetricEndpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'emptyaddressindex': return self.by.by_emptyaddressindex()
        return None

# Reusable structural pattern classes

class RealizedPattern3:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.adjusted_sopr: MetricPattern7[StoredF64] = MetricPattern7(client, _m(acc, 'adjusted_sopr'))
        self.adjusted_sopr_30d_ema: MetricPattern7[StoredF64] = MetricPattern7(client, _m(acc, 'adjusted_sopr_30d_ema'))
        self.adjusted_sopr_7d_ema: MetricPattern7[StoredF64] = MetricPattern7(client, _m(acc, 'adjusted_sopr_7d_ema'))
        self.adjusted_value_created: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'adjusted_value_created'))
        self.adjusted_value_destroyed: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'adjusted_value_destroyed'))
        self.mvrv: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'mvrv'))
        self.neg_realized_loss: BlockCountPattern[Dollars] = BlockCountPattern(client, _m(acc, 'neg_realized_loss'))
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
        self.realized_profit_to_loss_ratio: MetricPattern7[StoredF64] = MetricPattern7(client, _m(acc, 'realized_profit_to_loss_ratio'))
        self.realized_value: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_value'))
        self.sell_side_risk_ratio: MetricPattern7[StoredF32] = MetricPattern7(client, _m(acc, 'sell_side_risk_ratio'))
        self.sell_side_risk_ratio_30d_ema: MetricPattern7[StoredF32] = MetricPattern7(client, _m(acc, 'sell_side_risk_ratio_30d_ema'))
        self.sell_side_risk_ratio_7d_ema: MetricPattern7[StoredF32] = MetricPattern7(client, _m(acc, 'sell_side_risk_ratio_7d_ema'))
        self.sopr: MetricPattern7[StoredF64] = MetricPattern7(client, _m(acc, 'sopr'))
        self.sopr_30d_ema: MetricPattern7[StoredF64] = MetricPattern7(client, _m(acc, 'sopr_30d_ema'))
        self.sopr_7d_ema: MetricPattern7[StoredF64] = MetricPattern7(client, _m(acc, 'sopr_7d_ema'))
        self.total_realized_pnl: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'total_realized_pnl'))
        self.value_created: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'value_created'))
        self.value_destroyed: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'value_destroyed'))

class RealizedPattern4:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.adjusted_sopr: MetricPattern7[StoredF64] = MetricPattern7(client, _m(acc, 'adjusted_sopr'))
        self.adjusted_sopr_30d_ema: MetricPattern7[StoredF64] = MetricPattern7(client, _m(acc, 'adjusted_sopr_30d_ema'))
        self.adjusted_sopr_7d_ema: MetricPattern7[StoredF64] = MetricPattern7(client, _m(acc, 'adjusted_sopr_7d_ema'))
        self.adjusted_value_created: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'adjusted_value_created'))
        self.adjusted_value_destroyed: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'adjusted_value_destroyed'))
        self.mvrv: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'mvrv'))
        self.neg_realized_loss: BlockCountPattern[Dollars] = BlockCountPattern(client, _m(acc, 'neg_realized_loss'))
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
        self.sell_side_risk_ratio: MetricPattern7[StoredF32] = MetricPattern7(client, _m(acc, 'sell_side_risk_ratio'))
        self.sell_side_risk_ratio_30d_ema: MetricPattern7[StoredF32] = MetricPattern7(client, _m(acc, 'sell_side_risk_ratio_30d_ema'))
        self.sell_side_risk_ratio_7d_ema: MetricPattern7[StoredF32] = MetricPattern7(client, _m(acc, 'sell_side_risk_ratio_7d_ema'))
        self.sopr: MetricPattern7[StoredF64] = MetricPattern7(client, _m(acc, 'sopr'))
        self.sopr_30d_ema: MetricPattern7[StoredF64] = MetricPattern7(client, _m(acc, 'sopr_30d_ema'))
        self.sopr_7d_ema: MetricPattern7[StoredF64] = MetricPattern7(client, _m(acc, 'sopr_7d_ema'))
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
        self.neg_realized_loss: BlockCountPattern[Dollars] = BlockCountPattern(client, _m(acc, 'neg_realized_loss'))
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
        self.realized_profit_to_loss_ratio: MetricPattern7[StoredF64] = MetricPattern7(client, _m(acc, 'realized_profit_to_loss_ratio'))
        self.realized_value: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_value'))
        self.sell_side_risk_ratio: MetricPattern7[StoredF32] = MetricPattern7(client, _m(acc, 'sell_side_risk_ratio'))
        self.sell_side_risk_ratio_30d_ema: MetricPattern7[StoredF32] = MetricPattern7(client, _m(acc, 'sell_side_risk_ratio_30d_ema'))
        self.sell_side_risk_ratio_7d_ema: MetricPattern7[StoredF32] = MetricPattern7(client, _m(acc, 'sell_side_risk_ratio_7d_ema'))
        self.sopr: MetricPattern7[StoredF64] = MetricPattern7(client, _m(acc, 'sopr'))
        self.sopr_30d_ema: MetricPattern7[StoredF64] = MetricPattern7(client, _m(acc, 'sopr_30d_ema'))
        self.sopr_7d_ema: MetricPattern7[StoredF64] = MetricPattern7(client, _m(acc, 'sopr_7d_ema'))
        self.total_realized_pnl: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'total_realized_pnl'))
        self.value_created: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'value_created'))
        self.value_destroyed: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'value_destroyed'))

class RealizedPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.mvrv: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'mvrv'))
        self.neg_realized_loss: BlockCountPattern[Dollars] = BlockCountPattern(client, _m(acc, 'neg_realized_loss'))
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
        self.sell_side_risk_ratio: MetricPattern7[StoredF32] = MetricPattern7(client, _m(acc, 'sell_side_risk_ratio'))
        self.sell_side_risk_ratio_30d_ema: MetricPattern7[StoredF32] = MetricPattern7(client, _m(acc, 'sell_side_risk_ratio_30d_ema'))
        self.sell_side_risk_ratio_7d_ema: MetricPattern7[StoredF32] = MetricPattern7(client, _m(acc, 'sell_side_risk_ratio_7d_ema'))
        self.sopr: MetricPattern7[StoredF64] = MetricPattern7(client, _m(acc, 'sopr'))
        self.sopr_30d_ema: MetricPattern7[StoredF64] = MetricPattern7(client, _m(acc, 'sopr_30d_ema'))
        self.sopr_7d_ema: MetricPattern7[StoredF64] = MetricPattern7(client, _m(acc, 'sopr_7d_ema'))
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

class PercentilesPattern2:
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
        self.coinbase: UnclaimedRewardsPattern = UnclaimedRewardsPattern(client, _m(acc, 'coinbase'))
        self.days_since_block: MetricPattern4[StoredU16] = MetricPattern4(client, _m(acc, 'days_since_block'))
        self.dominance: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'dominance'))
        self.fee: UnclaimedRewardsPattern = UnclaimedRewardsPattern(client, _m(acc, 'fee'))
        self.subsidy: UnclaimedRewardsPattern = UnclaimedRewardsPattern(client, _m(acc, 'subsidy'))

class PriceAgoPattern(Generic[T]):
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self._10y: MetricPattern4[T] = MetricPattern4(client, _m(acc, '10y_ago'))
        self._1d: MetricPattern4[T] = MetricPattern4(client, _m(acc, '1d_ago'))
        self._1m: MetricPattern4[T] = MetricPattern4(client, _m(acc, '1m_ago'))
        self._1w: MetricPattern4[T] = MetricPattern4(client, _m(acc, '1w_ago'))
        self._1y: MetricPattern4[T] = MetricPattern4(client, _m(acc, '1y_ago'))
        self._2y: MetricPattern4[T] = MetricPattern4(client, _m(acc, '2y_ago'))
        self._3m: MetricPattern4[T] = MetricPattern4(client, _m(acc, '3m_ago'))
        self._3y: MetricPattern4[T] = MetricPattern4(client, _m(acc, '3y_ago'))
        self._4y: MetricPattern4[T] = MetricPattern4(client, _m(acc, '4y_ago'))
        self._5y: MetricPattern4[T] = MetricPattern4(client, _m(acc, '5y_ago'))
        self._6m: MetricPattern4[T] = MetricPattern4(client, _m(acc, '6m_ago'))
        self._6y: MetricPattern4[T] = MetricPattern4(client, _m(acc, '6y_ago'))
        self._8y: MetricPattern4[T] = MetricPattern4(client, _m(acc, '8y_ago'))

class PeriodLumpSumStackPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self._10y: _24hCoinbaseSumPattern = _24hCoinbaseSumPattern(client, (f'10y_{{acc}}' if acc else '10y'))
        self._1m: _24hCoinbaseSumPattern = _24hCoinbaseSumPattern(client, (f'1m_{{acc}}' if acc else '1m'))
        self._1w: _24hCoinbaseSumPattern = _24hCoinbaseSumPattern(client, (f'1w_{{acc}}' if acc else '1w'))
        self._1y: _24hCoinbaseSumPattern = _24hCoinbaseSumPattern(client, (f'1y_{{acc}}' if acc else '1y'))
        self._2y: _24hCoinbaseSumPattern = _24hCoinbaseSumPattern(client, (f'2y_{{acc}}' if acc else '2y'))
        self._3m: _24hCoinbaseSumPattern = _24hCoinbaseSumPattern(client, (f'3m_{{acc}}' if acc else '3m'))
        self._3y: _24hCoinbaseSumPattern = _24hCoinbaseSumPattern(client, (f'3y_{{acc}}' if acc else '3y'))
        self._4y: _24hCoinbaseSumPattern = _24hCoinbaseSumPattern(client, (f'4y_{{acc}}' if acc else '4y'))
        self._5y: _24hCoinbaseSumPattern = _24hCoinbaseSumPattern(client, (f'5y_{{acc}}' if acc else '5y'))
        self._6m: _24hCoinbaseSumPattern = _24hCoinbaseSumPattern(client, (f'6m_{{acc}}' if acc else '6m'))
        self._6y: _24hCoinbaseSumPattern = _24hCoinbaseSumPattern(client, (f'6y_{{acc}}' if acc else '6y'))
        self._8y: _24hCoinbaseSumPattern = _24hCoinbaseSumPattern(client, (f'8y_{{acc}}' if acc else '8y'))

class PeriodAveragePricePattern(Generic[T]):
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self._10y: MetricPattern4[T] = MetricPattern4(client, (f'10y_{{acc}}' if acc else '10y'))
        self._1m: MetricPattern4[T] = MetricPattern4(client, (f'1m_{{acc}}' if acc else '1m'))
        self._1w: MetricPattern4[T] = MetricPattern4(client, (f'1w_{{acc}}' if acc else '1w'))
        self._1y: MetricPattern4[T] = MetricPattern4(client, (f'1y_{{acc}}' if acc else '1y'))
        self._2y: MetricPattern4[T] = MetricPattern4(client, (f'2y_{{acc}}' if acc else '2y'))
        self._3m: MetricPattern4[T] = MetricPattern4(client, (f'3m_{{acc}}' if acc else '3m'))
        self._3y: MetricPattern4[T] = MetricPattern4(client, (f'3y_{{acc}}' if acc else '3y'))
        self._4y: MetricPattern4[T] = MetricPattern4(client, (f'4y_{{acc}}' if acc else '4y'))
        self._5y: MetricPattern4[T] = MetricPattern4(client, (f'5y_{{acc}}' if acc else '5y'))
        self._6m: MetricPattern4[T] = MetricPattern4(client, (f'6m_{{acc}}' if acc else '6m'))
        self._6y: MetricPattern4[T] = MetricPattern4(client, (f'6y_{{acc}}' if acc else '6y'))
        self._8y: MetricPattern4[T] = MetricPattern4(client, (f'8y_{{acc}}' if acc else '8y'))

class ClassAveragePricePattern(Generic[T]):
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self._2015: MetricPattern4[T] = MetricPattern4(client, _m(acc, '2015_average_price'))
        self._2016: MetricPattern4[T] = MetricPattern4(client, _m(acc, '2016_average_price'))
        self._2017: MetricPattern4[T] = MetricPattern4(client, _m(acc, '2017_average_price'))
        self._2018: MetricPattern4[T] = MetricPattern4(client, _m(acc, '2018_average_price'))
        self._2019: MetricPattern4[T] = MetricPattern4(client, _m(acc, '2019_average_price'))
        self._2020: MetricPattern4[T] = MetricPattern4(client, _m(acc, '2020_average_price'))
        self._2021: MetricPattern4[T] = MetricPattern4(client, _m(acc, '2021_average_price'))
        self._2022: MetricPattern4[T] = MetricPattern4(client, _m(acc, '2022_average_price'))
        self._2023: MetricPattern4[T] = MetricPattern4(client, _m(acc, '2023_average_price'))
        self._2024: MetricPattern4[T] = MetricPattern4(client, _m(acc, '2024_average_price'))
        self._2025: MetricPattern4[T] = MetricPattern4(client, _m(acc, '2025_average_price'))

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

class AddrCountPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.all: MetricPattern1[StoredU64] = MetricPattern1(client, (f'addr_{{acc}}' if acc else 'addr'))
        self.p2a: MetricPattern1[StoredU64] = MetricPattern1(client, (f'p2a_addr_{{acc}}' if acc else 'p2a_addr'))
        self.p2pk33: MetricPattern1[StoredU64] = MetricPattern1(client, (f'p2pk33_addr_{{acc}}' if acc else 'p2pk33_addr'))
        self.p2pk65: MetricPattern1[StoredU64] = MetricPattern1(client, (f'p2pk65_addr_{{acc}}' if acc else 'p2pk65_addr'))
        self.p2pkh: MetricPattern1[StoredU64] = MetricPattern1(client, (f'p2pkh_addr_{{acc}}' if acc else 'p2pkh_addr'))
        self.p2sh: MetricPattern1[StoredU64] = MetricPattern1(client, (f'p2sh_addr_{{acc}}' if acc else 'p2sh_addr'))
        self.p2tr: MetricPattern1[StoredU64] = MetricPattern1(client, (f'p2tr_addr_{{acc}}' if acc else 'p2tr_addr'))
        self.p2wpkh: MetricPattern1[StoredU64] = MetricPattern1(client, (f'p2wpkh_addr_{{acc}}' if acc else 'p2wpkh_addr'))
        self.p2wsh: MetricPattern1[StoredU64] = MetricPattern1(client, (f'p2wsh_addr_{{acc}}' if acc else 'p2wsh_addr'))

class OhlcPattern(Generic[T]):
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.dateindex: MetricPattern7[T] = MetricPattern7(client, acc)
        self.decade: MetricPattern8[T] = MetricPattern8(client, acc)
        self.difficultyepoch: MetricPattern9[T] = MetricPattern9(client, acc)
        self.height: MetricPattern12[T] = MetricPattern12(client, acc)
        self.month: MetricPattern14[T] = MetricPattern14(client, acc)
        self.quarter: MetricPattern26[T] = MetricPattern26(client, acc)
        self.semester: MetricPattern27[T] = MetricPattern27(client, acc)
        self.week: MetricPattern30[T] = MetricPattern30(client, acc)
        self.year: MetricPattern31[T] = MetricPattern31(client, acc)

class FeeRatePattern(Generic[T]):
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.average: MetricPattern1[T] = MetricPattern1(client, _m(acc, 'average'))
        self.max: MetricPattern1[T] = MetricPattern1(client, _m(acc, 'max'))
        self.median: MetricPattern12[T] = MetricPattern12(client, _m(acc, 'median'))
        self.min: MetricPattern1[T] = MetricPattern1(client, _m(acc, 'min'))
        self.pct10: MetricPattern12[T] = MetricPattern12(client, _m(acc, 'pct10'))
        self.pct25: MetricPattern12[T] = MetricPattern12(client, _m(acc, 'pct25'))
        self.pct75: MetricPattern12[T] = MetricPattern12(client, _m(acc, 'pct75'))
        self.pct90: MetricPattern12[T] = MetricPattern12(client, _m(acc, 'pct90'))
        self.txindex: MetricPattern28[T] = MetricPattern28(client, acc)

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

class UnrealizedPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.neg_unrealized_loss: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'neg_unrealized_loss'))
        self.net_unrealized_pnl: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'net_unrealized_pnl'))
        self.supply_in_loss: _24hCoinbaseSumPattern = _24hCoinbaseSumPattern(client, _m(acc, 'supply_in_loss'))
        self.supply_in_profit: _24hCoinbaseSumPattern = _24hCoinbaseSumPattern(client, _m(acc, 'supply_in_profit'))
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

class PeriodCagrPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self._10y: MetricPattern4[StoredF32] = MetricPattern4(client, (f'10y_{{acc}}' if acc else '10y'))
        self._2y: MetricPattern4[StoredF32] = MetricPattern4(client, (f'2y_{{acc}}' if acc else '2y'))
        self._3y: MetricPattern4[StoredF32] = MetricPattern4(client, (f'3y_{{acc}}' if acc else '3y'))
        self._4y: MetricPattern4[StoredF32] = MetricPattern4(client, (f'4y_{{acc}}' if acc else '4y'))
        self._5y: MetricPattern4[StoredF32] = MetricPattern4(client, (f'5y_{{acc}}' if acc else '5y'))
        self._6y: MetricPattern4[StoredF32] = MetricPattern4(client, (f'6y_{{acc}}' if acc else '6y'))
        self._8y: MetricPattern4[StoredF32] = MetricPattern4(client, (f'8y_{{acc}}' if acc else '8y'))

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

class BitcoinPattern(Generic[T]):
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.average: MetricPattern2[T] = MetricPattern2(client, _m(acc, 'average'))
        self.cumulative: MetricPattern1[T] = MetricPattern1(client, _m(acc, 'cumulative'))
        self.height: MetricPattern12[T] = MetricPattern12(client, acc)
        self.max: MetricPattern2[T] = MetricPattern2(client, _m(acc, 'max'))
        self.min: MetricPattern2[T] = MetricPattern2(client, _m(acc, 'min'))
        self.percentiles: PercentilesPattern[T] = PercentilesPattern(client, acc)
        self.sum: MetricPattern2[T] = MetricPattern2(client, _m(acc, 'sum'))

class SizePattern(Generic[T]):
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.average: MetricPattern2[T] = MetricPattern2(client, _m(acc, 'average'))
        self.cumulative: MetricPattern1[T] = MetricPattern1(client, _m(acc, 'cumulative'))
        self.max: MetricPattern2[T] = MetricPattern2(client, _m(acc, 'max'))
        self.min: MetricPattern2[T] = MetricPattern2(client, _m(acc, 'min'))
        self.percentiles: PercentilesPattern[T] = PercentilesPattern(client, acc)
        self.sum: MetricPattern2[T] = MetricPattern2(client, _m(acc, 'sum'))

class ActivityPattern2:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.coinblocks_destroyed: BlockCountPattern[StoredF64] = BlockCountPattern(client, _m(acc, 'coinblocks_destroyed'))
        self.coindays_destroyed: BlockCountPattern[StoredF64] = BlockCountPattern(client, _m(acc, 'coindays_destroyed'))
        self.satblocks_destroyed: MetricPattern12[Sats] = MetricPattern12(client, _m(acc, 'satblocks_destroyed'))
        self.satdays_destroyed: MetricPattern12[Sats] = MetricPattern12(client, _m(acc, 'satdays_destroyed'))
        self.sent: UnclaimedRewardsPattern = UnclaimedRewardsPattern(client, _m(acc, 'sent'))

class PercentilesPattern(Generic[T]):
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.median: MetricPattern7[T] = MetricPattern7(client, _m(acc, 'median'))
        self.pct10: MetricPattern7[T] = MetricPattern7(client, _m(acc, 'pct10'))
        self.pct25: MetricPattern7[T] = MetricPattern7(client, _m(acc, 'pct25'))
        self.pct75: MetricPattern7[T] = MetricPattern7(client, _m(acc, 'pct75'))
        self.pct90: MetricPattern7[T] = MetricPattern7(client, _m(acc, 'pct90'))

class DateindexPattern2:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.close: MetricPattern7[Cents] = MetricPattern7(client, _m(acc, 'close_cents'))
        self.high: MetricPattern7[Cents] = MetricPattern7(client, _m(acc, 'high_cents'))
        self.low: MetricPattern7[Cents] = MetricPattern7(client, _m(acc, 'low_cents'))
        self.open: MetricPattern7[Cents] = MetricPattern7(client, _m(acc, 'open_cents'))

class SplitPattern2(Generic[T]):
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.close: MetricPattern1[T] = MetricPattern1(client, _m(acc, 'close'))
        self.high: HighPattern[T] = HighPattern(client, _m(acc, 'high'))
        self.low: HighPattern[T] = HighPattern(client, _m(acc, 'low'))
        self.open: MetricPattern1[T] = MetricPattern1(client, _m(acc, 'open'))

class HighPattern(Generic[T]):
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.dateindex: MetricPattern7[T] = MetricPattern7(client, acc)
        self.difficultyepoch: MetricPattern9[T] = MetricPattern9(client, _m(acc, 'max'))
        self.height: MetricPattern12[T] = MetricPattern12(client, acc)
        self.rest: MetricPattern5[T] = MetricPattern5(client, _m(acc, 'max'))

class _24hCoinbaseSumPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.bitcoin: MetricPattern12[Bitcoin] = MetricPattern12(client, _m(acc, 'btc'))
        self.dollars: MetricPattern12[Dollars] = MetricPattern12(client, _m(acc, 'usd'))
        self.sats: MetricPattern12[Sats] = MetricPattern12(client, acc)

class SegwitAdoptionPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.cumulative: MetricPattern2[StoredF32] = MetricPattern2(client, _m(acc, 'cumulative'))
        self.height: MetricPattern12[StoredF32] = MetricPattern12(client, acc)
        self.sum: MetricPattern2[StoredF32] = MetricPattern2(client, _m(acc, 'sum'))

class CostBasisPattern2:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.max: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'max_cost_basis'))
        self.min: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'min_cost_basis'))
        self.percentiles: PercentilesPattern2 = PercentilesPattern2(client, _m(acc, 'cost_basis'))

class UnclaimedRewardsPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.bitcoin: BlockCountPattern[Bitcoin] = BlockCountPattern(client, _m(acc, 'btc'))
        self.dollars: BlockCountPattern[Dollars] = BlockCountPattern(client, _m(acc, 'usd'))
        self.sats: BlockCountPattern[Sats] = BlockCountPattern(client, acc)

class CoinbasePattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.bitcoin: BitcoinPattern[Bitcoin] = BitcoinPattern(client, _m(acc, 'btc'))
        self.dollars: BitcoinPattern[Dollars] = BitcoinPattern(client, _m(acc, 'usd'))
        self.sats: BitcoinPattern[Sats] = BitcoinPattern(client, acc)

class RelativePattern4:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.supply_in_loss_rel_to_own_supply: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'loss_rel_to_own_supply'))
        self.supply_in_profit_rel_to_own_supply: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'profit_rel_to_own_supply'))

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

class SupplyPattern2:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.halved: _24hCoinbaseSumPattern = _24hCoinbaseSumPattern(client, _m(acc, 'half'))
        self.total: _24hCoinbaseSumPattern = _24hCoinbaseSumPattern(client, acc)

class SatsPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.ohlc: OhlcPattern[OHLCSats] = OhlcPattern(client, _m(acc, 'ohlc_sats'))
        self.split: SplitPattern2[Sats] = SplitPattern2(client, _m(acc, 'sats'))

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

class EmptyPattern(Generic[T]):
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.identity: MetricPattern18[T] = MetricPattern18(client, acc)

# Catalog tree classes

class CatalogTree:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.addresses: CatalogTree_Addresses = CatalogTree_Addresses(client, f'{base_path}_addresses')
        self.blocks: CatalogTree_Blocks = CatalogTree_Blocks(client, f'{base_path}_blocks')
        self.cointime: CatalogTree_Cointime = CatalogTree_Cointime(client, f'{base_path}_cointime')
        self.constants: CatalogTree_Constants = CatalogTree_Constants(client, f'{base_path}_constants')
        self.distribution: CatalogTree_Distribution = CatalogTree_Distribution(client, f'{base_path}_distribution')
        self.indexes: CatalogTree_Indexes = CatalogTree_Indexes(client, f'{base_path}_indexes')
        self.inputs: CatalogTree_Inputs = CatalogTree_Inputs(client, f'{base_path}_inputs')
        self.market: CatalogTree_Market = CatalogTree_Market(client, f'{base_path}_market')
        self.outputs: CatalogTree_Outputs = CatalogTree_Outputs(client, f'{base_path}_outputs')
        self.pools: CatalogTree_Pools = CatalogTree_Pools(client, f'{base_path}_pools')
        self.positions: CatalogTree_Positions = CatalogTree_Positions(client, f'{base_path}_positions')
        self.price: CatalogTree_Price = CatalogTree_Price(client, f'{base_path}_price')
        self.scripts: CatalogTree_Scripts = CatalogTree_Scripts(client, f'{base_path}_scripts')
        self.supply: CatalogTree_Supply = CatalogTree_Supply(client, f'{base_path}_supply')
        self.transactions: CatalogTree_Transactions = CatalogTree_Transactions(client, f'{base_path}_transactions')

class CatalogTree_Addresses:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.first_p2aaddressindex: MetricPattern12[P2AAddressIndex] = MetricPattern12(client, f'{base_path}_first_p2aaddressindex')
        self.first_p2pk33addressindex: MetricPattern12[P2PK33AddressIndex] = MetricPattern12(client, f'{base_path}_first_p2pk33addressindex')
        self.first_p2pk65addressindex: MetricPattern12[P2PK65AddressIndex] = MetricPattern12(client, f'{base_path}_first_p2pk65addressindex')
        self.first_p2pkhaddressindex: MetricPattern12[P2PKHAddressIndex] = MetricPattern12(client, f'{base_path}_first_p2pkhaddressindex')
        self.first_p2shaddressindex: MetricPattern12[P2SHAddressIndex] = MetricPattern12(client, f'{base_path}_first_p2shaddressindex')
        self.first_p2traddressindex: MetricPattern12[P2TRAddressIndex] = MetricPattern12(client, f'{base_path}_first_p2traddressindex')
        self.first_p2wpkhaddressindex: MetricPattern12[P2WPKHAddressIndex] = MetricPattern12(client, f'{base_path}_first_p2wpkhaddressindex')
        self.first_p2wshaddressindex: MetricPattern12[P2WSHAddressIndex] = MetricPattern12(client, f'{base_path}_first_p2wshaddressindex')
        self.p2abytes: MetricPattern17[P2ABytes] = MetricPattern17(client, f'{base_path}_p2abytes')
        self.p2pk33bytes: MetricPattern19[P2PK33Bytes] = MetricPattern19(client, f'{base_path}_p2pk33bytes')
        self.p2pk65bytes: MetricPattern20[P2PK65Bytes] = MetricPattern20(client, f'{base_path}_p2pk65bytes')
        self.p2pkhbytes: MetricPattern21[P2PKHBytes] = MetricPattern21(client, f'{base_path}_p2pkhbytes')
        self.p2shbytes: MetricPattern22[P2SHBytes] = MetricPattern22(client, f'{base_path}_p2shbytes')
        self.p2trbytes: MetricPattern23[P2TRBytes] = MetricPattern23(client, f'{base_path}_p2trbytes')
        self.p2wpkhbytes: MetricPattern24[P2WPKHBytes] = MetricPattern24(client, f'{base_path}_p2wpkhbytes')
        self.p2wshbytes: MetricPattern25[P2WSHBytes] = MetricPattern25(client, f'{base_path}_p2wshbytes')

class CatalogTree_Blocks:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.blockhash: MetricPattern12[BlockHash] = MetricPattern12(client, f'{base_path}_blockhash')
        self.count: CatalogTree_Blocks_Count = CatalogTree_Blocks_Count(client, f'{base_path}_count')
        self.difficulty: CatalogTree_Blocks_Difficulty = CatalogTree_Blocks_Difficulty(client, f'{base_path}_difficulty')
        self.halving: CatalogTree_Blocks_Halving = CatalogTree_Blocks_Halving(client, f'{base_path}_halving')
        self.interval: CatalogTree_Blocks_Interval = CatalogTree_Blocks_Interval(client, f'{base_path}_interval')
        self.mining: CatalogTree_Blocks_Mining = CatalogTree_Blocks_Mining(client, f'{base_path}_mining')
        self.rewards: CatalogTree_Blocks_Rewards = CatalogTree_Blocks_Rewards(client, f'{base_path}_rewards')
        self.size: CatalogTree_Blocks_Size = CatalogTree_Blocks_Size(client, f'{base_path}_size')
        self.time: CatalogTree_Blocks_Time = CatalogTree_Blocks_Time(client, f'{base_path}_time')
        self.timestamp: MetricPattern12[Timestamp] = MetricPattern12(client, f'{base_path}_timestamp')
        self.total_size: MetricPattern12[StoredU64] = MetricPattern12(client, f'{base_path}_total_size')
        self.weight: CatalogTree_Blocks_Weight = CatalogTree_Blocks_Weight(client, f'{base_path}_weight')

class CatalogTree_Blocks_Count:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._1m_block_count: MetricPattern1[StoredU32] = MetricPattern1(client, f'{base_path}__1m_block_count')
        self._1m_start: MetricPattern12[Height] = MetricPattern12(client, f'{base_path}__1m_start')
        self._1w_block_count: MetricPattern1[StoredU32] = MetricPattern1(client, f'{base_path}__1w_block_count')
        self._1w_start: MetricPattern12[Height] = MetricPattern12(client, f'{base_path}__1w_start')
        self._1y_block_count: MetricPattern1[StoredU32] = MetricPattern1(client, f'{base_path}__1y_block_count')
        self._1y_start: MetricPattern12[Height] = MetricPattern12(client, f'{base_path}__1y_start')
        self._24h_block_count: MetricPattern1[StoredU32] = MetricPattern1(client, f'{base_path}__24h_block_count')
        self._24h_start: MetricPattern12[Height] = MetricPattern12(client, f'{base_path}__24h_start')
        self.block_count: BlockCountPattern[StoredU32] = BlockCountPattern(client, 'block_count')
        self.block_count_target: MetricPattern4[StoredU64] = MetricPattern4(client, f'{base_path}_block_count_target')

class CatalogTree_Blocks_Difficulty:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.base: MetricPattern12[StoredF64] = MetricPattern12(client, f'{base_path}_base')
        self.blocks_before_next_difficulty_adjustment: MetricPattern1[StoredU32] = MetricPattern1(client, f'{base_path}_blocks_before_next_difficulty_adjustment')
        self.days_before_next_difficulty_adjustment: MetricPattern1[StoredF32] = MetricPattern1(client, f'{base_path}_days_before_next_difficulty_adjustment')
        self.epoch: MetricPattern4[DifficultyEpoch] = MetricPattern4(client, f'{base_path}_epoch')

class CatalogTree_Blocks_Halving:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.blocks_before_next_halving: MetricPattern1[StoredU32] = MetricPattern1(client, f'{base_path}_blocks_before_next_halving')
        self.days_before_next_halving: MetricPattern1[StoredF32] = MetricPattern1(client, f'{base_path}_days_before_next_halving')
        self.epoch: MetricPattern4[HalvingEpoch] = MetricPattern4(client, f'{base_path}_epoch')

class CatalogTree_Blocks_Interval:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.average: MetricPattern2[Timestamp] = MetricPattern2(client, f'{base_path}_average')
        self.base: MetricPattern12[Timestamp] = MetricPattern12(client, f'{base_path}_base')
        self.max: MetricPattern2[Timestamp] = MetricPattern2(client, f'{base_path}_max')
        self.median: MetricPattern7[Timestamp] = MetricPattern7(client, f'{base_path}_median')
        self.min: MetricPattern2[Timestamp] = MetricPattern2(client, f'{base_path}_min')
        self.pct10: MetricPattern7[Timestamp] = MetricPattern7(client, f'{base_path}_pct10')
        self.pct25: MetricPattern7[Timestamp] = MetricPattern7(client, f'{base_path}_pct25')
        self.pct75: MetricPattern7[Timestamp] = MetricPattern7(client, f'{base_path}_pct75')
        self.pct90: MetricPattern7[Timestamp] = MetricPattern7(client, f'{base_path}_pct90')

class CatalogTree_Blocks_Mining:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.difficulty: MetricPattern2[StoredF64] = MetricPattern2(client, f'{base_path}_difficulty')
        self.difficulty_adjustment: MetricPattern1[StoredF32] = MetricPattern1(client, f'{base_path}_difficulty_adjustment')
        self.difficulty_as_hash: MetricPattern1[StoredF32] = MetricPattern1(client, f'{base_path}_difficulty_as_hash')
        self.hash_price_phs: MetricPattern1[StoredF32] = MetricPattern1(client, f'{base_path}_hash_price_phs')
        self.hash_price_phs_min: MetricPattern1[StoredF32] = MetricPattern1(client, f'{base_path}_hash_price_phs_min')
        self.hash_price_rebound: MetricPattern1[StoredF32] = MetricPattern1(client, f'{base_path}_hash_price_rebound')
        self.hash_price_ths: MetricPattern1[StoredF32] = MetricPattern1(client, f'{base_path}_hash_price_ths')
        self.hash_price_ths_min: MetricPattern1[StoredF32] = MetricPattern1(client, f'{base_path}_hash_price_ths_min')
        self.hash_rate: MetricPattern1[StoredF64] = MetricPattern1(client, f'{base_path}_hash_rate')
        self.hash_rate_1m_sma: MetricPattern4[StoredF32] = MetricPattern4(client, f'{base_path}_hash_rate_1m_sma')
        self.hash_rate_1w_sma: MetricPattern4[StoredF64] = MetricPattern4(client, f'{base_path}_hash_rate_1w_sma')
        self.hash_rate_1y_sma: MetricPattern4[StoredF32] = MetricPattern4(client, f'{base_path}_hash_rate_1y_sma')
        self.hash_rate_2m_sma: MetricPattern4[StoredF32] = MetricPattern4(client, f'{base_path}_hash_rate_2m_sma')
        self.hash_value_phs: MetricPattern1[StoredF32] = MetricPattern1(client, f'{base_path}_hash_value_phs')
        self.hash_value_phs_min: MetricPattern1[StoredF32] = MetricPattern1(client, f'{base_path}_hash_value_phs_min')
        self.hash_value_rebound: MetricPattern1[StoredF32] = MetricPattern1(client, f'{base_path}_hash_value_rebound')
        self.hash_value_ths: MetricPattern1[StoredF32] = MetricPattern1(client, f'{base_path}_hash_value_ths')
        self.hash_value_ths_min: MetricPattern1[StoredF32] = MetricPattern1(client, f'{base_path}_hash_value_ths_min')

class CatalogTree_Blocks_Rewards:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._24h_coinbase_sum: _24hCoinbaseSumPattern = _24hCoinbaseSumPattern(client, '24h_coinbase_sum')
        self.coinbase: CoinbasePattern = CoinbasePattern(client, 'coinbase')
        self.fee_dominance: MetricPattern7[StoredF32] = MetricPattern7(client, f'{base_path}_fee_dominance')
        self.subsidy: CoinbasePattern = CoinbasePattern(client, 'subsidy')
        self.subsidy_dominance: MetricPattern7[StoredF32] = MetricPattern7(client, f'{base_path}_subsidy_dominance')
        self.subsidy_usd_1y_sma: MetricPattern4[Dollars] = MetricPattern4(client, f'{base_path}_subsidy_usd_1y_sma')
        self.unclaimed_rewards: UnclaimedRewardsPattern = UnclaimedRewardsPattern(client, 'unclaimed_rewards')

class CatalogTree_Blocks_Size:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.size: SizePattern[StoredU64] = SizePattern(client, 'block_size')
        self.vbytes: CatalogTree_Blocks_Size_Vbytes = CatalogTree_Blocks_Size_Vbytes(client, f'{base_path}_vbytes')

class CatalogTree_Blocks_Size_Vbytes:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.average: MetricPattern2[StoredU64] = MetricPattern2(client, f'{base_path}_average')
        self.base: MetricPattern12[StoredU64] = MetricPattern12(client, f'{base_path}_base')
        self.cumulative: MetricPattern1[StoredU64] = MetricPattern1(client, f'{base_path}_cumulative')
        self.max: MetricPattern2[StoredU64] = MetricPattern2(client, f'{base_path}_max')
        self.min: MetricPattern2[StoredU64] = MetricPattern2(client, f'{base_path}_min')
        self.percentiles: PercentilesPattern[StoredU64] = PercentilesPattern(client, 'block_vbytes')
        self.sum: MetricPattern2[StoredU64] = MetricPattern2(client, f'{base_path}_sum')

class CatalogTree_Blocks_Time:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.date: MetricPattern12[Date] = MetricPattern12(client, f'{base_path}_date')
        self.date_fixed: MetricPattern12[Date] = MetricPattern12(client, f'{base_path}_date_fixed')
        self.timestamp: MetricPattern2[Timestamp] = MetricPattern2(client, f'{base_path}_timestamp')
        self.timestamp_fixed: MetricPattern12[Timestamp] = MetricPattern12(client, f'{base_path}_timestamp_fixed')

class CatalogTree_Blocks_Weight:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.base: MetricPattern12[Weight] = MetricPattern12(client, f'{base_path}_base')
        self.fullness: BitcoinPattern[StoredF32] = BitcoinPattern(client, 'block_fullness')
        self.weight: SizePattern[Weight] = SizePattern(client, 'block_weight')

class CatalogTree_Cointime:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.activity: CatalogTree_Cointime_Activity = CatalogTree_Cointime_Activity(client, f'{base_path}_activity')
        self.adjusted: CatalogTree_Cointime_Adjusted = CatalogTree_Cointime_Adjusted(client, f'{base_path}_adjusted')
        self.cap: CatalogTree_Cointime_Cap = CatalogTree_Cointime_Cap(client, f'{base_path}_cap')
        self.pricing: CatalogTree_Cointime_Pricing = CatalogTree_Cointime_Pricing(client, f'{base_path}_pricing')
        self.supply: CatalogTree_Cointime_Supply = CatalogTree_Cointime_Supply(client, f'{base_path}_supply')
        self.value: CatalogTree_Cointime_Value = CatalogTree_Cointime_Value(client, f'{base_path}_value')

class CatalogTree_Cointime_Activity:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.activity_to_vaultedness_ratio: MetricPattern1[StoredF64] = MetricPattern1(client, f'{base_path}_activity_to_vaultedness_ratio')
        self.coinblocks_created: BlockCountPattern[StoredF64] = BlockCountPattern(client, 'coinblocks_created')
        self.coinblocks_stored: BlockCountPattern[StoredF64] = BlockCountPattern(client, 'coinblocks_stored')
        self.liveliness: MetricPattern1[StoredF64] = MetricPattern1(client, f'{base_path}_liveliness')
        self.vaultedness: MetricPattern1[StoredF64] = MetricPattern1(client, f'{base_path}_vaultedness')

class CatalogTree_Cointime_Adjusted:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.cointime_adj_inflation_rate: MetricPattern4[StoredF32] = MetricPattern4(client, f'{base_path}_cointime_adj_inflation_rate')
        self.cointime_adj_tx_btc_velocity: MetricPattern4[StoredF64] = MetricPattern4(client, f'{base_path}_cointime_adj_tx_btc_velocity')
        self.cointime_adj_tx_usd_velocity: MetricPattern4[StoredF64] = MetricPattern4(client, f'{base_path}_cointime_adj_tx_usd_velocity')

class CatalogTree_Cointime_Cap:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.active_cap: MetricPattern1[Dollars] = MetricPattern1(client, f'{base_path}_active_cap')
        self.cointime_cap: MetricPattern1[Dollars] = MetricPattern1(client, f'{base_path}_cointime_cap')
        self.investor_cap: MetricPattern1[Dollars] = MetricPattern1(client, f'{base_path}_investor_cap')
        self.thermo_cap: MetricPattern1[Dollars] = MetricPattern1(client, f'{base_path}_thermo_cap')
        self.vaulted_cap: MetricPattern1[Dollars] = MetricPattern1(client, f'{base_path}_vaulted_cap')

class CatalogTree_Cointime_Pricing:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.active_price: MetricPattern1[Dollars] = MetricPattern1(client, f'{base_path}_active_price')
        self.active_price_ratio: ActivePriceRatioPattern = ActivePriceRatioPattern(client, 'active_price_ratio')
        self.cointime_price: MetricPattern1[Dollars] = MetricPattern1(client, f'{base_path}_cointime_price')
        self.cointime_price_ratio: ActivePriceRatioPattern = ActivePriceRatioPattern(client, 'cointime_price_ratio')
        self.true_market_mean: MetricPattern1[Dollars] = MetricPattern1(client, f'{base_path}_true_market_mean')
        self.true_market_mean_ratio: ActivePriceRatioPattern = ActivePriceRatioPattern(client, 'true_market_mean_ratio')
        self.vaulted_price: MetricPattern1[Dollars] = MetricPattern1(client, f'{base_path}_vaulted_price')
        self.vaulted_price_ratio: ActivePriceRatioPattern = ActivePriceRatioPattern(client, 'vaulted_price_ratio')

class CatalogTree_Cointime_Supply:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.active_supply: _24hCoinbaseSumPattern = _24hCoinbaseSumPattern(client, 'active_supply')
        self.vaulted_supply: _24hCoinbaseSumPattern = _24hCoinbaseSumPattern(client, 'vaulted_supply')

class CatalogTree_Cointime_Value:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.cointime_value_created: BlockCountPattern[StoredF64] = BlockCountPattern(client, 'cointime_value_created')
        self.cointime_value_destroyed: BlockCountPattern[StoredF64] = BlockCountPattern(client, 'cointime_value_destroyed')
        self.cointime_value_stored: BlockCountPattern[StoredF64] = BlockCountPattern(client, 'cointime_value_stored')

class CatalogTree_Constants:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.constant_0: MetricPattern3[StoredU16] = MetricPattern3(client, f'{base_path}_constant_0')
        self.constant_1: MetricPattern3[StoredU16] = MetricPattern3(client, f'{base_path}_constant_1')
        self.constant_100: MetricPattern3[StoredU16] = MetricPattern3(client, f'{base_path}_constant_100')
        self.constant_2: MetricPattern3[StoredU16] = MetricPattern3(client, f'{base_path}_constant_2')
        self.constant_20: MetricPattern3[StoredU16] = MetricPattern3(client, f'{base_path}_constant_20')
        self.constant_3: MetricPattern3[StoredU16] = MetricPattern3(client, f'{base_path}_constant_3')
        self.constant_30: MetricPattern3[StoredU16] = MetricPattern3(client, f'{base_path}_constant_30')
        self.constant_38_2: MetricPattern3[StoredF32] = MetricPattern3(client, f'{base_path}_constant_38_2')
        self.constant_4: MetricPattern3[StoredU16] = MetricPattern3(client, f'{base_path}_constant_4')
        self.constant_50: MetricPattern3[StoredU16] = MetricPattern3(client, f'{base_path}_constant_50')
        self.constant_600: MetricPattern3[StoredU16] = MetricPattern3(client, f'{base_path}_constant_600')
        self.constant_61_8: MetricPattern3[StoredF32] = MetricPattern3(client, f'{base_path}_constant_61_8')
        self.constant_70: MetricPattern3[StoredU16] = MetricPattern3(client, f'{base_path}_constant_70')
        self.constant_80: MetricPattern3[StoredU16] = MetricPattern3(client, f'{base_path}_constant_80')
        self.constant_minus_1: MetricPattern3[StoredI16] = MetricPattern3(client, f'{base_path}_constant_minus_1')
        self.constant_minus_2: MetricPattern3[StoredI16] = MetricPattern3(client, f'{base_path}_constant_minus_2')
        self.constant_minus_3: MetricPattern3[StoredI16] = MetricPattern3(client, f'{base_path}_constant_minus_3')
        self.constant_minus_4: MetricPattern3[StoredI16] = MetricPattern3(client, f'{base_path}_constant_minus_4')

class CatalogTree_Distribution:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.addr_count: AddrCountPattern = AddrCountPattern(client, 'addr_count')
        self.address_cohorts: CatalogTree_Distribution_AddressCohorts = CatalogTree_Distribution_AddressCohorts(client, f'{base_path}_address_cohorts')
        self.addresses_data: CatalogTree_Distribution_AddressesData = CatalogTree_Distribution_AddressesData(client, f'{base_path}_addresses_data')
        self.any_address_indexes: CatalogTree_Distribution_AnyAddressIndexes = CatalogTree_Distribution_AnyAddressIndexes(client, f'{base_path}_any_address_indexes')
        self.chain_state: MetricPattern12[SupplyState] = MetricPattern12(client, f'{base_path}_chain_state')
        self.empty_addr_count: AddrCountPattern = AddrCountPattern(client, 'empty_addr_count')
        self.emptyaddressindex: MetricPattern33[EmptyAddressIndex] = MetricPattern33(client, f'{base_path}_emptyaddressindex')
        self.loadedaddressindex: MetricPattern32[LoadedAddressIndex] = MetricPattern32(client, f'{base_path}_loadedaddressindex')
        self.utxo_cohorts: CatalogTree_Distribution_UtxoCohorts = CatalogTree_Distribution_UtxoCohorts(client, f'{base_path}_utxo_cohorts')

class CatalogTree_Distribution_AddressCohorts:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.amount_range: CatalogTree_Distribution_AddressCohorts_AmountRange = CatalogTree_Distribution_AddressCohorts_AmountRange(client, f'{base_path}_amount_range')
        self.ge_amount: CatalogTree_Distribution_AddressCohorts_GeAmount = CatalogTree_Distribution_AddressCohorts_GeAmount(client, f'{base_path}_ge_amount')
        self.lt_amount: CatalogTree_Distribution_AddressCohorts_LtAmount = CatalogTree_Distribution_AddressCohorts_LtAmount(client, f'{base_path}_lt_amount')

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
        self.empty: MetricPattern33[EmptyAddressData] = MetricPattern33(client, f'{base_path}_empty')
        self.loaded: MetricPattern32[LoadedAddressData] = MetricPattern32(client, f'{base_path}_loaded')

class CatalogTree_Distribution_AnyAddressIndexes:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.p2a: MetricPattern17[AnyAddressIndex] = MetricPattern17(client, f'{base_path}_p2a')
        self.p2pk33: MetricPattern19[AnyAddressIndex] = MetricPattern19(client, f'{base_path}_p2pk33')
        self.p2pk65: MetricPattern20[AnyAddressIndex] = MetricPattern20(client, f'{base_path}_p2pk65')
        self.p2pkh: MetricPattern21[AnyAddressIndex] = MetricPattern21(client, f'{base_path}_p2pkh')
        self.p2sh: MetricPattern22[AnyAddressIndex] = MetricPattern22(client, f'{base_path}_p2sh')
        self.p2tr: MetricPattern23[AnyAddressIndex] = MetricPattern23(client, f'{base_path}_p2tr')
        self.p2wpkh: MetricPattern24[AnyAddressIndex] = MetricPattern24(client, f'{base_path}_p2wpkh')
        self.p2wsh: MetricPattern25[AnyAddressIndex] = MetricPattern25(client, f'{base_path}_p2wsh')

class CatalogTree_Distribution_UtxoCohorts:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.age_range: CatalogTree_Distribution_UtxoCohorts_AgeRange = CatalogTree_Distribution_UtxoCohorts_AgeRange(client, f'{base_path}_age_range')
        self.all: CatalogTree_Distribution_UtxoCohorts_All = CatalogTree_Distribution_UtxoCohorts_All(client, f'{base_path}_all')
        self.amount_range: CatalogTree_Distribution_UtxoCohorts_AmountRange = CatalogTree_Distribution_UtxoCohorts_AmountRange(client, f'{base_path}_amount_range')
        self.epoch: CatalogTree_Distribution_UtxoCohorts_Epoch = CatalogTree_Distribution_UtxoCohorts_Epoch(client, f'{base_path}_epoch')
        self.ge_amount: CatalogTree_Distribution_UtxoCohorts_GeAmount = CatalogTree_Distribution_UtxoCohorts_GeAmount(client, f'{base_path}_ge_amount')
        self.lt_amount: CatalogTree_Distribution_UtxoCohorts_LtAmount = CatalogTree_Distribution_UtxoCohorts_LtAmount(client, f'{base_path}_lt_amount')
        self.max_age: CatalogTree_Distribution_UtxoCohorts_MaxAge = CatalogTree_Distribution_UtxoCohorts_MaxAge(client, f'{base_path}_max_age')
        self.min_age: CatalogTree_Distribution_UtxoCohorts_MinAge = CatalogTree_Distribution_UtxoCohorts_MinAge(client, f'{base_path}_min_age')
        self.term: CatalogTree_Distribution_UtxoCohorts_Term = CatalogTree_Distribution_UtxoCohorts_Term(client, f'{base_path}_term')
        self.type_: CatalogTree_Distribution_UtxoCohorts_Type = CatalogTree_Distribution_UtxoCohorts_Type(client, f'{base_path}_type_')
        self.year: CatalogTree_Distribution_UtxoCohorts_Year = CatalogTree_Distribution_UtxoCohorts_Year(client, f'{base_path}_year')

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
        self.activity: ActivityPattern2 = ActivityPattern2(client, '')
        self.cost_basis: CostBasisPattern2 = CostBasisPattern2(client, '')
        self.outputs: OutputsPattern = OutputsPattern(client, 'utxo_count')
        self.realized: RealizedPattern3 = RealizedPattern3(client, '')
        self.relative: CatalogTree_Distribution_UtxoCohorts_All_Relative = CatalogTree_Distribution_UtxoCohorts_All_Relative(client, f'{base_path}_relative')
        self.supply: SupplyPattern2 = SupplyPattern2(client, 'supply')
        self.unrealized: UnrealizedPattern = UnrealizedPattern(client, '')

class CatalogTree_Distribution_UtxoCohorts_All_Relative:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.neg_unrealized_loss_rel_to_own_total_unrealized_pnl: MetricPattern1[StoredF32] = MetricPattern1(client, f'{base_path}_neg_unrealized_loss_rel_to_own_total_unrealized_pnl')
        self.net_unrealized_pnl_rel_to_own_total_unrealized_pnl: MetricPattern1[StoredF32] = MetricPattern1(client, f'{base_path}_net_unrealized_pnl_rel_to_own_total_unrealized_pnl')
        self.supply_in_loss_rel_to_own_supply: MetricPattern1[StoredF64] = MetricPattern1(client, f'{base_path}_supply_in_loss_rel_to_own_supply')
        self.supply_in_profit_rel_to_own_supply: MetricPattern1[StoredF64] = MetricPattern1(client, f'{base_path}_supply_in_profit_rel_to_own_supply')
        self.unrealized_loss_rel_to_own_total_unrealized_pnl: MetricPattern1[StoredF32] = MetricPattern1(client, f'{base_path}_unrealized_loss_rel_to_own_total_unrealized_pnl')
        self.unrealized_profit_rel_to_own_total_unrealized_pnl: MetricPattern1[StoredF32] = MetricPattern1(client, f'{base_path}_unrealized_profit_rel_to_own_total_unrealized_pnl')

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
        self.long: CatalogTree_Distribution_UtxoCohorts_Term_Long = CatalogTree_Distribution_UtxoCohorts_Term_Long(client, f'{base_path}_long')
        self.short: CatalogTree_Distribution_UtxoCohorts_Term_Short = CatalogTree_Distribution_UtxoCohorts_Term_Short(client, f'{base_path}_short')

class CatalogTree_Distribution_UtxoCohorts_Term_Long:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.activity: ActivityPattern2 = ActivityPattern2(client, 'lth')
        self.cost_basis: CostBasisPattern2 = CostBasisPattern2(client, 'lth')
        self.outputs: OutputsPattern = OutputsPattern(client, 'lth_utxo_count')
        self.realized: RealizedPattern2 = RealizedPattern2(client, 'lth')
        self.relative: RelativePattern5 = RelativePattern5(client, 'lth')
        self.supply: SupplyPattern2 = SupplyPattern2(client, 'lth_supply')
        self.unrealized: UnrealizedPattern = UnrealizedPattern(client, 'lth')

class CatalogTree_Distribution_UtxoCohorts_Term_Short:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.activity: ActivityPattern2 = ActivityPattern2(client, 'sth')
        self.cost_basis: CostBasisPattern2 = CostBasisPattern2(client, 'sth')
        self.outputs: OutputsPattern = OutputsPattern(client, 'sth_utxo_count')
        self.realized: RealizedPattern3 = RealizedPattern3(client, 'sth')
        self.relative: RelativePattern5 = RelativePattern5(client, 'sth')
        self.supply: SupplyPattern2 = SupplyPattern2(client, 'sth_supply')
        self.unrealized: UnrealizedPattern = UnrealizedPattern(client, 'sth')

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
        self.address: CatalogTree_Indexes_Address = CatalogTree_Indexes_Address(client, f'{base_path}_address')
        self.dateindex: CatalogTree_Indexes_Dateindex = CatalogTree_Indexes_Dateindex(client, f'{base_path}_dateindex')
        self.decadeindex: CatalogTree_Indexes_Decadeindex = CatalogTree_Indexes_Decadeindex(client, f'{base_path}_decadeindex')
        self.difficultyepoch: CatalogTree_Indexes_Difficultyepoch = CatalogTree_Indexes_Difficultyepoch(client, f'{base_path}_difficultyepoch')
        self.halvingepoch: CatalogTree_Indexes_Halvingepoch = CatalogTree_Indexes_Halvingepoch(client, f'{base_path}_halvingepoch')
        self.height: CatalogTree_Indexes_Height = CatalogTree_Indexes_Height(client, f'{base_path}_height')
        self.monthindex: CatalogTree_Indexes_Monthindex = CatalogTree_Indexes_Monthindex(client, f'{base_path}_monthindex')
        self.quarterindex: CatalogTree_Indexes_Quarterindex = CatalogTree_Indexes_Quarterindex(client, f'{base_path}_quarterindex')
        self.semesterindex: CatalogTree_Indexes_Semesterindex = CatalogTree_Indexes_Semesterindex(client, f'{base_path}_semesterindex')
        self.txindex: CatalogTree_Indexes_Txindex = CatalogTree_Indexes_Txindex(client, f'{base_path}_txindex')
        self.txinindex: EmptyPattern[TxInIndex] = EmptyPattern(client, 'txinindex')
        self.txoutindex: EmptyPattern[TxOutIndex] = EmptyPattern(client, 'txoutindex')
        self.weekindex: CatalogTree_Indexes_Weekindex = CatalogTree_Indexes_Weekindex(client, f'{base_path}_weekindex')
        self.yearindex: CatalogTree_Indexes_Yearindex = CatalogTree_Indexes_Yearindex(client, f'{base_path}_yearindex')

class CatalogTree_Indexes_Address:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.empty: EmptyPattern[EmptyOutputIndex] = EmptyPattern(client, 'emptyoutputindex')
        self.opreturn: EmptyPattern[OpReturnIndex] = EmptyPattern(client, 'opreturnindex')
        self.p2a: EmptyPattern[P2AAddressIndex] = EmptyPattern(client, 'p2aaddressindex')
        self.p2ms: EmptyPattern[P2MSOutputIndex] = EmptyPattern(client, 'p2msoutputindex')
        self.p2pk33: EmptyPattern[P2PK33AddressIndex] = EmptyPattern(client, 'p2pk33addressindex')
        self.p2pk65: EmptyPattern[P2PK65AddressIndex] = EmptyPattern(client, 'p2pk65addressindex')
        self.p2pkh: EmptyPattern[P2PKHAddressIndex] = EmptyPattern(client, 'p2pkhaddressindex')
        self.p2sh: EmptyPattern[P2SHAddressIndex] = EmptyPattern(client, 'p2shaddressindex')
        self.p2tr: EmptyPattern[P2TRAddressIndex] = EmptyPattern(client, 'p2traddressindex')
        self.p2wpkh: EmptyPattern[P2WPKHAddressIndex] = EmptyPattern(client, 'p2wpkhaddressindex')
        self.p2wsh: EmptyPattern[P2WSHAddressIndex] = EmptyPattern(client, 'p2wshaddressindex')
        self.unknown: EmptyPattern[UnknownOutputIndex] = EmptyPattern(client, 'unknownoutputindex')

class CatalogTree_Indexes_Dateindex:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.date: MetricPattern7[Date] = MetricPattern7(client, f'{base_path}_date')
        self.first_height: MetricPattern7[Height] = MetricPattern7(client, f'{base_path}_first_height')
        self.height_count: MetricPattern7[StoredU64] = MetricPattern7(client, f'{base_path}_height_count')
        self.identity: MetricPattern7[DateIndex] = MetricPattern7(client, f'{base_path}_identity')
        self.monthindex: MetricPattern7[MonthIndex] = MetricPattern7(client, f'{base_path}_monthindex')
        self.weekindex: MetricPattern7[WeekIndex] = MetricPattern7(client, f'{base_path}_weekindex')

class CatalogTree_Indexes_Decadeindex:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.first_yearindex: MetricPattern8[YearIndex] = MetricPattern8(client, f'{base_path}_first_yearindex')
        self.identity: MetricPattern8[DecadeIndex] = MetricPattern8(client, f'{base_path}_identity')
        self.yearindex_count: MetricPattern8[StoredU64] = MetricPattern8(client, f'{base_path}_yearindex_count')

class CatalogTree_Indexes_Difficultyepoch:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.first_height: MetricPattern9[Height] = MetricPattern9(client, f'{base_path}_first_height')
        self.height_count: MetricPattern9[StoredU64] = MetricPattern9(client, f'{base_path}_height_count')
        self.identity: MetricPattern9[DifficultyEpoch] = MetricPattern9(client, f'{base_path}_identity')

class CatalogTree_Indexes_Halvingepoch:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.first_height: MetricPattern11[Height] = MetricPattern11(client, f'{base_path}_first_height')
        self.identity: MetricPattern11[HalvingEpoch] = MetricPattern11(client, f'{base_path}_identity')

class CatalogTree_Indexes_Height:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.dateindex: MetricPattern12[DateIndex] = MetricPattern12(client, f'{base_path}_dateindex')
        self.difficultyepoch: MetricPattern12[DifficultyEpoch] = MetricPattern12(client, f'{base_path}_difficultyepoch')
        self.halvingepoch: MetricPattern12[HalvingEpoch] = MetricPattern12(client, f'{base_path}_halvingepoch')
        self.identity: MetricPattern12[Height] = MetricPattern12(client, f'{base_path}_identity')
        self.txindex_count: MetricPattern12[StoredU64] = MetricPattern12(client, f'{base_path}_txindex_count')

class CatalogTree_Indexes_Monthindex:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.dateindex_count: MetricPattern14[StoredU64] = MetricPattern14(client, f'{base_path}_dateindex_count')
        self.first_dateindex: MetricPattern14[DateIndex] = MetricPattern14(client, f'{base_path}_first_dateindex')
        self.identity: MetricPattern14[MonthIndex] = MetricPattern14(client, f'{base_path}_identity')
        self.quarterindex: MetricPattern14[QuarterIndex] = MetricPattern14(client, f'{base_path}_quarterindex')
        self.semesterindex: MetricPattern14[SemesterIndex] = MetricPattern14(client, f'{base_path}_semesterindex')
        self.yearindex: MetricPattern14[YearIndex] = MetricPattern14(client, f'{base_path}_yearindex')

class CatalogTree_Indexes_Quarterindex:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.first_monthindex: MetricPattern26[MonthIndex] = MetricPattern26(client, f'{base_path}_first_monthindex')
        self.identity: MetricPattern26[QuarterIndex] = MetricPattern26(client, f'{base_path}_identity')
        self.monthindex_count: MetricPattern26[StoredU64] = MetricPattern26(client, f'{base_path}_monthindex_count')

class CatalogTree_Indexes_Semesterindex:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.first_monthindex: MetricPattern27[MonthIndex] = MetricPattern27(client, f'{base_path}_first_monthindex')
        self.identity: MetricPattern27[SemesterIndex] = MetricPattern27(client, f'{base_path}_identity')
        self.monthindex_count: MetricPattern27[StoredU64] = MetricPattern27(client, f'{base_path}_monthindex_count')

class CatalogTree_Indexes_Txindex:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern28[TxIndex] = MetricPattern28(client, f'{base_path}_identity')
        self.input_count: MetricPattern28[StoredU64] = MetricPattern28(client, f'{base_path}_input_count')
        self.output_count: MetricPattern28[StoredU64] = MetricPattern28(client, f'{base_path}_output_count')

class CatalogTree_Indexes_Weekindex:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.dateindex_count: MetricPattern30[StoredU64] = MetricPattern30(client, f'{base_path}_dateindex_count')
        self.first_dateindex: MetricPattern30[DateIndex] = MetricPattern30(client, f'{base_path}_first_dateindex')
        self.identity: MetricPattern30[WeekIndex] = MetricPattern30(client, f'{base_path}_identity')

class CatalogTree_Indexes_Yearindex:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.decadeindex: MetricPattern31[DecadeIndex] = MetricPattern31(client, f'{base_path}_decadeindex')
        self.first_monthindex: MetricPattern31[MonthIndex] = MetricPattern31(client, f'{base_path}_first_monthindex')
        self.identity: MetricPattern31[YearIndex] = MetricPattern31(client, f'{base_path}_identity')
        self.monthindex_count: MetricPattern31[StoredU64] = MetricPattern31(client, f'{base_path}_monthindex_count')

class CatalogTree_Inputs:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.count: SizePattern[StoredU64] = SizePattern(client, 'input_count')
        self.first_txinindex: MetricPattern12[TxInIndex] = MetricPattern12(client, f'{base_path}_first_txinindex')
        self.outpoint: MetricPattern13[OutPoint] = MetricPattern13(client, f'{base_path}_outpoint')
        self.outputtype: MetricPattern13[OutputType] = MetricPattern13(client, f'{base_path}_outputtype')
        self.spent: CatalogTree_Inputs_Spent = CatalogTree_Inputs_Spent(client, f'{base_path}_spent')
        self.txindex: MetricPattern13[TxIndex] = MetricPattern13(client, f'{base_path}_txindex')
        self.typeindex: MetricPattern13[TypeIndex] = MetricPattern13(client, f'{base_path}_typeindex')
        self.witness_size: MetricPattern13[StoredU32] = MetricPattern13(client, f'{base_path}_witness_size')

class CatalogTree_Inputs_Spent:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.txoutindex: MetricPattern13[TxOutIndex] = MetricPattern13(client, f'{base_path}_txoutindex')
        self.value: MetricPattern13[Sats] = MetricPattern13(client, f'{base_path}_value')

class CatalogTree_Market:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.ath: CatalogTree_Market_Ath = CatalogTree_Market_Ath(client, f'{base_path}_ath')
        self.dca: CatalogTree_Market_Dca = CatalogTree_Market_Dca(client, f'{base_path}_dca')
        self.indicators: CatalogTree_Market_Indicators = CatalogTree_Market_Indicators(client, f'{base_path}_indicators')
        self.lookback: CatalogTree_Market_Lookback = CatalogTree_Market_Lookback(client, f'{base_path}_lookback')
        self.moving_average: CatalogTree_Market_MovingAverage = CatalogTree_Market_MovingAverage(client, f'{base_path}_moving_average')
        self.range: CatalogTree_Market_Range = CatalogTree_Market_Range(client, f'{base_path}_range')
        self.returns: CatalogTree_Market_Returns = CatalogTree_Market_Returns(client, f'{base_path}_returns')
        self.volatility: CatalogTree_Market_Volatility = CatalogTree_Market_Volatility(client, f'{base_path}_volatility')

class CatalogTree_Market_Ath:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.days_since_price_ath: MetricPattern4[StoredU16] = MetricPattern4(client, f'{base_path}_days_since_price_ath')
        self.max_days_between_price_aths: MetricPattern4[StoredU16] = MetricPattern4(client, f'{base_path}_max_days_between_price_aths')
        self.max_years_between_price_aths: MetricPattern4[StoredF32] = MetricPattern4(client, f'{base_path}_max_years_between_price_aths')
        self.price_ath: MetricPattern1[Dollars] = MetricPattern1(client, f'{base_path}_price_ath')
        self.price_drawdown: MetricPattern3[StoredF32] = MetricPattern3(client, f'{base_path}_price_drawdown')
        self.years_since_price_ath: MetricPattern4[StoredF32] = MetricPattern4(client, f'{base_path}_years_since_price_ath')

class CatalogTree_Market_Dca:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.class_average_price: ClassAveragePricePattern[Dollars] = ClassAveragePricePattern(client, 'dca_class')
        self.class_returns: ClassAveragePricePattern[StoredF32] = ClassAveragePricePattern(client, 'dca_class')
        self.class_stack: CatalogTree_Market_Dca_ClassStack = CatalogTree_Market_Dca_ClassStack(client, f'{base_path}_class_stack')
        self.period_average_price: PeriodAveragePricePattern[Dollars] = PeriodAveragePricePattern(client, 'dca_average_price')
        self.period_cagr: PeriodCagrPattern = PeriodCagrPattern(client, 'dca_cagr')
        self.period_lump_sum_stack: PeriodLumpSumStackPattern = PeriodLumpSumStackPattern(client, '')
        self.period_returns: PeriodAveragePricePattern[StoredF32] = PeriodAveragePricePattern(client, 'dca_returns')
        self.period_stack: PeriodLumpSumStackPattern = PeriodLumpSumStackPattern(client, '')

class CatalogTree_Market_Dca_ClassStack:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._2015: _24hCoinbaseSumPattern = _24hCoinbaseSumPattern(client, 'dca_class_2015_stack')
        self._2016: _24hCoinbaseSumPattern = _24hCoinbaseSumPattern(client, 'dca_class_2016_stack')
        self._2017: _24hCoinbaseSumPattern = _24hCoinbaseSumPattern(client, 'dca_class_2017_stack')
        self._2018: _24hCoinbaseSumPattern = _24hCoinbaseSumPattern(client, 'dca_class_2018_stack')
        self._2019: _24hCoinbaseSumPattern = _24hCoinbaseSumPattern(client, 'dca_class_2019_stack')
        self._2020: _24hCoinbaseSumPattern = _24hCoinbaseSumPattern(client, 'dca_class_2020_stack')
        self._2021: _24hCoinbaseSumPattern = _24hCoinbaseSumPattern(client, 'dca_class_2021_stack')
        self._2022: _24hCoinbaseSumPattern = _24hCoinbaseSumPattern(client, 'dca_class_2022_stack')
        self._2023: _24hCoinbaseSumPattern = _24hCoinbaseSumPattern(client, 'dca_class_2023_stack')
        self._2024: _24hCoinbaseSumPattern = _24hCoinbaseSumPattern(client, 'dca_class_2024_stack')
        self._2025: _24hCoinbaseSumPattern = _24hCoinbaseSumPattern(client, 'dca_class_2025_stack')

class CatalogTree_Market_Indicators:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.gini: MetricPattern7[StoredF32] = MetricPattern7(client, f'{base_path}_gini')
        self.macd_histogram: MetricPattern7[StoredF32] = MetricPattern7(client, f'{base_path}_macd_histogram')
        self.macd_line: MetricPattern7[StoredF32] = MetricPattern7(client, f'{base_path}_macd_line')
        self.macd_signal: MetricPattern7[StoredF32] = MetricPattern7(client, f'{base_path}_macd_signal')
        self.nvt: MetricPattern4[StoredF32] = MetricPattern4(client, f'{base_path}_nvt')
        self.pi_cycle: MetricPattern7[StoredF32] = MetricPattern7(client, f'{base_path}_pi_cycle')
        self.puell_multiple: MetricPattern4[StoredF32] = MetricPattern4(client, f'{base_path}_puell_multiple')
        self.rsi_14d: MetricPattern7[StoredF32] = MetricPattern7(client, f'{base_path}_rsi_14d')
        self.rsi_14d_max: MetricPattern7[StoredF32] = MetricPattern7(client, f'{base_path}_rsi_14d_max')
        self.rsi_14d_min: MetricPattern7[StoredF32] = MetricPattern7(client, f'{base_path}_rsi_14d_min')
        self.rsi_average_gain_14d: MetricPattern7[StoredF32] = MetricPattern7(client, f'{base_path}_rsi_average_gain_14d')
        self.rsi_average_loss_14d: MetricPattern7[StoredF32] = MetricPattern7(client, f'{base_path}_rsi_average_loss_14d')
        self.rsi_gains: MetricPattern7[StoredF32] = MetricPattern7(client, f'{base_path}_rsi_gains')
        self.rsi_losses: MetricPattern7[StoredF32] = MetricPattern7(client, f'{base_path}_rsi_losses')
        self.stoch_d: MetricPattern7[StoredF32] = MetricPattern7(client, f'{base_path}_stoch_d')
        self.stoch_k: MetricPattern7[StoredF32] = MetricPattern7(client, f'{base_path}_stoch_k')
        self.stoch_rsi: MetricPattern7[StoredF32] = MetricPattern7(client, f'{base_path}_stoch_rsi')
        self.stoch_rsi_d: MetricPattern7[StoredF32] = MetricPattern7(client, f'{base_path}_stoch_rsi_d')
        self.stoch_rsi_k: MetricPattern7[StoredF32] = MetricPattern7(client, f'{base_path}_stoch_rsi_k')

class CatalogTree_Market_Lookback:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.price_ago: PriceAgoPattern[Dollars] = PriceAgoPattern(client, 'price')

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
        self.price_200d_sma_x0_8: MetricPattern4[Dollars] = MetricPattern4(client, f'{base_path}_price_200d_sma_x0_8')
        self.price_200d_sma_x2_4: MetricPattern4[Dollars] = MetricPattern4(client, f'{base_path}_price_200d_sma_x2_4')
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
        self.price_350d_sma_x2: MetricPattern4[Dollars] = MetricPattern4(client, f'{base_path}_price_350d_sma_x2')
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
        self.price_1m_max: MetricPattern4[Dollars] = MetricPattern4(client, f'{base_path}_price_1m_max')
        self.price_1m_min: MetricPattern4[Dollars] = MetricPattern4(client, f'{base_path}_price_1m_min')
        self.price_1w_max: MetricPattern4[Dollars] = MetricPattern4(client, f'{base_path}_price_1w_max')
        self.price_1w_min: MetricPattern4[Dollars] = MetricPattern4(client, f'{base_path}_price_1w_min')
        self.price_1y_max: MetricPattern4[Dollars] = MetricPattern4(client, f'{base_path}_price_1y_max')
        self.price_1y_min: MetricPattern4[Dollars] = MetricPattern4(client, f'{base_path}_price_1y_min')
        self.price_2w_choppiness_index: MetricPattern4[StoredF32] = MetricPattern4(client, f'{base_path}_price_2w_choppiness_index')
        self.price_2w_max: MetricPattern4[Dollars] = MetricPattern4(client, f'{base_path}_price_2w_max')
        self.price_2w_min: MetricPattern4[Dollars] = MetricPattern4(client, f'{base_path}_price_2w_min')
        self.price_true_range: MetricPattern7[StoredF32] = MetricPattern7(client, f'{base_path}_price_true_range')
        self.price_true_range_2w_sum: MetricPattern7[StoredF32] = MetricPattern7(client, f'{base_path}_price_true_range_2w_sum')

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
        self.downside_returns: MetricPattern7[StoredF32] = MetricPattern7(client, f'{base_path}_downside_returns')
        self.price_returns: PriceAgoPattern[StoredF32] = PriceAgoPattern(client, 'price_returns')

class CatalogTree_Market_Volatility:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.price_1m_volatility: MetricPattern4[StoredF32] = MetricPattern4(client, f'{base_path}_price_1m_volatility')
        self.price_1w_volatility: MetricPattern4[StoredF32] = MetricPattern4(client, f'{base_path}_price_1w_volatility')
        self.price_1y_volatility: MetricPattern4[StoredF32] = MetricPattern4(client, f'{base_path}_price_1y_volatility')
        self.sharpe_1m: MetricPattern7[StoredF32] = MetricPattern7(client, f'{base_path}_sharpe_1m')
        self.sharpe_1w: MetricPattern7[StoredF32] = MetricPattern7(client, f'{base_path}_sharpe_1w')
        self.sharpe_1y: MetricPattern7[StoredF32] = MetricPattern7(client, f'{base_path}_sharpe_1y')
        self.sortino_1m: MetricPattern7[StoredF32] = MetricPattern7(client, f'{base_path}_sortino_1m')
        self.sortino_1w: MetricPattern7[StoredF32] = MetricPattern7(client, f'{base_path}_sortino_1w')
        self.sortino_1y: MetricPattern7[StoredF32] = MetricPattern7(client, f'{base_path}_sortino_1y')

class CatalogTree_Outputs:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.count: CatalogTree_Outputs_Count = CatalogTree_Outputs_Count(client, f'{base_path}_count')
        self.first_txoutindex: MetricPattern12[TxOutIndex] = MetricPattern12(client, f'{base_path}_first_txoutindex')
        self.outputtype: MetricPattern16[OutputType] = MetricPattern16(client, f'{base_path}_outputtype')
        self.spent: CatalogTree_Outputs_Spent = CatalogTree_Outputs_Spent(client, f'{base_path}_spent')
        self.txindex: MetricPattern16[TxIndex] = MetricPattern16(client, f'{base_path}_txindex')
        self.typeindex: MetricPattern16[TypeIndex] = MetricPattern16(client, f'{base_path}_typeindex')
        self.value: MetricPattern16[Sats] = MetricPattern16(client, f'{base_path}_value')

class CatalogTree_Outputs_Count:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.total_count: SizePattern[StoredU64] = SizePattern(client, 'output_count')
        self.utxo_count: BitcoinPattern[StoredU64] = BitcoinPattern(client, 'exact_utxo_count')

class CatalogTree_Outputs_Spent:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.txinindex: MetricPattern16[TxInIndex] = MetricPattern16(client, f'{base_path}_txinindex')

class CatalogTree_Pools:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.pool: MetricPattern12[PoolSlug] = MetricPattern12(client, f'{base_path}_pool')
        self.vecs: CatalogTree_Pools_Vecs = CatalogTree_Pools_Vecs(client, f'{base_path}_vecs')

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
        self.block_position: MetricPattern12[BlkPosition] = MetricPattern12(client, f'{base_path}_block_position')
        self.tx_position: MetricPattern28[BlkPosition] = MetricPattern28(client, f'{base_path}_tx_position')

class CatalogTree_Price:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.cents: CatalogTree_Price_Cents = CatalogTree_Price_Cents(client, f'{base_path}_cents')
        self.sats: SatsPattern = SatsPattern(client, 'price')
        self.usd: SatsPattern = SatsPattern(client, 'price')

class CatalogTree_Price_Cents:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.ohlc: MetricPattern6[OHLCCents] = MetricPattern6(client, f'{base_path}_ohlc')
        self.split: CatalogTree_Price_Cents_Split = CatalogTree_Price_Cents_Split(client, f'{base_path}_split')

class CatalogTree_Price_Cents_Split:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.dateindex: DateindexPattern2 = DateindexPattern2(client, 'price')
        self.height: DateindexPattern2 = DateindexPattern2(client, 'price')

class CatalogTree_Scripts:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.count: CatalogTree_Scripts_Count = CatalogTree_Scripts_Count(client, f'{base_path}_count')
        self.empty_to_txindex: MetricPattern10[TxIndex] = MetricPattern10(client, f'{base_path}_empty_to_txindex')
        self.first_emptyoutputindex: MetricPattern12[EmptyOutputIndex] = MetricPattern12(client, f'{base_path}_first_emptyoutputindex')
        self.first_opreturnindex: MetricPattern12[OpReturnIndex] = MetricPattern12(client, f'{base_path}_first_opreturnindex')
        self.first_p2msoutputindex: MetricPattern12[P2MSOutputIndex] = MetricPattern12(client, f'{base_path}_first_p2msoutputindex')
        self.first_unknownoutputindex: MetricPattern12[UnknownOutputIndex] = MetricPattern12(client, f'{base_path}_first_unknownoutputindex')
        self.opreturn_to_txindex: MetricPattern15[TxIndex] = MetricPattern15(client, f'{base_path}_opreturn_to_txindex')
        self.p2ms_to_txindex: MetricPattern18[TxIndex] = MetricPattern18(client, f'{base_path}_p2ms_to_txindex')
        self.unknown_to_txindex: MetricPattern29[TxIndex] = MetricPattern29(client, f'{base_path}_unknown_to_txindex')
        self.value: CatalogTree_Scripts_Value = CatalogTree_Scripts_Value(client, f'{base_path}_value')

class CatalogTree_Scripts_Count:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.emptyoutput: BitcoinPattern[StoredU64] = BitcoinPattern(client, 'emptyoutput_count')
        self.opreturn: BitcoinPattern[StoredU64] = BitcoinPattern(client, 'opreturn_count')
        self.p2a: BitcoinPattern[StoredU64] = BitcoinPattern(client, 'p2a_count')
        self.p2ms: BitcoinPattern[StoredU64] = BitcoinPattern(client, 'p2ms_count')
        self.p2pk33: BitcoinPattern[StoredU64] = BitcoinPattern(client, 'p2pk33_count')
        self.p2pk65: BitcoinPattern[StoredU64] = BitcoinPattern(client, 'p2pk65_count')
        self.p2pkh: BitcoinPattern[StoredU64] = BitcoinPattern(client, 'p2pkh_count')
        self.p2sh: BitcoinPattern[StoredU64] = BitcoinPattern(client, 'p2sh_count')
        self.p2tr: BitcoinPattern[StoredU64] = BitcoinPattern(client, 'p2tr_count')
        self.p2wpkh: BitcoinPattern[StoredU64] = BitcoinPattern(client, 'p2wpkh_count')
        self.p2wsh: BitcoinPattern[StoredU64] = BitcoinPattern(client, 'p2wsh_count')
        self.segwit: BitcoinPattern[StoredU64] = BitcoinPattern(client, 'segwit_count')
        self.segwit_adoption: SegwitAdoptionPattern = SegwitAdoptionPattern(client, 'segwit_adoption')
        self.taproot_adoption: SegwitAdoptionPattern = SegwitAdoptionPattern(client, 'taproot_adoption')
        self.unknownoutput: BitcoinPattern[StoredU64] = BitcoinPattern(client, 'unknownoutput_count')

class CatalogTree_Scripts_Value:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.opreturn: CoinbasePattern = CoinbasePattern(client, 'opreturn_value')

class CatalogTree_Supply:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.burned: CatalogTree_Supply_Burned = CatalogTree_Supply_Burned(client, f'{base_path}_burned')
        self.circulating: _24hCoinbaseSumPattern = _24hCoinbaseSumPattern(client, 'circulating_supply')
        self.inflation: MetricPattern4[StoredF32] = MetricPattern4(client, f'{base_path}_inflation')
        self.market_cap: MetricPattern1[Dollars] = MetricPattern1(client, f'{base_path}_market_cap')
        self.velocity: CatalogTree_Supply_Velocity = CatalogTree_Supply_Velocity(client, f'{base_path}_velocity')

class CatalogTree_Supply_Burned:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.opreturn: UnclaimedRewardsPattern = UnclaimedRewardsPattern(client, 'opreturn_supply')
        self.unspendable: UnclaimedRewardsPattern = UnclaimedRewardsPattern(client, 'unspendable_supply')

class CatalogTree_Supply_Velocity:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.btc: MetricPattern4[StoredF64] = MetricPattern4(client, f'{base_path}_btc')
        self.usd: MetricPattern4[StoredF64] = MetricPattern4(client, f'{base_path}_usd')

class CatalogTree_Transactions:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.base_size: MetricPattern28[StoredU32] = MetricPattern28(client, f'{base_path}_base_size')
        self.count: CatalogTree_Transactions_Count = CatalogTree_Transactions_Count(client, f'{base_path}_count')
        self.fees: CatalogTree_Transactions_Fees = CatalogTree_Transactions_Fees(client, f'{base_path}_fees')
        self.first_txindex: MetricPattern12[TxIndex] = MetricPattern12(client, f'{base_path}_first_txindex')
        self.first_txinindex: MetricPattern28[TxInIndex] = MetricPattern28(client, f'{base_path}_first_txinindex')
        self.first_txoutindex: MetricPattern28[TxOutIndex] = MetricPattern28(client, f'{base_path}_first_txoutindex')
        self.height: MetricPattern28[Height] = MetricPattern28(client, f'{base_path}_height')
        self.is_explicitly_rbf: MetricPattern28[StoredBool] = MetricPattern28(client, f'{base_path}_is_explicitly_rbf')
        self.rawlocktime: MetricPattern28[RawLockTime] = MetricPattern28(client, f'{base_path}_rawlocktime')
        self.size: CatalogTree_Transactions_Size = CatalogTree_Transactions_Size(client, f'{base_path}_size')
        self.total_size: MetricPattern28[StoredU32] = MetricPattern28(client, f'{base_path}_total_size')
        self.txid: MetricPattern28[Txid] = MetricPattern28(client, f'{base_path}_txid')
        self.txversion: MetricPattern28[TxVersion] = MetricPattern28(client, f'{base_path}_txversion')
        self.versions: CatalogTree_Transactions_Versions = CatalogTree_Transactions_Versions(client, f'{base_path}_versions')
        self.volume: CatalogTree_Transactions_Volume = CatalogTree_Transactions_Volume(client, f'{base_path}_volume')

class CatalogTree_Transactions_Count:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.is_coinbase: MetricPattern28[StoredBool] = MetricPattern28(client, f'{base_path}_is_coinbase')
        self.tx_count: BitcoinPattern[StoredU64] = BitcoinPattern(client, 'tx_count')

class CatalogTree_Transactions_Fees:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.fee: CatalogTree_Transactions_Fees_Fee = CatalogTree_Transactions_Fees_Fee(client, f'{base_path}_fee')
        self.fee_rate: FeeRatePattern[FeeRate] = FeeRatePattern(client, 'fee_rate')
        self.input_value: MetricPattern28[Sats] = MetricPattern28(client, f'{base_path}_input_value')
        self.output_value: MetricPattern28[Sats] = MetricPattern28(client, f'{base_path}_output_value')

class CatalogTree_Transactions_Fees_Fee:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.bitcoin: SizePattern[Bitcoin] = SizePattern(client, 'fee_btc')
        self.dollars: SizePattern[Dollars] = SizePattern(client, 'fee_usd')
        self.sats: SizePattern[Sats] = SizePattern(client, 'fee')
        self.txindex: MetricPattern28[Sats] = MetricPattern28(client, f'{base_path}_txindex')

class CatalogTree_Transactions_Size:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.vsize: FeeRatePattern[VSize] = FeeRatePattern(client, '')
        self.weight: FeeRatePattern[Weight] = FeeRatePattern(client, '')

class CatalogTree_Transactions_Versions:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.v1: BlockCountPattern[StoredU64] = BlockCountPattern(client, 'tx_v1')
        self.v2: BlockCountPattern[StoredU64] = BlockCountPattern(client, 'tx_v2')
        self.v3: BlockCountPattern[StoredU64] = BlockCountPattern(client, 'tx_v3')

class CatalogTree_Transactions_Volume:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.annualized_volume: _24hCoinbaseSumPattern = _24hCoinbaseSumPattern(client, 'annualized_volume')
        self.inputs_per_sec: MetricPattern4[StoredF32] = MetricPattern4(client, f'{base_path}_inputs_per_sec')
        self.outputs_per_sec: MetricPattern4[StoredF32] = MetricPattern4(client, f'{base_path}_outputs_per_sec')
        self.sent_sum: _24hCoinbaseSumPattern = _24hCoinbaseSumPattern(client, 'sent_sum')
        self.tx_per_sec: MetricPattern4[StoredF32] = MetricPattern4(client, f'{base_path}_tx_per_sec')

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

    def get_address_txs(self, address: Address, after_txid: Optional[str] = None, limit: Optional[int] = None) -> List[Txid]:
        """Address transaction IDs.

        Get transaction IDs for an address, newest first. Use after_txid for pagination."""
        params = []
        if after_txid is not None: params.append(f'after_txid={after_txid}')
        if limit is not None: params.append(f'limit={limit}')
        query = '&'.join(params)
        return self.get(f'/api/address/{address}/txs{"?" + query if query else ""}')

    def get_address_txs_chain(self, address: Address, after_txid: Optional[str] = None, limit: Optional[int] = None) -> List[Txid]:
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

    def get_block_by_hash_raw(self, hash: BlockHash) -> List[int]:
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

    def get_v1_mining_reward_stats_by_block_count(self, block_count: int) -> RewardStats:
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

