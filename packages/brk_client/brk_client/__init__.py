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
    isvalid: bool
    address: Optional[str]
    scriptPubKey: Optional[str]
    isscript: Optional[bool]
    iswitness: Optional[bool]
    witness_version: Optional[int]
    witness_program: Optional[str]

AnyAddressIndex = TypeIndex
Bitcoin = float
BlkPosition = int
class BlockCountParam(TypedDict):
    block_count: int

Height = int
Timestamp = int
class BlockFeesEntry(TypedDict):
    avgHeight: Height
    timestamp: Timestamp
    avgFees: Sats

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
    id: BlockHash
    height: Height
    tx_count: int
    size: int
    weight: Weight
    timestamp: Timestamp
    difficulty: float

class BlockRewardsEntry(TypedDict):
    avgHeight: int
    timestamp: int
    avgRewards: int

class BlockSizeEntry(TypedDict):
    avgHeight: int
    timestamp: int
    avgSize: int

class BlockWeightEntry(TypedDict):
    avgHeight: int
    timestamp: int
    avgWeight: int

class BlockSizesWeights(TypedDict):
    sizes: List[BlockSizeEntry]
    weights: List[BlockWeightEntry]

class BlockStatus(TypedDict):
    in_best_chain: bool
    height: Union[Height, None]
    next_best: Union[BlockHash, None]

class BlockTimestamp(TypedDict):
    height: Height
    hash: BlockHash
    timestamp: str

Cents = int
Close = Cents
Format = Literal["json", "csv"]
class DataRangeFormat(TypedDict):
    from_: Optional[int]
    to: Optional[int]
    count: Optional[int]
    format: Format

Date = int
DateIndex = int
DecadeIndex = int
class DifficultyAdjustment(TypedDict):
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
    timestamp: Timestamp
    height: Height
    difficulty: float
    change_percent: float

class DifficultyEntry(TypedDict):
    timestamp: Timestamp
    difficulty: float
    height: Height

DifficultyEpoch = int
Dollars = float
class EmptyAddressData(TypedDict):
    tx_count: int
    funded_txo_count: int
    transfered: Sats

EmptyAddressIndex = TypeIndex
EmptyOutputIndex = TypeIndex
FeeRate = float
HalvingEpoch = int
class HashrateEntry(TypedDict):
    timestamp: Timestamp
    avgHashrate: int

class HashrateSummary(TypedDict):
    hashrates: List[HashrateEntry]
    difficulty: List[DifficultyEntry]
    currentHashrate: int
    currentDifficulty: float

class Health(TypedDict):
    status: str
    service: str
    timestamp: str

class HeightParam(TypedDict):
    height: Height

Hex = str
High = Cents
class IndexInfo(TypedDict):
    index: Index
    aliases: List[str]

Limit = int
class LimitParam(TypedDict):
    limit: Limit

class LoadedAddressData(TypedDict):
    tx_count: int
    funded_txo_count: int
    spent_txo_count: int
    received: Sats
    sent: Sats
    realized_cap: Dollars

LoadedAddressIndex = TypeIndex
Low = Cents
class MempoolBlock(TypedDict):
    blockSize: int
    blockVSize: float
    nTx: int
    totalFees: Sats
    medianFee: FeeRate
    feeRange: List[FeeRate]

VSize = int
class MempoolInfo(TypedDict):
    count: int
    vsize: VSize
    total_fee: Sats

Metric = str
class MetricCount(TypedDict):
    distinct_metrics: int
    total_endpoints: int
    lazy_endpoints: int
    stored_endpoints: int

class MetricData(TypedDict):
    total: int
    from_: int
    to: int
    data: List[Any]

class MetricParam(TypedDict):
    metric: Metric

Metrics = str
class MetricSelection(TypedDict):
    metrics: Metrics
    index: Index
    from_: Optional[int]
    to: Optional[int]
    count: Optional[int]
    format: Format

class MetricSelectionLegacy(TypedDict):
    index: Index
    ids: Metrics
    from_: Optional[int]
    to: Optional[int]
    count: Optional[int]
    format: Format

class MetricWithIndex(TypedDict):
    metric: Metric
    index: Index

MonthIndex = int
Open = Cents
class OHLCCents(TypedDict):
    open: Open
    high: High
    low: Low
    close: Close

class OHLCDollars(TypedDict):
    open: Open
    high: High
    low: Low
    close: Close

class OHLCSats(TypedDict):
    open: Open
    high: High
    low: Low
    close: Close

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
    all: int
    _24h: int
    _1w: int

class PoolBlockShares(TypedDict):
    all: float
    _24h: float
    _1w: float

PoolSlug = Literal["unknown", "blockfills", "ultimuspool", "terrapool", "luxor", "onethash", "btccom", "bitfarms", "huobipool", "wayicn", "canoepool", "btctop", "bitcoincom", "pool175btc", "gbminers", "axbt", "asicminer", "bitminter", "bitcoinrussia", "btcserv", "simplecoinus", "btcguild", "eligius", "ozcoin", "eclipsemc", "maxbtc", "triplemining", "coinlab", "pool50btc", "ghashio", "stminingcorp", "bitparking", "mmpool", "polmine", "kncminer", "bitalo", "f2pool", "hhtt", "megabigpower", "mtred", "nmcbit", "yourbtcnet", "givemecoins", "braiinspool", "antpool", "multicoinco", "bcpoolio", "cointerra", "kanopool", "solock", "ckpool", "nicehash", "bitclub", "bitcoinaffiliatenetwork", "btcc", "bwpool", "exxbw", "bitsolo", "bitfury", "twentyoneinc", "digitalbtc", "eightbaochi", "mybtccoinpool", "tbdice", "hashpool", "nexious", "bravomining", "hotpool", "okexpool", "bcmonster", "onehash", "bixin", "tatmaspool", "viabtc", "connectbtc", "batpool", "waterhole", "dcexploration", "dcex", "btpool", "fiftyeightcoin", "bitcoinindia", "shawnp0wers", "phashio", "rigpool", "haozhuzhu", "sevenpool", "miningkings", "hashbx", "dpool", "rawpool", "haominer", "helix", "bitcoinukraine", "poolin", "secretsuperstar", "tigerpoolnet", "sigmapoolcom", "okpooltop", "hummerpool", "tangpool", "bytepool", "spiderpool", "novablock", "miningcity", "binancepool", "minerium", "lubiancom", "okkong", "aaopool", "emcdpool", "foundryusa", "sbicrypto", "arkpool", "purebtccom", "marapool", "kucoinpool", "entrustcharitypool", "okminer", "titan", "pegapool", "btcnuggets", "cloudhashing", "digitalxmintsy", "telco214", "btcpoolparty", "multipool", "transactioncoinmining", "btcdig", "trickysbtcpool", "btcmp", "eobot", "unomp", "patels", "gogreenlight", "ekanembtc", "canoe", "tiger", "onem1x", "zulupool", "secpool", "ocean", "whitepool", "wk057", "futurebitapollosolo", "carbonnegative", "portlandhodl", "phoenix", "neopool", "maxipool", "bitfufupool", "luckypool", "miningdutch", "publicpool", "miningsquared", "innopolistech", "btclab", "parasite"]
class PoolDetailInfo(TypedDict):
    id: int
    name: str
    link: str
    addresses: List[str]
    regexes: List[str]
    slug: PoolSlug

class PoolDetail(TypedDict):
    pool: PoolDetailInfo
    blockCount: PoolBlockCounts
    blockShare: PoolBlockShares
    estimatedHashrate: int
    reportedHashrate: Optional[int]

class PoolInfo(TypedDict):
    name: str
    slug: PoolSlug
    unique_id: int

class PoolSlugParam(TypedDict):
    slug: PoolSlug

class PoolStats(TypedDict):
    poolId: int
    name: str
    link: str
    blockCount: int
    rank: int
    emptyBlocks: int
    slug: PoolSlug
    share: float

class PoolsSummary(TypedDict):
    pools: List[PoolStats]
    blockCount: int
    lastEstimatedHashrate: int

QuarterIndex = int
RawLockTime = int
class RecommendedFees(TypedDict):
    fastestFee: FeeRate
    halfHourFee: FeeRate
    hourFee: FeeRate
    economyFee: FeeRate
    minimumFee: FeeRate

class RewardStats(TypedDict):
    startBlock: Height
    endBlock: Height
    totalReward: Sats
    totalFee: Sats
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
    txid: Txid
    vout: Vout
    prevout: Union[TxOut, None]
    scriptsig: str
    scriptsig_asm: str
    is_coinbase: bool
    sequence: int
    inner_redeemscript_asm: Optional[str]

class TxStatus(TypedDict):
    confirmed: bool
    block_height: Union[Height, None]
    block_hash: Union[BlockHash, None]
    block_time: Union[Timestamp, None]

TxVersion = int
class Transaction(TypedDict):
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

TxInIndex = int
TxOutIndex = int
Vin = int
class TxOutspend(TypedDict):
    spent: bool
    txid: Union[Txid, None]
    vin: Union[Vin, None]
    status: Union[TxStatus, None]

class TxidParam(TypedDict):
    txid: Txid

class TxidVout(TypedDict):
    txid: Txid
    vout: Vout

UnknownOutputIndex = TypeIndex
class Utxo(TypedDict):
    txid: Txid
    vout: Vout
    status: TxStatus
    value: Sats

class ValidateAddressParam(TypedDict):
    address: str

WeekIndex = int
YearIndex = int
Index = Literal["dateindex", "decadeindex", "difficultyepoch", "emptyoutputindex", "halvingepoch", "height", "txinindex", "monthindex", "opreturnindex", "txoutindex", "p2aaddressindex", "p2msoutputindex", "p2pk33addressindex", "p2pk65addressindex", "p2pkhaddressindex", "p2shaddressindex", "p2traddressindex", "p2wpkhaddressindex", "p2wshaddressindex", "quarterindex", "semesterindex", "txindex", "unknownoutputindex", "weekindex", "yearindex", "loadedaddressindex", "emptyaddressindex"]
class MetricLeafWithSchema(TypedDict):
    name: str
    value_type: str
    indexes: List[Index]

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


class Endpoint(Generic[T]):
    """An endpoint for a specific metric + index combination."""

    def __init__(self, client: BrkClientBase, name: str, index: str):
        self._client = client
        self._name = name
        self._index = index

    def get(self) -> List[T]:
        """Fetch all data points for this metric/index."""
        return self._client.get(self.path())

    def range(self, from_val: Optional[int] = None, to_val: Optional[int] = None) -> List[T]:
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


class MetricPattern(Protocol[T]):
    """Protocol for metric patterns with different index sets."""

    @property
    def name(self) -> str:
        """Get the metric name."""
        ...

    def indexes(self) -> List[str]:
        """Get the list of available indexes for this metric."""
        ...

    def get(self, index: str) -> Optional[Endpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        ...


# Index accessor classes

class _MetricPattern1By(Generic[T]):
    """Index endpoint methods container."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def by_dateindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'dateindex')

    def by_decadeindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'decadeindex')

    def by_difficultyepoch(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'difficultyepoch')

    def by_height(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'height')

    def by_monthindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'monthindex')

    def by_quarterindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'quarterindex')

    def by_semesterindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'semesterindex')

    def by_weekindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'weekindex')

    def by_yearindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'yearindex')

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

    def get(self, index: str) -> Optional[Endpoint[T]]:
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

    def by_dateindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'dateindex')

    def by_decadeindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'decadeindex')

    def by_difficultyepoch(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'difficultyepoch')

    def by_monthindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'monthindex')

    def by_quarterindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'quarterindex')

    def by_semesterindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'semesterindex')

    def by_weekindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'weekindex')

    def by_yearindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'yearindex')

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

    def get(self, index: str) -> Optional[Endpoint[T]]:
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

    def by_dateindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'dateindex')

    def by_decadeindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'decadeindex')

    def by_height(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'height')

    def by_monthindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'monthindex')

    def by_quarterindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'quarterindex')

    def by_semesterindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'semesterindex')

    def by_weekindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'weekindex')

    def by_yearindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'yearindex')

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

    def get(self, index: str) -> Optional[Endpoint[T]]:
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

    def by_dateindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'dateindex')

    def by_decadeindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'decadeindex')

    def by_monthindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'monthindex')

    def by_quarterindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'quarterindex')

    def by_semesterindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'semesterindex')

    def by_weekindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'weekindex')

    def by_yearindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'yearindex')

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

    def get(self, index: str) -> Optional[Endpoint[T]]:
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

    def by_decadeindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'decadeindex')

    def by_height(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'height')

    def by_monthindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'monthindex')

    def by_quarterindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'quarterindex')

    def by_semesterindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'semesterindex')

    def by_weekindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'weekindex')

    def by_yearindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'yearindex')

class MetricPattern5(Generic[T]):
    """Index accessor for metrics with 7 indexes."""
    
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
        return ['decadeindex', 'height', 'monthindex', 'quarterindex', 'semesterindex', 'weekindex', 'yearindex']

    def get(self, index: str) -> Optional[Endpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'decadeindex': return self.by.by_decadeindex()
        elif index == 'height': return self.by.by_height()
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

    def by_decadeindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'decadeindex')

    def by_monthindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'monthindex')

    def by_quarterindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'quarterindex')

    def by_semesterindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'semesterindex')

    def by_weekindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'weekindex')

    def by_yearindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'yearindex')

class MetricPattern6(Generic[T]):
    """Index accessor for metrics with 6 indexes."""
    
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
        return ['decadeindex', 'monthindex', 'quarterindex', 'semesterindex', 'weekindex', 'yearindex']

    def get(self, index: str) -> Optional[Endpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'decadeindex': return self.by.by_decadeindex()
        elif index == 'monthindex': return self.by.by_monthindex()
        elif index == 'quarterindex': return self.by.by_quarterindex()
        elif index == 'semesterindex': return self.by.by_semesterindex()
        elif index == 'weekindex': return self.by.by_weekindex()
        elif index == 'yearindex': return self.by.by_yearindex()
        return None

class _MetricPattern7By(Generic[T]):
    """Index endpoint methods container."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def by_emptyoutputindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'emptyoutputindex')

    def by_opreturnindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'opreturnindex')

    def by_p2msoutputindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'p2msoutputindex')

    def by_unknownoutputindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'unknownoutputindex')

class MetricPattern7(Generic[T]):
    """Index accessor for metrics with 4 indexes."""
    
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
        return ['emptyoutputindex', 'opreturnindex', 'p2msoutputindex', 'unknownoutputindex']

    def get(self, index: str) -> Optional[Endpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'emptyoutputindex': return self.by.by_emptyoutputindex()
        elif index == 'opreturnindex': return self.by.by_opreturnindex()
        elif index == 'p2msoutputindex': return self.by.by_p2msoutputindex()
        elif index == 'unknownoutputindex': return self.by.by_unknownoutputindex()
        return None

class _MetricPattern8By(Generic[T]):
    """Index endpoint methods container."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def by_quarterindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'quarterindex')

    def by_semesterindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'semesterindex')

    def by_yearindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'yearindex')

class MetricPattern8(Generic[T]):
    """Index accessor for metrics with 3 indexes."""
    
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
        return ['quarterindex', 'semesterindex', 'yearindex']

    def get(self, index: str) -> Optional[Endpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'quarterindex': return self.by.by_quarterindex()
        elif index == 'semesterindex': return self.by.by_semesterindex()
        elif index == 'yearindex': return self.by.by_yearindex()
        return None

class _MetricPattern9By(Generic[T]):
    """Index endpoint methods container."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def by_dateindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'dateindex')

    def by_height(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'height')

class MetricPattern9(Generic[T]):
    """Index accessor for metrics with 2 indexes."""
    
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
        return ['dateindex', 'height']

    def get(self, index: str) -> Optional[Endpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'dateindex': return self.by.by_dateindex()
        elif index == 'height': return self.by.by_height()
        return None

class _MetricPattern10By(Generic[T]):
    """Index endpoint methods container."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def by_dateindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'dateindex')

    def by_monthindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'monthindex')

class MetricPattern10(Generic[T]):
    """Index accessor for metrics with 2 indexes."""
    
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
        return ['dateindex', 'monthindex']

    def get(self, index: str) -> Optional[Endpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'dateindex': return self.by.by_dateindex()
        elif index == 'monthindex': return self.by.by_monthindex()
        return None

class _MetricPattern11By(Generic[T]):
    """Index endpoint methods container."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def by_dateindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'dateindex')

    def by_weekindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'weekindex')

class MetricPattern11(Generic[T]):
    """Index accessor for metrics with 2 indexes."""
    
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
        return ['dateindex', 'weekindex']

    def get(self, index: str) -> Optional[Endpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'dateindex': return self.by.by_dateindex()
        elif index == 'weekindex': return self.by.by_weekindex()
        return None

class _MetricPattern12By(Generic[T]):
    """Index endpoint methods container."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def by_decadeindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'decadeindex')

    def by_yearindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'yearindex')

class MetricPattern12(Generic[T]):
    """Index accessor for metrics with 2 indexes."""
    
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
        return ['decadeindex', 'yearindex']

    def get(self, index: str) -> Optional[Endpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'decadeindex': return self.by.by_decadeindex()
        elif index == 'yearindex': return self.by.by_yearindex()
        return None

class _MetricPattern13By(Generic[T]):
    """Index endpoint methods container."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def by_difficultyepoch(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'difficultyepoch')

    def by_halvingepoch(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'halvingepoch')

class MetricPattern13(Generic[T]):
    """Index accessor for metrics with 2 indexes."""
    
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
        return ['difficultyepoch', 'halvingepoch']

    def get(self, index: str) -> Optional[Endpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'difficultyepoch': return self.by.by_difficultyepoch()
        elif index == 'halvingepoch': return self.by.by_halvingepoch()
        return None

class _MetricPattern14By(Generic[T]):
    """Index endpoint methods container."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def by_difficultyepoch(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'difficultyepoch')

    def by_height(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'height')

class MetricPattern14(Generic[T]):
    """Index accessor for metrics with 2 indexes."""
    
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
        return ['difficultyepoch', 'height']

    def get(self, index: str) -> Optional[Endpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'difficultyepoch': return self.by.by_difficultyepoch()
        elif index == 'height': return self.by.by_height()
        return None

class _MetricPattern15By(Generic[T]):
    """Index endpoint methods container."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def by_halvingepoch(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'halvingepoch')

    def by_height(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'height')

class MetricPattern15(Generic[T]):
    """Index accessor for metrics with 2 indexes."""
    
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
        return ['halvingepoch', 'height']

    def get(self, index: str) -> Optional[Endpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'halvingepoch': return self.by.by_halvingepoch()
        elif index == 'height': return self.by.by_height()
        return None

class _MetricPattern16By(Generic[T]):
    """Index endpoint methods container."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def by_height(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'height')

    def by_txindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'txindex')

class MetricPattern16(Generic[T]):
    """Index accessor for metrics with 2 indexes."""
    
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
        return ['height', 'txindex']

    def get(self, index: str) -> Optional[Endpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'height': return self.by.by_height()
        elif index == 'txindex': return self.by.by_txindex()
        return None

class _MetricPattern17By(Generic[T]):
    """Index endpoint methods container."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def by_monthindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'monthindex')

    def by_quarterindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'quarterindex')

class MetricPattern17(Generic[T]):
    """Index accessor for metrics with 2 indexes."""
    
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
        return ['monthindex', 'quarterindex']

    def get(self, index: str) -> Optional[Endpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'monthindex': return self.by.by_monthindex()
        elif index == 'quarterindex': return self.by.by_quarterindex()
        return None

class _MetricPattern18By(Generic[T]):
    """Index endpoint methods container."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def by_monthindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'monthindex')

    def by_semesterindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'semesterindex')

class MetricPattern18(Generic[T]):
    """Index accessor for metrics with 2 indexes."""
    
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
        return ['monthindex', 'semesterindex']

    def get(self, index: str) -> Optional[Endpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'monthindex': return self.by.by_monthindex()
        elif index == 'semesterindex': return self.by.by_semesterindex()
        return None

class _MetricPattern19By(Generic[T]):
    """Index endpoint methods container."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def by_monthindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'monthindex')

    def by_weekindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'weekindex')

class MetricPattern19(Generic[T]):
    """Index accessor for metrics with 2 indexes."""
    
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
        return ['monthindex', 'weekindex']

    def get(self, index: str) -> Optional[Endpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'monthindex': return self.by.by_monthindex()
        elif index == 'weekindex': return self.by.by_weekindex()
        return None

class _MetricPattern20By(Generic[T]):
    """Index endpoint methods container."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def by_monthindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'monthindex')

    def by_yearindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'yearindex')

class MetricPattern20(Generic[T]):
    """Index accessor for metrics with 2 indexes."""
    
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
        return ['monthindex', 'yearindex']

    def get(self, index: str) -> Optional[Endpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'monthindex': return self.by.by_monthindex()
        elif index == 'yearindex': return self.by.by_yearindex()
        return None

class _MetricPattern21By(Generic[T]):
    """Index endpoint methods container."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def by_dateindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'dateindex')

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
        return ['dateindex']

    def get(self, index: str) -> Optional[Endpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'dateindex': return self.by.by_dateindex()
        return None

class _MetricPattern22By(Generic[T]):
    """Index endpoint methods container."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def by_decadeindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'decadeindex')

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
        return ['decadeindex']

    def get(self, index: str) -> Optional[Endpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'decadeindex': return self.by.by_decadeindex()
        return None

class _MetricPattern23By(Generic[T]):
    """Index endpoint methods container."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def by_difficultyepoch(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'difficultyepoch')

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
        return ['difficultyepoch']

    def get(self, index: str) -> Optional[Endpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'difficultyepoch': return self.by.by_difficultyepoch()
        return None

class _MetricPattern24By(Generic[T]):
    """Index endpoint methods container."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def by_emptyoutputindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'emptyoutputindex')

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
        return ['emptyoutputindex']

    def get(self, index: str) -> Optional[Endpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'emptyoutputindex': return self.by.by_emptyoutputindex()
        return None

class _MetricPattern25By(Generic[T]):
    """Index endpoint methods container."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def by_height(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'height')

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
        return ['height']

    def get(self, index: str) -> Optional[Endpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'height': return self.by.by_height()
        return None

class _MetricPattern26By(Generic[T]):
    """Index endpoint methods container."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def by_txinindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'txinindex')

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
        return ['txinindex']

    def get(self, index: str) -> Optional[Endpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'txinindex': return self.by.by_txinindex()
        return None

class _MetricPattern27By(Generic[T]):
    """Index endpoint methods container."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def by_opreturnindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'opreturnindex')

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
        return ['opreturnindex']

    def get(self, index: str) -> Optional[Endpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'opreturnindex': return self.by.by_opreturnindex()
        return None

class _MetricPattern28By(Generic[T]):
    """Index endpoint methods container."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def by_txoutindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'txoutindex')

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
        return ['txoutindex']

    def get(self, index: str) -> Optional[Endpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'txoutindex': return self.by.by_txoutindex()
        return None

class _MetricPattern29By(Generic[T]):
    """Index endpoint methods container."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def by_p2aaddressindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'p2aaddressindex')

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
        return ['p2aaddressindex']

    def get(self, index: str) -> Optional[Endpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'p2aaddressindex': return self.by.by_p2aaddressindex()
        return None

class _MetricPattern30By(Generic[T]):
    """Index endpoint methods container."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def by_p2msoutputindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'p2msoutputindex')

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
        return ['p2msoutputindex']

    def get(self, index: str) -> Optional[Endpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'p2msoutputindex': return self.by.by_p2msoutputindex()
        return None

class _MetricPattern31By(Generic[T]):
    """Index endpoint methods container."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def by_p2pk33addressindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'p2pk33addressindex')

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
        return ['p2pk33addressindex']

    def get(self, index: str) -> Optional[Endpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'p2pk33addressindex': return self.by.by_p2pk33addressindex()
        return None

class _MetricPattern32By(Generic[T]):
    """Index endpoint methods container."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def by_p2pk65addressindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'p2pk65addressindex')

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
        return ['p2pk65addressindex']

    def get(self, index: str) -> Optional[Endpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'p2pk65addressindex': return self.by.by_p2pk65addressindex()
        return None

class _MetricPattern33By(Generic[T]):
    """Index endpoint methods container."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def by_p2pkhaddressindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'p2pkhaddressindex')

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
        return ['p2pkhaddressindex']

    def get(self, index: str) -> Optional[Endpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'p2pkhaddressindex': return self.by.by_p2pkhaddressindex()
        return None

class _MetricPattern34By(Generic[T]):
    """Index endpoint methods container."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def by_p2shaddressindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'p2shaddressindex')

class MetricPattern34(Generic[T]):
    """Index accessor for metrics with 1 indexes."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name
        self.by: _MetricPattern34By[T] = _MetricPattern34By(client, name)

    @property
    def name(self) -> str:
        """Get the metric name."""
        return self._name

    def indexes(self) -> List[str]:
        """Get the list of available indexes."""
        return ['p2shaddressindex']

    def get(self, index: str) -> Optional[Endpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'p2shaddressindex': return self.by.by_p2shaddressindex()
        return None

class _MetricPattern35By(Generic[T]):
    """Index endpoint methods container."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def by_p2traddressindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'p2traddressindex')

class MetricPattern35(Generic[T]):
    """Index accessor for metrics with 1 indexes."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name
        self.by: _MetricPattern35By[T] = _MetricPattern35By(client, name)

    @property
    def name(self) -> str:
        """Get the metric name."""
        return self._name

    def indexes(self) -> List[str]:
        """Get the list of available indexes."""
        return ['p2traddressindex']

    def get(self, index: str) -> Optional[Endpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'p2traddressindex': return self.by.by_p2traddressindex()
        return None

class _MetricPattern36By(Generic[T]):
    """Index endpoint methods container."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def by_p2wpkhaddressindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'p2wpkhaddressindex')

class MetricPattern36(Generic[T]):
    """Index accessor for metrics with 1 indexes."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name
        self.by: _MetricPattern36By[T] = _MetricPattern36By(client, name)

    @property
    def name(self) -> str:
        """Get the metric name."""
        return self._name

    def indexes(self) -> List[str]:
        """Get the list of available indexes."""
        return ['p2wpkhaddressindex']

    def get(self, index: str) -> Optional[Endpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'p2wpkhaddressindex': return self.by.by_p2wpkhaddressindex()
        return None

class _MetricPattern37By(Generic[T]):
    """Index endpoint methods container."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def by_p2wshaddressindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'p2wshaddressindex')

class MetricPattern37(Generic[T]):
    """Index accessor for metrics with 1 indexes."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name
        self.by: _MetricPattern37By[T] = _MetricPattern37By(client, name)

    @property
    def name(self) -> str:
        """Get the metric name."""
        return self._name

    def indexes(self) -> List[str]:
        """Get the list of available indexes."""
        return ['p2wshaddressindex']

    def get(self, index: str) -> Optional[Endpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'p2wshaddressindex': return self.by.by_p2wshaddressindex()
        return None

class _MetricPattern38By(Generic[T]):
    """Index endpoint methods container."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def by_txindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'txindex')

class MetricPattern38(Generic[T]):
    """Index accessor for metrics with 1 indexes."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name
        self.by: _MetricPattern38By[T] = _MetricPattern38By(client, name)

    @property
    def name(self) -> str:
        """Get the metric name."""
        return self._name

    def indexes(self) -> List[str]:
        """Get the list of available indexes."""
        return ['txindex']

    def get(self, index: str) -> Optional[Endpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'txindex': return self.by.by_txindex()
        return None

class _MetricPattern39By(Generic[T]):
    """Index endpoint methods container."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def by_unknownoutputindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'unknownoutputindex')

class MetricPattern39(Generic[T]):
    """Index accessor for metrics with 1 indexes."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name
        self.by: _MetricPattern39By[T] = _MetricPattern39By(client, name)

    @property
    def name(self) -> str:
        """Get the metric name."""
        return self._name

    def indexes(self) -> List[str]:
        """Get the list of available indexes."""
        return ['unknownoutputindex']

    def get(self, index: str) -> Optional[Endpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'unknownoutputindex': return self.by.by_unknownoutputindex()
        return None

class _MetricPattern40By(Generic[T]):
    """Index endpoint methods container."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def by_loadedaddressindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'loadedaddressindex')

class MetricPattern40(Generic[T]):
    """Index accessor for metrics with 1 indexes."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name
        self.by: _MetricPattern40By[T] = _MetricPattern40By(client, name)

    @property
    def name(self) -> str:
        """Get the metric name."""
        return self._name

    def indexes(self) -> List[str]:
        """Get the list of available indexes."""
        return ['loadedaddressindex']

    def get(self, index: str) -> Optional[Endpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'loadedaddressindex': return self.by.by_loadedaddressindex()
        return None

class _MetricPattern41By(Generic[T]):
    """Index endpoint methods container."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def by_emptyaddressindex(self) -> Endpoint[T]:
        return Endpoint(self._client, self._name, 'emptyaddressindex')

class MetricPattern41(Generic[T]):
    """Index accessor for metrics with 1 indexes."""
    
    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name
        self.by: _MetricPattern41By[T] = _MetricPattern41By(client, name)

    @property
    def name(self) -> str:
        """Get the metric name."""
        return self._name

    def indexes(self) -> List[str]:
        """Get the list of available indexes."""
        return ['emptyaddressindex']

    def get(self, index: str) -> Optional[Endpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'emptyaddressindex': return self.by.by_emptyaddressindex()
        return None

# Reusable structural pattern classes

class RealizedPattern3:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.adjusted_sopr: MetricPattern21[StoredF64] = MetricPattern21(client, _m(acc, 'adjusted_sopr'))
        self.adjusted_sopr_30d_ema: MetricPattern21[StoredF64] = MetricPattern21(client, _m(acc, 'adjusted_sopr_30d_ema'))
        self.adjusted_sopr_7d_ema: MetricPattern21[StoredF64] = MetricPattern21(client, _m(acc, 'adjusted_sopr_7d_ema'))
        self.adjusted_value_created: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'adjusted_value_created'))
        self.adjusted_value_destroyed: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'adjusted_value_destroyed'))
        self.mvrv: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'mvrv'))
        self.neg_realized_loss: BlockCountPattern[Dollars] = BlockCountPattern(client, _m(acc, 'neg_realized_loss'))
        self.net_realized_pnl: BlockCountPattern[Dollars] = BlockCountPattern(client, _m(acc, 'net_realized_pnl'))
        self.net_realized_pnl_cumulative_30d_delta: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta'))
        self.net_realized_pnl_cumulative_30d_delta_rel_to_market_cap: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta_rel_to_market_cap'))
        self.net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap'))
        self.net_realized_pnl_rel_to_realized_cap: MetricPattern25[StoredF32] = MetricPattern25(client, _m(acc, 'net_realized_pnl_rel_to_realized_cap'))
        self.realized_cap: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_cap'))
        self.realized_cap_30d_delta: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'realized_cap_30d_delta'))
        self.realized_cap_rel_to_own_market_cap: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'realized_cap_rel_to_own_market_cap'))
        self.realized_loss: BlockCountPattern[Dollars] = BlockCountPattern(client, _m(acc, 'realized_loss'))
        self.realized_loss_rel_to_realized_cap: MetricPattern25[StoredF32] = MetricPattern25(client, _m(acc, 'realized_loss_rel_to_realized_cap'))
        self.realized_price: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_price'))
        self.realized_price_extra: ActivePriceRatioPattern = ActivePriceRatioPattern(client, _m(acc, 'realized_price_ratio'))
        self.realized_profit: BlockCountPattern[Dollars] = BlockCountPattern(client, _m(acc, 'realized_profit'))
        self.realized_profit_rel_to_realized_cap: MetricPattern25[StoredF32] = MetricPattern25(client, _m(acc, 'realized_profit_rel_to_realized_cap'))
        self.realized_profit_to_loss_ratio: MetricPattern21[StoredF64] = MetricPattern21(client, _m(acc, 'realized_profit_to_loss_ratio'))
        self.realized_value: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_value'))
        self.sell_side_risk_ratio: MetricPattern21[StoredF32] = MetricPattern21(client, _m(acc, 'sell_side_risk_ratio'))
        self.sell_side_risk_ratio_30d_ema: MetricPattern21[StoredF32] = MetricPattern21(client, _m(acc, 'sell_side_risk_ratio_30d_ema'))
        self.sell_side_risk_ratio_7d_ema: MetricPattern21[StoredF32] = MetricPattern21(client, _m(acc, 'sell_side_risk_ratio_7d_ema'))
        self.sopr: MetricPattern21[StoredF64] = MetricPattern21(client, _m(acc, 'sopr'))
        self.sopr_30d_ema: MetricPattern21[StoredF64] = MetricPattern21(client, _m(acc, 'sopr_30d_ema'))
        self.sopr_7d_ema: MetricPattern21[StoredF64] = MetricPattern21(client, _m(acc, 'sopr_7d_ema'))
        self.total_realized_pnl: TotalRealizedPnlPattern[Dollars] = TotalRealizedPnlPattern(client, _m(acc, 'total_realized_pnl'))
        self.value_created: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'value_created'))
        self.value_destroyed: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'value_destroyed'))

class RealizedPattern4:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.adjusted_sopr: MetricPattern21[StoredF64] = MetricPattern21(client, _m(acc, 'adjusted_sopr'))
        self.adjusted_sopr_30d_ema: MetricPattern21[StoredF64] = MetricPattern21(client, _m(acc, 'adjusted_sopr_30d_ema'))
        self.adjusted_sopr_7d_ema: MetricPattern21[StoredF64] = MetricPattern21(client, _m(acc, 'adjusted_sopr_7d_ema'))
        self.adjusted_value_created: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'adjusted_value_created'))
        self.adjusted_value_destroyed: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'adjusted_value_destroyed'))
        self.mvrv: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'mvrv'))
        self.neg_realized_loss: BlockCountPattern[Dollars] = BlockCountPattern(client, _m(acc, 'neg_realized_loss'))
        self.net_realized_pnl: BlockCountPattern[Dollars] = BlockCountPattern(client, _m(acc, 'net_realized_pnl'))
        self.net_realized_pnl_cumulative_30d_delta: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta'))
        self.net_realized_pnl_cumulative_30d_delta_rel_to_market_cap: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta_rel_to_market_cap'))
        self.net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap'))
        self.net_realized_pnl_rel_to_realized_cap: MetricPattern25[StoredF32] = MetricPattern25(client, _m(acc, 'net_realized_pnl_rel_to_realized_cap'))
        self.realized_cap: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_cap'))
        self.realized_cap_30d_delta: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'realized_cap_30d_delta'))
        self.realized_loss: BlockCountPattern[Dollars] = BlockCountPattern(client, _m(acc, 'realized_loss'))
        self.realized_loss_rel_to_realized_cap: MetricPattern25[StoredF32] = MetricPattern25(client, _m(acc, 'realized_loss_rel_to_realized_cap'))
        self.realized_price: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_price'))
        self.realized_price_extra: RealizedPriceExtraPattern = RealizedPriceExtraPattern(client, _m(acc, 'realized_price'))
        self.realized_profit: BlockCountPattern[Dollars] = BlockCountPattern(client, _m(acc, 'realized_profit'))
        self.realized_profit_rel_to_realized_cap: MetricPattern25[StoredF32] = MetricPattern25(client, _m(acc, 'realized_profit_rel_to_realized_cap'))
        self.realized_value: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_value'))
        self.sell_side_risk_ratio: MetricPattern21[StoredF32] = MetricPattern21(client, _m(acc, 'sell_side_risk_ratio'))
        self.sell_side_risk_ratio_30d_ema: MetricPattern21[StoredF32] = MetricPattern21(client, _m(acc, 'sell_side_risk_ratio_30d_ema'))
        self.sell_side_risk_ratio_7d_ema: MetricPattern21[StoredF32] = MetricPattern21(client, _m(acc, 'sell_side_risk_ratio_7d_ema'))
        self.sopr: MetricPattern21[StoredF64] = MetricPattern21(client, _m(acc, 'sopr'))
        self.sopr_30d_ema: MetricPattern21[StoredF64] = MetricPattern21(client, _m(acc, 'sopr_30d_ema'))
        self.sopr_7d_ema: MetricPattern21[StoredF64] = MetricPattern21(client, _m(acc, 'sopr_7d_ema'))
        self.total_realized_pnl: TotalRealizedPnlPattern[Dollars] = TotalRealizedPnlPattern(client, _m(acc, 'total_realized_pnl'))
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
        self.net_realized_pnl_rel_to_realized_cap: MetricPattern25[StoredF32] = MetricPattern25(client, _m(acc, 'net_realized_pnl_rel_to_realized_cap'))
        self.realized_cap: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_cap'))
        self.realized_cap_30d_delta: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'realized_cap_30d_delta'))
        self.realized_cap_rel_to_own_market_cap: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'realized_cap_rel_to_own_market_cap'))
        self.realized_loss: BlockCountPattern[Dollars] = BlockCountPattern(client, _m(acc, 'realized_loss'))
        self.realized_loss_rel_to_realized_cap: MetricPattern25[StoredF32] = MetricPattern25(client, _m(acc, 'realized_loss_rel_to_realized_cap'))
        self.realized_price: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_price'))
        self.realized_price_extra: ActivePriceRatioPattern = ActivePriceRatioPattern(client, _m(acc, 'realized_price_ratio'))
        self.realized_profit: BlockCountPattern[Dollars] = BlockCountPattern(client, _m(acc, 'realized_profit'))
        self.realized_profit_rel_to_realized_cap: MetricPattern25[StoredF32] = MetricPattern25(client, _m(acc, 'realized_profit_rel_to_realized_cap'))
        self.realized_profit_to_loss_ratio: MetricPattern21[StoredF64] = MetricPattern21(client, _m(acc, 'realized_profit_to_loss_ratio'))
        self.realized_value: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_value'))
        self.sell_side_risk_ratio: MetricPattern21[StoredF32] = MetricPattern21(client, _m(acc, 'sell_side_risk_ratio'))
        self.sell_side_risk_ratio_30d_ema: MetricPattern21[StoredF32] = MetricPattern21(client, _m(acc, 'sell_side_risk_ratio_30d_ema'))
        self.sell_side_risk_ratio_7d_ema: MetricPattern21[StoredF32] = MetricPattern21(client, _m(acc, 'sell_side_risk_ratio_7d_ema'))
        self.sopr: MetricPattern21[StoredF64] = MetricPattern21(client, _m(acc, 'sopr'))
        self.sopr_30d_ema: MetricPattern21[StoredF64] = MetricPattern21(client, _m(acc, 'sopr_30d_ema'))
        self.sopr_7d_ema: MetricPattern21[StoredF64] = MetricPattern21(client, _m(acc, 'sopr_7d_ema'))
        self.total_realized_pnl: TotalRealizedPnlPattern[Dollars] = TotalRealizedPnlPattern(client, _m(acc, 'total_realized_pnl'))
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
        self.net_realized_pnl_rel_to_realized_cap: MetricPattern25[StoredF32] = MetricPattern25(client, _m(acc, 'net_realized_pnl_rel_to_realized_cap'))
        self.realized_cap: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_cap'))
        self.realized_cap_30d_delta: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'realized_cap_30d_delta'))
        self.realized_loss: BlockCountPattern[Dollars] = BlockCountPattern(client, _m(acc, 'realized_loss'))
        self.realized_loss_rel_to_realized_cap: MetricPattern25[StoredF32] = MetricPattern25(client, _m(acc, 'realized_loss_rel_to_realized_cap'))
        self.realized_price: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_price'))
        self.realized_price_extra: RealizedPriceExtraPattern = RealizedPriceExtraPattern(client, _m(acc, 'realized_price'))
        self.realized_profit: BlockCountPattern[Dollars] = BlockCountPattern(client, _m(acc, 'realized_profit'))
        self.realized_profit_rel_to_realized_cap: MetricPattern25[StoredF32] = MetricPattern25(client, _m(acc, 'realized_profit_rel_to_realized_cap'))
        self.realized_value: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_value'))
        self.sell_side_risk_ratio: MetricPattern21[StoredF32] = MetricPattern21(client, _m(acc, 'sell_side_risk_ratio'))
        self.sell_side_risk_ratio_30d_ema: MetricPattern21[StoredF32] = MetricPattern21(client, _m(acc, 'sell_side_risk_ratio_30d_ema'))
        self.sell_side_risk_ratio_7d_ema: MetricPattern21[StoredF32] = MetricPattern21(client, _m(acc, 'sell_side_risk_ratio_7d_ema'))
        self.sopr: MetricPattern21[StoredF64] = MetricPattern21(client, _m(acc, 'sopr'))
        self.sopr_30d_ema: MetricPattern21[StoredF64] = MetricPattern21(client, _m(acc, 'sopr_30d_ema'))
        self.sopr_7d_ema: MetricPattern21[StoredF64] = MetricPattern21(client, _m(acc, 'sopr_7d_ema'))
        self.total_realized_pnl: TotalRealizedPnlPattern[Dollars] = TotalRealizedPnlPattern(client, _m(acc, 'total_realized_pnl'))
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

class RelativePattern2:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.neg_unrealized_loss_rel_to_market_cap: MetricPattern5[StoredF32] = MetricPattern5(client, _m(acc, 'neg_unrealized_loss_rel_to_market_cap'))
        self.neg_unrealized_loss_rel_to_own_market_cap: MetricPattern5[StoredF32] = MetricPattern5(client, _m(acc, 'neg_unrealized_loss_rel_to_own_market_cap'))
        self.neg_unrealized_loss_rel_to_own_total_unrealized_pnl: MetricPattern5[StoredF32] = MetricPattern5(client, _m(acc, 'neg_unrealized_loss_rel_to_own_total_unrealized_pnl'))
        self.net_unrealized_pnl_rel_to_market_cap: MetricPattern3[StoredF32] = MetricPattern3(client, _m(acc, 'net_unrealized_pnl_rel_to_market_cap'))
        self.net_unrealized_pnl_rel_to_own_market_cap: MetricPattern3[StoredF32] = MetricPattern3(client, _m(acc, 'net_unrealized_pnl_rel_to_own_market_cap'))
        self.net_unrealized_pnl_rel_to_own_total_unrealized_pnl: MetricPattern3[StoredF32] = MetricPattern3(client, _m(acc, 'net_unrealized_pnl_rel_to_own_total_unrealized_pnl'))
        self.nupl: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'nupl'))
        self.supply_in_loss_rel_to_circulating_supply: MetricPattern5[StoredF64] = MetricPattern5(client, _m(acc, 'supply_in_loss_rel_to_circulating_supply'))
        self.supply_in_loss_rel_to_own_supply: MetricPattern5[StoredF64] = MetricPattern5(client, _m(acc, 'supply_in_loss_rel_to_own_supply'))
        self.supply_in_profit_rel_to_circulating_supply: MetricPattern5[StoredF64] = MetricPattern5(client, _m(acc, 'supply_in_profit_rel_to_circulating_supply'))
        self.supply_in_profit_rel_to_own_supply: MetricPattern5[StoredF64] = MetricPattern5(client, _m(acc, 'supply_in_profit_rel_to_own_supply'))
        self.supply_rel_to_circulating_supply: MetricPattern4[StoredF64] = MetricPattern4(client, _m(acc, 'supply_rel_to_circulating_supply'))
        self.unrealized_loss_rel_to_market_cap: MetricPattern5[StoredF32] = MetricPattern5(client, _m(acc, 'unrealized_loss_rel_to_market_cap'))
        self.unrealized_loss_rel_to_own_market_cap: MetricPattern5[StoredF32] = MetricPattern5(client, _m(acc, 'unrealized_loss_rel_to_own_market_cap'))
        self.unrealized_loss_rel_to_own_total_unrealized_pnl: MetricPattern5[StoredF32] = MetricPattern5(client, _m(acc, 'unrealized_loss_rel_to_own_total_unrealized_pnl'))
        self.unrealized_profit_rel_to_market_cap: MetricPattern5[StoredF32] = MetricPattern5(client, _m(acc, 'unrealized_profit_rel_to_market_cap'))
        self.unrealized_profit_rel_to_own_market_cap: MetricPattern5[StoredF32] = MetricPattern5(client, _m(acc, 'unrealized_profit_rel_to_own_market_cap'))
        self.unrealized_profit_rel_to_own_total_unrealized_pnl: MetricPattern5[StoredF32] = MetricPattern5(client, _m(acc, 'unrealized_profit_rel_to_own_total_unrealized_pnl'))

class AXbtPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self._1d_dominance: BlockCountPattern[StoredF32] = BlockCountPattern(client, _m(acc, '1d_dominance'))
        self._1m_blocks_mined: MetricPattern4[StoredU32] = MetricPattern4(client, _m(acc, '1m_blocks_mined'))
        self._1m_dominance: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, '1m_dominance'))
        self._1w_blocks_mined: MetricPattern4[StoredU32] = MetricPattern4(client, _m(acc, '1w_blocks_mined'))
        self._1w_dominance: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, '1w_dominance'))
        self._1y_blocks_mined: MetricPattern4[StoredU32] = MetricPattern4(client, _m(acc, '1y_blocks_mined'))
        self._1y_dominance: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, '1y_dominance'))
        self.blocks_mined: BlockCountPattern[StoredU32] = BlockCountPattern(client, _m(acc, 'blocks_mined'))
        self.coinbase: UnclaimedRewardsPattern = UnclaimedRewardsPattern(client, _m(acc, 'coinbase'))
        self.days_since_block: MetricPattern4[StoredU16] = MetricPattern4(client, _m(acc, 'days_since_block'))
        self.dominance: BlockCountPattern[StoredF32] = BlockCountPattern(client, _m(acc, 'dominance'))
        self.fee: SentPattern = SentPattern(client, _m(acc, 'fee'))
        self.subsidy: SentPattern = SentPattern(client, _m(acc, 'subsidy'))

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
        self._10y: ActiveSupplyPattern = ActiveSupplyPattern(client, (f'10y_{{acc}}' if acc else '10y'))
        self._1m: ActiveSupplyPattern = ActiveSupplyPattern(client, (f'1m_{{acc}}' if acc else '1m'))
        self._1w: ActiveSupplyPattern = ActiveSupplyPattern(client, (f'1w_{{acc}}' if acc else '1w'))
        self._1y: ActiveSupplyPattern = ActiveSupplyPattern(client, (f'1y_{{acc}}' if acc else '1y'))
        self._2y: ActiveSupplyPattern = ActiveSupplyPattern(client, (f'2y_{{acc}}' if acc else '2y'))
        self._3m: ActiveSupplyPattern = ActiveSupplyPattern(client, (f'3m_{{acc}}' if acc else '3m'))
        self._3y: ActiveSupplyPattern = ActiveSupplyPattern(client, (f'3y_{{acc}}' if acc else '3y'))
        self._4y: ActiveSupplyPattern = ActiveSupplyPattern(client, (f'4y_{{acc}}' if acc else '4y'))
        self._5y: ActiveSupplyPattern = ActiveSupplyPattern(client, (f'5y_{{acc}}' if acc else '5y'))
        self._6m: ActiveSupplyPattern = ActiveSupplyPattern(client, (f'6m_{{acc}}' if acc else '6m'))
        self._6y: ActiveSupplyPattern = ActiveSupplyPattern(client, (f'6y_{{acc}}' if acc else '6y'))
        self._8y: ActiveSupplyPattern = ActiveSupplyPattern(client, (f'8y_{{acc}}' if acc else '8y'))

class PeriodAvgPricePattern(Generic[T]):
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

class ClassAvgPricePattern(Generic[T]):
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self._2015: MetricPattern4[T] = MetricPattern4(client, _m(acc, '2015_avg_price'))
        self._2016: MetricPattern4[T] = MetricPattern4(client, _m(acc, '2016_avg_price'))
        self._2017: MetricPattern4[T] = MetricPattern4(client, _m(acc, '2017_avg_price'))
        self._2018: MetricPattern4[T] = MetricPattern4(client, _m(acc, '2018_avg_price'))
        self._2019: MetricPattern4[T] = MetricPattern4(client, _m(acc, '2019_avg_price'))
        self._2020: MetricPattern4[T] = MetricPattern4(client, _m(acc, '2020_avg_price'))
        self._2021: MetricPattern4[T] = MetricPattern4(client, _m(acc, '2021_avg_price'))
        self._2022: MetricPattern4[T] = MetricPattern4(client, _m(acc, '2022_avg_price'))
        self._2023: MetricPattern4[T] = MetricPattern4(client, _m(acc, '2023_avg_price'))
        self._2024: MetricPattern4[T] = MetricPattern4(client, _m(acc, '2024_avg_price'))
        self._2025: MetricPattern4[T] = MetricPattern4(client, _m(acc, '2025_avg_price'))

class BitcoinPattern(Generic[T]):
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.average: MetricPattern2[T] = MetricPattern2(client, _m(acc, 'avg'))
        self.base: MetricPattern25[T] = MetricPattern25(client, acc)
        self.cumulative: MetricPattern1[T] = MetricPattern1(client, _m(acc, 'cumulative'))
        self.max: MetricPattern2[T] = MetricPattern2(client, _m(acc, 'max'))
        self.median: MetricPattern21[T] = MetricPattern21(client, _m(acc, 'median'))
        self.min: MetricPattern2[T] = MetricPattern2(client, _m(acc, 'min'))
        self.pct10: MetricPattern21[T] = MetricPattern21(client, _m(acc, 'pct10'))
        self.pct25: MetricPattern21[T] = MetricPattern21(client, _m(acc, 'pct25'))
        self.pct75: MetricPattern21[T] = MetricPattern21(client, _m(acc, 'pct75'))
        self.pct90: MetricPattern21[T] = MetricPattern21(client, _m(acc, 'pct90'))
        self.sum: MetricPattern2[T] = MetricPattern2(client, _m(acc, 'sum'))

class RelativePattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.neg_unrealized_loss_rel_to_market_cap: MetricPattern5[StoredF32] = MetricPattern5(client, _m(acc, 'neg_unrealized_loss_rel_to_market_cap'))
        self.net_unrealized_pnl_rel_to_market_cap: MetricPattern3[StoredF32] = MetricPattern3(client, _m(acc, 'net_unrealized_pnl_rel_to_market_cap'))
        self.nupl: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'nupl'))
        self.supply_in_loss_rel_to_circulating_supply: MetricPattern5[StoredF64] = MetricPattern5(client, _m(acc, 'supply_in_loss_rel_to_circulating_supply'))
        self.supply_in_loss_rel_to_own_supply: MetricPattern5[StoredF64] = MetricPattern5(client, _m(acc, 'supply_in_loss_rel_to_own_supply'))
        self.supply_in_profit_rel_to_circulating_supply: MetricPattern5[StoredF64] = MetricPattern5(client, _m(acc, 'supply_in_profit_rel_to_circulating_supply'))
        self.supply_in_profit_rel_to_own_supply: MetricPattern5[StoredF64] = MetricPattern5(client, _m(acc, 'supply_in_profit_rel_to_own_supply'))
        self.supply_rel_to_circulating_supply: MetricPattern4[StoredF64] = MetricPattern4(client, _m(acc, 'supply_rel_to_circulating_supply'))
        self.unrealized_loss_rel_to_market_cap: MetricPattern5[StoredF32] = MetricPattern5(client, _m(acc, 'unrealized_loss_rel_to_market_cap'))
        self.unrealized_profit_rel_to_market_cap: MetricPattern5[StoredF32] = MetricPattern5(client, _m(acc, 'unrealized_profit_rel_to_market_cap'))

class BlockSizePattern(Generic[T]):
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.average: MetricPattern1[T] = MetricPattern1(client, _m(acc, 'avg'))
        self.cumulative: MetricPattern1[T] = MetricPattern1(client, _m(acc, 'cumulative'))
        self.max: MetricPattern1[T] = MetricPattern1(client, _m(acc, 'max'))
        self.median: MetricPattern25[T] = MetricPattern25(client, _m(acc, 'median'))
        self.min: MetricPattern1[T] = MetricPattern1(client, _m(acc, 'min'))
        self.pct10: MetricPattern25[T] = MetricPattern25(client, _m(acc, 'pct10'))
        self.pct25: MetricPattern25[T] = MetricPattern25(client, _m(acc, 'pct25'))
        self.pct75: MetricPattern25[T] = MetricPattern25(client, _m(acc, 'pct75'))
        self.pct90: MetricPattern25[T] = MetricPattern25(client, _m(acc, 'pct90'))
        self.sum: MetricPattern1[T] = MetricPattern1(client, _m(acc, 'sum'))

class UnrealizedPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.neg_unrealized_loss: MetricPattern3[Dollars] = MetricPattern3(client, _m(acc, 'neg_unrealized_loss'))
        self.net_unrealized_pnl: MetricPattern3[Dollars] = MetricPattern3(client, _m(acc, 'net_unrealized_pnl'))
        self.supply_in_loss: SupplyPattern2 = SupplyPattern2(client, _m(acc, 'supply_in_loss'))
        self.supply_in_loss_value: SupplyValuePattern = SupplyValuePattern(client, _m(acc, 'supply_in_loss'))
        self.supply_in_profit: SupplyPattern2 = SupplyPattern2(client, _m(acc, 'supply_in_profit'))
        self.supply_in_profit_value: SupplyValuePattern = SupplyValuePattern(client, _m(acc, 'supply_in_profit'))
        self.total_unrealized_pnl: MetricPattern3[Dollars] = MetricPattern3(client, _m(acc, 'total_unrealized_pnl'))
        self.unrealized_loss: MetricPattern3[Dollars] = MetricPattern3(client, _m(acc, 'unrealized_loss'))
        self.unrealized_profit: MetricPattern3[Dollars] = MetricPattern3(client, _m(acc, 'unrealized_profit'))

class AddresstypeToHeightToAddrCountPattern(Generic[T]):
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.p2a: MetricPattern25[T] = MetricPattern25(client, (f'p2a_{{acc}}' if acc else 'p2a'))
        self.p2pk33: MetricPattern25[T] = MetricPattern25(client, (f'p2pk33_{{acc}}' if acc else 'p2pk33'))
        self.p2pk65: MetricPattern25[T] = MetricPattern25(client, (f'p2pk65_{{acc}}' if acc else 'p2pk65'))
        self.p2pkh: MetricPattern25[T] = MetricPattern25(client, (f'p2pkh_{{acc}}' if acc else 'p2pkh'))
        self.p2sh: MetricPattern25[T] = MetricPattern25(client, (f'p2sh_{{acc}}' if acc else 'p2sh'))
        self.p2tr: MetricPattern25[T] = MetricPattern25(client, (f'p2tr_{{acc}}' if acc else 'p2tr'))
        self.p2wpkh: MetricPattern25[T] = MetricPattern25(client, (f'p2wpkh_{{acc}}' if acc else 'p2wpkh'))
        self.p2wsh: MetricPattern25[T] = MetricPattern25(client, (f'p2wsh_{{acc}}' if acc else 'p2wsh'))

class BlockIntervalPattern(Generic[T]):
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.average: MetricPattern2[T] = MetricPattern2(client, _m(acc, 'avg'))
        self.max: MetricPattern2[T] = MetricPattern2(client, _m(acc, 'max'))
        self.median: MetricPattern21[T] = MetricPattern21(client, _m(acc, 'median'))
        self.min: MetricPattern2[T] = MetricPattern2(client, _m(acc, 'min'))
        self.pct10: MetricPattern21[T] = MetricPattern21(client, _m(acc, 'pct10'))
        self.pct25: MetricPattern21[T] = MetricPattern21(client, _m(acc, 'pct25'))
        self.pct75: MetricPattern21[T] = MetricPattern21(client, _m(acc, 'pct75'))
        self.pct90: MetricPattern21[T] = MetricPattern21(client, _m(acc, 'pct90'))

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

class _0satsPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.activity: ActivityPattern2 = ActivityPattern2(client, acc)
        self.addr_count: MetricPattern1[StoredU64] = MetricPattern1(client, _m(acc, 'addr_count'))
        self.cost_basis: CostBasisPattern = CostBasisPattern(client, acc)
        self.realized: RealizedPattern = RealizedPattern(client, acc)
        self.relative: RelativePattern = RelativePattern(client, acc)
        self.supply: SupplyPattern3 = SupplyPattern3(client, acc)
        self.unrealized: UnrealizedPattern = UnrealizedPattern(client, acc)

class UpTo1dPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.activity: ActivityPattern2 = ActivityPattern2(client, acc)
        self.cost_basis: CostBasisPattern2 = CostBasisPattern2(client, acc)
        self.realized: RealizedPattern3 = RealizedPattern3(client, acc)
        self.relative: RelativePattern2 = RelativePattern2(client, acc)
        self.supply: SupplyPattern3 = SupplyPattern3(client, acc)
        self.unrealized: UnrealizedPattern = UnrealizedPattern(client, acc)

class _10yPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.activity: ActivityPattern2 = ActivityPattern2(client, acc)
        self.cost_basis: CostBasisPattern = CostBasisPattern(client, acc)
        self.realized: RealizedPattern4 = RealizedPattern4(client, acc)
        self.relative: RelativePattern = RelativePattern(client, acc)
        self.supply: SupplyPattern3 = SupplyPattern3(client, acc)
        self.unrealized: UnrealizedPattern = UnrealizedPattern(client, acc)

class _10yTo12yPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.activity: ActivityPattern2 = ActivityPattern2(client, acc)
        self.cost_basis: CostBasisPattern2 = CostBasisPattern2(client, acc)
        self.realized: RealizedPattern2 = RealizedPattern2(client, acc)
        self.relative: RelativePattern2 = RelativePattern2(client, acc)
        self.supply: SupplyPattern3 = SupplyPattern3(client, acc)
        self.unrealized: UnrealizedPattern = UnrealizedPattern(client, acc)

class _0satsPattern2:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.activity: ActivityPattern2 = ActivityPattern2(client, acc)
        self.cost_basis: CostBasisPattern = CostBasisPattern(client, acc)
        self.realized: RealizedPattern = RealizedPattern(client, acc)
        self.relative: RelativePattern = RelativePattern(client, acc)
        self.supply: SupplyPattern3 = SupplyPattern3(client, acc)
        self.unrealized: UnrealizedPattern = UnrealizedPattern(client, acc)

class SegwitAdoptionPattern(Generic[T]):
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.average: MetricPattern2[T] = MetricPattern2(client, _m(acc, 'avg'))
        self.base: MetricPattern25[T] = MetricPattern25(client, acc)
        self.cumulative: MetricPattern1[T] = MetricPattern1(client, _m(acc, 'cumulative'))
        self.max: MetricPattern2[T] = MetricPattern2(client, _m(acc, 'max'))
        self.min: MetricPattern2[T] = MetricPattern2(client, _m(acc, 'min'))
        self.sum: MetricPattern2[T] = MetricPattern2(client, _m(acc, 'sum'))

class SupplyPattern3:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.supply: SupplyPattern2 = SupplyPattern2(client, _m(acc, 'supply'))
        self.supply_half: ActiveSupplyPattern = ActiveSupplyPattern(client, _m(acc, 'supply_half'))
        self.supply_half_value: ActiveSupplyPattern = ActiveSupplyPattern(client, _m(acc, 'supply_half'))
        self.supply_value: SupplyValuePattern = SupplyValuePattern(client, _m(acc, 'supply'))
        self.utxo_count: MetricPattern1[StoredU64] = MetricPattern1(client, _m(acc, 'utxo_count'))

class ActivityPattern2:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.coinblocks_destroyed: BlockCountPattern[StoredF64] = BlockCountPattern(client, _m(acc, 'coinblocks_destroyed'))
        self.coindays_destroyed: BlockCountPattern[StoredF64] = BlockCountPattern(client, _m(acc, 'coindays_destroyed'))
        self.satblocks_destroyed: MetricPattern25[Sats] = MetricPattern25(client, _m(acc, 'satblocks_destroyed'))
        self.satdays_destroyed: MetricPattern25[Sats] = MetricPattern25(client, _m(acc, 'satdays_destroyed'))
        self.sent: SentPattern = SentPattern(client, _m(acc, 'sent'))

class SupplyPattern2:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.base: MetricPattern25[Sats] = MetricPattern25(client, acc)
        self.bitcoin: MetricPattern4[Bitcoin] = MetricPattern4(client, _m(acc, 'btc'))
        self.dollars: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'usd'))
        self.sats: MetricPattern4[Sats] = MetricPattern4(client, acc)

class SentPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.base: MetricPattern25[Sats] = MetricPattern25(client, acc)
        self.bitcoin: BlockCountPattern[Bitcoin] = BlockCountPattern(client, _m(acc, 'btc'))
        self.dollars: BlockCountPattern[Dollars] = BlockCountPattern(client, _m(acc, 'usd'))
        self.sats: SatsPattern = SatsPattern(client, acc)

class OpreturnPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.base: MetricPattern25[Sats] = MetricPattern25(client, acc)
        self.bitcoin: BitcoinPattern2[Bitcoin] = BitcoinPattern2(client, _m(acc, 'btc'))
        self.dollars: BitcoinPattern2[Dollars] = BitcoinPattern2(client, _m(acc, 'usd'))
        self.sats: SatsPattern4 = SatsPattern4(client, acc)

class CostBasisPattern2:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.max_cost_basis: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'max_cost_basis'))
        self.min_cost_basis: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'min_cost_basis'))
        self.percentiles: PercentilesPattern = PercentilesPattern(client, _m(acc, 'cost_basis'))

class CoinbasePattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.bitcoin: BitcoinPattern[Bitcoin] = BitcoinPattern(client, _m(acc, 'btc'))
        self.dollars: BitcoinPattern[Dollars] = BitcoinPattern(client, _m(acc, 'usd'))
        self.sats: BitcoinPattern[Sats] = BitcoinPattern(client, acc)

class UnclaimedRewardsPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.bitcoin: BlockCountPattern[Bitcoin] = BlockCountPattern(client, _m(acc, 'btc'))
        self.dollars: BlockCountPattern[Dollars] = BlockCountPattern(client, _m(acc, 'usd'))
        self.sats: BlockCountPattern[Sats] = BlockCountPattern(client, acc)

class ActiveSupplyPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.bitcoin: MetricPattern1[Bitcoin] = MetricPattern1(client, _m(acc, 'btc'))
        self.dollars: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'usd'))
        self.sats: MetricPattern1[Sats] = MetricPattern1(client, acc)

class BlockCountPattern(Generic[T]):
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.base: MetricPattern25[T] = MetricPattern25(client, acc)
        self.cumulative: MetricPattern1[T] = MetricPattern1(client, _m(acc, 'cumulative'))
        self.sum: MetricPattern2[T] = MetricPattern2(client, _m(acc, 'sum'))

class BitcoinPattern2(Generic[T]):
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.base: MetricPattern25[T] = MetricPattern25(client, acc)
        self.cumulative: MetricPattern1[T] = MetricPattern1(client, _m(acc, 'cumulative'))
        self.last: MetricPattern2[T] = MetricPattern2(client, acc)

class CostBasisPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.max_cost_basis: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'max_cost_basis'))
        self.min_cost_basis: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'min_cost_basis'))

class SatsPattern4:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.cumulative: MetricPattern1[Sats] = MetricPattern1(client, _m(acc, 'cumulative'))
        self.last: MetricPattern2[Sats] = MetricPattern2(client, acc)

class SupplyValuePattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.bitcoin: MetricPattern25[Bitcoin] = MetricPattern25(client, _m(acc, 'btc'))
        self.dollars: MetricPattern25[Dollars] = MetricPattern25(client, _m(acc, 'usd'))

class SatsPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.cumulative: MetricPattern1[Sats] = MetricPattern1(client, _m(acc, 'cumulative'))
        self.sum: MetricPattern2[Sats] = MetricPattern2(client, acc)

class _1dReturns1mSdPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.sd: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'sd'))
        self.sma: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'sma'))

class TotalRealizedPnlPattern(Generic[T]):
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.base: MetricPattern25[T] = MetricPattern25(client, acc)
        self.sum: MetricPattern2[T] = MetricPattern2(client, _m(acc, 'sum'))

class RealizedPriceExtraPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.ratio: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'ratio'))

# Catalog tree classes

class CatalogTree:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.computed: CatalogTree_Computed = CatalogTree_Computed(client, f'{base_path}_computed')
        self.indexed: CatalogTree_Indexed = CatalogTree_Indexed(client, f'{base_path}_indexed')

class CatalogTree_Computed:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.blocks: CatalogTree_Computed_Blocks = CatalogTree_Computed_Blocks(client, f'{base_path}_blocks')
        self.cointime: CatalogTree_Computed_Cointime = CatalogTree_Computed_Cointime(client, f'{base_path}_cointime')
        self.constants: CatalogTree_Computed_Constants = CatalogTree_Computed_Constants(client, f'{base_path}_constants')
        self.distribution: CatalogTree_Computed_Distribution = CatalogTree_Computed_Distribution(client, f'{base_path}_distribution')
        self.indexes: CatalogTree_Computed_Indexes = CatalogTree_Computed_Indexes(client, f'{base_path}_indexes')
        self.inputs: CatalogTree_Computed_Inputs = CatalogTree_Computed_Inputs(client, f'{base_path}_inputs')
        self.market: CatalogTree_Computed_Market = CatalogTree_Computed_Market(client, f'{base_path}_market')
        self.outputs: CatalogTree_Computed_Outputs = CatalogTree_Computed_Outputs(client, f'{base_path}_outputs')
        self.pools: CatalogTree_Computed_Pools = CatalogTree_Computed_Pools(client, f'{base_path}_pools')
        self.positions: CatalogTree_Computed_Positions = CatalogTree_Computed_Positions(client, f'{base_path}_positions')
        self.price: CatalogTree_Computed_Price = CatalogTree_Computed_Price(client, f'{base_path}_price')
        self.scripts: CatalogTree_Computed_Scripts = CatalogTree_Computed_Scripts(client, f'{base_path}_scripts')
        self.supply: CatalogTree_Computed_Supply = CatalogTree_Computed_Supply(client, f'{base_path}_supply')
        self.transactions: CatalogTree_Computed_Transactions = CatalogTree_Computed_Transactions(client, f'{base_path}_transactions')

class CatalogTree_Computed_Blocks:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.count: CatalogTree_Computed_Blocks_Count = CatalogTree_Computed_Blocks_Count(client, f'{base_path}_count')
        self.difficulty: CatalogTree_Computed_Blocks_Difficulty = CatalogTree_Computed_Blocks_Difficulty(client, f'{base_path}_difficulty')
        self.halving: CatalogTree_Computed_Blocks_Halving = CatalogTree_Computed_Blocks_Halving(client, f'{base_path}_halving')
        self.interval: CatalogTree_Computed_Blocks_Interval = CatalogTree_Computed_Blocks_Interval(client, f'{base_path}_interval')
        self.mining: CatalogTree_Computed_Blocks_Mining = CatalogTree_Computed_Blocks_Mining(client, f'{base_path}_mining')
        self.rewards: CatalogTree_Computed_Blocks_Rewards = CatalogTree_Computed_Blocks_Rewards(client, f'{base_path}_rewards')
        self.size: CatalogTree_Computed_Blocks_Size = CatalogTree_Computed_Blocks_Size(client, f'{base_path}_size')
        self.time: CatalogTree_Computed_Blocks_Time = CatalogTree_Computed_Blocks_Time(client, f'{base_path}_time')
        self.weight: CatalogTree_Computed_Blocks_Weight = CatalogTree_Computed_Blocks_Weight(client, f'{base_path}_weight')

class CatalogTree_Computed_Blocks_Count:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._1m_block_count: MetricPattern4[StoredU32] = MetricPattern4(client, f'{base_path}_1m_block_count')
        self._1w_block_count: MetricPattern4[StoredU32] = MetricPattern4(client, f'{base_path}_1w_block_count')
        self._1y_block_count: MetricPattern4[StoredU32] = MetricPattern4(client, f'{base_path}_1y_block_count')
        self._24h_block_count: MetricPattern25[StoredU32] = MetricPattern25(client, f'{base_path}_24h_block_count')
        self.block_count: BlockCountPattern[StoredU32] = BlockCountPattern(client, 'block_count')
        self.block_count_target: MetricPattern4[StoredU64] = MetricPattern4(client, f'{base_path}_block_count_target')

class CatalogTree_Computed_Blocks_Difficulty:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.blocks_before_next_difficulty_adjustment: MetricPattern1[StoredU32] = MetricPattern1(client, f'{base_path}_blocks_before_next_difficulty_adjustment')
        self.days_before_next_difficulty_adjustment: MetricPattern1[StoredF32] = MetricPattern1(client, f'{base_path}_days_before_next_difficulty_adjustment')
        self.difficultyepoch: MetricPattern4[DifficultyEpoch] = MetricPattern4(client, f'{base_path}_difficultyepoch')

class CatalogTree_Computed_Blocks_Halving:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.blocks_before_next_halving: MetricPattern1[StoredU32] = MetricPattern1(client, f'{base_path}_blocks_before_next_halving')
        self.days_before_next_halving: MetricPattern1[StoredF32] = MetricPattern1(client, f'{base_path}_days_before_next_halving')
        self.halvingepoch: MetricPattern4[HalvingEpoch] = MetricPattern4(client, f'{base_path}_halvingepoch')

class CatalogTree_Computed_Blocks_Interval:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.block_interval: BlockIntervalPattern[Timestamp] = BlockIntervalPattern(client, 'block_interval')
        self.interval: MetricPattern25[Timestamp] = MetricPattern25(client, f'{base_path}_interval')

class CatalogTree_Computed_Blocks_Mining:
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

class CatalogTree_Computed_Blocks_Rewards:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._24h_coinbase_sum: MetricPattern25[Sats] = MetricPattern25(client, f'{base_path}_24h_coinbase_sum')
        self._24h_coinbase_usd_sum: MetricPattern25[Dollars] = MetricPattern25(client, f'{base_path}_24h_coinbase_usd_sum')
        self.coinbase: CoinbasePattern = CoinbasePattern(client, 'coinbase')
        self.fee_dominance: MetricPattern21[StoredF32] = MetricPattern21(client, f'{base_path}_fee_dominance')
        self.subsidy: CoinbasePattern = CoinbasePattern(client, 'subsidy')
        self.subsidy_dominance: MetricPattern21[StoredF32] = MetricPattern21(client, f'{base_path}_subsidy_dominance')
        self.subsidy_usd_1y_sma: MetricPattern4[Dollars] = MetricPattern4(client, f'{base_path}_subsidy_usd_1y_sma')
        self.unclaimed_rewards: UnclaimedRewardsPattern = UnclaimedRewardsPattern(client, 'unclaimed_rewards')

class CatalogTree_Computed_Blocks_Size:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.block_size: BlockSizePattern[StoredU64] = BlockSizePattern(client, 'block_size')
        self.block_vbytes: BlockSizePattern[StoredU64] = BlockSizePattern(client, 'block_vbytes')
        self.vbytes: MetricPattern25[StoredU64] = MetricPattern25(client, f'{base_path}_vbytes')

class CatalogTree_Computed_Blocks_Time:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.date: MetricPattern25[Date] = MetricPattern25(client, f'{base_path}_date')
        self.date_fixed: MetricPattern25[Date] = MetricPattern25(client, f'{base_path}_date_fixed')
        self.timestamp: MetricPattern2[Timestamp] = MetricPattern2(client, f'{base_path}_timestamp')
        self.timestamp_fixed: MetricPattern25[Timestamp] = MetricPattern25(client, f'{base_path}_timestamp_fixed')

class CatalogTree_Computed_Blocks_Weight:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.block_fullness: BitcoinPattern[StoredF32] = BitcoinPattern(client, 'block_fullness')
        self.block_weight: BlockSizePattern[Weight] = BlockSizePattern(client, 'block_weight')

class CatalogTree_Computed_Cointime:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.activity: CatalogTree_Computed_Cointime_Activity = CatalogTree_Computed_Cointime_Activity(client, f'{base_path}_activity')
        self.adjusted: CatalogTree_Computed_Cointime_Adjusted = CatalogTree_Computed_Cointime_Adjusted(client, f'{base_path}_adjusted')
        self.cap: CatalogTree_Computed_Cointime_Cap = CatalogTree_Computed_Cointime_Cap(client, f'{base_path}_cap')
        self.pricing: CatalogTree_Computed_Cointime_Pricing = CatalogTree_Computed_Cointime_Pricing(client, f'{base_path}_pricing')
        self.supply: CatalogTree_Computed_Cointime_Supply = CatalogTree_Computed_Cointime_Supply(client, f'{base_path}_supply')
        self.value: CatalogTree_Computed_Cointime_Value = CatalogTree_Computed_Cointime_Value(client, f'{base_path}_value')

class CatalogTree_Computed_Cointime_Activity:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.activity_to_vaultedness_ratio: MetricPattern1[StoredF64] = MetricPattern1(client, f'{base_path}_activity_to_vaultedness_ratio')
        self.coinblocks_created: BlockCountPattern[StoredF64] = BlockCountPattern(client, 'coinblocks_created')
        self.coinblocks_stored: BlockCountPattern[StoredF64] = BlockCountPattern(client, 'coinblocks_stored')
        self.liveliness: MetricPattern1[StoredF64] = MetricPattern1(client, f'{base_path}_liveliness')
        self.vaultedness: MetricPattern1[StoredF64] = MetricPattern1(client, f'{base_path}_vaultedness')

class CatalogTree_Computed_Cointime_Adjusted:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.cointime_adj_inflation_rate: MetricPattern4[StoredF32] = MetricPattern4(client, f'{base_path}_cointime_adj_inflation_rate')
        self.cointime_adj_tx_btc_velocity: MetricPattern4[StoredF64] = MetricPattern4(client, f'{base_path}_cointime_adj_tx_btc_velocity')
        self.cointime_adj_tx_usd_velocity: MetricPattern4[StoredF64] = MetricPattern4(client, f'{base_path}_cointime_adj_tx_usd_velocity')

class CatalogTree_Computed_Cointime_Cap:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.active_cap: MetricPattern1[Dollars] = MetricPattern1(client, f'{base_path}_active_cap')
        self.cointime_cap: MetricPattern1[Dollars] = MetricPattern1(client, f'{base_path}_cointime_cap')
        self.investor_cap: MetricPattern1[Dollars] = MetricPattern1(client, f'{base_path}_investor_cap')
        self.thermo_cap: MetricPattern1[Dollars] = MetricPattern1(client, f'{base_path}_thermo_cap')
        self.vaulted_cap: MetricPattern1[Dollars] = MetricPattern1(client, f'{base_path}_vaulted_cap')

class CatalogTree_Computed_Cointime_Pricing:
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

class CatalogTree_Computed_Cointime_Supply:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.active_supply: ActiveSupplyPattern = ActiveSupplyPattern(client, 'active_supply')
        self.vaulted_supply: ActiveSupplyPattern = ActiveSupplyPattern(client, 'vaulted_supply')

class CatalogTree_Computed_Cointime_Value:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.cointime_value_created: BlockCountPattern[StoredF64] = BlockCountPattern(client, 'cointime_value_created')
        self.cointime_value_destroyed: BlockCountPattern[StoredF64] = BlockCountPattern(client, 'cointime_value_destroyed')
        self.cointime_value_stored: BlockCountPattern[StoredF64] = BlockCountPattern(client, 'cointime_value_stored')

class CatalogTree_Computed_Constants:
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

class CatalogTree_Computed_Distribution:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.addr_count: MetricPattern1[StoredU64] = MetricPattern1(client, f'{base_path}_addr_count')
        self.address_cohorts: CatalogTree_Computed_Distribution_AddressCohorts = CatalogTree_Computed_Distribution_AddressCohorts(client, f'{base_path}_address_cohorts')
        self.addresses_data: CatalogTree_Computed_Distribution_AddressesData = CatalogTree_Computed_Distribution_AddressesData(client, f'{base_path}_addresses_data')
        self.addresstype_to_height_to_addr_count: AddresstypeToHeightToAddrCountPattern[StoredU64] = AddresstypeToHeightToAddrCountPattern(client, 'addr_count')
        self.addresstype_to_height_to_empty_addr_count: AddresstypeToHeightToAddrCountPattern[StoredU64] = AddresstypeToHeightToAddrCountPattern(client, 'empty_addr_count')
        self.addresstype_to_indexes_to_addr_count: AddresstypeToHeightToAddrCountPattern[StoredU64] = AddresstypeToHeightToAddrCountPattern(client, 'addr_count')
        self.addresstype_to_indexes_to_empty_addr_count: AddresstypeToHeightToAddrCountPattern[StoredU64] = AddresstypeToHeightToAddrCountPattern(client, 'empty_addr_count')
        self.any_address_indexes: AddresstypeToHeightToAddrCountPattern[AnyAddressIndex] = AddresstypeToHeightToAddrCountPattern(client, 'anyaddressindex')
        self.chain_state: MetricPattern25[SupplyState] = MetricPattern25(client, f'{base_path}_chain_state')
        self.empty_addr_count: MetricPattern1[StoredU64] = MetricPattern1(client, f'{base_path}_empty_addr_count')
        self.emptyaddressindex: MetricPattern41[EmptyAddressIndex] = MetricPattern41(client, f'{base_path}_emptyaddressindex')
        self.loadedaddressindex: MetricPattern40[LoadedAddressIndex] = MetricPattern40(client, f'{base_path}_loadedaddressindex')
        self.utxo_cohorts: CatalogTree_Computed_Distribution_UtxoCohorts = CatalogTree_Computed_Distribution_UtxoCohorts(client, f'{base_path}_utxo_cohorts')

class CatalogTree_Computed_Distribution_AddressCohorts:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.amount_range: CatalogTree_Computed_Distribution_AddressCohorts_AmountRange = CatalogTree_Computed_Distribution_AddressCohorts_AmountRange(client, f'{base_path}_amount_range')
        self.ge_amount: CatalogTree_Computed_Distribution_AddressCohorts_GeAmount = CatalogTree_Computed_Distribution_AddressCohorts_GeAmount(client, f'{base_path}_ge_amount')
        self.lt_amount: CatalogTree_Computed_Distribution_AddressCohorts_LtAmount = CatalogTree_Computed_Distribution_AddressCohorts_LtAmount(client, f'{base_path}_lt_amount')

class CatalogTree_Computed_Distribution_AddressCohorts_AmountRange:
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

class CatalogTree_Computed_Distribution_AddressCohorts_GeAmount:
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

class CatalogTree_Computed_Distribution_AddressCohorts_LtAmount:
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

class CatalogTree_Computed_Distribution_AddressesData:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.empty: MetricPattern41[EmptyAddressData] = MetricPattern41(client, f'{base_path}_empty')
        self.loaded: MetricPattern40[LoadedAddressData] = MetricPattern40(client, f'{base_path}_loaded')

class CatalogTree_Computed_Distribution_UtxoCohorts:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.age_range: CatalogTree_Computed_Distribution_UtxoCohorts_AgeRange = CatalogTree_Computed_Distribution_UtxoCohorts_AgeRange(client, f'{base_path}_age_range')
        self.all: CatalogTree_Computed_Distribution_UtxoCohorts_All = CatalogTree_Computed_Distribution_UtxoCohorts_All(client, f'{base_path}_all')
        self.amount_range: CatalogTree_Computed_Distribution_UtxoCohorts_AmountRange = CatalogTree_Computed_Distribution_UtxoCohorts_AmountRange(client, f'{base_path}_amount_range')
        self.epoch: CatalogTree_Computed_Distribution_UtxoCohorts_Epoch = CatalogTree_Computed_Distribution_UtxoCohorts_Epoch(client, f'{base_path}_epoch')
        self.ge_amount: CatalogTree_Computed_Distribution_UtxoCohorts_GeAmount = CatalogTree_Computed_Distribution_UtxoCohorts_GeAmount(client, f'{base_path}_ge_amount')
        self.lt_amount: CatalogTree_Computed_Distribution_UtxoCohorts_LtAmount = CatalogTree_Computed_Distribution_UtxoCohorts_LtAmount(client, f'{base_path}_lt_amount')
        self.max_age: CatalogTree_Computed_Distribution_UtxoCohorts_MaxAge = CatalogTree_Computed_Distribution_UtxoCohorts_MaxAge(client, f'{base_path}_max_age')
        self.min_age: CatalogTree_Computed_Distribution_UtxoCohorts_MinAge = CatalogTree_Computed_Distribution_UtxoCohorts_MinAge(client, f'{base_path}_min_age')
        self.term: CatalogTree_Computed_Distribution_UtxoCohorts_Term = CatalogTree_Computed_Distribution_UtxoCohorts_Term(client, f'{base_path}_term')
        self.type_: CatalogTree_Computed_Distribution_UtxoCohorts_Type = CatalogTree_Computed_Distribution_UtxoCohorts_Type(client, f'{base_path}_type_')
        self.year: CatalogTree_Computed_Distribution_UtxoCohorts_Year = CatalogTree_Computed_Distribution_UtxoCohorts_Year(client, f'{base_path}_year')

class CatalogTree_Computed_Distribution_UtxoCohorts_AgeRange:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._10y_to_12y: _10yTo12yPattern = _10yTo12yPattern(client, 'utxos_at_least_10y_up_to_12y_old')
        self._12y_to_15y: _10yTo12yPattern = _10yTo12yPattern(client, 'utxos_at_least_12y_up_to_15y_old')
        self._1d_to_1w: _10yTo12yPattern = _10yTo12yPattern(client, 'utxos_at_least_1d_up_to_1w_old')
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
        self.up_to_1d: UpTo1dPattern = UpTo1dPattern(client, 'utxos_up_to_1d_old')

class CatalogTree_Computed_Distribution_UtxoCohorts_All:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.activity: ActivityPattern2 = ActivityPattern2(client, '')
        self.cost_basis: CostBasisPattern2 = CostBasisPattern2(client, '')
        self.realized: RealizedPattern3 = RealizedPattern3(client, '')
        self.relative: CatalogTree_Computed_Distribution_UtxoCohorts_All_Relative = CatalogTree_Computed_Distribution_UtxoCohorts_All_Relative(client, f'{base_path}_relative')
        self.supply: SupplyPattern3 = SupplyPattern3(client, '')
        self.unrealized: UnrealizedPattern = UnrealizedPattern(client, '')

class CatalogTree_Computed_Distribution_UtxoCohorts_All_Relative:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.neg_unrealized_loss_rel_to_own_total_unrealized_pnl: MetricPattern5[StoredF32] = MetricPattern5(client, f'{base_path}_neg_unrealized_loss_rel_to_own_total_unrealized_pnl')
        self.net_unrealized_pnl_rel_to_own_total_unrealized_pnl: MetricPattern3[StoredF32] = MetricPattern3(client, f'{base_path}_net_unrealized_pnl_rel_to_own_total_unrealized_pnl')
        self.supply_in_loss_rel_to_own_supply: MetricPattern5[StoredF64] = MetricPattern5(client, f'{base_path}_supply_in_loss_rel_to_own_supply')
        self.supply_in_profit_rel_to_own_supply: MetricPattern5[StoredF64] = MetricPattern5(client, f'{base_path}_supply_in_profit_rel_to_own_supply')
        self.unrealized_loss_rel_to_own_total_unrealized_pnl: MetricPattern5[StoredF32] = MetricPattern5(client, f'{base_path}_unrealized_loss_rel_to_own_total_unrealized_pnl')
        self.unrealized_profit_rel_to_own_total_unrealized_pnl: MetricPattern5[StoredF32] = MetricPattern5(client, f'{base_path}_unrealized_profit_rel_to_own_total_unrealized_pnl')

class CatalogTree_Computed_Distribution_UtxoCohorts_AmountRange:
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

class CatalogTree_Computed_Distribution_UtxoCohorts_Epoch:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._0: _0satsPattern2 = _0satsPattern2(client, 'epoch_0')
        self._1: _0satsPattern2 = _0satsPattern2(client, 'epoch_1')
        self._2: _0satsPattern2 = _0satsPattern2(client, 'epoch_2')
        self._3: _0satsPattern2 = _0satsPattern2(client, 'epoch_3')
        self._4: _0satsPattern2 = _0satsPattern2(client, 'epoch_4')

class CatalogTree_Computed_Distribution_UtxoCohorts_GeAmount:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._100btc: _0satsPattern2 = _0satsPattern2(client, 'utxos_above_100btc')
        self._100k_sats: _0satsPattern2 = _0satsPattern2(client, 'utxos_above_100k_sats')
        self._100sats: _0satsPattern2 = _0satsPattern2(client, 'utxos_above_100sats')
        self._10btc: _0satsPattern2 = _0satsPattern2(client, 'utxos_above_10btc')
        self._10k_btc: _0satsPattern2 = _0satsPattern2(client, 'utxos_above_10k_btc')
        self._10k_sats: _0satsPattern2 = _0satsPattern2(client, 'utxos_above_10k_sats')
        self._10m_sats: _0satsPattern2 = _0satsPattern2(client, 'utxos_above_10m_sats')
        self._10sats: _0satsPattern2 = _0satsPattern2(client, 'utxos_above_10sats')
        self._1btc: _0satsPattern2 = _0satsPattern2(client, 'utxos_above_1btc')
        self._1k_btc: _0satsPattern2 = _0satsPattern2(client, 'utxos_above_1k_btc')
        self._1k_sats: _0satsPattern2 = _0satsPattern2(client, 'utxos_above_1k_sats')
        self._1m_sats: _0satsPattern2 = _0satsPattern2(client, 'utxos_above_1m_sats')
        self._1sat: _0satsPattern2 = _0satsPattern2(client, 'utxos_above_1sat')

class CatalogTree_Computed_Distribution_UtxoCohorts_LtAmount:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._100btc: _0satsPattern2 = _0satsPattern2(client, 'utxos_under_100btc')
        self._100k_btc: _0satsPattern2 = _0satsPattern2(client, 'utxos_under_100k_btc')
        self._100k_sats: _0satsPattern2 = _0satsPattern2(client, 'utxos_under_100k_sats')
        self._100sats: _0satsPattern2 = _0satsPattern2(client, 'utxos_under_100sats')
        self._10btc: _0satsPattern2 = _0satsPattern2(client, 'utxos_under_10btc')
        self._10k_btc: _0satsPattern2 = _0satsPattern2(client, 'utxos_under_10k_btc')
        self._10k_sats: _0satsPattern2 = _0satsPattern2(client, 'utxos_under_10k_sats')
        self._10m_sats: _0satsPattern2 = _0satsPattern2(client, 'utxos_under_10m_sats')
        self._10sats: _0satsPattern2 = _0satsPattern2(client, 'utxos_under_10sats')
        self._1btc: _0satsPattern2 = _0satsPattern2(client, 'utxos_under_1btc')
        self._1k_btc: _0satsPattern2 = _0satsPattern2(client, 'utxos_under_1k_btc')
        self._1k_sats: _0satsPattern2 = _0satsPattern2(client, 'utxos_under_1k_sats')
        self._1m_sats: _0satsPattern2 = _0satsPattern2(client, 'utxos_under_1m_sats')

class CatalogTree_Computed_Distribution_UtxoCohorts_MaxAge:
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

class CatalogTree_Computed_Distribution_UtxoCohorts_MinAge:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._10y: _0satsPattern2 = _0satsPattern2(client, 'utxos_at_least_10y_old')
        self._12y: _0satsPattern2 = _0satsPattern2(client, 'utxos_at_least_12y_old')
        self._1d: _0satsPattern2 = _0satsPattern2(client, 'utxos_at_least_1d_old')
        self._1m: _0satsPattern2 = _0satsPattern2(client, 'utxos_at_least_1m_old')
        self._1w: _0satsPattern2 = _0satsPattern2(client, 'utxos_at_least_1w_old')
        self._1y: _0satsPattern2 = _0satsPattern2(client, 'utxos_at_least_1y_old')
        self._2m: _0satsPattern2 = _0satsPattern2(client, 'utxos_at_least_2m_old')
        self._2y: _0satsPattern2 = _0satsPattern2(client, 'utxos_at_least_2y_old')
        self._3m: _0satsPattern2 = _0satsPattern2(client, 'utxos_at_least_3m_old')
        self._3y: _0satsPattern2 = _0satsPattern2(client, 'utxos_at_least_3y_old')
        self._4m: _0satsPattern2 = _0satsPattern2(client, 'utxos_at_least_4m_old')
        self._4y: _0satsPattern2 = _0satsPattern2(client, 'utxos_at_least_4y_old')
        self._5m: _0satsPattern2 = _0satsPattern2(client, 'utxos_at_least_5m_old')
        self._5y: _0satsPattern2 = _0satsPattern2(client, 'utxos_at_least_5y_old')
        self._6m: _0satsPattern2 = _0satsPattern2(client, 'utxos_at_least_6m_old')
        self._6y: _0satsPattern2 = _0satsPattern2(client, 'utxos_at_least_6y_old')
        self._7y: _0satsPattern2 = _0satsPattern2(client, 'utxos_at_least_7y_old')
        self._8y: _0satsPattern2 = _0satsPattern2(client, 'utxos_at_least_8y_old')

class CatalogTree_Computed_Distribution_UtxoCohorts_Term:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.long: UpTo1dPattern = UpTo1dPattern(client, 'lth')
        self.short: UpTo1dPattern = UpTo1dPattern(client, 'sth')

class CatalogTree_Computed_Distribution_UtxoCohorts_Type:
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

class CatalogTree_Computed_Distribution_UtxoCohorts_Year:
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

class CatalogTree_Computed_Indexes:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.address: CatalogTree_Computed_Indexes_Address = CatalogTree_Computed_Indexes_Address(client, f'{base_path}_address')
        self.block: CatalogTree_Computed_Indexes_Block = CatalogTree_Computed_Indexes_Block(client, f'{base_path}_block')
        self.time: CatalogTree_Computed_Indexes_Time = CatalogTree_Computed_Indexes_Time(client, f'{base_path}_time')
        self.transaction: CatalogTree_Computed_Indexes_Transaction = CatalogTree_Computed_Indexes_Transaction(client, f'{base_path}_transaction')

class CatalogTree_Computed_Indexes_Address:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.emptyoutputindex: MetricPattern24[EmptyOutputIndex] = MetricPattern24(client, f'{base_path}_emptyoutputindex')
        self.opreturnindex: MetricPattern27[OpReturnIndex] = MetricPattern27(client, f'{base_path}_opreturnindex')
        self.p2aaddressindex: MetricPattern29[P2AAddressIndex] = MetricPattern29(client, f'{base_path}_p2aaddressindex')
        self.p2msoutputindex: MetricPattern30[P2MSOutputIndex] = MetricPattern30(client, f'{base_path}_p2msoutputindex')
        self.p2pk33addressindex: MetricPattern31[P2PK33AddressIndex] = MetricPattern31(client, f'{base_path}_p2pk33addressindex')
        self.p2pk65addressindex: MetricPattern32[P2PK65AddressIndex] = MetricPattern32(client, f'{base_path}_p2pk65addressindex')
        self.p2pkhaddressindex: MetricPattern33[P2PKHAddressIndex] = MetricPattern33(client, f'{base_path}_p2pkhaddressindex')
        self.p2shaddressindex: MetricPattern34[P2SHAddressIndex] = MetricPattern34(client, f'{base_path}_p2shaddressindex')
        self.p2traddressindex: MetricPattern35[P2TRAddressIndex] = MetricPattern35(client, f'{base_path}_p2traddressindex')
        self.p2wpkhaddressindex: MetricPattern36[P2WPKHAddressIndex] = MetricPattern36(client, f'{base_path}_p2wpkhaddressindex')
        self.p2wshaddressindex: MetricPattern37[P2WSHAddressIndex] = MetricPattern37(client, f'{base_path}_p2wshaddressindex')
        self.unknownoutputindex: MetricPattern39[UnknownOutputIndex] = MetricPattern39(client, f'{base_path}_unknownoutputindex')

class CatalogTree_Computed_Indexes_Block:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.dateindex: MetricPattern25[DateIndex] = MetricPattern25(client, f'{base_path}_dateindex')
        self.difficultyepoch: MetricPattern14[DifficultyEpoch] = MetricPattern14(client, f'{base_path}_difficultyepoch')
        self.first_height: MetricPattern13[Height] = MetricPattern13(client, f'{base_path}_first_height')
        self.halvingepoch: MetricPattern15[HalvingEpoch] = MetricPattern15(client, f'{base_path}_halvingepoch')
        self.height: MetricPattern25[Height] = MetricPattern25(client, f'{base_path}_height')
        self.height_count: MetricPattern23[StoredU64] = MetricPattern23(client, f'{base_path}_height_count')
        self.txindex_count: MetricPattern25[StoredU64] = MetricPattern25(client, f'{base_path}_txindex_count')

class CatalogTree_Computed_Indexes_Time:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.date: MetricPattern21[Date] = MetricPattern21(client, f'{base_path}_date')
        self.dateindex: MetricPattern21[DateIndex] = MetricPattern21(client, f'{base_path}_dateindex')
        self.dateindex_count: MetricPattern19[StoredU64] = MetricPattern19(client, f'{base_path}_dateindex_count')
        self.decadeindex: MetricPattern12[DecadeIndex] = MetricPattern12(client, f'{base_path}_decadeindex')
        self.first_dateindex: MetricPattern19[DateIndex] = MetricPattern19(client, f'{base_path}_first_dateindex')
        self.first_height: MetricPattern21[Height] = MetricPattern21(client, f'{base_path}_first_height')
        self.first_monthindex: MetricPattern8[MonthIndex] = MetricPattern8(client, f'{base_path}_first_monthindex')
        self.first_yearindex: MetricPattern22[YearIndex] = MetricPattern22(client, f'{base_path}_first_yearindex')
        self.height_count: MetricPattern21[StoredU64] = MetricPattern21(client, f'{base_path}_height_count')
        self.monthindex: MetricPattern10[MonthIndex] = MetricPattern10(client, f'{base_path}_monthindex')
        self.monthindex_count: MetricPattern8[StoredU64] = MetricPattern8(client, f'{base_path}_monthindex_count')
        self.quarterindex: MetricPattern17[QuarterIndex] = MetricPattern17(client, f'{base_path}_quarterindex')
        self.semesterindex: MetricPattern18[SemesterIndex] = MetricPattern18(client, f'{base_path}_semesterindex')
        self.weekindex: MetricPattern11[WeekIndex] = MetricPattern11(client, f'{base_path}_weekindex')
        self.yearindex: MetricPattern20[YearIndex] = MetricPattern20(client, f'{base_path}_yearindex')
        self.yearindex_count: MetricPattern22[StoredU64] = MetricPattern22(client, f'{base_path}_yearindex_count')

class CatalogTree_Computed_Indexes_Transaction:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.input_count: MetricPattern38[StoredU64] = MetricPattern38(client, f'{base_path}_input_count')
        self.output_count: MetricPattern38[StoredU64] = MetricPattern38(client, f'{base_path}_output_count')
        self.txindex: MetricPattern38[TxIndex] = MetricPattern38(client, f'{base_path}_txindex')
        self.txinindex: MetricPattern26[TxInIndex] = MetricPattern26(client, f'{base_path}_txinindex')
        self.txoutindex: MetricPattern28[TxOutIndex] = MetricPattern28(client, f'{base_path}_txoutindex')

class CatalogTree_Computed_Inputs:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.count: CatalogTree_Computed_Inputs_Count = CatalogTree_Computed_Inputs_Count(client, f'{base_path}_count')
        self.spent: CatalogTree_Computed_Inputs_Spent = CatalogTree_Computed_Inputs_Spent(client, f'{base_path}_spent')

class CatalogTree_Computed_Inputs_Count:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.count: BlockSizePattern[StoredU64] = BlockSizePattern(client, 'input_count')

class CatalogTree_Computed_Inputs_Spent:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.txoutindex: MetricPattern26[TxOutIndex] = MetricPattern26(client, f'{base_path}_txoutindex')
        self.value: MetricPattern26[Sats] = MetricPattern26(client, f'{base_path}_value')

class CatalogTree_Computed_Market:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.ath: CatalogTree_Computed_Market_Ath = CatalogTree_Computed_Market_Ath(client, f'{base_path}_ath')
        self.dca: CatalogTree_Computed_Market_Dca = CatalogTree_Computed_Market_Dca(client, f'{base_path}_dca')
        self.indicators: CatalogTree_Computed_Market_Indicators = CatalogTree_Computed_Market_Indicators(client, f'{base_path}_indicators')
        self.lookback: CatalogTree_Computed_Market_Lookback = CatalogTree_Computed_Market_Lookback(client, f'{base_path}_lookback')
        self.moving_average: CatalogTree_Computed_Market_MovingAverage = CatalogTree_Computed_Market_MovingAverage(client, f'{base_path}_moving_average')
        self.range: CatalogTree_Computed_Market_Range = CatalogTree_Computed_Market_Range(client, f'{base_path}_range')
        self.returns: CatalogTree_Computed_Market_Returns = CatalogTree_Computed_Market_Returns(client, f'{base_path}_returns')
        self.volatility: CatalogTree_Computed_Market_Volatility = CatalogTree_Computed_Market_Volatility(client, f'{base_path}_volatility')

class CatalogTree_Computed_Market_Ath:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.days_since_price_ath: MetricPattern4[StoredU16] = MetricPattern4(client, f'{base_path}_days_since_price_ath')
        self.max_days_between_price_aths: MetricPattern4[StoredU16] = MetricPattern4(client, f'{base_path}_max_days_between_price_aths')
        self.max_years_between_price_aths: MetricPattern4[StoredF32] = MetricPattern4(client, f'{base_path}_max_years_between_price_aths')
        self.price_ath: MetricPattern3[Dollars] = MetricPattern3(client, f'{base_path}_price_ath')
        self.price_drawdown: MetricPattern3[StoredF32] = MetricPattern3(client, f'{base_path}_price_drawdown')

class CatalogTree_Computed_Market_Dca:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.class_avg_price: ClassAvgPricePattern[Dollars] = ClassAvgPricePattern(client, 'dca_class')
        self.class_returns: ClassAvgPricePattern[StoredF32] = ClassAvgPricePattern(client, 'dca_class')
        self.class_stack: CatalogTree_Computed_Market_Dca_ClassStack = CatalogTree_Computed_Market_Dca_ClassStack(client, f'{base_path}_class_stack')
        self.period_avg_price: PeriodAvgPricePattern[Dollars] = PeriodAvgPricePattern(client, 'dca_avg_price')
        self.period_cagr: PeriodCagrPattern = PeriodCagrPattern(client, 'dca_cagr')
        self.period_lump_sum_stack: PeriodLumpSumStackPattern = PeriodLumpSumStackPattern(client, '')
        self.period_returns: PeriodAvgPricePattern[StoredF32] = PeriodAvgPricePattern(client, 'dca_returns')
        self.period_stack: PeriodLumpSumStackPattern = PeriodLumpSumStackPattern(client, '')

class CatalogTree_Computed_Market_Dca_ClassStack:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._2015: ActiveSupplyPattern = ActiveSupplyPattern(client, 'dca_class_2015_stack')
        self._2016: ActiveSupplyPattern = ActiveSupplyPattern(client, 'dca_class_2016_stack')
        self._2017: ActiveSupplyPattern = ActiveSupplyPattern(client, 'dca_class_2017_stack')
        self._2018: ActiveSupplyPattern = ActiveSupplyPattern(client, 'dca_class_2018_stack')
        self._2019: ActiveSupplyPattern = ActiveSupplyPattern(client, 'dca_class_2019_stack')
        self._2020: ActiveSupplyPattern = ActiveSupplyPattern(client, 'dca_class_2020_stack')
        self._2021: ActiveSupplyPattern = ActiveSupplyPattern(client, 'dca_class_2021_stack')
        self._2022: ActiveSupplyPattern = ActiveSupplyPattern(client, 'dca_class_2022_stack')
        self._2023: ActiveSupplyPattern = ActiveSupplyPattern(client, 'dca_class_2023_stack')
        self._2024: ActiveSupplyPattern = ActiveSupplyPattern(client, 'dca_class_2024_stack')
        self._2025: ActiveSupplyPattern = ActiveSupplyPattern(client, 'dca_class_2025_stack')

class CatalogTree_Computed_Market_Indicators:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.gini: MetricPattern21[StoredF32] = MetricPattern21(client, f'{base_path}_gini')
        self.macd_histogram: MetricPattern21[StoredF32] = MetricPattern21(client, f'{base_path}_macd_histogram')
        self.macd_line: MetricPattern21[StoredF32] = MetricPattern21(client, f'{base_path}_macd_line')
        self.macd_signal: MetricPattern21[StoredF32] = MetricPattern21(client, f'{base_path}_macd_signal')
        self.nvt: MetricPattern21[StoredF32] = MetricPattern21(client, f'{base_path}_nvt')
        self.pi_cycle: MetricPattern21[StoredF32] = MetricPattern21(client, f'{base_path}_pi_cycle')
        self.puell_multiple: MetricPattern4[StoredF32] = MetricPattern4(client, f'{base_path}_puell_multiple')
        self.rsi_14d: MetricPattern21[StoredF32] = MetricPattern21(client, f'{base_path}_rsi_14d')
        self.rsi_14d_max: MetricPattern21[StoredF32] = MetricPattern21(client, f'{base_path}_rsi_14d_max')
        self.rsi_14d_min: MetricPattern21[StoredF32] = MetricPattern21(client, f'{base_path}_rsi_14d_min')
        self.rsi_avg_gain_14d: MetricPattern21[StoredF32] = MetricPattern21(client, f'{base_path}_rsi_avg_gain_14d')
        self.rsi_avg_loss_14d: MetricPattern21[StoredF32] = MetricPattern21(client, f'{base_path}_rsi_avg_loss_14d')
        self.rsi_gains: MetricPattern21[StoredF32] = MetricPattern21(client, f'{base_path}_rsi_gains')
        self.rsi_losses: MetricPattern21[StoredF32] = MetricPattern21(client, f'{base_path}_rsi_losses')
        self.stoch_d: MetricPattern21[StoredF32] = MetricPattern21(client, f'{base_path}_stoch_d')
        self.stoch_k: MetricPattern21[StoredF32] = MetricPattern21(client, f'{base_path}_stoch_k')
        self.stoch_rsi: MetricPattern21[StoredF32] = MetricPattern21(client, f'{base_path}_stoch_rsi')
        self.stoch_rsi_d: MetricPattern21[StoredF32] = MetricPattern21(client, f'{base_path}_stoch_rsi_d')
        self.stoch_rsi_k: MetricPattern21[StoredF32] = MetricPattern21(client, f'{base_path}_stoch_rsi_k')

class CatalogTree_Computed_Market_Lookback:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.price_ago: PriceAgoPattern[Dollars] = PriceAgoPattern(client, 'price')

class CatalogTree_Computed_Market_MovingAverage:
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

class CatalogTree_Computed_Market_Range:
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
        self.price_true_range: MetricPattern21[StoredF32] = MetricPattern21(client, f'{base_path}_price_true_range')
        self.price_true_range_2w_sum: MetricPattern21[StoredF32] = MetricPattern21(client, f'{base_path}_price_true_range_2w_sum')

class CatalogTree_Computed_Market_Returns:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._1d_returns_1m_sd: _1dReturns1mSdPattern = _1dReturns1mSdPattern(client, '1d_returns_1m_sd')
        self._1d_returns_1w_sd: _1dReturns1mSdPattern = _1dReturns1mSdPattern(client, '1d_returns_1w_sd')
        self._1d_returns_1y_sd: _1dReturns1mSdPattern = _1dReturns1mSdPattern(client, '1d_returns_1y_sd')
        self.cagr: PeriodCagrPattern = PeriodCagrPattern(client, 'cagr')
        self.downside_1m_sd: _1dReturns1mSdPattern = _1dReturns1mSdPattern(client, 'downside_1m_sd')
        self.downside_1w_sd: _1dReturns1mSdPattern = _1dReturns1mSdPattern(client, 'downside_1w_sd')
        self.downside_1y_sd: _1dReturns1mSdPattern = _1dReturns1mSdPattern(client, 'downside_1y_sd')
        self.downside_returns: MetricPattern21[StoredF32] = MetricPattern21(client, f'{base_path}_downside_returns')
        self.price_returns: PriceAgoPattern[StoredF32] = PriceAgoPattern(client, 'price_returns')

class CatalogTree_Computed_Market_Volatility:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.price_1m_volatility: MetricPattern4[StoredF32] = MetricPattern4(client, f'{base_path}_price_1m_volatility')
        self.price_1w_volatility: MetricPattern4[StoredF32] = MetricPattern4(client, f'{base_path}_price_1w_volatility')
        self.price_1y_volatility: MetricPattern4[StoredF32] = MetricPattern4(client, f'{base_path}_price_1y_volatility')
        self.sharpe_1m: MetricPattern21[StoredF32] = MetricPattern21(client, f'{base_path}_sharpe_1m')
        self.sharpe_1w: MetricPattern21[StoredF32] = MetricPattern21(client, f'{base_path}_sharpe_1w')
        self.sharpe_1y: MetricPattern21[StoredF32] = MetricPattern21(client, f'{base_path}_sharpe_1y')
        self.sortino_1m: MetricPattern21[StoredF32] = MetricPattern21(client, f'{base_path}_sortino_1m')
        self.sortino_1w: MetricPattern21[StoredF32] = MetricPattern21(client, f'{base_path}_sortino_1w')
        self.sortino_1y: MetricPattern21[StoredF32] = MetricPattern21(client, f'{base_path}_sortino_1y')

class CatalogTree_Computed_Outputs:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.count: CatalogTree_Computed_Outputs_Count = CatalogTree_Computed_Outputs_Count(client, f'{base_path}_count')
        self.spent: CatalogTree_Computed_Outputs_Spent = CatalogTree_Computed_Outputs_Spent(client, f'{base_path}_spent')

class CatalogTree_Computed_Outputs_Count:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.count: BlockSizePattern[StoredU64] = BlockSizePattern(client, 'output_count')
        self.utxo_count: BitcoinPattern[StoredU64] = BitcoinPattern(client, 'exact_utxo_count')

class CatalogTree_Computed_Outputs_Spent:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.txinindex: MetricPattern28[TxInIndex] = MetricPattern28(client, f'{base_path}_txinindex')

class CatalogTree_Computed_Pools:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.pool: MetricPattern25[PoolSlug] = MetricPattern25(client, f'{base_path}_pool')
        self.vecs: CatalogTree_Computed_Pools_Vecs = CatalogTree_Computed_Pools_Vecs(client, f'{base_path}_vecs')

class CatalogTree_Computed_Pools_Vecs:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.axbt: AXbtPattern = AXbtPattern(client, 'axbt')
        self.aaopool: AXbtPattern = AXbtPattern(client, 'aaopool')
        self.antpool: AXbtPattern = AXbtPattern(client, 'antpool')
        self.arkpool: AXbtPattern = AXbtPattern(client, 'arkpool')
        self.asicminer: AXbtPattern = AXbtPattern(client, 'asicminer')
        self.batpool: AXbtPattern = AXbtPattern(client, 'batpool')
        self.bcmonster: AXbtPattern = AXbtPattern(client, 'bcmonster')
        self.bcpoolio: AXbtPattern = AXbtPattern(client, 'bcpoolio')
        self.binancepool: AXbtPattern = AXbtPattern(client, 'binancepool')
        self.bitclub: AXbtPattern = AXbtPattern(client, 'bitclub')
        self.bitfufupool: AXbtPattern = AXbtPattern(client, 'bitfufupool')
        self.bitfury: AXbtPattern = AXbtPattern(client, 'bitfury')
        self.bitminter: AXbtPattern = AXbtPattern(client, 'bitminter')
        self.bitalo: AXbtPattern = AXbtPattern(client, 'bitalo')
        self.bitcoinaffiliatenetwork: AXbtPattern = AXbtPattern(client, 'bitcoinaffiliatenetwork')
        self.bitcoincom: AXbtPattern = AXbtPattern(client, 'bitcoincom')
        self.bitcoinindia: AXbtPattern = AXbtPattern(client, 'bitcoinindia')
        self.bitcoinrussia: AXbtPattern = AXbtPattern(client, 'bitcoinrussia')
        self.bitcoinukraine: AXbtPattern = AXbtPattern(client, 'bitcoinukraine')
        self.bitfarms: AXbtPattern = AXbtPattern(client, 'bitfarms')
        self.bitparking: AXbtPattern = AXbtPattern(client, 'bitparking')
        self.bitsolo: AXbtPattern = AXbtPattern(client, 'bitsolo')
        self.bixin: AXbtPattern = AXbtPattern(client, 'bixin')
        self.blockfills: AXbtPattern = AXbtPattern(client, 'blockfills')
        self.braiinspool: AXbtPattern = AXbtPattern(client, 'braiinspool')
        self.bravomining: AXbtPattern = AXbtPattern(client, 'bravomining')
        self.btpool: AXbtPattern = AXbtPattern(client, 'btpool')
        self.btccom: AXbtPattern = AXbtPattern(client, 'btccom')
        self.btcdig: AXbtPattern = AXbtPattern(client, 'btcdig')
        self.btcguild: AXbtPattern = AXbtPattern(client, 'btcguild')
        self.btclab: AXbtPattern = AXbtPattern(client, 'btclab')
        self.btcmp: AXbtPattern = AXbtPattern(client, 'btcmp')
        self.btcnuggets: AXbtPattern = AXbtPattern(client, 'btcnuggets')
        self.btcpoolparty: AXbtPattern = AXbtPattern(client, 'btcpoolparty')
        self.btcserv: AXbtPattern = AXbtPattern(client, 'btcserv')
        self.btctop: AXbtPattern = AXbtPattern(client, 'btctop')
        self.btcc: AXbtPattern = AXbtPattern(client, 'btcc')
        self.bwpool: AXbtPattern = AXbtPattern(client, 'bwpool')
        self.bytepool: AXbtPattern = AXbtPattern(client, 'bytepool')
        self.canoe: AXbtPattern = AXbtPattern(client, 'canoe')
        self.canoepool: AXbtPattern = AXbtPattern(client, 'canoepool')
        self.carbonnegative: AXbtPattern = AXbtPattern(client, 'carbonnegative')
        self.ckpool: AXbtPattern = AXbtPattern(client, 'ckpool')
        self.cloudhashing: AXbtPattern = AXbtPattern(client, 'cloudhashing')
        self.coinlab: AXbtPattern = AXbtPattern(client, 'coinlab')
        self.cointerra: AXbtPattern = AXbtPattern(client, 'cointerra')
        self.connectbtc: AXbtPattern = AXbtPattern(client, 'connectbtc')
        self.dpool: AXbtPattern = AXbtPattern(client, 'dpool')
        self.dcexploration: AXbtPattern = AXbtPattern(client, 'dcexploration')
        self.dcex: AXbtPattern = AXbtPattern(client, 'dcex')
        self.digitalbtc: AXbtPattern = AXbtPattern(client, 'digitalbtc')
        self.digitalxmintsy: AXbtPattern = AXbtPattern(client, 'digitalxmintsy')
        self.eclipsemc: AXbtPattern = AXbtPattern(client, 'eclipsemc')
        self.eightbaochi: AXbtPattern = AXbtPattern(client, 'eightbaochi')
        self.ekanembtc: AXbtPattern = AXbtPattern(client, 'ekanembtc')
        self.eligius: AXbtPattern = AXbtPattern(client, 'eligius')
        self.emcdpool: AXbtPattern = AXbtPattern(client, 'emcdpool')
        self.entrustcharitypool: AXbtPattern = AXbtPattern(client, 'entrustcharitypool')
        self.eobot: AXbtPattern = AXbtPattern(client, 'eobot')
        self.exxbw: AXbtPattern = AXbtPattern(client, 'exxbw')
        self.f2pool: AXbtPattern = AXbtPattern(client, 'f2pool')
        self.fiftyeightcoin: AXbtPattern = AXbtPattern(client, 'fiftyeightcoin')
        self.foundryusa: AXbtPattern = AXbtPattern(client, 'foundryusa')
        self.futurebitapollosolo: AXbtPattern = AXbtPattern(client, 'futurebitapollosolo')
        self.gbminers: AXbtPattern = AXbtPattern(client, 'gbminers')
        self.ghashio: AXbtPattern = AXbtPattern(client, 'ghashio')
        self.givemecoins: AXbtPattern = AXbtPattern(client, 'givemecoins')
        self.gogreenlight: AXbtPattern = AXbtPattern(client, 'gogreenlight')
        self.haozhuzhu: AXbtPattern = AXbtPattern(client, 'haozhuzhu')
        self.haominer: AXbtPattern = AXbtPattern(client, 'haominer')
        self.hashbx: AXbtPattern = AXbtPattern(client, 'hashbx')
        self.hashpool: AXbtPattern = AXbtPattern(client, 'hashpool')
        self.helix: AXbtPattern = AXbtPattern(client, 'helix')
        self.hhtt: AXbtPattern = AXbtPattern(client, 'hhtt')
        self.hotpool: AXbtPattern = AXbtPattern(client, 'hotpool')
        self.hummerpool: AXbtPattern = AXbtPattern(client, 'hummerpool')
        self.huobipool: AXbtPattern = AXbtPattern(client, 'huobipool')
        self.innopolistech: AXbtPattern = AXbtPattern(client, 'innopolistech')
        self.kanopool: AXbtPattern = AXbtPattern(client, 'kanopool')
        self.kncminer: AXbtPattern = AXbtPattern(client, 'kncminer')
        self.kucoinpool: AXbtPattern = AXbtPattern(client, 'kucoinpool')
        self.lubiancom: AXbtPattern = AXbtPattern(client, 'lubiancom')
        self.luckypool: AXbtPattern = AXbtPattern(client, 'luckypool')
        self.luxor: AXbtPattern = AXbtPattern(client, 'luxor')
        self.marapool: AXbtPattern = AXbtPattern(client, 'marapool')
        self.maxbtc: AXbtPattern = AXbtPattern(client, 'maxbtc')
        self.maxipool: AXbtPattern = AXbtPattern(client, 'maxipool')
        self.megabigpower: AXbtPattern = AXbtPattern(client, 'megabigpower')
        self.minerium: AXbtPattern = AXbtPattern(client, 'minerium')
        self.miningcity: AXbtPattern = AXbtPattern(client, 'miningcity')
        self.miningdutch: AXbtPattern = AXbtPattern(client, 'miningdutch')
        self.miningkings: AXbtPattern = AXbtPattern(client, 'miningkings')
        self.miningsquared: AXbtPattern = AXbtPattern(client, 'miningsquared')
        self.mmpool: AXbtPattern = AXbtPattern(client, 'mmpool')
        self.mtred: AXbtPattern = AXbtPattern(client, 'mtred')
        self.multicoinco: AXbtPattern = AXbtPattern(client, 'multicoinco')
        self.multipool: AXbtPattern = AXbtPattern(client, 'multipool')
        self.mybtccoinpool: AXbtPattern = AXbtPattern(client, 'mybtccoinpool')
        self.neopool: AXbtPattern = AXbtPattern(client, 'neopool')
        self.nexious: AXbtPattern = AXbtPattern(client, 'nexious')
        self.nicehash: AXbtPattern = AXbtPattern(client, 'nicehash')
        self.nmcbit: AXbtPattern = AXbtPattern(client, 'nmcbit')
        self.novablock: AXbtPattern = AXbtPattern(client, 'novablock')
        self.ocean: AXbtPattern = AXbtPattern(client, 'ocean')
        self.okexpool: AXbtPattern = AXbtPattern(client, 'okexpool')
        self.okminer: AXbtPattern = AXbtPattern(client, 'okminer')
        self.okkong: AXbtPattern = AXbtPattern(client, 'okkong')
        self.okpooltop: AXbtPattern = AXbtPattern(client, 'okpooltop')
        self.onehash: AXbtPattern = AXbtPattern(client, 'onehash')
        self.onem1x: AXbtPattern = AXbtPattern(client, 'onem1x')
        self.onethash: AXbtPattern = AXbtPattern(client, 'onethash')
        self.ozcoin: AXbtPattern = AXbtPattern(client, 'ozcoin')
        self.phashio: AXbtPattern = AXbtPattern(client, 'phashio')
        self.parasite: AXbtPattern = AXbtPattern(client, 'parasite')
        self.patels: AXbtPattern = AXbtPattern(client, 'patels')
        self.pegapool: AXbtPattern = AXbtPattern(client, 'pegapool')
        self.phoenix: AXbtPattern = AXbtPattern(client, 'phoenix')
        self.polmine: AXbtPattern = AXbtPattern(client, 'polmine')
        self.pool175btc: AXbtPattern = AXbtPattern(client, 'pool175btc')
        self.pool50btc: AXbtPattern = AXbtPattern(client, 'pool50btc')
        self.poolin: AXbtPattern = AXbtPattern(client, 'poolin')
        self.portlandhodl: AXbtPattern = AXbtPattern(client, 'portlandhodl')
        self.publicpool: AXbtPattern = AXbtPattern(client, 'publicpool')
        self.purebtccom: AXbtPattern = AXbtPattern(client, 'purebtccom')
        self.rawpool: AXbtPattern = AXbtPattern(client, 'rawpool')
        self.rigpool: AXbtPattern = AXbtPattern(client, 'rigpool')
        self.sbicrypto: AXbtPattern = AXbtPattern(client, 'sbicrypto')
        self.secpool: AXbtPattern = AXbtPattern(client, 'secpool')
        self.secretsuperstar: AXbtPattern = AXbtPattern(client, 'secretsuperstar')
        self.sevenpool: AXbtPattern = AXbtPattern(client, 'sevenpool')
        self.shawnp0wers: AXbtPattern = AXbtPattern(client, 'shawnp0wers')
        self.sigmapoolcom: AXbtPattern = AXbtPattern(client, 'sigmapoolcom')
        self.simplecoinus: AXbtPattern = AXbtPattern(client, 'simplecoinus')
        self.solock: AXbtPattern = AXbtPattern(client, 'solock')
        self.spiderpool: AXbtPattern = AXbtPattern(client, 'spiderpool')
        self.stminingcorp: AXbtPattern = AXbtPattern(client, 'stminingcorp')
        self.tangpool: AXbtPattern = AXbtPattern(client, 'tangpool')
        self.tatmaspool: AXbtPattern = AXbtPattern(client, 'tatmaspool')
        self.tbdice: AXbtPattern = AXbtPattern(client, 'tbdice')
        self.telco214: AXbtPattern = AXbtPattern(client, 'telco214')
        self.terrapool: AXbtPattern = AXbtPattern(client, 'terrapool')
        self.tiger: AXbtPattern = AXbtPattern(client, 'tiger')
        self.tigerpoolnet: AXbtPattern = AXbtPattern(client, 'tigerpoolnet')
        self.titan: AXbtPattern = AXbtPattern(client, 'titan')
        self.transactioncoinmining: AXbtPattern = AXbtPattern(client, 'transactioncoinmining')
        self.trickysbtcpool: AXbtPattern = AXbtPattern(client, 'trickysbtcpool')
        self.triplemining: AXbtPattern = AXbtPattern(client, 'triplemining')
        self.twentyoneinc: AXbtPattern = AXbtPattern(client, 'twentyoneinc')
        self.ultimuspool: AXbtPattern = AXbtPattern(client, 'ultimuspool')
        self.unknown: AXbtPattern = AXbtPattern(client, 'unknown')
        self.unomp: AXbtPattern = AXbtPattern(client, 'unomp')
        self.viabtc: AXbtPattern = AXbtPattern(client, 'viabtc')
        self.waterhole: AXbtPattern = AXbtPattern(client, 'waterhole')
        self.wayicn: AXbtPattern = AXbtPattern(client, 'wayicn')
        self.whitepool: AXbtPattern = AXbtPattern(client, 'whitepool')
        self.wk057: AXbtPattern = AXbtPattern(client, 'wk057')
        self.yourbtcnet: AXbtPattern = AXbtPattern(client, 'yourbtcnet')
        self.zulupool: AXbtPattern = AXbtPattern(client, 'zulupool')

class CatalogTree_Computed_Positions:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.position: MetricPattern16[BlkPosition] = MetricPattern16(client, f'{base_path}_position')

class CatalogTree_Computed_Price:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.ohlc: CatalogTree_Computed_Price_Ohlc = CatalogTree_Computed_Price_Ohlc(client, f'{base_path}_ohlc')
        self.sats: CatalogTree_Computed_Price_Sats = CatalogTree_Computed_Price_Sats(client, f'{base_path}_sats')
        self.usd: CatalogTree_Computed_Price_Usd = CatalogTree_Computed_Price_Usd(client, f'{base_path}_usd')

class CatalogTree_Computed_Price_Ohlc:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.ohlc_in_cents: MetricPattern9[OHLCCents] = MetricPattern9(client, f'{base_path}_ohlc_in_cents')

class CatalogTree_Computed_Price_Sats:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.price_close_in_sats: MetricPattern1[Sats] = MetricPattern1(client, f'{base_path}_price_close_in_sats')
        self.price_high_in_sats: MetricPattern1[Sats] = MetricPattern1(client, f'{base_path}_price_high_in_sats')
        self.price_low_in_sats: MetricPattern1[Sats] = MetricPattern1(client, f'{base_path}_price_low_in_sats')
        self.price_ohlc_in_sats: MetricPattern1[OHLCSats] = MetricPattern1(client, f'{base_path}_price_ohlc_in_sats')
        self.price_open_in_sats: MetricPattern1[Sats] = MetricPattern1(client, f'{base_path}_price_open_in_sats')

class CatalogTree_Computed_Price_Usd:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.price_close: MetricPattern1[Dollars] = MetricPattern1(client, f'{base_path}_price_close')
        self.price_close_in_cents: MetricPattern9[Cents] = MetricPattern9(client, f'{base_path}_price_close_in_cents')
        self.price_high: MetricPattern1[Dollars] = MetricPattern1(client, f'{base_path}_price_high')
        self.price_high_in_cents: MetricPattern9[Cents] = MetricPattern9(client, f'{base_path}_price_high_in_cents')
        self.price_low: MetricPattern1[Dollars] = MetricPattern1(client, f'{base_path}_price_low')
        self.price_low_in_cents: MetricPattern9[Cents] = MetricPattern9(client, f'{base_path}_price_low_in_cents')
        self.price_ohlc: MetricPattern1[OHLCDollars] = MetricPattern1(client, f'{base_path}_price_ohlc')
        self.price_open: MetricPattern1[Dollars] = MetricPattern1(client, f'{base_path}_price_open')
        self.price_open_in_cents: MetricPattern9[Cents] = MetricPattern9(client, f'{base_path}_price_open_in_cents')

class CatalogTree_Computed_Scripts:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.count: CatalogTree_Computed_Scripts_Count = CatalogTree_Computed_Scripts_Count(client, f'{base_path}_count')
        self.value: CatalogTree_Computed_Scripts_Value = CatalogTree_Computed_Scripts_Value(client, f'{base_path}_value')

class CatalogTree_Computed_Scripts_Count:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.emptyoutput_count: BitcoinPattern[StoredU64] = BitcoinPattern(client, 'emptyoutput_count')
        self.opreturn_count: BitcoinPattern[StoredU64] = BitcoinPattern(client, 'opreturn_count')
        self.p2a_count: BitcoinPattern[StoredU64] = BitcoinPattern(client, 'p2a_count')
        self.p2ms_count: BitcoinPattern[StoredU64] = BitcoinPattern(client, 'p2ms_count')
        self.p2pk33_count: BitcoinPattern[StoredU64] = BitcoinPattern(client, 'p2pk33_count')
        self.p2pk65_count: BitcoinPattern[StoredU64] = BitcoinPattern(client, 'p2pk65_count')
        self.p2pkh_count: BitcoinPattern[StoredU64] = BitcoinPattern(client, 'p2pkh_count')
        self.p2sh_count: BitcoinPattern[StoredU64] = BitcoinPattern(client, 'p2sh_count')
        self.p2tr_count: BitcoinPattern[StoredU64] = BitcoinPattern(client, 'p2tr_count')
        self.p2wpkh_count: BitcoinPattern[StoredU64] = BitcoinPattern(client, 'p2wpkh_count')
        self.p2wsh_count: BitcoinPattern[StoredU64] = BitcoinPattern(client, 'p2wsh_count')
        self.segwit_adoption: SegwitAdoptionPattern[StoredF32] = SegwitAdoptionPattern(client, 'segwit_adoption')
        self.segwit_count: BitcoinPattern[StoredU64] = BitcoinPattern(client, 'segwit_count')
        self.taproot_adoption: SegwitAdoptionPattern[StoredF32] = SegwitAdoptionPattern(client, 'taproot_adoption')
        self.unknownoutput_count: BitcoinPattern[StoredU64] = BitcoinPattern(client, 'unknownoutput_count')

class CatalogTree_Computed_Scripts_Value:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.opreturn_value: CatalogTree_Computed_Scripts_Value_OpreturnValue = CatalogTree_Computed_Scripts_Value_OpreturnValue(client, f'{base_path}_opreturn_value')

class CatalogTree_Computed_Scripts_Value_OpreturnValue:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.base: MetricPattern25[Sats] = MetricPattern25(client, f'{base_path}_base')
        self.bitcoin: SegwitAdoptionPattern[Bitcoin] = SegwitAdoptionPattern(client, 'opreturn_value_btc')
        self.dollars: SegwitAdoptionPattern[Dollars] = SegwitAdoptionPattern(client, 'opreturn_value_usd')
        self.sats: CatalogTree_Computed_Scripts_Value_OpreturnValue_Sats = CatalogTree_Computed_Scripts_Value_OpreturnValue_Sats(client, f'{base_path}_sats')

class CatalogTree_Computed_Scripts_Value_OpreturnValue_Sats:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.average: MetricPattern2[Sats] = MetricPattern2(client, f'{base_path}_average')
        self.cumulative: MetricPattern1[Sats] = MetricPattern1(client, f'{base_path}_cumulative')
        self.max: MetricPattern2[Sats] = MetricPattern2(client, f'{base_path}_max')
        self.min: MetricPattern2[Sats] = MetricPattern2(client, f'{base_path}_min')
        self.sum: MetricPattern2[Sats] = MetricPattern2(client, f'{base_path}_sum')

class CatalogTree_Computed_Supply:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.burned: CatalogTree_Computed_Supply_Burned = CatalogTree_Computed_Supply_Burned(client, f'{base_path}_burned')
        self.circulating: CatalogTree_Computed_Supply_Circulating = CatalogTree_Computed_Supply_Circulating(client, f'{base_path}_circulating')
        self.inflation: CatalogTree_Computed_Supply_Inflation = CatalogTree_Computed_Supply_Inflation(client, f'{base_path}_inflation')
        self.market_cap: CatalogTree_Computed_Supply_MarketCap = CatalogTree_Computed_Supply_MarketCap(client, f'{base_path}_market_cap')
        self.velocity: CatalogTree_Computed_Supply_Velocity = CatalogTree_Computed_Supply_Velocity(client, f'{base_path}_velocity')

class CatalogTree_Computed_Supply_Burned:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.opreturn: OpreturnPattern = OpreturnPattern(client, 'opreturn_supply')
        self.unspendable: OpreturnPattern = OpreturnPattern(client, 'unspendable_supply')

class CatalogTree_Computed_Supply_Circulating:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.btc: MetricPattern25[Bitcoin] = MetricPattern25(client, f'{base_path}_btc')
        self.indexes: ActiveSupplyPattern = ActiveSupplyPattern(client, 'circulating')
        self.sats: MetricPattern25[Sats] = MetricPattern25(client, f'{base_path}_sats')
        self.usd: MetricPattern25[Dollars] = MetricPattern25(client, f'{base_path}_usd')

class CatalogTree_Computed_Supply_Inflation:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.indexes: MetricPattern4[StoredF32] = MetricPattern4(client, f'{base_path}_indexes')

class CatalogTree_Computed_Supply_MarketCap:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.height: MetricPattern25[Dollars] = MetricPattern25(client, f'{base_path}_height')
        self.indexes: MetricPattern4[Dollars] = MetricPattern4(client, f'{base_path}_indexes')

class CatalogTree_Computed_Supply_Velocity:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.btc: MetricPattern4[StoredF64] = MetricPattern4(client, f'{base_path}_btc')
        self.usd: MetricPattern4[StoredF64] = MetricPattern4(client, f'{base_path}_usd')

class CatalogTree_Computed_Transactions:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.count: CatalogTree_Computed_Transactions_Count = CatalogTree_Computed_Transactions_Count(client, f'{base_path}_count')
        self.fees: CatalogTree_Computed_Transactions_Fees = CatalogTree_Computed_Transactions_Fees(client, f'{base_path}_fees')
        self.size: CatalogTree_Computed_Transactions_Size = CatalogTree_Computed_Transactions_Size(client, f'{base_path}_size')
        self.versions: CatalogTree_Computed_Transactions_Versions = CatalogTree_Computed_Transactions_Versions(client, f'{base_path}_versions')
        self.volume: CatalogTree_Computed_Transactions_Volume = CatalogTree_Computed_Transactions_Volume(client, f'{base_path}_volume')

class CatalogTree_Computed_Transactions_Count:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.is_coinbase: MetricPattern38[StoredBool] = MetricPattern38(client, f'{base_path}_is_coinbase')
        self.tx_count: BitcoinPattern[StoredU64] = BitcoinPattern(client, 'tx_count')

class CatalogTree_Computed_Transactions_Fees:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.fee: CatalogTree_Computed_Transactions_Fees_Fee = CatalogTree_Computed_Transactions_Fees_Fee(client, f'{base_path}_fee')
        self.fee_rate: CatalogTree_Computed_Transactions_Fees_FeeRate = CatalogTree_Computed_Transactions_Fees_FeeRate(client, f'{base_path}_fee_rate')
        self.input_value: MetricPattern38[Sats] = MetricPattern38(client, f'{base_path}_input_value')
        self.output_value: MetricPattern38[Sats] = MetricPattern38(client, f'{base_path}_output_value')

class CatalogTree_Computed_Transactions_Fees_Fee:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.base: MetricPattern38[Sats] = MetricPattern38(client, f'{base_path}_base')
        self.bitcoin: BlockSizePattern[Bitcoin] = BlockSizePattern(client, 'fee_btc')
        self.bitcoin_txindex: MetricPattern38[Bitcoin] = MetricPattern38(client, f'{base_path}_bitcoin_txindex')
        self.dollars: BlockSizePattern[Dollars] = BlockSizePattern(client, 'fee_usd')
        self.dollars_txindex: MetricPattern38[Dollars] = MetricPattern38(client, f'{base_path}_dollars_txindex')
        self.sats: BlockSizePattern[Sats] = BlockSizePattern(client, 'fee')

class CatalogTree_Computed_Transactions_Fees_FeeRate:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.average: MetricPattern1[FeeRate] = MetricPattern1(client, f'{base_path}_average')
        self.base: MetricPattern38[FeeRate] = MetricPattern38(client, f'{base_path}_base')
        self.max: MetricPattern1[FeeRate] = MetricPattern1(client, f'{base_path}_max')
        self.median: MetricPattern25[FeeRate] = MetricPattern25(client, f'{base_path}_median')
        self.min: MetricPattern1[FeeRate] = MetricPattern1(client, f'{base_path}_min')
        self.pct10: MetricPattern25[FeeRate] = MetricPattern25(client, f'{base_path}_pct10')
        self.pct25: MetricPattern25[FeeRate] = MetricPattern25(client, f'{base_path}_pct25')
        self.pct75: MetricPattern25[FeeRate] = MetricPattern25(client, f'{base_path}_pct75')
        self.pct90: MetricPattern25[FeeRate] = MetricPattern25(client, f'{base_path}_pct90')

class CatalogTree_Computed_Transactions_Size:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.tx_vsize: BlockIntervalPattern[VSize] = BlockIntervalPattern(client, 'tx_vsize')
        self.tx_weight: BlockIntervalPattern[Weight] = BlockIntervalPattern(client, 'tx_weight')
        self.vsize: MetricPattern38[VSize] = MetricPattern38(client, f'{base_path}_vsize')
        self.weight: MetricPattern38[Weight] = MetricPattern38(client, f'{base_path}_weight')

class CatalogTree_Computed_Transactions_Versions:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.tx_v1: BlockCountPattern[StoredU64] = BlockCountPattern(client, 'tx_v1')
        self.tx_v2: BlockCountPattern[StoredU64] = BlockCountPattern(client, 'tx_v2')
        self.tx_v3: BlockCountPattern[StoredU64] = BlockCountPattern(client, 'tx_v3')

class CatalogTree_Computed_Transactions_Volume:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.annualized_volume: MetricPattern4[Sats] = MetricPattern4(client, f'{base_path}_annualized_volume')
        self.annualized_volume_btc: MetricPattern4[Bitcoin] = MetricPattern4(client, f'{base_path}_annualized_volume_btc')
        self.annualized_volume_usd: MetricPattern4[Dollars] = MetricPattern4(client, f'{base_path}_annualized_volume_usd')
        self.inputs_per_sec: MetricPattern4[StoredF32] = MetricPattern4(client, f'{base_path}_inputs_per_sec')
        self.outputs_per_sec: MetricPattern4[StoredF32] = MetricPattern4(client, f'{base_path}_outputs_per_sec')
        self.sent_sum: CatalogTree_Computed_Transactions_Volume_SentSum = CatalogTree_Computed_Transactions_Volume_SentSum(client, f'{base_path}_sent_sum')
        self.tx_per_sec: MetricPattern4[StoredF32] = MetricPattern4(client, f'{base_path}_tx_per_sec')

class CatalogTree_Computed_Transactions_Volume_SentSum:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.bitcoin: TotalRealizedPnlPattern[Bitcoin] = TotalRealizedPnlPattern(client, 'sent_sum_btc')
        self.dollars: MetricPattern1[Dollars] = MetricPattern1(client, f'{base_path}_dollars')
        self.sats: MetricPattern1[Sats] = MetricPattern1(client, f'{base_path}_sats')

class CatalogTree_Indexed:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.address: CatalogTree_Indexed_Address = CatalogTree_Indexed_Address(client, f'{base_path}_address')
        self.block: CatalogTree_Indexed_Block = CatalogTree_Indexed_Block(client, f'{base_path}_block')
        self.output: CatalogTree_Indexed_Output = CatalogTree_Indexed_Output(client, f'{base_path}_output')
        self.tx: CatalogTree_Indexed_Tx = CatalogTree_Indexed_Tx(client, f'{base_path}_tx')
        self.txin: CatalogTree_Indexed_Txin = CatalogTree_Indexed_Txin(client, f'{base_path}_txin')
        self.txout: CatalogTree_Indexed_Txout = CatalogTree_Indexed_Txout(client, f'{base_path}_txout')

class CatalogTree_Indexed_Address:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.first_p2aaddressindex: MetricPattern25[P2AAddressIndex] = MetricPattern25(client, f'{base_path}_first_p2aaddressindex')
        self.first_p2pk33addressindex: MetricPattern25[P2PK33AddressIndex] = MetricPattern25(client, f'{base_path}_first_p2pk33addressindex')
        self.first_p2pk65addressindex: MetricPattern25[P2PK65AddressIndex] = MetricPattern25(client, f'{base_path}_first_p2pk65addressindex')
        self.first_p2pkhaddressindex: MetricPattern25[P2PKHAddressIndex] = MetricPattern25(client, f'{base_path}_first_p2pkhaddressindex')
        self.first_p2shaddressindex: MetricPattern25[P2SHAddressIndex] = MetricPattern25(client, f'{base_path}_first_p2shaddressindex')
        self.first_p2traddressindex: MetricPattern25[P2TRAddressIndex] = MetricPattern25(client, f'{base_path}_first_p2traddressindex')
        self.first_p2wpkhaddressindex: MetricPattern25[P2WPKHAddressIndex] = MetricPattern25(client, f'{base_path}_first_p2wpkhaddressindex')
        self.first_p2wshaddressindex: MetricPattern25[P2WSHAddressIndex] = MetricPattern25(client, f'{base_path}_first_p2wshaddressindex')
        self.p2abytes: MetricPattern29[P2ABytes] = MetricPattern29(client, f'{base_path}_p2abytes')
        self.p2pk33bytes: MetricPattern31[P2PK33Bytes] = MetricPattern31(client, f'{base_path}_p2pk33bytes')
        self.p2pk65bytes: MetricPattern32[P2PK65Bytes] = MetricPattern32(client, f'{base_path}_p2pk65bytes')
        self.p2pkhbytes: MetricPattern33[P2PKHBytes] = MetricPattern33(client, f'{base_path}_p2pkhbytes')
        self.p2shbytes: MetricPattern34[P2SHBytes] = MetricPattern34(client, f'{base_path}_p2shbytes')
        self.p2trbytes: MetricPattern35[P2TRBytes] = MetricPattern35(client, f'{base_path}_p2trbytes')
        self.p2wpkhbytes: MetricPattern36[P2WPKHBytes] = MetricPattern36(client, f'{base_path}_p2wpkhbytes')
        self.p2wshbytes: MetricPattern37[P2WSHBytes] = MetricPattern37(client, f'{base_path}_p2wshbytes')

class CatalogTree_Indexed_Block:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.blockhash: MetricPattern25[BlockHash] = MetricPattern25(client, f'{base_path}_blockhash')
        self.difficulty: MetricPattern25[StoredF64] = MetricPattern25(client, f'{base_path}_difficulty')
        self.timestamp: MetricPattern25[Timestamp] = MetricPattern25(client, f'{base_path}_timestamp')
        self.total_size: MetricPattern25[StoredU64] = MetricPattern25(client, f'{base_path}_total_size')
        self.weight: MetricPattern25[Weight] = MetricPattern25(client, f'{base_path}_weight')

class CatalogTree_Indexed_Output:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.first_emptyoutputindex: MetricPattern25[EmptyOutputIndex] = MetricPattern25(client, f'{base_path}_first_emptyoutputindex')
        self.first_opreturnindex: MetricPattern25[OpReturnIndex] = MetricPattern25(client, f'{base_path}_first_opreturnindex')
        self.first_p2msoutputindex: MetricPattern25[P2MSOutputIndex] = MetricPattern25(client, f'{base_path}_first_p2msoutputindex')
        self.first_unknownoutputindex: MetricPattern25[UnknownOutputIndex] = MetricPattern25(client, f'{base_path}_first_unknownoutputindex')
        self.txindex: MetricPattern7[TxIndex] = MetricPattern7(client, f'{base_path}_txindex')

class CatalogTree_Indexed_Tx:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.base_size: MetricPattern38[StoredU32] = MetricPattern38(client, f'{base_path}_base_size')
        self.first_txindex: MetricPattern25[TxIndex] = MetricPattern25(client, f'{base_path}_first_txindex')
        self.first_txinindex: MetricPattern38[TxInIndex] = MetricPattern38(client, f'{base_path}_first_txinindex')
        self.first_txoutindex: MetricPattern38[TxOutIndex] = MetricPattern38(client, f'{base_path}_first_txoutindex')
        self.height: MetricPattern38[Height] = MetricPattern38(client, f'{base_path}_height')
        self.is_explicitly_rbf: MetricPattern38[StoredBool] = MetricPattern38(client, f'{base_path}_is_explicitly_rbf')
        self.rawlocktime: MetricPattern38[RawLockTime] = MetricPattern38(client, f'{base_path}_rawlocktime')
        self.total_size: MetricPattern38[StoredU32] = MetricPattern38(client, f'{base_path}_total_size')
        self.txid: MetricPattern38[Txid] = MetricPattern38(client, f'{base_path}_txid')
        self.txversion: MetricPattern38[TxVersion] = MetricPattern38(client, f'{base_path}_txversion')

class CatalogTree_Indexed_Txin:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.first_txinindex: MetricPattern25[TxInIndex] = MetricPattern25(client, f'{base_path}_first_txinindex')
        self.outpoint: MetricPattern26[OutPoint] = MetricPattern26(client, f'{base_path}_outpoint')
        self.outputtype: MetricPattern26[OutputType] = MetricPattern26(client, f'{base_path}_outputtype')
        self.txindex: MetricPattern26[TxIndex] = MetricPattern26(client, f'{base_path}_txindex')
        self.typeindex: MetricPattern26[TypeIndex] = MetricPattern26(client, f'{base_path}_typeindex')

class CatalogTree_Indexed_Txout:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.first_txoutindex: MetricPattern25[TxOutIndex] = MetricPattern25(client, f'{base_path}_first_txoutindex')
        self.outputtype: MetricPattern28[OutputType] = MetricPattern28(client, f'{base_path}_outputtype')
        self.txindex: MetricPattern28[TxIndex] = MetricPattern28(client, f'{base_path}_txindex')
        self.typeindex: MetricPattern28[TypeIndex] = MetricPattern28(client, f'{base_path}_typeindex')
        self.value: MetricPattern28[Sats] = MetricPattern28(client, f'{base_path}_value')

class BrkClient(BrkClientBase):
    """Main BRK client with catalog tree and API methods."""

    VERSION = "v0.1.0-alpha.1"

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
      "up_to_1d": {
        "id": "up_to_1d_old",
        "short": "<1d",
        "long": "Up to 1 Day Old"
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

    def get_metric_by_index(self, metric: Metric, index: Index, from_: Optional[Any] = None, to: Optional[Any] = None, count: Optional[Any] = None, format: Optional[Format] = None) -> MetricData:
        """Get metric data.

        Fetch data for a specific metric at the given index. Use query parameters to filter by date range and format (json/csv)."""
        params = []
        if from_ is not None: params.append(f'from={from_}')
        if to is not None: params.append(f'to={to}')
        if count is not None: params.append(f'count={count}')
        if format is not None: params.append(f'format={format}')
        query = '&'.join(params)
        return self.get(f'/api/metric/{metric}/{index}{"?" + query if query else ""}')

    def get_metrics_bulk(self, metrics: Metrics, index: Index, from_: Optional[Any] = None, to: Optional[Any] = None, count: Optional[Any] = None, format: Optional[Format] = None) -> List[MetricData]:
        """Bulk metric data.

        Fetch multiple metrics in a single request. Supports filtering by index and date range. Returns an array of MetricData objects."""
        params = []
        params.append(f'metrics={metrics}')
        params.append(f'index={index}')
        if from_ is not None: params.append(f'from={from_}')
        if to is not None: params.append(f'to={to}')
        if count is not None: params.append(f'count={count}')
        if format is not None: params.append(f'format={format}')
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

