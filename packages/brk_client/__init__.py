# Auto-generated BRK Python client
# Do not edit manually

from __future__ import annotations
from typing import TypeVar, Generic, Any, Optional, List, Literal, TypedDict, Final
import httpx

T = TypeVar('T')

# Constants

VERSION: Final[str] = "v0.1.0-alpha.1"

INDEXES: Final[tuple[str, ...]] = (
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
)

POOL_ID_TO_POOL_NAME: Final[dict[str, str]] = {
    "pool175btc": "175btc",
    "onehash": "1Hash",
    "onem1x": "1M1X",
    "onethash": "1THash",
    "twentyoneinc": "21 Inc.",
    "pool50btc": "50BTC",
    "fiftyeightcoin": "58COIN",
    "sevenpool": "7pool",
    "eightbaochi": "8baochi",
    "axbt": "A-XBT",
    "aaopool": "AAO Pool",
    "antpool": "AntPool",
    "arkpool": "ArkPool",
    "asicminer": "ASICMiner",
    "batpool": "BATPOOL",
    "bcmonster": "BCMonster",
    "bcpoolio": "bcpool.io",
    "binancepool": "Binance Pool",
    "bitalo": "Bitalo",
    "bitclub": "BitClub",
    "bitcoinaffiliatenetwork": "Bitcoin Affiliate Network",
    "bitcoinindia": "Bitcoin India",
    "bitcoinukraine": "Bitcoin-Ukraine",
    "bitcoincom": "Bitcoin.com",
    "bitcoinrussia": "BitcoinRussia",
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
    "btcguild": "BTC Guild",
    "btcnuggets": "BTC Nuggets",
    "btcpoolparty": "BTC Pool Party",
    "btccom": "BTC.com",
    "btctop": "BTC.TOP",
    "btcc": "BTCC",
    "btcdig": "BTCDig",
    "btclab": "BTCLab",
    "btcmp": "BTCMP",
    "btcserv": "BTCServ",
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
    "ekanembtc": "EkanemBTC",
    "eligius": "Eligius",
    "emcdpool": "EMCDPool",
    "entrustcharitypool": "Entrust Charity Pool",
    "eobot": "Eobot",
    "exxbw": "EXX&BW",
    "f2pool": "F2Pool",
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
    "miningsquared": "Mining Squared",
    "miningdutch": "Mining-Dutch",
    "miningcity": "MiningCity",
    "miningkings": "MiningKings",
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
    "ozcoin": "OzCoin",
    "parasite": "Parasite",
    "patels": "Patels",
    "pegapool": "PEGA Pool",
    "phashio": "PHash.IO",
    "phoenix": "Phoenix",
    "polmine": "Polmine",
    "poolin": "Poolin",
    "portlandhodl": "Portland.HODL",
    "publicpool": "Public Pool",
    "purebtccom": "PureBTC.COM",
    "rawpool": "Rawpool",
    "rigpool": "RigPool",
    "sbicrypto": "SBI Crypto",
    "secpool": "SECPOOL",
    "secretsuperstar": "SecretSuperstar",
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
    "ultimuspool": "ULTIMUSPOOL",
    "unknown": "Unknown",
    "unomp": "UNOMP",
    "viabtc": "ViaBTC",
    "waterhole": "Waterhole",
    "wayicn": "WAYI.CN",
    "whitepool": "WhitePool",
    "wk057": "wk057",
    "yourbtcnet": "Yourbtc.net",
    "zulupool": "Zulupool",
}

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
    mempool_stats: AddressMempoolStats | None

Txid = str
class AddressTxidsParam(TypedDict):
    after_txid: Txid | None
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
    height: Height | None
    next_best: BlockHash | None

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
    prevout: TxOut | None
    scriptsig: str
    scriptsig_asm: str
    is_coinbase: bool
    sequence: int
    inner_redeemscript_asm: Optional[str]

class TxStatus(TypedDict):
    confirmed: bool
    block_height: Height | None
    block_hash: BlockHash | None
    block_time: Timestamp | None

TxVersion = int
class Transaction(TypedDict):
    index: TxIndex | None
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
    txid: Txid | None
    vin: Vin | None
    status: TxStatus | None

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

TreeNode = dict[str, "TreeNode"] | MetricLeafWithSchema

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
            response = self._client.get(f"{self.base_url}{path}")
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


class MetricNode(Generic[T]):
    """A metric node that can fetch data for different indexes."""

    def __init__(self, client: BrkClientBase, path: str):
        self._client = client
        self._path = path

    def get(self) -> List[T]:
        """Fetch all data points for this metric."""
        return self._client.get(self._path)

    def get_range(self, from_date: str, to_date: str) -> List[T]:
        """Fetch data points within a date range."""
        return self._client.get(f"{self._path}?from={from_date}&to={to_date}")


# Index accessor classes

class Indexes3(Generic[T]):
    """Index accessor for metrics with 9 indexes."""
    
    def __init__(self, client: BrkClientBase, base_path: str):
        self.by_dateindex: MetricNode[T] = MetricNode(client, f'{base_path}/dateindex')
        self.by_decadeindex: MetricNode[T] = MetricNode(client, f'{base_path}/decadeindex')
        self.by_difficultyepoch: MetricNode[T] = MetricNode(client, f'{base_path}/difficultyepoch')
        self.by_height: MetricNode[T] = MetricNode(client, f'{base_path}/height')
        self.by_monthindex: MetricNode[T] = MetricNode(client, f'{base_path}/monthindex')
        self.by_quarterindex: MetricNode[T] = MetricNode(client, f'{base_path}/quarterindex')
        self.by_semesterindex: MetricNode[T] = MetricNode(client, f'{base_path}/semesterindex')
        self.by_weekindex: MetricNode[T] = MetricNode(client, f'{base_path}/weekindex')
        self.by_yearindex: MetricNode[T] = MetricNode(client, f'{base_path}/yearindex')

class Indexes4(Generic[T]):
    """Index accessor for metrics with 8 indexes."""
    
    def __init__(self, client: BrkClientBase, base_path: str):
        self.by_dateindex: MetricNode[T] = MetricNode(client, f'{base_path}/dateindex')
        self.by_decadeindex: MetricNode[T] = MetricNode(client, f'{base_path}/decadeindex')
        self.by_difficultyepoch: MetricNode[T] = MetricNode(client, f'{base_path}/difficultyepoch')
        self.by_monthindex: MetricNode[T] = MetricNode(client, f'{base_path}/monthindex')
        self.by_quarterindex: MetricNode[T] = MetricNode(client, f'{base_path}/quarterindex')
        self.by_semesterindex: MetricNode[T] = MetricNode(client, f'{base_path}/semesterindex')
        self.by_weekindex: MetricNode[T] = MetricNode(client, f'{base_path}/weekindex')
        self.by_yearindex: MetricNode[T] = MetricNode(client, f'{base_path}/yearindex')

class Indexes26(Generic[T]):
    """Index accessor for metrics with 8 indexes."""
    
    def __init__(self, client: BrkClientBase, base_path: str):
        self.by_dateindex: MetricNode[T] = MetricNode(client, f'{base_path}/dateindex')
        self.by_decadeindex: MetricNode[T] = MetricNode(client, f'{base_path}/decadeindex')
        self.by_height: MetricNode[T] = MetricNode(client, f'{base_path}/height')
        self.by_monthindex: MetricNode[T] = MetricNode(client, f'{base_path}/monthindex')
        self.by_quarterindex: MetricNode[T] = MetricNode(client, f'{base_path}/quarterindex')
        self.by_semesterindex: MetricNode[T] = MetricNode(client, f'{base_path}/semesterindex')
        self.by_weekindex: MetricNode[T] = MetricNode(client, f'{base_path}/weekindex')
        self.by_yearindex: MetricNode[T] = MetricNode(client, f'{base_path}/yearindex')

class Indexes(Generic[T]):
    """Index accessor for metrics with 7 indexes."""
    
    def __init__(self, client: BrkClientBase, base_path: str):
        self.by_dateindex: MetricNode[T] = MetricNode(client, f'{base_path}/dateindex')
        self.by_decadeindex: MetricNode[T] = MetricNode(client, f'{base_path}/decadeindex')
        self.by_monthindex: MetricNode[T] = MetricNode(client, f'{base_path}/monthindex')
        self.by_quarterindex: MetricNode[T] = MetricNode(client, f'{base_path}/quarterindex')
        self.by_semesterindex: MetricNode[T] = MetricNode(client, f'{base_path}/semesterindex')
        self.by_weekindex: MetricNode[T] = MetricNode(client, f'{base_path}/weekindex')
        self.by_yearindex: MetricNode[T] = MetricNode(client, f'{base_path}/yearindex')

class Indexes27(Generic[T]):
    """Index accessor for metrics with 7 indexes."""
    
    def __init__(self, client: BrkClientBase, base_path: str):
        self.by_decadeindex: MetricNode[T] = MetricNode(client, f'{base_path}/decadeindex')
        self.by_height: MetricNode[T] = MetricNode(client, f'{base_path}/height')
        self.by_monthindex: MetricNode[T] = MetricNode(client, f'{base_path}/monthindex')
        self.by_quarterindex: MetricNode[T] = MetricNode(client, f'{base_path}/quarterindex')
        self.by_semesterindex: MetricNode[T] = MetricNode(client, f'{base_path}/semesterindex')
        self.by_weekindex: MetricNode[T] = MetricNode(client, f'{base_path}/weekindex')
        self.by_yearindex: MetricNode[T] = MetricNode(client, f'{base_path}/yearindex')

class Indexes28(Generic[T]):
    """Index accessor for metrics with 6 indexes."""
    
    def __init__(self, client: BrkClientBase, base_path: str):
        self.by_decadeindex: MetricNode[T] = MetricNode(client, f'{base_path}/decadeindex')
        self.by_monthindex: MetricNode[T] = MetricNode(client, f'{base_path}/monthindex')
        self.by_quarterindex: MetricNode[T] = MetricNode(client, f'{base_path}/quarterindex')
        self.by_semesterindex: MetricNode[T] = MetricNode(client, f'{base_path}/semesterindex')
        self.by_weekindex: MetricNode[T] = MetricNode(client, f'{base_path}/weekindex')
        self.by_yearindex: MetricNode[T] = MetricNode(client, f'{base_path}/yearindex')

class Indexes15(Generic[T]):
    """Index accessor for metrics with 3 indexes."""
    
    def __init__(self, client: BrkClientBase, base_path: str):
        self.by_quarterindex: MetricNode[T] = MetricNode(client, f'{base_path}/quarterindex')
        self.by_semesterindex: MetricNode[T] = MetricNode(client, f'{base_path}/semesterindex')
        self.by_yearindex: MetricNode[T] = MetricNode(client, f'{base_path}/yearindex')

class Indexes13(Generic[T]):
    """Index accessor for metrics with 2 indexes."""
    
    def __init__(self, client: BrkClientBase, base_path: str):
        self.by_dateindex: MetricNode[T] = MetricNode(client, f'{base_path}/dateindex')
        self.by_height: MetricNode[T] = MetricNode(client, f'{base_path}/height')

class Indexes14(Generic[T]):
    """Index accessor for metrics with 2 indexes."""
    
    def __init__(self, client: BrkClientBase, base_path: str):
        self.by_monthindex: MetricNode[T] = MetricNode(client, f'{base_path}/monthindex')
        self.by_weekindex: MetricNode[T] = MetricNode(client, f'{base_path}/weekindex')

class Indexes2(Generic[T]):
    """Index accessor for metrics with 1 indexes."""
    
    def __init__(self, client: BrkClientBase, base_path: str):
        self.by_height: MetricNode[T] = MetricNode(client, f'{base_path}/height')

class Indexes5(Generic[T]):
    """Index accessor for metrics with 1 indexes."""
    
    def __init__(self, client: BrkClientBase, base_path: str):
        self.by_dateindex: MetricNode[T] = MetricNode(client, f'{base_path}/dateindex')

class Indexes6(Generic[T]):
    """Index accessor for metrics with 1 indexes."""
    
    def __init__(self, client: BrkClientBase, base_path: str):
        self.by_txindex: MetricNode[T] = MetricNode(client, f'{base_path}/txindex')

class Indexes7(Generic[T]):
    """Index accessor for metrics with 1 indexes."""
    
    def __init__(self, client: BrkClientBase, base_path: str):
        self.by_decadeindex: MetricNode[T] = MetricNode(client, f'{base_path}/decadeindex')

class Indexes8(Generic[T]):
    """Index accessor for metrics with 1 indexes."""
    
    def __init__(self, client: BrkClientBase, base_path: str):
        self.by_monthindex: MetricNode[T] = MetricNode(client, f'{base_path}/monthindex')

class Indexes9(Generic[T]):
    """Index accessor for metrics with 1 indexes."""
    
    def __init__(self, client: BrkClientBase, base_path: str):
        self.by_quarterindex: MetricNode[T] = MetricNode(client, f'{base_path}/quarterindex')

class Indexes10(Generic[T]):
    """Index accessor for metrics with 1 indexes."""
    
    def __init__(self, client: BrkClientBase, base_path: str):
        self.by_semesterindex: MetricNode[T] = MetricNode(client, f'{base_path}/semesterindex')

class Indexes11(Generic[T]):
    """Index accessor for metrics with 1 indexes."""
    
    def __init__(self, client: BrkClientBase, base_path: str):
        self.by_weekindex: MetricNode[T] = MetricNode(client, f'{base_path}/weekindex')

class Indexes12(Generic[T]):
    """Index accessor for metrics with 1 indexes."""
    
    def __init__(self, client: BrkClientBase, base_path: str):
        self.by_yearindex: MetricNode[T] = MetricNode(client, f'{base_path}/yearindex')

class Indexes16(Generic[T]):
    """Index accessor for metrics with 1 indexes."""
    
    def __init__(self, client: BrkClientBase, base_path: str):
        self.by_p2aaddressindex: MetricNode[T] = MetricNode(client, f'{base_path}/p2aaddressindex')

class Indexes17(Generic[T]):
    """Index accessor for metrics with 1 indexes."""
    
    def __init__(self, client: BrkClientBase, base_path: str):
        self.by_p2pk33addressindex: MetricNode[T] = MetricNode(client, f'{base_path}/p2pk33addressindex')

class Indexes18(Generic[T]):
    """Index accessor for metrics with 1 indexes."""
    
    def __init__(self, client: BrkClientBase, base_path: str):
        self.by_p2pk65addressindex: MetricNode[T] = MetricNode(client, f'{base_path}/p2pk65addressindex')

class Indexes19(Generic[T]):
    """Index accessor for metrics with 1 indexes."""
    
    def __init__(self, client: BrkClientBase, base_path: str):
        self.by_p2pkhaddressindex: MetricNode[T] = MetricNode(client, f'{base_path}/p2pkhaddressindex')

class Indexes20(Generic[T]):
    """Index accessor for metrics with 1 indexes."""
    
    def __init__(self, client: BrkClientBase, base_path: str):
        self.by_p2shaddressindex: MetricNode[T] = MetricNode(client, f'{base_path}/p2shaddressindex')

class Indexes21(Generic[T]):
    """Index accessor for metrics with 1 indexes."""
    
    def __init__(self, client: BrkClientBase, base_path: str):
        self.by_p2traddressindex: MetricNode[T] = MetricNode(client, f'{base_path}/p2traddressindex')

class Indexes22(Generic[T]):
    """Index accessor for metrics with 1 indexes."""
    
    def __init__(self, client: BrkClientBase, base_path: str):
        self.by_p2wpkhaddressindex: MetricNode[T] = MetricNode(client, f'{base_path}/p2wpkhaddressindex')

class Indexes23(Generic[T]):
    """Index accessor for metrics with 1 indexes."""
    
    def __init__(self, client: BrkClientBase, base_path: str):
        self.by_p2wshaddressindex: MetricNode[T] = MetricNode(client, f'{base_path}/p2wshaddressindex')

class Indexes24(Generic[T]):
    """Index accessor for metrics with 1 indexes."""
    
    def __init__(self, client: BrkClientBase, base_path: str):
        self.by_txinindex: MetricNode[T] = MetricNode(client, f'{base_path}/txinindex')

class Indexes25(Generic[T]):
    """Index accessor for metrics with 1 indexes."""
    
    def __init__(self, client: BrkClientBase, base_path: str):
        self.by_txoutindex: MetricNode[T] = MetricNode(client, f'{base_path}/txoutindex')

class Indexes29(Generic[T]):
    """Index accessor for metrics with 1 indexes."""
    
    def __init__(self, client: BrkClientBase, base_path: str):
        self.by_emptyaddressindex: MetricNode[T] = MetricNode(client, f'{base_path}/emptyaddressindex')

class Indexes30(Generic[T]):
    """Index accessor for metrics with 1 indexes."""
    
    def __init__(self, client: BrkClientBase, base_path: str):
        self.by_loadedaddressindex: MetricNode[T] = MetricNode(client, f'{base_path}/loadedaddressindex')

# Reusable structural pattern classes

class RealizedPattern3:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, base_path: str):
        self.adjusted_sopr: Indexes5[StoredF64] = Indexes5(client, f'{base_path}/adjusted_sopr')
        self.adjusted_sopr_30d_ema: Indexes5[StoredF64] = Indexes5(client, f'{base_path}/adjusted_sopr_30d_ema')
        self.adjusted_sopr_7d_ema: Indexes5[StoredF64] = Indexes5(client, f'{base_path}/adjusted_sopr_7d_ema')
        self.adjusted_value_created: Indexes3[Dollars] = Indexes3(client, f'{base_path}/adjusted_value_created')
        self.adjusted_value_destroyed: Indexes3[Dollars] = Indexes3(client, f'{base_path}/adjusted_value_destroyed')
        self.neg_realized_loss: BlockCountPattern[Dollars] = BlockCountPattern(client, f'{base_path}/neg_realized_loss')
        self.net_realized_pnl: BlockCountPattern[Dollars] = BlockCountPattern(client, f'{base_path}/net_realized_pnl')
        self.net_realized_pnl_cumulative_30d_delta: Indexes[Dollars] = Indexes(client, f'{base_path}/net_realized_pnl_cumulative_30d_delta')
        self.net_realized_pnl_cumulative_30d_delta_rel_to_market_cap: Indexes[StoredF32] = Indexes(client, f'{base_path}/net_realized_pnl_cumulative_30d_delta_rel_to_market_cap')
        self.net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap: Indexes[StoredF32] = Indexes(client, f'{base_path}/net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap')
        self.net_realized_pnl_rel_to_realized_cap: Indexes2[StoredF32] = Indexes2(client, f'{base_path}/net_realized_pnl_rel_to_realized_cap')
        self.realized_cap: Indexes3[Dollars] = Indexes3(client, f'{base_path}/realized_cap')
        self.realized_cap_30d_delta: Indexes[Dollars] = Indexes(client, f'{base_path}/realized_cap_30d_delta')
        self.realized_cap_rel_to_own_market_cap: Indexes3[StoredF32] = Indexes3(client, f'{base_path}/realized_cap_rel_to_own_market_cap')
        self.realized_loss: BlockCountPattern[Dollars] = BlockCountPattern(client, f'{base_path}/realized_loss')
        self.realized_loss_rel_to_realized_cap: Indexes2[StoredF32] = Indexes2(client, f'{base_path}/realized_loss_rel_to_realized_cap')
        self.realized_price: Indexes3[Dollars] = Indexes3(client, f'{base_path}/realized_price')
        self.realized_price_extra: ActivePriceRatioPattern = ActivePriceRatioPattern(client, f'{base_path}/realized_price_extra')
        self.realized_profit: BlockCountPattern[Dollars] = BlockCountPattern(client, f'{base_path}/realized_profit')
        self.realized_profit_rel_to_realized_cap: Indexes2[StoredF32] = Indexes2(client, f'{base_path}/realized_profit_rel_to_realized_cap')
        self.realized_profit_to_loss_ratio: Indexes5[StoredF64] = Indexes5(client, f'{base_path}/realized_profit_to_loss_ratio')
        self.realized_value: Indexes3[Dollars] = Indexes3(client, f'{base_path}/realized_value')
        self.sell_side_risk_ratio: Indexes5[StoredF32] = Indexes5(client, f'{base_path}/sell_side_risk_ratio')
        self.sell_side_risk_ratio_30d_ema: Indexes5[StoredF32] = Indexes5(client, f'{base_path}/sell_side_risk_ratio_30d_ema')
        self.sell_side_risk_ratio_7d_ema: Indexes5[StoredF32] = Indexes5(client, f'{base_path}/sell_side_risk_ratio_7d_ema')
        self.sopr: Indexes5[StoredF64] = Indexes5(client, f'{base_path}/sopr')
        self.sopr_30d_ema: Indexes5[StoredF64] = Indexes5(client, f'{base_path}/sopr_30d_ema')
        self.sopr_7d_ema: Indexes5[StoredF64] = Indexes5(client, f'{base_path}/sopr_7d_ema')
        self.total_realized_pnl: BitcoinPattern2[Dollars] = BitcoinPattern2(client, f'{base_path}/total_realized_pnl')
        self.value_created: Indexes3[Dollars] = Indexes3(client, f'{base_path}/value_created')
        self.value_destroyed: Indexes3[Dollars] = Indexes3(client, f'{base_path}/value_destroyed')

class Ratio1ySdPattern2:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, base_path: str):
        self._0sd_usd: Indexes[Dollars] = Indexes(client, f'{base_path}/_0sd_usd')
        self.m0_5sd: Indexes[StoredF32] = Indexes(client, f'{base_path}/m0_5sd')
        self.m0_5sd_usd: Indexes[Dollars] = Indexes(client, f'{base_path}/m0_5sd_usd')
        self.m1_5sd: Indexes[StoredF32] = Indexes(client, f'{base_path}/m1_5sd')
        self.m1_5sd_usd: Indexes[Dollars] = Indexes(client, f'{base_path}/m1_5sd_usd')
        self.m1sd: Indexes[StoredF32] = Indexes(client, f'{base_path}/m1sd')
        self.m1sd_usd: Indexes[Dollars] = Indexes(client, f'{base_path}/m1sd_usd')
        self.m2_5sd: Indexes[StoredF32] = Indexes(client, f'{base_path}/m2_5sd')
        self.m2_5sd_usd: Indexes[Dollars] = Indexes(client, f'{base_path}/m2_5sd_usd')
        self.m2sd: Indexes[StoredF32] = Indexes(client, f'{base_path}/m2sd')
        self.m2sd_usd: Indexes[Dollars] = Indexes(client, f'{base_path}/m2sd_usd')
        self.m3sd: Indexes[StoredF32] = Indexes(client, f'{base_path}/m3sd')
        self.m3sd_usd: Indexes[Dollars] = Indexes(client, f'{base_path}/m3sd_usd')
        self.p0_5sd: Indexes[StoredF32] = Indexes(client, f'{base_path}/p0_5sd')
        self.p0_5sd_usd: Indexes[Dollars] = Indexes(client, f'{base_path}/p0_5sd_usd')
        self.p1_5sd: Indexes[StoredF32] = Indexes(client, f'{base_path}/p1_5sd')
        self.p1_5sd_usd: Indexes[Dollars] = Indexes(client, f'{base_path}/p1_5sd_usd')
        self.p1sd: Indexes[StoredF32] = Indexes(client, f'{base_path}/p1sd')
        self.p1sd_usd: Indexes[Dollars] = Indexes(client, f'{base_path}/p1sd_usd')
        self.p2_5sd: Indexes[StoredF32] = Indexes(client, f'{base_path}/p2_5sd')
        self.p2_5sd_usd: Indexes[Dollars] = Indexes(client, f'{base_path}/p2_5sd_usd')
        self.p2sd: Indexes[StoredF32] = Indexes(client, f'{base_path}/p2sd')
        self.p2sd_usd: Indexes[Dollars] = Indexes(client, f'{base_path}/p2sd_usd')
        self.p3sd: Indexes[StoredF32] = Indexes(client, f'{base_path}/p3sd')
        self.p3sd_usd: Indexes[Dollars] = Indexes(client, f'{base_path}/p3sd_usd')
        self.sd: Indexes[StoredF32] = Indexes(client, f'{base_path}/sd')
        self.sma: Indexes[StoredF32] = Indexes(client, f'{base_path}/sma')
        self.zscore: Indexes[StoredF32] = Indexes(client, f'{base_path}/zscore')

class RealizedPattern2:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, base_path: str):
        self.neg_realized_loss: BlockCountPattern[Dollars] = BlockCountPattern(client, f'{base_path}/neg_realized_loss')
        self.net_realized_pnl: BlockCountPattern[Dollars] = BlockCountPattern(client, f'{base_path}/net_realized_pnl')
        self.net_realized_pnl_cumulative_30d_delta: Indexes[Dollars] = Indexes(client, f'{base_path}/net_realized_pnl_cumulative_30d_delta')
        self.net_realized_pnl_cumulative_30d_delta_rel_to_market_cap: Indexes[StoredF32] = Indexes(client, f'{base_path}/net_realized_pnl_cumulative_30d_delta_rel_to_market_cap')
        self.net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap: Indexes[StoredF32] = Indexes(client, f'{base_path}/net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap')
        self.net_realized_pnl_rel_to_realized_cap: Indexes2[StoredF32] = Indexes2(client, f'{base_path}/net_realized_pnl_rel_to_realized_cap')
        self.realized_cap: Indexes3[Dollars] = Indexes3(client, f'{base_path}/realized_cap')
        self.realized_cap_30d_delta: Indexes[Dollars] = Indexes(client, f'{base_path}/realized_cap_30d_delta')
        self.realized_cap_rel_to_own_market_cap: Indexes3[StoredF32] = Indexes3(client, f'{base_path}/realized_cap_rel_to_own_market_cap')
        self.realized_loss: BlockCountPattern[Dollars] = BlockCountPattern(client, f'{base_path}/realized_loss')
        self.realized_loss_rel_to_realized_cap: Indexes2[StoredF32] = Indexes2(client, f'{base_path}/realized_loss_rel_to_realized_cap')
        self.realized_price: Indexes3[Dollars] = Indexes3(client, f'{base_path}/realized_price')
        self.realized_price_extra: ActivePriceRatioPattern = ActivePriceRatioPattern(client, f'{base_path}/realized_price_extra')
        self.realized_profit: BlockCountPattern[Dollars] = BlockCountPattern(client, f'{base_path}/realized_profit')
        self.realized_profit_rel_to_realized_cap: Indexes2[StoredF32] = Indexes2(client, f'{base_path}/realized_profit_rel_to_realized_cap')
        self.realized_profit_to_loss_ratio: Indexes5[StoredF64] = Indexes5(client, f'{base_path}/realized_profit_to_loss_ratio')
        self.realized_value: Indexes3[Dollars] = Indexes3(client, f'{base_path}/realized_value')
        self.sell_side_risk_ratio: Indexes5[StoredF32] = Indexes5(client, f'{base_path}/sell_side_risk_ratio')
        self.sell_side_risk_ratio_30d_ema: Indexes5[StoredF32] = Indexes5(client, f'{base_path}/sell_side_risk_ratio_30d_ema')
        self.sell_side_risk_ratio_7d_ema: Indexes5[StoredF32] = Indexes5(client, f'{base_path}/sell_side_risk_ratio_7d_ema')
        self.sopr: Indexes5[StoredF64] = Indexes5(client, f'{base_path}/sopr')
        self.sopr_30d_ema: Indexes5[StoredF64] = Indexes5(client, f'{base_path}/sopr_30d_ema')
        self.sopr_7d_ema: Indexes5[StoredF64] = Indexes5(client, f'{base_path}/sopr_7d_ema')
        self.total_realized_pnl: BitcoinPattern2[Dollars] = BitcoinPattern2(client, f'{base_path}/total_realized_pnl')
        self.value_created: Indexes3[Dollars] = Indexes3(client, f'{base_path}/value_created')
        self.value_destroyed: Indexes3[Dollars] = Indexes3(client, f'{base_path}/value_destroyed')

class RealizedPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, base_path: str):
        self.neg_realized_loss: BlockCountPattern[Dollars] = BlockCountPattern(client, f'{base_path}/neg_realized_loss')
        self.net_realized_pnl: BlockCountPattern[Dollars] = BlockCountPattern(client, f'{base_path}/net_realized_pnl')
        self.net_realized_pnl_cumulative_30d_delta: Indexes[Dollars] = Indexes(client, f'{base_path}/net_realized_pnl_cumulative_30d_delta')
        self.net_realized_pnl_cumulative_30d_delta_rel_to_market_cap: Indexes[StoredF32] = Indexes(client, f'{base_path}/net_realized_pnl_cumulative_30d_delta_rel_to_market_cap')
        self.net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap: Indexes[StoredF32] = Indexes(client, f'{base_path}/net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap')
        self.net_realized_pnl_rel_to_realized_cap: Indexes2[StoredF32] = Indexes2(client, f'{base_path}/net_realized_pnl_rel_to_realized_cap')
        self.realized_cap: Indexes3[Dollars] = Indexes3(client, f'{base_path}/realized_cap')
        self.realized_cap_30d_delta: Indexes[Dollars] = Indexes(client, f'{base_path}/realized_cap_30d_delta')
        self.realized_loss: BlockCountPattern[Dollars] = BlockCountPattern(client, f'{base_path}/realized_loss')
        self.realized_loss_rel_to_realized_cap: Indexes2[StoredF32] = Indexes2(client, f'{base_path}/realized_loss_rel_to_realized_cap')
        self.realized_price: Indexes3[Dollars] = Indexes3(client, f'{base_path}/realized_price')
        self.realized_price_extra: RealizedPriceExtraPattern = RealizedPriceExtraPattern(client, f'{base_path}/realized_price_extra')
        self.realized_profit: BlockCountPattern[Dollars] = BlockCountPattern(client, f'{base_path}/realized_profit')
        self.realized_profit_rel_to_realized_cap: Indexes2[StoredF32] = Indexes2(client, f'{base_path}/realized_profit_rel_to_realized_cap')
        self.realized_value: Indexes3[Dollars] = Indexes3(client, f'{base_path}/realized_value')
        self.sell_side_risk_ratio: Indexes5[StoredF32] = Indexes5(client, f'{base_path}/sell_side_risk_ratio')
        self.sell_side_risk_ratio_30d_ema: Indexes5[StoredF32] = Indexes5(client, f'{base_path}/sell_side_risk_ratio_30d_ema')
        self.sell_side_risk_ratio_7d_ema: Indexes5[StoredF32] = Indexes5(client, f'{base_path}/sell_side_risk_ratio_7d_ema')
        self.sopr: Indexes5[StoredF64] = Indexes5(client, f'{base_path}/sopr')
        self.sopr_30d_ema: Indexes5[StoredF64] = Indexes5(client, f'{base_path}/sopr_30d_ema')
        self.sopr_7d_ema: Indexes5[StoredF64] = Indexes5(client, f'{base_path}/sopr_7d_ema')
        self.total_realized_pnl: BitcoinPattern2[Dollars] = BitcoinPattern2(client, f'{base_path}/total_realized_pnl')
        self.value_created: Indexes3[Dollars] = Indexes3(client, f'{base_path}/value_created')
        self.value_destroyed: Indexes3[Dollars] = Indexes3(client, f'{base_path}/value_destroyed')

class Price13dEmaPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.price: Indexes[Dollars] = Indexes(client, f'/{acc}')
        self.ratio: Indexes[StoredF32] = Indexes(client, f'/{acc}_ratio')
        self.ratio_1m_sma: Indexes[StoredF32] = Indexes(client, f'/{acc}_ratio_1m_sma')
        self.ratio_1w_sma: Indexes[StoredF32] = Indexes(client, f'/{acc}_ratio_1w_sma')
        self.ratio_1y_sd: Ratio1ySdPattern2 = Ratio1ySdPattern2(client, f'{acc}_ratio_1y_sd')
        self.ratio_2y_sd: Ratio1ySdPattern2 = Ratio1ySdPattern2(client, f'{acc}_ratio_2y_sd')
        self.ratio_4y_sd: Ratio1ySdPattern2 = Ratio1ySdPattern2(client, f'{acc}_ratio_4y_sd')
        self.ratio_pct1: Indexes[StoredF32] = Indexes(client, f'/{acc}_ratio_pct1')
        self.ratio_pct1_usd: Indexes[Dollars] = Indexes(client, f'/{acc}_ratio_pct1_usd')
        self.ratio_pct2: Indexes[StoredF32] = Indexes(client, f'/{acc}_ratio_pct2')
        self.ratio_pct2_usd: Indexes[Dollars] = Indexes(client, f'/{acc}_ratio_pct2_usd')
        self.ratio_pct5: Indexes[StoredF32] = Indexes(client, f'/{acc}_ratio_pct5')
        self.ratio_pct5_usd: Indexes[Dollars] = Indexes(client, f'/{acc}_ratio_pct5_usd')
        self.ratio_pct95: Indexes[StoredF32] = Indexes(client, f'/{acc}_ratio_pct95')
        self.ratio_pct95_usd: Indexes[Dollars] = Indexes(client, f'/{acc}_ratio_pct95_usd')
        self.ratio_pct98: Indexes[StoredF32] = Indexes(client, f'/{acc}_ratio_pct98')
        self.ratio_pct98_usd: Indexes[Dollars] = Indexes(client, f'/{acc}_ratio_pct98_usd')
        self.ratio_pct99: Indexes[StoredF32] = Indexes(client, f'/{acc}_ratio_pct99')
        self.ratio_pct99_usd: Indexes[Dollars] = Indexes(client, f'/{acc}_ratio_pct99_usd')
        self.ratio_sd: Ratio1ySdPattern2 = Ratio1ySdPattern2(client, f'{acc}_ratio_sd')

class PricePercentilesPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, base_path: str):
        self.pct05: Indexes[Dollars] = Indexes(client, f'{base_path}/pct05')
        self.pct10: Indexes[Dollars] = Indexes(client, f'{base_path}/pct10')
        self.pct15: Indexes[Dollars] = Indexes(client, f'{base_path}/pct15')
        self.pct20: Indexes[Dollars] = Indexes(client, f'{base_path}/pct20')
        self.pct25: Indexes[Dollars] = Indexes(client, f'{base_path}/pct25')
        self.pct30: Indexes[Dollars] = Indexes(client, f'{base_path}/pct30')
        self.pct35: Indexes[Dollars] = Indexes(client, f'{base_path}/pct35')
        self.pct40: Indexes[Dollars] = Indexes(client, f'{base_path}/pct40')
        self.pct45: Indexes[Dollars] = Indexes(client, f'{base_path}/pct45')
        self.pct50: Indexes[Dollars] = Indexes(client, f'{base_path}/pct50')
        self.pct55: Indexes[Dollars] = Indexes(client, f'{base_path}/pct55')
        self.pct60: Indexes[Dollars] = Indexes(client, f'{base_path}/pct60')
        self.pct65: Indexes[Dollars] = Indexes(client, f'{base_path}/pct65')
        self.pct70: Indexes[Dollars] = Indexes(client, f'{base_path}/pct70')
        self.pct75: Indexes[Dollars] = Indexes(client, f'{base_path}/pct75')
        self.pct80: Indexes[Dollars] = Indexes(client, f'{base_path}/pct80')
        self.pct85: Indexes[Dollars] = Indexes(client, f'{base_path}/pct85')
        self.pct90: Indexes[Dollars] = Indexes(client, f'{base_path}/pct90')
        self.pct95: Indexes[Dollars] = Indexes(client, f'{base_path}/pct95')

class RelativePattern2:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, base_path: str):
        self.neg_unrealized_loss_rel_to_market_cap: Indexes27[StoredF32] = Indexes27(client, f'{base_path}/neg_unrealized_loss_rel_to_market_cap')
        self.neg_unrealized_loss_rel_to_own_market_cap: Indexes27[StoredF32] = Indexes27(client, f'{base_path}/neg_unrealized_loss_rel_to_own_market_cap')
        self.neg_unrealized_loss_rel_to_own_total_unrealized_pnl: Indexes27[StoredF32] = Indexes27(client, f'{base_path}/neg_unrealized_loss_rel_to_own_total_unrealized_pnl')
        self.net_unrealized_pnl_rel_to_market_cap: Indexes26[StoredF32] = Indexes26(client, f'{base_path}/net_unrealized_pnl_rel_to_market_cap')
        self.net_unrealized_pnl_rel_to_own_market_cap: Indexes26[StoredF32] = Indexes26(client, f'{base_path}/net_unrealized_pnl_rel_to_own_market_cap')
        self.net_unrealized_pnl_rel_to_own_total_unrealized_pnl: Indexes26[StoredF32] = Indexes26(client, f'{base_path}/net_unrealized_pnl_rel_to_own_total_unrealized_pnl')
        self.supply_in_loss_rel_to_circulating_supply: Indexes27[StoredF64] = Indexes27(client, f'{base_path}/supply_in_loss_rel_to_circulating_supply')
        self.supply_in_loss_rel_to_own_supply: Indexes27[StoredF64] = Indexes27(client, f'{base_path}/supply_in_loss_rel_to_own_supply')
        self.supply_in_profit_rel_to_circulating_supply: Indexes27[StoredF64] = Indexes27(client, f'{base_path}/supply_in_profit_rel_to_circulating_supply')
        self.supply_in_profit_rel_to_own_supply: Indexes27[StoredF64] = Indexes27(client, f'{base_path}/supply_in_profit_rel_to_own_supply')
        self.supply_rel_to_circulating_supply: Indexes[StoredF64] = Indexes(client, f'{base_path}/supply_rel_to_circulating_supply')
        self.unrealized_loss_rel_to_market_cap: Indexes27[StoredF32] = Indexes27(client, f'{base_path}/unrealized_loss_rel_to_market_cap')
        self.unrealized_loss_rel_to_own_market_cap: Indexes27[StoredF32] = Indexes27(client, f'{base_path}/unrealized_loss_rel_to_own_market_cap')
        self.unrealized_loss_rel_to_own_total_unrealized_pnl: Indexes27[StoredF32] = Indexes27(client, f'{base_path}/unrealized_loss_rel_to_own_total_unrealized_pnl')
        self.unrealized_profit_rel_to_market_cap: Indexes27[StoredF32] = Indexes27(client, f'{base_path}/unrealized_profit_rel_to_market_cap')
        self.unrealized_profit_rel_to_own_market_cap: Indexes27[StoredF32] = Indexes27(client, f'{base_path}/unrealized_profit_rel_to_own_market_cap')
        self.unrealized_profit_rel_to_own_total_unrealized_pnl: Indexes27[StoredF32] = Indexes27(client, f'{base_path}/unrealized_profit_rel_to_own_total_unrealized_pnl')

class Ratio1ySdPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, base_path: str):
        self.m0_5sd: Indexes[StoredF32] = Indexes(client, f'{base_path}/m0_5sd')
        self.m1_5sd: Indexes[StoredF32] = Indexes(client, f'{base_path}/m1_5sd')
        self.m1sd: Indexes[StoredF32] = Indexes(client, f'{base_path}/m1sd')
        self.m2_5sd: Indexes[StoredF32] = Indexes(client, f'{base_path}/m2_5sd')
        self.m2sd: Indexes[StoredF32] = Indexes(client, f'{base_path}/m2sd')
        self.m3sd: Indexes[StoredF32] = Indexes(client, f'{base_path}/m3sd')
        self.p0_5sd: Indexes[StoredF32] = Indexes(client, f'{base_path}/p0_5sd')
        self.p1_5sd: Indexes[StoredF32] = Indexes(client, f'{base_path}/p1_5sd')
        self.p1sd: Indexes[StoredF32] = Indexes(client, f'{base_path}/p1sd')
        self.p2_5sd: Indexes[StoredF32] = Indexes(client, f'{base_path}/p2_5sd')
        self.p2sd: Indexes[StoredF32] = Indexes(client, f'{base_path}/p2sd')
        self.p3sd: Indexes[StoredF32] = Indexes(client, f'{base_path}/p3sd')
        self.sd: Indexes[StoredF32] = Indexes(client, f'{base_path}/sd')
        self.sma: Indexes[StoredF32] = Indexes(client, f'{base_path}/sma')
        self.zscore: Indexes[StoredF32] = Indexes(client, f'{base_path}/zscore')

class ActivePriceRatioPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, base_path: str):
        self.ratio: Indexes[StoredF32] = Indexes(client, f'{base_path}/ratio')
        self.ratio_1m_sma: Indexes[StoredF32] = Indexes(client, f'{base_path}/ratio_1m_sma')
        self.ratio_1w_sma: Indexes[StoredF32] = Indexes(client, f'{base_path}/ratio_1w_sma')
        self.ratio_1y_sd: Ratio1ySdPattern = Ratio1ySdPattern(client, f'{base_path}/ratio_1y_sd')
        self.ratio_2y_sd: Ratio1ySdPattern = Ratio1ySdPattern(client, f'{base_path}/ratio_2y_sd')
        self.ratio_4y_sd: Ratio1ySdPattern = Ratio1ySdPattern(client, f'{base_path}/ratio_4y_sd')
        self.ratio_pct1: Indexes[StoredF32] = Indexes(client, f'{base_path}/ratio_pct1')
        self.ratio_pct2: Indexes[StoredF32] = Indexes(client, f'{base_path}/ratio_pct2')
        self.ratio_pct5: Indexes[StoredF32] = Indexes(client, f'{base_path}/ratio_pct5')
        self.ratio_pct95: Indexes[StoredF32] = Indexes(client, f'{base_path}/ratio_pct95')
        self.ratio_pct98: Indexes[StoredF32] = Indexes(client, f'{base_path}/ratio_pct98')
        self.ratio_pct99: Indexes[StoredF32] = Indexes(client, f'{base_path}/ratio_pct99')
        self.ratio_sd: Ratio1ySdPattern = Ratio1ySdPattern(client, f'{base_path}/ratio_sd')

class AXbtPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, base_path: str):
        self._1d_dominance: BlockCountPattern[StoredF32] = BlockCountPattern(client, f'{base_path}/1d_dominance')
        self._1m_blocks_mined: Indexes[StoredU32] = Indexes(client, f'{base_path}/1m_blocks_mined')
        self._1m_dominance: Indexes[StoredF32] = Indexes(client, f'{base_path}/1m_dominance')
        self._1w_blocks_mined: Indexes[StoredU32] = Indexes(client, f'{base_path}/1w_blocks_mined')
        self._1w_dominance: Indexes[StoredF32] = Indexes(client, f'{base_path}/1w_dominance')
        self._1y_blocks_mined: Indexes[StoredU32] = Indexes(client, f'{base_path}/1y_blocks_mined')
        self._1y_dominance: Indexes[StoredF32] = Indexes(client, f'{base_path}/1y_dominance')
        self.blocks_mined: BlockCountPattern[StoredU32] = BlockCountPattern(client, f'{base_path}/blocks_mined')
        self.coinbase: UnclaimedRewardsPattern = UnclaimedRewardsPattern(client, f'{base_path}/coinbase')
        self.days_since_block: Indexes[StoredU16] = Indexes(client, f'{base_path}/days_since_block')
        self.dominance: BlockCountPattern[StoredF32] = BlockCountPattern(client, f'{base_path}/dominance')
        self.fee: UnclaimedRewardsPattern = UnclaimedRewardsPattern(client, f'{base_path}/fee')
        self.subsidy: UnclaimedRewardsPattern = UnclaimedRewardsPattern(client, f'{base_path}/subsidy')

class BitcoinPattern(Generic[T]):
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, base_path: str):
        self.average: Indexes4[T] = Indexes4(client, f'{base_path}/average')
        self.base: Indexes2[T] = Indexes2(client, f'{base_path}/base')
        self.cumulative: Indexes3[T] = Indexes3(client, f'{base_path}/cumulative')
        self.max: Indexes4[T] = Indexes4(client, f'{base_path}/max')
        self.median: Indexes5[T] = Indexes5(client, f'{base_path}/median')
        self.min: Indexes4[T] = Indexes4(client, f'{base_path}/min')
        self.pct10: Indexes5[T] = Indexes5(client, f'{base_path}/pct10')
        self.pct25: Indexes5[T] = Indexes5(client, f'{base_path}/pct25')
        self.pct75: Indexes5[T] = Indexes5(client, f'{base_path}/pct75')
        self.pct90: Indexes5[T] = Indexes5(client, f'{base_path}/pct90')
        self.sum: Indexes4[T] = Indexes4(client, f'{base_path}/sum')

class BlockSizePattern(Generic[T]):
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, base_path: str):
        self.average: Indexes4[T] = Indexes4(client, f'{base_path}/average')
        self.cumulative: Indexes3[T] = Indexes3(client, f'{base_path}/cumulative')
        self.max: Indexes4[T] = Indexes4(client, f'{base_path}/max')
        self.median: Indexes5[T] = Indexes5(client, f'{base_path}/median')
        self.min: Indexes4[T] = Indexes4(client, f'{base_path}/min')
        self.pct10: Indexes5[T] = Indexes5(client, f'{base_path}/pct10')
        self.pct25: Indexes5[T] = Indexes5(client, f'{base_path}/pct25')
        self.pct75: Indexes5[T] = Indexes5(client, f'{base_path}/pct75')
        self.pct90: Indexes5[T] = Indexes5(client, f'{base_path}/pct90')
        self.sum: Indexes4[T] = Indexes4(client, f'{base_path}/sum')

class UnrealizedPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, base_path: str):
        self.neg_unrealized_loss: Indexes26[Dollars] = Indexes26(client, f'{base_path}/neg_unrealized_loss')
        self.net_unrealized_pnl: Indexes26[Dollars] = Indexes26(client, f'{base_path}/net_unrealized_pnl')
        self.supply_in_loss: SupplyPattern = SupplyPattern(client, f'{base_path}/supply_in_loss')
        self.supply_in_loss_value: SupplyValuePattern = SupplyValuePattern(client, f'{base_path}/supply_in_loss_value')
        self.supply_in_profit: SupplyPattern = SupplyPattern(client, f'{base_path}/supply_in_profit')
        self.supply_in_profit_value: SupplyValuePattern = SupplyValuePattern(client, f'{base_path}/supply_in_profit_value')
        self.total_unrealized_pnl: Indexes26[Dollars] = Indexes26(client, f'{base_path}/total_unrealized_pnl')
        self.unrealized_loss: Indexes26[Dollars] = Indexes26(client, f'{base_path}/unrealized_loss')
        self.unrealized_profit: Indexes26[Dollars] = Indexes26(client, f'{base_path}/unrealized_profit')

class RelativePattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, base_path: str):
        self.neg_unrealized_loss_rel_to_market_cap: Indexes27[StoredF32] = Indexes27(client, f'{base_path}/neg_unrealized_loss_rel_to_market_cap')
        self.net_unrealized_pnl_rel_to_market_cap: Indexes26[StoredF32] = Indexes26(client, f'{base_path}/net_unrealized_pnl_rel_to_market_cap')
        self.supply_in_loss_rel_to_circulating_supply: Indexes27[StoredF64] = Indexes27(client, f'{base_path}/supply_in_loss_rel_to_circulating_supply')
        self.supply_in_loss_rel_to_own_supply: Indexes27[StoredF64] = Indexes27(client, f'{base_path}/supply_in_loss_rel_to_own_supply')
        self.supply_in_profit_rel_to_circulating_supply: Indexes27[StoredF64] = Indexes27(client, f'{base_path}/supply_in_profit_rel_to_circulating_supply')
        self.supply_in_profit_rel_to_own_supply: Indexes27[StoredF64] = Indexes27(client, f'{base_path}/supply_in_profit_rel_to_own_supply')
        self.supply_rel_to_circulating_supply: Indexes[StoredF64] = Indexes(client, f'{base_path}/supply_rel_to_circulating_supply')
        self.unrealized_loss_rel_to_market_cap: Indexes27[StoredF32] = Indexes27(client, f'{base_path}/unrealized_loss_rel_to_market_cap')
        self.unrealized_profit_rel_to_market_cap: Indexes27[StoredF32] = Indexes27(client, f'{base_path}/unrealized_profit_rel_to_market_cap')

class AddresstypeToHeightToAddrCountPattern(Generic[T]):
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, base_path: str):
        self.p2a: Indexes16[T] = Indexes16(client, f'{base_path}/p2a')
        self.p2pk33: Indexes17[T] = Indexes17(client, f'{base_path}/p2pk33')
        self.p2pk65: Indexes18[T] = Indexes18(client, f'{base_path}/p2pk65')
        self.p2pkh: Indexes19[T] = Indexes19(client, f'{base_path}/p2pkh')
        self.p2sh: Indexes20[T] = Indexes20(client, f'{base_path}/p2sh')
        self.p2tr: Indexes21[T] = Indexes21(client, f'{base_path}/p2tr')
        self.p2wpkh: Indexes22[T] = Indexes22(client, f'{base_path}/p2wpkh')
        self.p2wsh: Indexes23[T] = Indexes23(client, f'{base_path}/p2wsh')

class BlockIntervalPattern(Generic[T]):
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.average: Indexes3[T] = Indexes3(client, f'/{acc}_avg')
        self.max: Indexes3[T] = Indexes3(client, f'/{acc}_max')
        self.median: Indexes2[T] = Indexes2(client, f'/{acc}_median')
        self.min: Indexes3[T] = Indexes3(client, f'/{acc}_min')
        self.pct10: Indexes2[T] = Indexes2(client, f'/{acc}_pct10')
        self.pct25: Indexes2[T] = Indexes2(client, f'/{acc}_pct25')
        self.pct75: Indexes2[T] = Indexes2(client, f'/{acc}_pct75')
        self.pct90: Indexes2[T] = Indexes2(client, f'/{acc}_pct90')

class Constant0Pattern(Generic[T]):
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.dateindex: Indexes5[T] = Indexes5(client, f'/{acc}')
        self.decadeindex: Indexes7[T] = Indexes7(client, f'/{acc}')
        self.height: Indexes2[T] = Indexes2(client, f'/{acc}')
        self.monthindex: Indexes8[T] = Indexes8(client, f'/{acc}')
        self.quarterindex: Indexes9[T] = Indexes9(client, f'/{acc}')
        self.semesterindex: Indexes10[T] = Indexes10(client, f'/{acc}')
        self.weekindex: Indexes11[T] = Indexes11(client, f'/{acc}')
        self.yearindex: Indexes12[T] = Indexes12(client, f'/{acc}')

class _0satsPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, base_path: str):
        self.activity: ActivityPattern = ActivityPattern(client, f'{base_path}/activity')
        self.addr_count: Indexes3[StoredU64] = Indexes3(client, f'{base_path}/addr_count')
        self.price_paid: PricePaidPattern = PricePaidPattern(client, f'{base_path}/price_paid')
        self.realized: RealizedPattern = RealizedPattern(client, f'{base_path}/realized')
        self.relative: RelativePattern = RelativePattern(client, f'{base_path}/relative')
        self.supply: SupplyPattern2 = SupplyPattern2(client, f'{base_path}/supply')
        self.unrealized: UnrealizedPattern = UnrealizedPattern(client, f'{base_path}/unrealized')

class UpTo1dPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, base_path: str):
        self.activity: ActivityPattern = ActivityPattern(client, f'{base_path}/activity')
        self.price_paid: PricePaidPattern2 = PricePaidPattern2(client, f'{base_path}/price_paid')
        self.realized: RealizedPattern3 = RealizedPattern3(client, f'{base_path}/realized')
        self.relative: RelativePattern2 = RelativePattern2(client, f'{base_path}/relative')
        self.supply: SupplyPattern2 = SupplyPattern2(client, f'{base_path}/supply')
        self.unrealized: UnrealizedPattern = UnrealizedPattern(client, f'{base_path}/unrealized')

class _0satsPattern2:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, base_path: str):
        self.activity: ActivityPattern = ActivityPattern(client, f'{base_path}/activity')
        self.price_paid: PricePaidPattern = PricePaidPattern(client, f'{base_path}/price_paid')
        self.realized: RealizedPattern = RealizedPattern(client, f'{base_path}/realized')
        self.relative: RelativePattern = RelativePattern(client, f'{base_path}/relative')
        self.supply: SupplyPattern2 = SupplyPattern2(client, f'{base_path}/supply')
        self.unrealized: UnrealizedPattern = UnrealizedPattern(client, f'{base_path}/unrealized')

class _10yTo12yPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, base_path: str):
        self.activity: ActivityPattern = ActivityPattern(client, f'{base_path}/activity')
        self.price_paid: PricePaidPattern2 = PricePaidPattern2(client, f'{base_path}/price_paid')
        self.realized: RealizedPattern2 = RealizedPattern2(client, f'{base_path}/realized')
        self.relative: RelativePattern2 = RelativePattern2(client, f'{base_path}/relative')
        self.supply: SupplyPattern2 = SupplyPattern2(client, f'{base_path}/supply')
        self.unrealized: UnrealizedPattern = UnrealizedPattern(client, f'{base_path}/unrealized')

class SupplyPattern2:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, base_path: str):
        self.supply: SupplyPattern = SupplyPattern(client, f'{base_path}/supply')
        self.supply_half: ActiveSupplyPattern = ActiveSupplyPattern(client, f'{base_path}/supply_half')
        self.supply_half_value: ActiveSupplyPattern = ActiveSupplyPattern(client, f'{base_path}/supply_half_value')
        self.supply_value: SupplyValuePattern = SupplyValuePattern(client, f'{base_path}/supply_value')
        self.utxo_count: Indexes3[StoredU64] = Indexes3(client, f'{base_path}/utxo_count')

class ActivityPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, base_path: str):
        self.coinblocks_destroyed: BlockCountPattern[StoredF64] = BlockCountPattern(client, f'{base_path}/coinblocks_destroyed')
        self.coindays_destroyed: BlockCountPattern[StoredF64] = BlockCountPattern(client, f'{base_path}/coindays_destroyed')
        self.satblocks_destroyed: Indexes2[Sats] = Indexes2(client, f'{base_path}/satblocks_destroyed')
        self.satdays_destroyed: Indexes2[Sats] = Indexes2(client, f'{base_path}/satdays_destroyed')
        self.sent: SentPattern = SentPattern(client, f'{base_path}/sent')

class SentPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, base_path: str):
        self.base: Indexes2[Sats] = Indexes2(client, f'{base_path}/base')
        self.bitcoin: BlockCountPattern[Bitcoin] = BlockCountPattern(client, f'{base_path}/bitcoin')
        self.dollars: BlockCountPattern[Dollars] = BlockCountPattern(client, f'{base_path}/dollars')
        self.sats: SatsPattern = SatsPattern(client, f'{base_path}/sats')

class SupplyPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, base_path: str):
        self.base: Indexes2[Sats] = Indexes2(client, f'{base_path}/base')
        self.bitcoin: Indexes[Bitcoin] = Indexes(client, f'{base_path}/bitcoin')
        self.dollars: Indexes[Dollars] = Indexes(client, f'{base_path}/dollars')
        self.sats: Indexes[Sats] = Indexes(client, f'{base_path}/sats')

class CoinbasePattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, base_path: str):
        self.bitcoin: BitcoinPattern[Bitcoin] = BitcoinPattern(client, f'{base_path}/bitcoin')
        self.dollars: BitcoinPattern[Dollars] = BitcoinPattern(client, f'{base_path}/dollars')
        self.sats: BitcoinPattern[Sats] = BitcoinPattern(client, f'{base_path}/sats')

class ActiveSupplyPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, base_path: str):
        self.bitcoin: Indexes3[Bitcoin] = Indexes3(client, f'{base_path}/bitcoin')
        self.dollars: Indexes3[Dollars] = Indexes3(client, f'{base_path}/dollars')
        self.sats: Indexes3[Sats] = Indexes3(client, f'{base_path}/sats')

class UnclaimedRewardsPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, base_path: str):
        self.bitcoin: BlockCountPattern[Bitcoin] = BlockCountPattern(client, f'{base_path}/bitcoin')
        self.dollars: BlockCountPattern[Dollars] = BlockCountPattern(client, f'{base_path}/dollars')
        self.sats: BlockCountPattern[Sats] = BlockCountPattern(client, f'{base_path}/sats')

class PricePaidPattern2:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, base_path: str):
        self.max_price_paid: Indexes3[Dollars] = Indexes3(client, f'{base_path}/max_price_paid')
        self.min_price_paid: Indexes3[Dollars] = Indexes3(client, f'{base_path}/min_price_paid')
        self.price_percentiles: PricePercentilesPattern = PricePercentilesPattern(client, f'{base_path}/price_percentiles')

class BlockCountPattern(Generic[T]):
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, base_path: str):
        self.base: Indexes2[T] = Indexes2(client, f'{base_path}/base')
        self.cumulative: Indexes3[T] = Indexes3(client, f'{base_path}/cumulative')
        self.sum: Indexes4[T] = Indexes4(client, f'{base_path}/sum')

class SupplyValuePattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, base_path: str):
        self.bitcoin: Indexes2[Bitcoin] = Indexes2(client, f'{base_path}/bitcoin')
        self.dollars: Indexes2[Dollars] = Indexes2(client, f'{base_path}/dollars')

class PricePaidPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, base_path: str):
        self.max_price_paid: Indexes3[Dollars] = Indexes3(client, f'{base_path}/max_price_paid')
        self.min_price_paid: Indexes3[Dollars] = Indexes3(client, f'{base_path}/min_price_paid')

class SatsPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, base_path: str):
        self.cumulative: Indexes3[Sats] = Indexes3(client, f'{base_path}/cumulative')
        self.sum: Indexes4[Sats] = Indexes4(client, f'{base_path}/sum')

class _1dReturns1mSdPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.sd: Indexes[StoredF32] = Indexes(client, f'/{acc}_sd')
        self.sma: Indexes[StoredF32] = Indexes(client, f'/{acc}_sma')

class BitcoinPattern2(Generic[T]):
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, base_path: str):
        self.base: Indexes2[T] = Indexes2(client, f'{base_path}/base')
        self.sum: Indexes4[T] = Indexes4(client, f'{base_path}/sum')

class RealizedPriceExtraPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, base_path: str):
        self.ratio: Indexes[StoredF32] = Indexes(client, f'{base_path}/ratio')

# Catalog tree classes

class CatalogTree:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.computed: CatalogTree_Computed = CatalogTree_Computed(client, f'{base_path}/computed')
        self.indexed: CatalogTree_Indexed = CatalogTree_Indexed(client, f'{base_path}/indexed')

class CatalogTree_Computed:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.blks: CatalogTree_Computed_Blks = CatalogTree_Computed_Blks(client, f'{base_path}/blks')
        self.chain: CatalogTree_Computed_Chain = CatalogTree_Computed_Chain(client, f'{base_path}/chain')
        self.cointime: CatalogTree_Computed_Cointime = CatalogTree_Computed_Cointime(client, f'{base_path}/cointime')
        self.constants: CatalogTree_Computed_Constants = CatalogTree_Computed_Constants(client, f'{base_path}/constants')
        self.fetched: CatalogTree_Computed_Fetched = CatalogTree_Computed_Fetched(client, f'{base_path}/fetched')
        self.indexes: CatalogTree_Computed_Indexes = CatalogTree_Computed_Indexes(client, f'{base_path}/indexes')
        self.market: CatalogTree_Computed_Market = CatalogTree_Computed_Market(client, f'{base_path}/market')
        self.pools: CatalogTree_Computed_Pools = CatalogTree_Computed_Pools(client, f'{base_path}/pools')
        self.price: CatalogTree_Computed_Price = CatalogTree_Computed_Price(client, f'{base_path}/price')
        self.stateful: CatalogTree_Computed_Stateful = CatalogTree_Computed_Stateful(client, f'{base_path}/stateful')
        self.txins: CatalogTree_Computed_Txins = CatalogTree_Computed_Txins(client, f'{base_path}/txins')
        self.txouts: CatalogTree_Computed_Txouts = CatalogTree_Computed_Txouts(client, f'{base_path}/txouts')

class CatalogTree_Computed_Blks:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.position: MetricNode[BlkPosition] = MetricNode(client, f'{base_path}/position')

class CatalogTree_Computed_Chain:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._1m_block_count: Indexes[StoredU32] = Indexes(client, f'{base_path}/1m_block_count')
        self._1w_block_count: Indexes[StoredU32] = Indexes(client, f'{base_path}/1w_block_count')
        self._1y_block_count: Indexes[StoredU32] = Indexes(client, f'{base_path}/1y_block_count')
        self._24h_block_count: Indexes2[StoredU32] = Indexes2(client, f'{base_path}/24h_block_count')
        self._24h_coinbase_sum: Indexes2[Sats] = Indexes2(client, f'{base_path}/24h_coinbase_sum')
        self._24h_coinbase_usd_sum: Indexes2[Dollars] = Indexes2(client, f'{base_path}/24h_coinbase_usd_sum')
        self.annualized_volume: Indexes[Sats] = Indexes(client, f'{base_path}/annualized_volume')
        self.annualized_volume_btc: Indexes[Bitcoin] = Indexes(client, f'{base_path}/annualized_volume_btc')
        self.annualized_volume_usd: Indexes[Dollars] = Indexes(client, f'{base_path}/annualized_volume_usd')
        self.block_count: BlockCountPattern[StoredU32] = BlockCountPattern(client, f'{base_path}/block_count')
        self.block_count_target: Indexes[StoredU64] = Indexes(client, f'{base_path}/block_count_target')
        self.block_interval: BlockIntervalPattern[Timestamp] = BlockIntervalPattern(client, 'block_interval')
        self.block_size: BlockSizePattern[StoredU64] = BlockSizePattern(client, f'{base_path}/block_size')
        self.block_vbytes: BlockSizePattern[StoredU64] = BlockSizePattern(client, f'{base_path}/block_vbytes')
        self.block_weight: BlockSizePattern[Weight] = BlockSizePattern(client, f'{base_path}/block_weight')
        self.blocks_before_next_difficulty_adjustment: Indexes3[StoredU32] = Indexes3(client, f'{base_path}/blocks_before_next_difficulty_adjustment')
        self.blocks_before_next_halving: Indexes3[StoredU32] = Indexes3(client, f'{base_path}/blocks_before_next_halving')
        self.coinbase: CoinbasePattern = CoinbasePattern(client, f'{base_path}/coinbase')
        self.days_before_next_difficulty_adjustment: Indexes3[StoredF32] = Indexes3(client, f'{base_path}/days_before_next_difficulty_adjustment')
        self.days_before_next_halving: Indexes3[StoredF32] = Indexes3(client, f'{base_path}/days_before_next_halving')
        self.difficulty: Indexes4[StoredF64] = Indexes4(client, f'{base_path}/difficulty')
        self.difficulty_adjustment: Indexes3[StoredF32] = Indexes3(client, f'{base_path}/difficulty_adjustment')
        self.difficulty_as_hash: Indexes3[StoredF32] = Indexes3(client, f'{base_path}/difficulty_as_hash')
        self.difficultyepoch: Indexes[DifficultyEpoch] = Indexes(client, f'{base_path}/difficultyepoch')
        self.emptyoutput_count: BitcoinPattern[StoredU64] = BitcoinPattern(client, f'{base_path}/emptyoutput_count')
        self.exact_utxo_count: Indexes3[StoredU64] = Indexes3(client, f'{base_path}/exact_utxo_count')
        self.fee: CatalogTree_Computed_Chain_Fee = CatalogTree_Computed_Chain_Fee(client, f'{base_path}/fee')
        self.fee_dominance: Indexes5[StoredF32] = Indexes5(client, f'{base_path}/fee_dominance')
        self.fee_rate: CatalogTree_Computed_Chain_FeeRate = CatalogTree_Computed_Chain_FeeRate(client, f'{base_path}/fee_rate')
        self.halvingepoch: Indexes[HalvingEpoch] = Indexes(client, f'{base_path}/halvingepoch')
        self.hash_price_phs: Indexes3[StoredF32] = Indexes3(client, f'{base_path}/hash_price_phs')
        self.hash_price_phs_min: Indexes3[StoredF32] = Indexes3(client, f'{base_path}/hash_price_phs_min')
        self.hash_price_rebound: Indexes3[StoredF32] = Indexes3(client, f'{base_path}/hash_price_rebound')
        self.hash_price_ths: Indexes3[StoredF32] = Indexes3(client, f'{base_path}/hash_price_ths')
        self.hash_price_ths_min: Indexes3[StoredF32] = Indexes3(client, f'{base_path}/hash_price_ths_min')
        self.hash_rate: Indexes3[StoredF64] = Indexes3(client, f'{base_path}/hash_rate')
        self.hash_rate_1m_sma: Indexes[StoredF32] = Indexes(client, f'{base_path}/hash_rate_1m_sma')
        self.hash_rate_1w_sma: Indexes[StoredF64] = Indexes(client, f'{base_path}/hash_rate_1w_sma')
        self.hash_rate_1y_sma: Indexes[StoredF32] = Indexes(client, f'{base_path}/hash_rate_1y_sma')
        self.hash_rate_2m_sma: Indexes[StoredF32] = Indexes(client, f'{base_path}/hash_rate_2m_sma')
        self.hash_value_phs: Indexes3[StoredF32] = Indexes3(client, f'{base_path}/hash_value_phs')
        self.hash_value_phs_min: Indexes3[StoredF32] = Indexes3(client, f'{base_path}/hash_value_phs_min')
        self.hash_value_rebound: Indexes3[StoredF32] = Indexes3(client, f'{base_path}/hash_value_rebound')
        self.hash_value_ths: Indexes3[StoredF32] = Indexes3(client, f'{base_path}/hash_value_ths')
        self.hash_value_ths_min: Indexes3[StoredF32] = Indexes3(client, f'{base_path}/hash_value_ths_min')
        self.inflation_rate: Indexes[StoredF32] = Indexes(client, f'{base_path}/inflation_rate')
        self.input_count: BlockSizePattern[StoredU64] = BlockSizePattern(client, f'{base_path}/input_count')
        self.input_value: Indexes6[Sats] = Indexes6(client, f'{base_path}/input_value')
        self.inputs_per_sec: Indexes[StoredF32] = Indexes(client, f'{base_path}/inputs_per_sec')
        self.interval: Indexes2[Timestamp] = Indexes2(client, f'{base_path}/interval')
        self.is_coinbase: Indexes6[StoredBool] = Indexes6(client, f'{base_path}/is_coinbase')
        self.opreturn_count: BitcoinPattern[StoredU64] = BitcoinPattern(client, f'{base_path}/opreturn_count')
        self.output_count: BlockSizePattern[StoredU64] = BlockSizePattern(client, f'{base_path}/output_count')
        self.output_value: Indexes6[Sats] = Indexes6(client, f'{base_path}/output_value')
        self.outputs_per_sec: Indexes[StoredF32] = Indexes(client, f'{base_path}/outputs_per_sec')
        self.p2a_count: BitcoinPattern[StoredU64] = BitcoinPattern(client, f'{base_path}/p2a_count')
        self.p2ms_count: BitcoinPattern[StoredU64] = BitcoinPattern(client, f'{base_path}/p2ms_count')
        self.p2pk33_count: BitcoinPattern[StoredU64] = BitcoinPattern(client, f'{base_path}/p2pk33_count')
        self.p2pk65_count: BitcoinPattern[StoredU64] = BitcoinPattern(client, f'{base_path}/p2pk65_count')
        self.p2pkh_count: BitcoinPattern[StoredU64] = BitcoinPattern(client, f'{base_path}/p2pkh_count')
        self.p2sh_count: BitcoinPattern[StoredU64] = BitcoinPattern(client, f'{base_path}/p2sh_count')
        self.p2tr_count: BitcoinPattern[StoredU64] = BitcoinPattern(client, f'{base_path}/p2tr_count')
        self.p2wpkh_count: BitcoinPattern[StoredU64] = BitcoinPattern(client, f'{base_path}/p2wpkh_count')
        self.p2wsh_count: BitcoinPattern[StoredU64] = BitcoinPattern(client, f'{base_path}/p2wsh_count')
        self.puell_multiple: Indexes[StoredF32] = Indexes(client, f'{base_path}/puell_multiple')
        self.sent_sum: CatalogTree_Computed_Chain_SentSum = CatalogTree_Computed_Chain_SentSum(client, f'{base_path}/sent_sum')
        self.subsidy: CoinbasePattern = CoinbasePattern(client, f'{base_path}/subsidy')
        self.subsidy_dominance: Indexes5[StoredF32] = Indexes5(client, f'{base_path}/subsidy_dominance')
        self.subsidy_usd_1y_sma: Indexes[Dollars] = Indexes(client, f'{base_path}/subsidy_usd_1y_sma')
        self.timestamp: MetricNode[Timestamp] = MetricNode(client, f'{base_path}/timestamp')
        self.tx_btc_velocity: Indexes[StoredF64] = Indexes(client, f'{base_path}/tx_btc_velocity')
        self.tx_count: BitcoinPattern[StoredU64] = BitcoinPattern(client, f'{base_path}/tx_count')
        self.tx_per_sec: Indexes[StoredF32] = Indexes(client, f'{base_path}/tx_per_sec')
        self.tx_usd_velocity: Indexes[StoredF64] = Indexes(client, f'{base_path}/tx_usd_velocity')
        self.tx_v1: BlockCountPattern[StoredU64] = BlockCountPattern(client, f'{base_path}/tx_v1')
        self.tx_v2: BlockCountPattern[StoredU64] = BlockCountPattern(client, f'{base_path}/tx_v2')
        self.tx_v3: BlockCountPattern[StoredU64] = BlockCountPattern(client, f'{base_path}/tx_v3')
        self.tx_vsize: BlockIntervalPattern[VSize] = BlockIntervalPattern(client, 'tx_vsize')
        self.tx_weight: BlockIntervalPattern[Weight] = BlockIntervalPattern(client, 'tx_weight')
        self.unclaimed_rewards: UnclaimedRewardsPattern = UnclaimedRewardsPattern(client, f'{base_path}/unclaimed_rewards')
        self.unknownoutput_count: BitcoinPattern[StoredU64] = BitcoinPattern(client, f'{base_path}/unknownoutput_count')
        self.vbytes: Indexes2[StoredU64] = Indexes2(client, f'{base_path}/vbytes')
        self.vsize: Indexes6[VSize] = Indexes6(client, f'{base_path}/vsize')
        self.weight: Indexes6[Weight] = Indexes6(client, f'{base_path}/weight')

class CatalogTree_Computed_Chain_Fee:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.base: Indexes6[Sats] = Indexes6(client, f'{base_path}/base')
        self.bitcoin: BlockSizePattern[Bitcoin] = BlockSizePattern(client, f'{base_path}/bitcoin')
        self.bitcoin_txindex: Indexes6[Bitcoin] = Indexes6(client, f'{base_path}/bitcoin_txindex')
        self.dollars: BlockSizePattern[Dollars] = BlockSizePattern(client, f'{base_path}/dollars')
        self.dollars_txindex: Indexes6[Dollars] = Indexes6(client, f'{base_path}/dollars_txindex')
        self.sats: BlockSizePattern[Sats] = BlockSizePattern(client, f'{base_path}/sats')

class CatalogTree_Computed_Chain_FeeRate:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.average: Indexes3[FeeRate] = Indexes3(client, f'{base_path}/average')
        self.base: Indexes6[FeeRate] = Indexes6(client, f'{base_path}/base')
        self.max: Indexes3[FeeRate] = Indexes3(client, f'{base_path}/max')
        self.median: Indexes2[FeeRate] = Indexes2(client, f'{base_path}/median')
        self.min: Indexes3[FeeRate] = Indexes3(client, f'{base_path}/min')
        self.pct10: Indexes2[FeeRate] = Indexes2(client, f'{base_path}/pct10')
        self.pct25: Indexes2[FeeRate] = Indexes2(client, f'{base_path}/pct25')
        self.pct75: Indexes2[FeeRate] = Indexes2(client, f'{base_path}/pct75')
        self.pct90: Indexes2[FeeRate] = Indexes2(client, f'{base_path}/pct90')

class CatalogTree_Computed_Chain_SentSum:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.bitcoin: BitcoinPattern2[Bitcoin] = BitcoinPattern2(client, f'{base_path}/bitcoin')
        self.dollars: Indexes3[Dollars] = Indexes3(client, f'{base_path}/dollars')
        self.sats: Indexes3[Sats] = Indexes3(client, f'{base_path}/sats')

class CatalogTree_Computed_Cointime:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.active_cap: Indexes3[Dollars] = Indexes3(client, f'{base_path}/active_cap')
        self.active_price: Indexes3[Dollars] = Indexes3(client, f'{base_path}/active_price')
        self.active_price_ratio: ActivePriceRatioPattern = ActivePriceRatioPattern(client, f'{base_path}/active_price_ratio')
        self.active_supply: ActiveSupplyPattern = ActiveSupplyPattern(client, f'{base_path}/active_supply')
        self.activity_to_vaultedness_ratio: Indexes3[StoredF64] = Indexes3(client, f'{base_path}/activity_to_vaultedness_ratio')
        self.coinblocks_created: BlockCountPattern[StoredF64] = BlockCountPattern(client, f'{base_path}/coinblocks_created')
        self.coinblocks_stored: BlockCountPattern[StoredF64] = BlockCountPattern(client, f'{base_path}/coinblocks_stored')
        self.cointime_adj_inflation_rate: Indexes[StoredF32] = Indexes(client, f'{base_path}/cointime_adj_inflation_rate')
        self.cointime_adj_tx_btc_velocity: Indexes[StoredF64] = Indexes(client, f'{base_path}/cointime_adj_tx_btc_velocity')
        self.cointime_adj_tx_usd_velocity: Indexes[StoredF64] = Indexes(client, f'{base_path}/cointime_adj_tx_usd_velocity')
        self.cointime_cap: Indexes3[Dollars] = Indexes3(client, f'{base_path}/cointime_cap')
        self.cointime_price: Indexes3[Dollars] = Indexes3(client, f'{base_path}/cointime_price')
        self.cointime_price_ratio: ActivePriceRatioPattern = ActivePriceRatioPattern(client, f'{base_path}/cointime_price_ratio')
        self.cointime_value_created: BlockCountPattern[StoredF64] = BlockCountPattern(client, f'{base_path}/cointime_value_created')
        self.cointime_value_destroyed: BlockCountPattern[StoredF64] = BlockCountPattern(client, f'{base_path}/cointime_value_destroyed')
        self.cointime_value_stored: BlockCountPattern[StoredF64] = BlockCountPattern(client, f'{base_path}/cointime_value_stored')
        self.investor_cap: Indexes3[Dollars] = Indexes3(client, f'{base_path}/investor_cap')
        self.liveliness: Indexes3[StoredF64] = Indexes3(client, f'{base_path}/liveliness')
        self.thermo_cap: Indexes3[Dollars] = Indexes3(client, f'{base_path}/thermo_cap')
        self.true_market_mean: Indexes3[Dollars] = Indexes3(client, f'{base_path}/true_market_mean')
        self.true_market_mean_ratio: ActivePriceRatioPattern = ActivePriceRatioPattern(client, f'{base_path}/true_market_mean_ratio')
        self.vaulted_cap: Indexes3[Dollars] = Indexes3(client, f'{base_path}/vaulted_cap')
        self.vaulted_price: Indexes3[Dollars] = Indexes3(client, f'{base_path}/vaulted_price')
        self.vaulted_price_ratio: ActivePriceRatioPattern = ActivePriceRatioPattern(client, f'{base_path}/vaulted_price_ratio')
        self.vaulted_supply: ActiveSupplyPattern = ActiveSupplyPattern(client, f'{base_path}/vaulted_supply')
        self.vaultedness: Indexes3[StoredF64] = Indexes3(client, f'{base_path}/vaultedness')

class CatalogTree_Computed_Constants:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.constant_0: Constant0Pattern[StoredU16] = Constant0Pattern(client, 'constant_0')
        self.constant_1: Constant0Pattern[StoredU16] = Constant0Pattern(client, 'constant_1')
        self.constant_100: Constant0Pattern[StoredU16] = Constant0Pattern(client, 'constant_100')
        self.constant_2: Constant0Pattern[StoredU16] = Constant0Pattern(client, 'constant_2')
        self.constant_3: Constant0Pattern[StoredU16] = Constant0Pattern(client, 'constant_3')
        self.constant_38_2: Constant0Pattern[StoredF32] = Constant0Pattern(client, 'constant_38_2')
        self.constant_4: Constant0Pattern[StoredU16] = Constant0Pattern(client, 'constant_4')
        self.constant_50: Constant0Pattern[StoredU16] = Constant0Pattern(client, 'constant_50')
        self.constant_600: Constant0Pattern[StoredU16] = Constant0Pattern(client, 'constant_600')
        self.constant_61_8: Constant0Pattern[StoredF32] = Constant0Pattern(client, 'constant_61_8')
        self.constant_minus_1: Constant0Pattern[StoredI16] = Constant0Pattern(client, 'constant_minus_1')
        self.constant_minus_2: Constant0Pattern[StoredI16] = Constant0Pattern(client, 'constant_minus_2')
        self.constant_minus_3: Constant0Pattern[StoredI16] = Constant0Pattern(client, 'constant_minus_3')
        self.constant_minus_4: Constant0Pattern[StoredI16] = Constant0Pattern(client, 'constant_minus_4')

class CatalogTree_Computed_Fetched:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.price_ohlc_in_cents: Indexes13[OHLCCents] = Indexes13(client, f'{base_path}/price_ohlc_in_cents')

class CatalogTree_Computed_Indexes:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.date: Indexes13[Date] = Indexes13(client, f'{base_path}/date')
        self.date_fixed: Indexes2[Date] = Indexes2(client, f'{base_path}/date_fixed')
        self.dateindex: Indexes13[DateIndex] = Indexes13(client, f'{base_path}/dateindex')
        self.dateindex_count: Indexes14[StoredU64] = Indexes14(client, f'{base_path}/dateindex_count')
        self.decadeindex: MetricNode[DecadeIndex] = MetricNode(client, f'{base_path}/decadeindex')
        self.difficultyepoch: MetricNode[DifficultyEpoch] = MetricNode(client, f'{base_path}/difficultyepoch')
        self.emptyoutputindex: MetricNode[EmptyOutputIndex] = MetricNode(client, f'{base_path}/emptyoutputindex')
        self.first_dateindex: Indexes14[DateIndex] = Indexes14(client, f'{base_path}/first_dateindex')
        self.first_height: MetricNode[Height] = MetricNode(client, f'{base_path}/first_height')
        self.first_monthindex: Indexes15[MonthIndex] = Indexes15(client, f'{base_path}/first_monthindex')
        self.first_yearindex: Indexes7[YearIndex] = Indexes7(client, f'{base_path}/first_yearindex')
        self.halvingepoch: MetricNode[HalvingEpoch] = MetricNode(client, f'{base_path}/halvingepoch')
        self.height: Indexes2[Height] = Indexes2(client, f'{base_path}/height')
        self.height_count: MetricNode[StoredU64] = MetricNode(client, f'{base_path}/height_count')
        self.input_count: Indexes6[StoredU64] = Indexes6(client, f'{base_path}/input_count')
        self.monthindex: MetricNode[MonthIndex] = MetricNode(client, f'{base_path}/monthindex')
        self.monthindex_count: Indexes15[StoredU64] = Indexes15(client, f'{base_path}/monthindex_count')
        self.opreturnindex: MetricNode[OpReturnIndex] = MetricNode(client, f'{base_path}/opreturnindex')
        self.output_count: Indexes6[StoredU64] = Indexes6(client, f'{base_path}/output_count')
        self.p2aaddressindex: Indexes16[P2AAddressIndex] = Indexes16(client, f'{base_path}/p2aaddressindex')
        self.p2msoutputindex: MetricNode[P2MSOutputIndex] = MetricNode(client, f'{base_path}/p2msoutputindex')
        self.p2pk33addressindex: Indexes17[P2PK33AddressIndex] = Indexes17(client, f'{base_path}/p2pk33addressindex')
        self.p2pk65addressindex: Indexes18[P2PK65AddressIndex] = Indexes18(client, f'{base_path}/p2pk65addressindex')
        self.p2pkhaddressindex: Indexes19[P2PKHAddressIndex] = Indexes19(client, f'{base_path}/p2pkhaddressindex')
        self.p2shaddressindex: Indexes20[P2SHAddressIndex] = Indexes20(client, f'{base_path}/p2shaddressindex')
        self.p2traddressindex: Indexes21[P2TRAddressIndex] = Indexes21(client, f'{base_path}/p2traddressindex')
        self.p2wpkhaddressindex: Indexes22[P2WPKHAddressIndex] = Indexes22(client, f'{base_path}/p2wpkhaddressindex')
        self.p2wshaddressindex: Indexes23[P2WSHAddressIndex] = Indexes23(client, f'{base_path}/p2wshaddressindex')
        self.quarterindex: MetricNode[QuarterIndex] = MetricNode(client, f'{base_path}/quarterindex')
        self.semesterindex: MetricNode[SemesterIndex] = MetricNode(client, f'{base_path}/semesterindex')
        self.timestamp_fixed: Indexes2[Timestamp] = Indexes2(client, f'{base_path}/timestamp_fixed')
        self.txindex: Indexes6[TxIndex] = Indexes6(client, f'{base_path}/txindex')
        self.txindex_count: Indexes2[StoredU64] = Indexes2(client, f'{base_path}/txindex_count')
        self.txinindex: Indexes24[TxInIndex] = Indexes24(client, f'{base_path}/txinindex')
        self.txoutindex: Indexes25[TxOutIndex] = Indexes25(client, f'{base_path}/txoutindex')
        self.unknownoutputindex: MetricNode[UnknownOutputIndex] = MetricNode(client, f'{base_path}/unknownoutputindex')
        self.weekindex: MetricNode[WeekIndex] = MetricNode(client, f'{base_path}/weekindex')
        self.yearindex: MetricNode[YearIndex] = MetricNode(client, f'{base_path}/yearindex')
        self.yearindex_count: Indexes7[StoredU64] = Indexes7(client, f'{base_path}/yearindex_count')

class CatalogTree_Computed_Market:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._1d_returns_1m_sd: _1dReturns1mSdPattern = _1dReturns1mSdPattern(client, '1d_returns_1m_sd')
        self._1d_returns_1w_sd: _1dReturns1mSdPattern = _1dReturns1mSdPattern(client, '1d_returns_1w_sd')
        self._1d_returns_1y_sd: _1dReturns1mSdPattern = _1dReturns1mSdPattern(client, '1d_returns_1y_sd')
        self._10y_cagr: Indexes[StoredF32] = Indexes(client, f'{base_path}/_10y_cagr')
        self._10y_dca_avg_price: Indexes[Dollars] = Indexes(client, f'{base_path}/_10y_dca_avg_price')
        self._10y_dca_cagr: Indexes[StoredF32] = Indexes(client, f'{base_path}/_10y_dca_cagr')
        self._10y_dca_returns: Indexes[StoredF32] = Indexes(client, f'{base_path}/_10y_dca_returns')
        self._10y_dca_stack: Indexes[Sats] = Indexes(client, f'{base_path}/_10y_dca_stack')
        self._10y_price_returns: Indexes[StoredF32] = Indexes(client, f'{base_path}/_10y_price_returns')
        self._1d_price_returns: Indexes[StoredF32] = Indexes(client, f'{base_path}/_1d_price_returns')
        self._1m_dca_avg_price: Indexes[Dollars] = Indexes(client, f'{base_path}/_1m_dca_avg_price')
        self._1m_dca_returns: Indexes[StoredF32] = Indexes(client, f'{base_path}/_1m_dca_returns')
        self._1m_dca_stack: Indexes[Sats] = Indexes(client, f'{base_path}/_1m_dca_stack')
        self._1m_price_returns: Indexes[StoredF32] = Indexes(client, f'{base_path}/_1m_price_returns')
        self._1w_dca_avg_price: Indexes[Dollars] = Indexes(client, f'{base_path}/_1w_dca_avg_price')
        self._1w_dca_returns: Indexes[StoredF32] = Indexes(client, f'{base_path}/_1w_dca_returns')
        self._1w_dca_stack: Indexes[Sats] = Indexes(client, f'{base_path}/_1w_dca_stack')
        self._1w_price_returns: Indexes[StoredF32] = Indexes(client, f'{base_path}/_1w_price_returns')
        self._1y_dca_avg_price: Indexes[Dollars] = Indexes(client, f'{base_path}/_1y_dca_avg_price')
        self._1y_dca_returns: Indexes[StoredF32] = Indexes(client, f'{base_path}/_1y_dca_returns')
        self._1y_dca_stack: Indexes[Sats] = Indexes(client, f'{base_path}/_1y_dca_stack')
        self._1y_price_returns: Indexes[StoredF32] = Indexes(client, f'{base_path}/_1y_price_returns')
        self._2y_cagr: Indexes[StoredF32] = Indexes(client, f'{base_path}/_2y_cagr')
        self._2y_dca_avg_price: Indexes[Dollars] = Indexes(client, f'{base_path}/_2y_dca_avg_price')
        self._2y_dca_cagr: Indexes[StoredF32] = Indexes(client, f'{base_path}/_2y_dca_cagr')
        self._2y_dca_returns: Indexes[StoredF32] = Indexes(client, f'{base_path}/_2y_dca_returns')
        self._2y_dca_stack: Indexes[Sats] = Indexes(client, f'{base_path}/_2y_dca_stack')
        self._2y_price_returns: Indexes[StoredF32] = Indexes(client, f'{base_path}/_2y_price_returns')
        self._3m_dca_avg_price: Indexes[Dollars] = Indexes(client, f'{base_path}/_3m_dca_avg_price')
        self._3m_dca_returns: Indexes[StoredF32] = Indexes(client, f'{base_path}/_3m_dca_returns')
        self._3m_dca_stack: Indexes[Sats] = Indexes(client, f'{base_path}/_3m_dca_stack')
        self._3m_price_returns: Indexes[StoredF32] = Indexes(client, f'{base_path}/_3m_price_returns')
        self._3y_cagr: Indexes[StoredF32] = Indexes(client, f'{base_path}/_3y_cagr')
        self._3y_dca_avg_price: Indexes[Dollars] = Indexes(client, f'{base_path}/_3y_dca_avg_price')
        self._3y_dca_cagr: Indexes[StoredF32] = Indexes(client, f'{base_path}/_3y_dca_cagr')
        self._3y_dca_returns: Indexes[StoredF32] = Indexes(client, f'{base_path}/_3y_dca_returns')
        self._3y_dca_stack: Indexes[Sats] = Indexes(client, f'{base_path}/_3y_dca_stack')
        self._3y_price_returns: Indexes[StoredF32] = Indexes(client, f'{base_path}/_3y_price_returns')
        self._4y_cagr: Indexes[StoredF32] = Indexes(client, f'{base_path}/_4y_cagr')
        self._4y_dca_avg_price: Indexes[Dollars] = Indexes(client, f'{base_path}/_4y_dca_avg_price')
        self._4y_dca_cagr: Indexes[StoredF32] = Indexes(client, f'{base_path}/_4y_dca_cagr')
        self._4y_dca_returns: Indexes[StoredF32] = Indexes(client, f'{base_path}/_4y_dca_returns')
        self._4y_dca_stack: Indexes[Sats] = Indexes(client, f'{base_path}/_4y_dca_stack')
        self._4y_price_returns: Indexes[StoredF32] = Indexes(client, f'{base_path}/_4y_price_returns')
        self._5y_cagr: Indexes[StoredF32] = Indexes(client, f'{base_path}/_5y_cagr')
        self._5y_dca_avg_price: Indexes[Dollars] = Indexes(client, f'{base_path}/_5y_dca_avg_price')
        self._5y_dca_cagr: Indexes[StoredF32] = Indexes(client, f'{base_path}/_5y_dca_cagr')
        self._5y_dca_returns: Indexes[StoredF32] = Indexes(client, f'{base_path}/_5y_dca_returns')
        self._5y_dca_stack: Indexes[Sats] = Indexes(client, f'{base_path}/_5y_dca_stack')
        self._5y_price_returns: Indexes[StoredF32] = Indexes(client, f'{base_path}/_5y_price_returns')
        self._6m_dca_avg_price: Indexes[Dollars] = Indexes(client, f'{base_path}/_6m_dca_avg_price')
        self._6m_dca_returns: Indexes[StoredF32] = Indexes(client, f'{base_path}/_6m_dca_returns')
        self._6m_dca_stack: Indexes[Sats] = Indexes(client, f'{base_path}/_6m_dca_stack')
        self._6m_price_returns: Indexes[StoredF32] = Indexes(client, f'{base_path}/_6m_price_returns')
        self._6y_cagr: Indexes[StoredF32] = Indexes(client, f'{base_path}/_6y_cagr')
        self._6y_dca_avg_price: Indexes[Dollars] = Indexes(client, f'{base_path}/_6y_dca_avg_price')
        self._6y_dca_cagr: Indexes[StoredF32] = Indexes(client, f'{base_path}/_6y_dca_cagr')
        self._6y_dca_returns: Indexes[StoredF32] = Indexes(client, f'{base_path}/_6y_dca_returns')
        self._6y_dca_stack: Indexes[Sats] = Indexes(client, f'{base_path}/_6y_dca_stack')
        self._6y_price_returns: Indexes[StoredF32] = Indexes(client, f'{base_path}/_6y_price_returns')
        self._8y_cagr: Indexes[StoredF32] = Indexes(client, f'{base_path}/_8y_cagr')
        self._8y_dca_avg_price: Indexes[Dollars] = Indexes(client, f'{base_path}/_8y_dca_avg_price')
        self._8y_dca_cagr: Indexes[StoredF32] = Indexes(client, f'{base_path}/_8y_dca_cagr')
        self._8y_dca_returns: Indexes[StoredF32] = Indexes(client, f'{base_path}/_8y_dca_returns')
        self._8y_dca_stack: Indexes[Sats] = Indexes(client, f'{base_path}/_8y_dca_stack')
        self._8y_price_returns: Indexes[StoredF32] = Indexes(client, f'{base_path}/_8y_price_returns')
        self.days_since_price_ath: Indexes[StoredU16] = Indexes(client, f'{base_path}/days_since_price_ath')
        self.dca_class_2015_avg_price: Indexes[Dollars] = Indexes(client, f'{base_path}/dca_class_2015_avg_price')
        self.dca_class_2015_returns: Indexes[StoredF32] = Indexes(client, f'{base_path}/dca_class_2015_returns')
        self.dca_class_2015_stack: Indexes[Sats] = Indexes(client, f'{base_path}/dca_class_2015_stack')
        self.dca_class_2016_avg_price: Indexes[Dollars] = Indexes(client, f'{base_path}/dca_class_2016_avg_price')
        self.dca_class_2016_returns: Indexes[StoredF32] = Indexes(client, f'{base_path}/dca_class_2016_returns')
        self.dca_class_2016_stack: Indexes[Sats] = Indexes(client, f'{base_path}/dca_class_2016_stack')
        self.dca_class_2017_avg_price: Indexes[Dollars] = Indexes(client, f'{base_path}/dca_class_2017_avg_price')
        self.dca_class_2017_returns: Indexes[StoredF32] = Indexes(client, f'{base_path}/dca_class_2017_returns')
        self.dca_class_2017_stack: Indexes[Sats] = Indexes(client, f'{base_path}/dca_class_2017_stack')
        self.dca_class_2018_avg_price: Indexes[Dollars] = Indexes(client, f'{base_path}/dca_class_2018_avg_price')
        self.dca_class_2018_returns: Indexes[StoredF32] = Indexes(client, f'{base_path}/dca_class_2018_returns')
        self.dca_class_2018_stack: Indexes[Sats] = Indexes(client, f'{base_path}/dca_class_2018_stack')
        self.dca_class_2019_avg_price: Indexes[Dollars] = Indexes(client, f'{base_path}/dca_class_2019_avg_price')
        self.dca_class_2019_returns: Indexes[StoredF32] = Indexes(client, f'{base_path}/dca_class_2019_returns')
        self.dca_class_2019_stack: Indexes[Sats] = Indexes(client, f'{base_path}/dca_class_2019_stack')
        self.dca_class_2020_avg_price: Indexes[Dollars] = Indexes(client, f'{base_path}/dca_class_2020_avg_price')
        self.dca_class_2020_returns: Indexes[StoredF32] = Indexes(client, f'{base_path}/dca_class_2020_returns')
        self.dca_class_2020_stack: Indexes[Sats] = Indexes(client, f'{base_path}/dca_class_2020_stack')
        self.dca_class_2021_avg_price: Indexes[Dollars] = Indexes(client, f'{base_path}/dca_class_2021_avg_price')
        self.dca_class_2021_returns: Indexes[StoredF32] = Indexes(client, f'{base_path}/dca_class_2021_returns')
        self.dca_class_2021_stack: Indexes[Sats] = Indexes(client, f'{base_path}/dca_class_2021_stack')
        self.dca_class_2022_avg_price: Indexes[Dollars] = Indexes(client, f'{base_path}/dca_class_2022_avg_price')
        self.dca_class_2022_returns: Indexes[StoredF32] = Indexes(client, f'{base_path}/dca_class_2022_returns')
        self.dca_class_2022_stack: Indexes[Sats] = Indexes(client, f'{base_path}/dca_class_2022_stack')
        self.dca_class_2023_avg_price: Indexes[Dollars] = Indexes(client, f'{base_path}/dca_class_2023_avg_price')
        self.dca_class_2023_returns: Indexes[StoredF32] = Indexes(client, f'{base_path}/dca_class_2023_returns')
        self.dca_class_2023_stack: Indexes[Sats] = Indexes(client, f'{base_path}/dca_class_2023_stack')
        self.dca_class_2024_avg_price: Indexes[Dollars] = Indexes(client, f'{base_path}/dca_class_2024_avg_price')
        self.dca_class_2024_returns: Indexes[StoredF32] = Indexes(client, f'{base_path}/dca_class_2024_returns')
        self.dca_class_2024_stack: Indexes[Sats] = Indexes(client, f'{base_path}/dca_class_2024_stack')
        self.dca_class_2025_avg_price: Indexes[Dollars] = Indexes(client, f'{base_path}/dca_class_2025_avg_price')
        self.dca_class_2025_returns: Indexes[StoredF32] = Indexes(client, f'{base_path}/dca_class_2025_returns')
        self.dca_class_2025_stack: Indexes[Sats] = Indexes(client, f'{base_path}/dca_class_2025_stack')
        self.max_days_between_price_aths: Indexes[StoredU16] = Indexes(client, f'{base_path}/max_days_between_price_aths')
        self.max_years_between_price_aths: Indexes[StoredF32] = Indexes(client, f'{base_path}/max_years_between_price_aths')
        self.price_10y_ago: Indexes[Dollars] = Indexes(client, f'{base_path}/price_10y_ago')
        self.price_13d_ema: Price13dEmaPattern = Price13dEmaPattern(client, 'price_13d_ema')
        self.price_13d_sma: Price13dEmaPattern = Price13dEmaPattern(client, 'price_13d_sma')
        self.price_144d_ema: Price13dEmaPattern = Price13dEmaPattern(client, 'price_144d_ema')
        self.price_144d_sma: Price13dEmaPattern = Price13dEmaPattern(client, 'price_144d_sma')
        self.price_1d_ago: Indexes[Dollars] = Indexes(client, f'{base_path}/price_1d_ago')
        self.price_1m_ago: Indexes[Dollars] = Indexes(client, f'{base_path}/price_1m_ago')
        self.price_1m_ema: Price13dEmaPattern = Price13dEmaPattern(client, 'price_1m_ema')
        self.price_1m_max: Indexes[Dollars] = Indexes(client, f'{base_path}/price_1m_max')
        self.price_1m_min: Indexes[Dollars] = Indexes(client, f'{base_path}/price_1m_min')
        self.price_1m_sma: Price13dEmaPattern = Price13dEmaPattern(client, 'price_1m_sma')
        self.price_1m_volatility: Indexes[StoredF32] = Indexes(client, f'{base_path}/price_1m_volatility')
        self.price_1w_ago: Indexes[Dollars] = Indexes(client, f'{base_path}/price_1w_ago')
        self.price_1w_ema: Price13dEmaPattern = Price13dEmaPattern(client, 'price_1w_ema')
        self.price_1w_max: Indexes[Dollars] = Indexes(client, f'{base_path}/price_1w_max')
        self.price_1w_min: Indexes[Dollars] = Indexes(client, f'{base_path}/price_1w_min')
        self.price_1w_sma: Price13dEmaPattern = Price13dEmaPattern(client, 'price_1w_sma')
        self.price_1w_volatility: Indexes[StoredF32] = Indexes(client, f'{base_path}/price_1w_volatility')
        self.price_1y_ago: Indexes[Dollars] = Indexes(client, f'{base_path}/price_1y_ago')
        self.price_1y_ema: Price13dEmaPattern = Price13dEmaPattern(client, 'price_1y_ema')
        self.price_1y_max: Indexes[Dollars] = Indexes(client, f'{base_path}/price_1y_max')
        self.price_1y_min: Indexes[Dollars] = Indexes(client, f'{base_path}/price_1y_min')
        self.price_1y_sma: Price13dEmaPattern = Price13dEmaPattern(client, 'price_1y_sma')
        self.price_1y_volatility: Indexes[StoredF32] = Indexes(client, f'{base_path}/price_1y_volatility')
        self.price_200d_ema: Price13dEmaPattern = Price13dEmaPattern(client, 'price_200d_ema')
        self.price_200d_sma: Price13dEmaPattern = Price13dEmaPattern(client, 'price_200d_sma')
        self.price_200d_sma_x0_8: Indexes[Dollars] = Indexes(client, f'{base_path}/price_200d_sma_x0_8')
        self.price_200d_sma_x2_4: Indexes[Dollars] = Indexes(client, f'{base_path}/price_200d_sma_x2_4')
        self.price_200w_ema: Price13dEmaPattern = Price13dEmaPattern(client, 'price_200w_ema')
        self.price_200w_sma: Price13dEmaPattern = Price13dEmaPattern(client, 'price_200w_sma')
        self.price_21d_ema: Price13dEmaPattern = Price13dEmaPattern(client, 'price_21d_ema')
        self.price_21d_sma: Price13dEmaPattern = Price13dEmaPattern(client, 'price_21d_sma')
        self.price_2w_choppiness_index: Indexes[StoredF32] = Indexes(client, f'{base_path}/price_2w_choppiness_index')
        self.price_2w_max: Indexes[Dollars] = Indexes(client, f'{base_path}/price_2w_max')
        self.price_2w_min: Indexes[Dollars] = Indexes(client, f'{base_path}/price_2w_min')
        self.price_2y_ago: Indexes[Dollars] = Indexes(client, f'{base_path}/price_2y_ago')
        self.price_2y_ema: Price13dEmaPattern = Price13dEmaPattern(client, 'price_2y_ema')
        self.price_2y_sma: Price13dEmaPattern = Price13dEmaPattern(client, 'price_2y_sma')
        self.price_34d_ema: Price13dEmaPattern = Price13dEmaPattern(client, 'price_34d_ema')
        self.price_34d_sma: Price13dEmaPattern = Price13dEmaPattern(client, 'price_34d_sma')
        self.price_3m_ago: Indexes[Dollars] = Indexes(client, f'{base_path}/price_3m_ago')
        self.price_3y_ago: Indexes[Dollars] = Indexes(client, f'{base_path}/price_3y_ago')
        self.price_4y_ago: Indexes[Dollars] = Indexes(client, f'{base_path}/price_4y_ago')
        self.price_4y_ema: Price13dEmaPattern = Price13dEmaPattern(client, 'price_4y_ema')
        self.price_4y_sma: Price13dEmaPattern = Price13dEmaPattern(client, 'price_4y_sma')
        self.price_55d_ema: Price13dEmaPattern = Price13dEmaPattern(client, 'price_55d_ema')
        self.price_55d_sma: Price13dEmaPattern = Price13dEmaPattern(client, 'price_55d_sma')
        self.price_5y_ago: Indexes[Dollars] = Indexes(client, f'{base_path}/price_5y_ago')
        self.price_6m_ago: Indexes[Dollars] = Indexes(client, f'{base_path}/price_6m_ago')
        self.price_6y_ago: Indexes[Dollars] = Indexes(client, f'{base_path}/price_6y_ago')
        self.price_89d_ema: Price13dEmaPattern = Price13dEmaPattern(client, 'price_89d_ema')
        self.price_89d_sma: Price13dEmaPattern = Price13dEmaPattern(client, 'price_89d_sma')
        self.price_8d_ema: Price13dEmaPattern = Price13dEmaPattern(client, 'price_8d_ema')
        self.price_8d_sma: Price13dEmaPattern = Price13dEmaPattern(client, 'price_8d_sma')
        self.price_8y_ago: Indexes[Dollars] = Indexes(client, f'{base_path}/price_8y_ago')
        self.price_ath: Indexes26[Dollars] = Indexes26(client, f'{base_path}/price_ath')
        self.price_drawdown: Indexes26[StoredF32] = Indexes26(client, f'{base_path}/price_drawdown')
        self.price_true_range: Indexes5[StoredF32] = Indexes5(client, f'{base_path}/price_true_range')
        self.price_true_range_2w_sum: Indexes5[StoredF32] = Indexes5(client, f'{base_path}/price_true_range_2w_sum')

class CatalogTree_Computed_Pools:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.pool: Indexes2[PoolSlug] = Indexes2(client, f'{base_path}/pool')
        self.vecs: CatalogTree_Computed_Pools_Vecs = CatalogTree_Computed_Pools_Vecs(client, f'{base_path}/vecs')

class CatalogTree_Computed_Pools_Vecs:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.AXbt: AXbtPattern = AXbtPattern(client, f'{base_path}/AXbt')
        self.AaoPool: AXbtPattern = AXbtPattern(client, f'{base_path}/AaoPool')
        self.AntPool: AXbtPattern = AXbtPattern(client, f'{base_path}/AntPool')
        self.ArkPool: AXbtPattern = AXbtPattern(client, f'{base_path}/ArkPool')
        self.AsicMiner: AXbtPattern = AXbtPattern(client, f'{base_path}/AsicMiner')
        self.BatPool: AXbtPattern = AXbtPattern(client, f'{base_path}/BatPool')
        self.BcMonster: AXbtPattern = AXbtPattern(client, f'{base_path}/BcMonster')
        self.BcpoolIo: AXbtPattern = AXbtPattern(client, f'{base_path}/BcpoolIo')
        self.BinancePool: AXbtPattern = AXbtPattern(client, f'{base_path}/BinancePool')
        self.BitClub: AXbtPattern = AXbtPattern(client, f'{base_path}/BitClub')
        self.BitFuFuPool: AXbtPattern = AXbtPattern(client, f'{base_path}/BitFuFuPool')
        self.BitFury: AXbtPattern = AXbtPattern(client, f'{base_path}/BitFury')
        self.BitMinter: AXbtPattern = AXbtPattern(client, f'{base_path}/BitMinter')
        self.Bitalo: AXbtPattern = AXbtPattern(client, f'{base_path}/Bitalo')
        self.BitcoinAffiliateNetwork: AXbtPattern = AXbtPattern(client, f'{base_path}/BitcoinAffiliateNetwork')
        self.BitcoinCom: AXbtPattern = AXbtPattern(client, f'{base_path}/BitcoinCom')
        self.BitcoinIndia: AXbtPattern = AXbtPattern(client, f'{base_path}/BitcoinIndia')
        self.BitcoinRussia: AXbtPattern = AXbtPattern(client, f'{base_path}/BitcoinRussia')
        self.BitcoinUkraine: AXbtPattern = AXbtPattern(client, f'{base_path}/BitcoinUkraine')
        self.Bitfarms: AXbtPattern = AXbtPattern(client, f'{base_path}/Bitfarms')
        self.Bitparking: AXbtPattern = AXbtPattern(client, f'{base_path}/Bitparking')
        self.Bitsolo: AXbtPattern = AXbtPattern(client, f'{base_path}/Bitsolo')
        self.Bixin: AXbtPattern = AXbtPattern(client, f'{base_path}/Bixin')
        self.BlockFills: AXbtPattern = AXbtPattern(client, f'{base_path}/BlockFills')
        self.BraiinsPool: AXbtPattern = AXbtPattern(client, f'{base_path}/BraiinsPool')
        self.BravoMining: AXbtPattern = AXbtPattern(client, f'{base_path}/BravoMining')
        self.BtPool: AXbtPattern = AXbtPattern(client, f'{base_path}/BtPool')
        self.BtcCom: AXbtPattern = AXbtPattern(client, f'{base_path}/BtcCom')
        self.BtcDig: AXbtPattern = AXbtPattern(client, f'{base_path}/BtcDig')
        self.BtcGuild: AXbtPattern = AXbtPattern(client, f'{base_path}/BtcGuild')
        self.BtcLab: AXbtPattern = AXbtPattern(client, f'{base_path}/BtcLab')
        self.BtcMp: AXbtPattern = AXbtPattern(client, f'{base_path}/BtcMp')
        self.BtcNuggets: AXbtPattern = AXbtPattern(client, f'{base_path}/BtcNuggets')
        self.BtcPoolParty: AXbtPattern = AXbtPattern(client, f'{base_path}/BtcPoolParty')
        self.BtcServ: AXbtPattern = AXbtPattern(client, f'{base_path}/BtcServ')
        self.BtcTop: AXbtPattern = AXbtPattern(client, f'{base_path}/BtcTop')
        self.Btcc: AXbtPattern = AXbtPattern(client, f'{base_path}/Btcc')
        self.BwPool: AXbtPattern = AXbtPattern(client, f'{base_path}/BwPool')
        self.BytePool: AXbtPattern = AXbtPattern(client, f'{base_path}/BytePool')
        self.Canoe: AXbtPattern = AXbtPattern(client, f'{base_path}/Canoe')
        self.CanoePool: AXbtPattern = AXbtPattern(client, f'{base_path}/CanoePool')
        self.CarbonNegative: AXbtPattern = AXbtPattern(client, f'{base_path}/CarbonNegative')
        self.CkPool: AXbtPattern = AXbtPattern(client, f'{base_path}/CkPool')
        self.CloudHashing: AXbtPattern = AXbtPattern(client, f'{base_path}/CloudHashing')
        self.CoinLab: AXbtPattern = AXbtPattern(client, f'{base_path}/CoinLab')
        self.Cointerra: AXbtPattern = AXbtPattern(client, f'{base_path}/Cointerra')
        self.ConnectBtc: AXbtPattern = AXbtPattern(client, f'{base_path}/ConnectBtc')
        self.DPool: AXbtPattern = AXbtPattern(client, f'{base_path}/DPool')
        self.DcExploration: AXbtPattern = AXbtPattern(client, f'{base_path}/DcExploration')
        self.Dcex: AXbtPattern = AXbtPattern(client, f'{base_path}/Dcex')
        self.DigitalBtc: AXbtPattern = AXbtPattern(client, f'{base_path}/DigitalBtc')
        self.DigitalXMintsy: AXbtPattern = AXbtPattern(client, f'{base_path}/DigitalXMintsy')
        self.EclipseMc: AXbtPattern = AXbtPattern(client, f'{base_path}/EclipseMc')
        self.EightBaochi: AXbtPattern = AXbtPattern(client, f'{base_path}/EightBaochi')
        self.EkanemBtc: AXbtPattern = AXbtPattern(client, f'{base_path}/EkanemBtc')
        self.Eligius: AXbtPattern = AXbtPattern(client, f'{base_path}/Eligius')
        self.EmcdPool: AXbtPattern = AXbtPattern(client, f'{base_path}/EmcdPool')
        self.EntrustCharityPool: AXbtPattern = AXbtPattern(client, f'{base_path}/EntrustCharityPool')
        self.Eobot: AXbtPattern = AXbtPattern(client, f'{base_path}/Eobot')
        self.ExxBw: AXbtPattern = AXbtPattern(client, f'{base_path}/ExxBw')
        self.F2Pool: AXbtPattern = AXbtPattern(client, f'{base_path}/F2Pool')
        self.FiftyEightCoin: AXbtPattern = AXbtPattern(client, f'{base_path}/FiftyEightCoin')
        self.FoundryUsa: AXbtPattern = AXbtPattern(client, f'{base_path}/FoundryUsa')
        self.FutureBitApolloSolo: AXbtPattern = AXbtPattern(client, f'{base_path}/FutureBitApolloSolo')
        self.GbMiners: AXbtPattern = AXbtPattern(client, f'{base_path}/GbMiners')
        self.GhashIo: AXbtPattern = AXbtPattern(client, f'{base_path}/GhashIo')
        self.GiveMeCoins: AXbtPattern = AXbtPattern(client, f'{base_path}/GiveMeCoins')
        self.GoGreenLight: AXbtPattern = AXbtPattern(client, f'{base_path}/GoGreenLight')
        self.HaoZhuZhu: AXbtPattern = AXbtPattern(client, f'{base_path}/HaoZhuZhu')
        self.Haominer: AXbtPattern = AXbtPattern(client, f'{base_path}/Haominer')
        self.HashBx: AXbtPattern = AXbtPattern(client, f'{base_path}/HashBx')
        self.HashPool: AXbtPattern = AXbtPattern(client, f'{base_path}/HashPool')
        self.Helix: AXbtPattern = AXbtPattern(client, f'{base_path}/Helix')
        self.Hhtt: AXbtPattern = AXbtPattern(client, f'{base_path}/Hhtt')
        self.HotPool: AXbtPattern = AXbtPattern(client, f'{base_path}/HotPool')
        self.Hummerpool: AXbtPattern = AXbtPattern(client, f'{base_path}/Hummerpool')
        self.HuobiPool: AXbtPattern = AXbtPattern(client, f'{base_path}/HuobiPool')
        self.InnopolisTech: AXbtPattern = AXbtPattern(client, f'{base_path}/InnopolisTech')
        self.KanoPool: AXbtPattern = AXbtPattern(client, f'{base_path}/KanoPool')
        self.KncMiner: AXbtPattern = AXbtPattern(client, f'{base_path}/KncMiner')
        self.KuCoinPool: AXbtPattern = AXbtPattern(client, f'{base_path}/KuCoinPool')
        self.LubianCom: AXbtPattern = AXbtPattern(client, f'{base_path}/LubianCom')
        self.LuckyPool: AXbtPattern = AXbtPattern(client, f'{base_path}/LuckyPool')
        self.Luxor: AXbtPattern = AXbtPattern(client, f'{base_path}/Luxor')
        self.MaraPool: AXbtPattern = AXbtPattern(client, f'{base_path}/MaraPool')
        self.MaxBtc: AXbtPattern = AXbtPattern(client, f'{base_path}/MaxBtc')
        self.MaxiPool: AXbtPattern = AXbtPattern(client, f'{base_path}/MaxiPool')
        self.MegaBigPower: AXbtPattern = AXbtPattern(client, f'{base_path}/MegaBigPower')
        self.Minerium: AXbtPattern = AXbtPattern(client, f'{base_path}/Minerium')
        self.MiningCity: AXbtPattern = AXbtPattern(client, f'{base_path}/MiningCity')
        self.MiningDutch: AXbtPattern = AXbtPattern(client, f'{base_path}/MiningDutch')
        self.MiningKings: AXbtPattern = AXbtPattern(client, f'{base_path}/MiningKings')
        self.MiningSquared: AXbtPattern = AXbtPattern(client, f'{base_path}/MiningSquared')
        self.Mmpool: AXbtPattern = AXbtPattern(client, f'{base_path}/Mmpool')
        self.MtRed: AXbtPattern = AXbtPattern(client, f'{base_path}/MtRed')
        self.MultiCoinCo: AXbtPattern = AXbtPattern(client, f'{base_path}/MultiCoinCo')
        self.Multipool: AXbtPattern = AXbtPattern(client, f'{base_path}/Multipool')
        self.MyBtcCoinPool: AXbtPattern = AXbtPattern(client, f'{base_path}/MyBtcCoinPool')
        self.Neopool: AXbtPattern = AXbtPattern(client, f'{base_path}/Neopool')
        self.Nexious: AXbtPattern = AXbtPattern(client, f'{base_path}/Nexious')
        self.NiceHash: AXbtPattern = AXbtPattern(client, f'{base_path}/NiceHash')
        self.NmcBit: AXbtPattern = AXbtPattern(client, f'{base_path}/NmcBit')
        self.NovaBlock: AXbtPattern = AXbtPattern(client, f'{base_path}/NovaBlock')
        self.Ocean: AXbtPattern = AXbtPattern(client, f'{base_path}/Ocean')
        self.OkExPool: AXbtPattern = AXbtPattern(client, f'{base_path}/OkExPool')
        self.OkMiner: AXbtPattern = AXbtPattern(client, f'{base_path}/OkMiner')
        self.Okkong: AXbtPattern = AXbtPattern(client, f'{base_path}/Okkong')
        self.OkpoolTop: AXbtPattern = AXbtPattern(client, f'{base_path}/OkpoolTop')
        self.OneHash: AXbtPattern = AXbtPattern(client, f'{base_path}/OneHash')
        self.OneM1x: AXbtPattern = AXbtPattern(client, f'{base_path}/OneM1x')
        self.OneThash: AXbtPattern = AXbtPattern(client, f'{base_path}/OneThash')
        self.OzCoin: AXbtPattern = AXbtPattern(client, f'{base_path}/OzCoin')
        self.PHashIo: AXbtPattern = AXbtPattern(client, f'{base_path}/PHashIo')
        self.Parasite: AXbtPattern = AXbtPattern(client, f'{base_path}/Parasite')
        self.Patels: AXbtPattern = AXbtPattern(client, f'{base_path}/Patels')
        self.PegaPool: AXbtPattern = AXbtPattern(client, f'{base_path}/PegaPool')
        self.Phoenix: AXbtPattern = AXbtPattern(client, f'{base_path}/Phoenix')
        self.Polmine: AXbtPattern = AXbtPattern(client, f'{base_path}/Polmine')
        self.Pool175btc: AXbtPattern = AXbtPattern(client, f'{base_path}/Pool175btc')
        self.Pool50btc: AXbtPattern = AXbtPattern(client, f'{base_path}/Pool50btc')
        self.Poolin: AXbtPattern = AXbtPattern(client, f'{base_path}/Poolin')
        self.PortlandHodl: AXbtPattern = AXbtPattern(client, f'{base_path}/PortlandHodl')
        self.PublicPool: AXbtPattern = AXbtPattern(client, f'{base_path}/PublicPool')
        self.PureBtcCom: AXbtPattern = AXbtPattern(client, f'{base_path}/PureBtcCom')
        self.Rawpool: AXbtPattern = AXbtPattern(client, f'{base_path}/Rawpool')
        self.RigPool: AXbtPattern = AXbtPattern(client, f'{base_path}/RigPool')
        self.SbiCrypto: AXbtPattern = AXbtPattern(client, f'{base_path}/SbiCrypto')
        self.SecPool: AXbtPattern = AXbtPattern(client, f'{base_path}/SecPool')
        self.SecretSuperstar: AXbtPattern = AXbtPattern(client, f'{base_path}/SecretSuperstar')
        self.SevenPool: AXbtPattern = AXbtPattern(client, f'{base_path}/SevenPool')
        self.ShawnP0wers: AXbtPattern = AXbtPattern(client, f'{base_path}/ShawnP0wers')
        self.SigmapoolCom: AXbtPattern = AXbtPattern(client, f'{base_path}/SigmapoolCom')
        self.SimplecoinUs: AXbtPattern = AXbtPattern(client, f'{base_path}/SimplecoinUs')
        self.SoloCk: AXbtPattern = AXbtPattern(client, f'{base_path}/SoloCk')
        self.SpiderPool: AXbtPattern = AXbtPattern(client, f'{base_path}/SpiderPool')
        self.StMiningCorp: AXbtPattern = AXbtPattern(client, f'{base_path}/StMiningCorp')
        self.Tangpool: AXbtPattern = AXbtPattern(client, f'{base_path}/Tangpool')
        self.TatmasPool: AXbtPattern = AXbtPattern(client, f'{base_path}/TatmasPool')
        self.TbDice: AXbtPattern = AXbtPattern(client, f'{base_path}/TbDice')
        self.Telco214: AXbtPattern = AXbtPattern(client, f'{base_path}/Telco214')
        self.TerraPool: AXbtPattern = AXbtPattern(client, f'{base_path}/TerraPool')
        self.Tiger: AXbtPattern = AXbtPattern(client, f'{base_path}/Tiger')
        self.TigerpoolNet: AXbtPattern = AXbtPattern(client, f'{base_path}/TigerpoolNet')
        self.Titan: AXbtPattern = AXbtPattern(client, f'{base_path}/Titan')
        self.TransactionCoinMining: AXbtPattern = AXbtPattern(client, f'{base_path}/TransactionCoinMining')
        self.TrickysBtcPool: AXbtPattern = AXbtPattern(client, f'{base_path}/TrickysBtcPool')
        self.TripleMining: AXbtPattern = AXbtPattern(client, f'{base_path}/TripleMining')
        self.TwentyOneInc: AXbtPattern = AXbtPattern(client, f'{base_path}/TwentyOneInc')
        self.UltimusPool: AXbtPattern = AXbtPattern(client, f'{base_path}/UltimusPool')
        self.Unknown: AXbtPattern = AXbtPattern(client, f'{base_path}/Unknown')
        self.Unomp: AXbtPattern = AXbtPattern(client, f'{base_path}/Unomp')
        self.ViaBtc: AXbtPattern = AXbtPattern(client, f'{base_path}/ViaBtc')
        self.Waterhole: AXbtPattern = AXbtPattern(client, f'{base_path}/Waterhole')
        self.WayiCn: AXbtPattern = AXbtPattern(client, f'{base_path}/WayiCn')
        self.WhitePool: AXbtPattern = AXbtPattern(client, f'{base_path}/WhitePool')
        self.Wk057: AXbtPattern = AXbtPattern(client, f'{base_path}/Wk057')
        self.YourbtcNet: AXbtPattern = AXbtPattern(client, f'{base_path}/YourbtcNet')
        self.Zulupool: AXbtPattern = AXbtPattern(client, f'{base_path}/Zulupool')

class CatalogTree_Computed_Price:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.price_close: Indexes3[Dollars] = Indexes3(client, f'{base_path}/price_close')
        self.price_close_in_cents: Indexes13[Cents] = Indexes13(client, f'{base_path}/price_close_in_cents')
        self.price_close_in_sats: Indexes3[Sats] = Indexes3(client, f'{base_path}/price_close_in_sats')
        self.price_high: Indexes3[Dollars] = Indexes3(client, f'{base_path}/price_high')
        self.price_high_in_cents: Indexes13[Cents] = Indexes13(client, f'{base_path}/price_high_in_cents')
        self.price_high_in_sats: Indexes3[Sats] = Indexes3(client, f'{base_path}/price_high_in_sats')
        self.price_low: Indexes3[Dollars] = Indexes3(client, f'{base_path}/price_low')
        self.price_low_in_cents: Indexes13[Cents] = Indexes13(client, f'{base_path}/price_low_in_cents')
        self.price_low_in_sats: Indexes3[Sats] = Indexes3(client, f'{base_path}/price_low_in_sats')
        self.price_ohlc: Indexes3[OHLCDollars] = Indexes3(client, f'{base_path}/price_ohlc')
        self.price_ohlc_in_sats: Indexes3[OHLCSats] = Indexes3(client, f'{base_path}/price_ohlc_in_sats')
        self.price_open: Indexes3[Dollars] = Indexes3(client, f'{base_path}/price_open')
        self.price_open_in_cents: Indexes13[Cents] = Indexes13(client, f'{base_path}/price_open_in_cents')
        self.price_open_in_sats: Indexes3[Sats] = Indexes3(client, f'{base_path}/price_open_in_sats')

class CatalogTree_Computed_Stateful:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.addr_count: Indexes3[StoredU64] = Indexes3(client, f'{base_path}/addr_count')
        self.address_cohorts: CatalogTree_Computed_Stateful_AddressCohorts = CatalogTree_Computed_Stateful_AddressCohorts(client, f'{base_path}/address_cohorts')
        self.addresses_data: CatalogTree_Computed_Stateful_AddressesData = CatalogTree_Computed_Stateful_AddressesData(client, f'{base_path}/addresses_data')
        self.addresstype_to_height_to_addr_count: AddresstypeToHeightToAddrCountPattern[StoredU64] = AddresstypeToHeightToAddrCountPattern(client, f'{base_path}/addresstype_to_height_to_addr_count')
        self.addresstype_to_height_to_empty_addr_count: AddresstypeToHeightToAddrCountPattern[StoredU64] = AddresstypeToHeightToAddrCountPattern(client, f'{base_path}/addresstype_to_height_to_empty_addr_count')
        self.addresstype_to_indexes_to_addr_count: AddresstypeToHeightToAddrCountPattern[StoredU64] = AddresstypeToHeightToAddrCountPattern(client, f'{base_path}/addresstype_to_indexes_to_addr_count')
        self.addresstype_to_indexes_to_empty_addr_count: AddresstypeToHeightToAddrCountPattern[StoredU64] = AddresstypeToHeightToAddrCountPattern(client, f'{base_path}/addresstype_to_indexes_to_empty_addr_count')
        self.any_address_indexes: AddresstypeToHeightToAddrCountPattern[AnyAddressIndex] = AddresstypeToHeightToAddrCountPattern(client, f'{base_path}/any_address_indexes')
        self.chain_state: Indexes2[SupplyState] = Indexes2(client, f'{base_path}/chain_state')
        self.empty_addr_count: Indexes3[StoredU64] = Indexes3(client, f'{base_path}/empty_addr_count')
        self.emptyaddressindex: Indexes29[EmptyAddressIndex] = Indexes29(client, f'{base_path}/emptyaddressindex')
        self.loadedaddressindex: Indexes30[LoadedAddressIndex] = Indexes30(client, f'{base_path}/loadedaddressindex')
        self.market_cap: Indexes26[Dollars] = Indexes26(client, f'{base_path}/market_cap')
        self.opreturn_supply: SupplyPattern = SupplyPattern(client, f'{base_path}/opreturn_supply')
        self.unspendable_supply: SupplyPattern = SupplyPattern(client, f'{base_path}/unspendable_supply')
        self.utxo_cohorts: CatalogTree_Computed_Stateful_UtxoCohorts = CatalogTree_Computed_Stateful_UtxoCohorts(client, f'{base_path}/utxo_cohorts')

class CatalogTree_Computed_Stateful_AddressCohorts:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.amount_range: CatalogTree_Computed_Stateful_AddressCohorts_AmountRange = CatalogTree_Computed_Stateful_AddressCohorts_AmountRange(client, f'{base_path}/amount_range')
        self.ge_amount: CatalogTree_Computed_Stateful_AddressCohorts_GeAmount = CatalogTree_Computed_Stateful_AddressCohorts_GeAmount(client, f'{base_path}/ge_amount')
        self.lt_amount: CatalogTree_Computed_Stateful_AddressCohorts_LtAmount = CatalogTree_Computed_Stateful_AddressCohorts_LtAmount(client, f'{base_path}/lt_amount')

class CatalogTree_Computed_Stateful_AddressCohorts_AmountRange:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._0sats: _0satsPattern = _0satsPattern(client, f'{base_path}/_0sats')
        self._100btc_to_1k_btc: _0satsPattern = _0satsPattern(client, f'{base_path}/_100btc_to_1k_btc')
        self._100k_btc_or_more: _0satsPattern = _0satsPattern(client, f'{base_path}/_100k_btc_or_more')
        self._100k_sats_to_1m_sats: _0satsPattern = _0satsPattern(client, f'{base_path}/_100k_sats_to_1m_sats')
        self._100sats_to_1k_sats: _0satsPattern = _0satsPattern(client, f'{base_path}/_100sats_to_1k_sats')
        self._10btc_to_100btc: _0satsPattern = _0satsPattern(client, f'{base_path}/_10btc_to_100btc')
        self._10k_btc_to_100k_btc: _0satsPattern = _0satsPattern(client, f'{base_path}/_10k_btc_to_100k_btc')
        self._10k_sats_to_100k_sats: _0satsPattern = _0satsPattern(client, f'{base_path}/_10k_sats_to_100k_sats')
        self._10m_sats_to_1btc: _0satsPattern = _0satsPattern(client, f'{base_path}/_10m_sats_to_1btc')
        self._10sats_to_100sats: _0satsPattern = _0satsPattern(client, f'{base_path}/_10sats_to_100sats')
        self._1btc_to_10btc: _0satsPattern = _0satsPattern(client, f'{base_path}/_1btc_to_10btc')
        self._1k_btc_to_10k_btc: _0satsPattern = _0satsPattern(client, f'{base_path}/_1k_btc_to_10k_btc')
        self._1k_sats_to_10k_sats: _0satsPattern = _0satsPattern(client, f'{base_path}/_1k_sats_to_10k_sats')
        self._1m_sats_to_10m_sats: _0satsPattern = _0satsPattern(client, f'{base_path}/_1m_sats_to_10m_sats')
        self._1sat_to_10sats: _0satsPattern = _0satsPattern(client, f'{base_path}/_1sat_to_10sats')

class CatalogTree_Computed_Stateful_AddressCohorts_GeAmount:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._100btc: _0satsPattern = _0satsPattern(client, f'{base_path}/_100btc')
        self._100k_sats: _0satsPattern = _0satsPattern(client, f'{base_path}/_100k_sats')
        self._100sats: _0satsPattern = _0satsPattern(client, f'{base_path}/_100sats')
        self._10btc: _0satsPattern = _0satsPattern(client, f'{base_path}/_10btc')
        self._10k_btc: _0satsPattern = _0satsPattern(client, f'{base_path}/_10k_btc')
        self._10k_sats: _0satsPattern = _0satsPattern(client, f'{base_path}/_10k_sats')
        self._10m_sats: _0satsPattern = _0satsPattern(client, f'{base_path}/_10m_sats')
        self._10sats: _0satsPattern = _0satsPattern(client, f'{base_path}/_10sats')
        self._1btc: _0satsPattern = _0satsPattern(client, f'{base_path}/_1btc')
        self._1k_btc: _0satsPattern = _0satsPattern(client, f'{base_path}/_1k_btc')
        self._1k_sats: _0satsPattern = _0satsPattern(client, f'{base_path}/_1k_sats')
        self._1m_sats: _0satsPattern = _0satsPattern(client, f'{base_path}/_1m_sats')
        self._1sat: _0satsPattern = _0satsPattern(client, f'{base_path}/_1sat')

class CatalogTree_Computed_Stateful_AddressCohorts_LtAmount:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._100btc: _0satsPattern = _0satsPattern(client, f'{base_path}/_100btc')
        self._100k_btc: _0satsPattern = _0satsPattern(client, f'{base_path}/_100k_btc')
        self._100k_sats: _0satsPattern = _0satsPattern(client, f'{base_path}/_100k_sats')
        self._100sats: _0satsPattern = _0satsPattern(client, f'{base_path}/_100sats')
        self._10btc: _0satsPattern = _0satsPattern(client, f'{base_path}/_10btc')
        self._10k_btc: _0satsPattern = _0satsPattern(client, f'{base_path}/_10k_btc')
        self._10k_sats: _0satsPattern = _0satsPattern(client, f'{base_path}/_10k_sats')
        self._10m_sats: _0satsPattern = _0satsPattern(client, f'{base_path}/_10m_sats')
        self._10sats: _0satsPattern = _0satsPattern(client, f'{base_path}/_10sats')
        self._1btc: _0satsPattern = _0satsPattern(client, f'{base_path}/_1btc')
        self._1k_btc: _0satsPattern = _0satsPattern(client, f'{base_path}/_1k_btc')
        self._1k_sats: _0satsPattern = _0satsPattern(client, f'{base_path}/_1k_sats')
        self._1m_sats: _0satsPattern = _0satsPattern(client, f'{base_path}/_1m_sats')

class CatalogTree_Computed_Stateful_AddressesData:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.empty: Indexes29[EmptyAddressData] = Indexes29(client, f'{base_path}/empty')
        self.loaded: Indexes30[LoadedAddressData] = Indexes30(client, f'{base_path}/loaded')

class CatalogTree_Computed_Stateful_UtxoCohorts:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.age_range: CatalogTree_Computed_Stateful_UtxoCohorts_AgeRange = CatalogTree_Computed_Stateful_UtxoCohorts_AgeRange(client, f'{base_path}/age_range')
        self.all: CatalogTree_Computed_Stateful_UtxoCohorts_All = CatalogTree_Computed_Stateful_UtxoCohorts_All(client, f'{base_path}/all')
        self.amount_range: CatalogTree_Computed_Stateful_UtxoCohorts_AmountRange = CatalogTree_Computed_Stateful_UtxoCohorts_AmountRange(client, f'{base_path}/amount_range')
        self.epoch: CatalogTree_Computed_Stateful_UtxoCohorts_Epoch = CatalogTree_Computed_Stateful_UtxoCohorts_Epoch(client, f'{base_path}/epoch')
        self.ge_amount: CatalogTree_Computed_Stateful_UtxoCohorts_GeAmount = CatalogTree_Computed_Stateful_UtxoCohorts_GeAmount(client, f'{base_path}/ge_amount')
        self.lt_amount: CatalogTree_Computed_Stateful_UtxoCohorts_LtAmount = CatalogTree_Computed_Stateful_UtxoCohorts_LtAmount(client, f'{base_path}/lt_amount')
        self.max_age: CatalogTree_Computed_Stateful_UtxoCohorts_MaxAge = CatalogTree_Computed_Stateful_UtxoCohorts_MaxAge(client, f'{base_path}/max_age')
        self.min_age: CatalogTree_Computed_Stateful_UtxoCohorts_MinAge = CatalogTree_Computed_Stateful_UtxoCohorts_MinAge(client, f'{base_path}/min_age')
        self.term: CatalogTree_Computed_Stateful_UtxoCohorts_Term = CatalogTree_Computed_Stateful_UtxoCohorts_Term(client, f'{base_path}/term')
        self.type_: CatalogTree_Computed_Stateful_UtxoCohorts_Type = CatalogTree_Computed_Stateful_UtxoCohorts_Type(client, f'{base_path}/type_')
        self.year: CatalogTree_Computed_Stateful_UtxoCohorts_Year = CatalogTree_Computed_Stateful_UtxoCohorts_Year(client, f'{base_path}/year')

class CatalogTree_Computed_Stateful_UtxoCohorts_AgeRange:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._10y_to_12y: _10yTo12yPattern = _10yTo12yPattern(client, f'{base_path}/_10y_to_12y')
        self._12y_to_15y: _10yTo12yPattern = _10yTo12yPattern(client, f'{base_path}/_12y_to_15y')
        self._1d_to_1w: _10yTo12yPattern = _10yTo12yPattern(client, f'{base_path}/_1d_to_1w')
        self._1m_to_2m: _10yTo12yPattern = _10yTo12yPattern(client, f'{base_path}/_1m_to_2m')
        self._1w_to_1m: _10yTo12yPattern = _10yTo12yPattern(client, f'{base_path}/_1w_to_1m')
        self._1y_to_2y: _10yTo12yPattern = _10yTo12yPattern(client, f'{base_path}/_1y_to_2y')
        self._2m_to_3m: _10yTo12yPattern = _10yTo12yPattern(client, f'{base_path}/_2m_to_3m')
        self._2y_to_3y: _10yTo12yPattern = _10yTo12yPattern(client, f'{base_path}/_2y_to_3y')
        self._3m_to_4m: _10yTo12yPattern = _10yTo12yPattern(client, f'{base_path}/_3m_to_4m')
        self._3y_to_4y: _10yTo12yPattern = _10yTo12yPattern(client, f'{base_path}/_3y_to_4y')
        self._4m_to_5m: _10yTo12yPattern = _10yTo12yPattern(client, f'{base_path}/_4m_to_5m')
        self._4y_to_5y: _10yTo12yPattern = _10yTo12yPattern(client, f'{base_path}/_4y_to_5y')
        self._5m_to_6m: _10yTo12yPattern = _10yTo12yPattern(client, f'{base_path}/_5m_to_6m')
        self._5y_to_6y: _10yTo12yPattern = _10yTo12yPattern(client, f'{base_path}/_5y_to_6y')
        self._6m_to_1y: _10yTo12yPattern = _10yTo12yPattern(client, f'{base_path}/_6m_to_1y')
        self._6y_to_7y: _10yTo12yPattern = _10yTo12yPattern(client, f'{base_path}/_6y_to_7y')
        self._7y_to_8y: _10yTo12yPattern = _10yTo12yPattern(client, f'{base_path}/_7y_to_8y')
        self._8y_to_10y: _10yTo12yPattern = _10yTo12yPattern(client, f'{base_path}/_8y_to_10y')
        self.from_15y: _10yTo12yPattern = _10yTo12yPattern(client, f'{base_path}/from_15y')
        self.up_to_1d: UpTo1dPattern = UpTo1dPattern(client, f'{base_path}/up_to_1d')

class CatalogTree_Computed_Stateful_UtxoCohorts_All:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.activity: ActivityPattern = ActivityPattern(client, f'{base_path}/activity')
        self.price_paid: PricePaidPattern2 = PricePaidPattern2(client, f'{base_path}/price_paid')
        self.realized: RealizedPattern3 = RealizedPattern3(client, f'{base_path}/realized')
        self.relative: CatalogTree_Computed_Stateful_UtxoCohorts_All_Relative = CatalogTree_Computed_Stateful_UtxoCohorts_All_Relative(client, f'{base_path}/relative')
        self.supply: SupplyPattern2 = SupplyPattern2(client, f'{base_path}/supply')
        self.unrealized: UnrealizedPattern = UnrealizedPattern(client, f'{base_path}/unrealized')

class CatalogTree_Computed_Stateful_UtxoCohorts_All_Relative:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.neg_unrealized_loss_rel_to_own_total_unrealized_pnl: Indexes27[StoredF32] = Indexes27(client, f'{base_path}/neg_unrealized_loss_rel_to_own_total_unrealized_pnl')
        self.net_unrealized_pnl_rel_to_own_total_unrealized_pnl: Indexes26[StoredF32] = Indexes26(client, f'{base_path}/net_unrealized_pnl_rel_to_own_total_unrealized_pnl')
        self.supply_in_loss_rel_to_own_supply: Indexes27[StoredF64] = Indexes27(client, f'{base_path}/supply_in_loss_rel_to_own_supply')
        self.supply_in_profit_rel_to_own_supply: Indexes27[StoredF64] = Indexes27(client, f'{base_path}/supply_in_profit_rel_to_own_supply')
        self.unrealized_loss_rel_to_own_total_unrealized_pnl: Indexes27[StoredF32] = Indexes27(client, f'{base_path}/unrealized_loss_rel_to_own_total_unrealized_pnl')
        self.unrealized_profit_rel_to_own_total_unrealized_pnl: Indexes27[StoredF32] = Indexes27(client, f'{base_path}/unrealized_profit_rel_to_own_total_unrealized_pnl')

class CatalogTree_Computed_Stateful_UtxoCohorts_AmountRange:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._0sats: _0satsPattern2 = _0satsPattern2(client, f'{base_path}/_0sats')
        self._100btc_to_1k_btc: _0satsPattern2 = _0satsPattern2(client, f'{base_path}/_100btc_to_1k_btc')
        self._100k_btc_or_more: _0satsPattern2 = _0satsPattern2(client, f'{base_path}/_100k_btc_or_more')
        self._100k_sats_to_1m_sats: _0satsPattern2 = _0satsPattern2(client, f'{base_path}/_100k_sats_to_1m_sats')
        self._100sats_to_1k_sats: _0satsPattern2 = _0satsPattern2(client, f'{base_path}/_100sats_to_1k_sats')
        self._10btc_to_100btc: _0satsPattern2 = _0satsPattern2(client, f'{base_path}/_10btc_to_100btc')
        self._10k_btc_to_100k_btc: _0satsPattern2 = _0satsPattern2(client, f'{base_path}/_10k_btc_to_100k_btc')
        self._10k_sats_to_100k_sats: _0satsPattern2 = _0satsPattern2(client, f'{base_path}/_10k_sats_to_100k_sats')
        self._10m_sats_to_1btc: _0satsPattern2 = _0satsPattern2(client, f'{base_path}/_10m_sats_to_1btc')
        self._10sats_to_100sats: _0satsPattern2 = _0satsPattern2(client, f'{base_path}/_10sats_to_100sats')
        self._1btc_to_10btc: _0satsPattern2 = _0satsPattern2(client, f'{base_path}/_1btc_to_10btc')
        self._1k_btc_to_10k_btc: _0satsPattern2 = _0satsPattern2(client, f'{base_path}/_1k_btc_to_10k_btc')
        self._1k_sats_to_10k_sats: _0satsPattern2 = _0satsPattern2(client, f'{base_path}/_1k_sats_to_10k_sats')
        self._1m_sats_to_10m_sats: _0satsPattern2 = _0satsPattern2(client, f'{base_path}/_1m_sats_to_10m_sats')
        self._1sat_to_10sats: _0satsPattern2 = _0satsPattern2(client, f'{base_path}/_1sat_to_10sats')

class CatalogTree_Computed_Stateful_UtxoCohorts_Epoch:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._0: _10yTo12yPattern = _10yTo12yPattern(client, f'{base_path}/_0')
        self._1: _10yTo12yPattern = _10yTo12yPattern(client, f'{base_path}/_1')
        self._2: _10yTo12yPattern = _10yTo12yPattern(client, f'{base_path}/_2')
        self._3: _10yTo12yPattern = _10yTo12yPattern(client, f'{base_path}/_3')
        self._4: _10yTo12yPattern = _10yTo12yPattern(client, f'{base_path}/_4')

class CatalogTree_Computed_Stateful_UtxoCohorts_GeAmount:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._100btc: _0satsPattern2 = _0satsPattern2(client, f'{base_path}/_100btc')
        self._100k_sats: _0satsPattern2 = _0satsPattern2(client, f'{base_path}/_100k_sats')
        self._100sats: _0satsPattern2 = _0satsPattern2(client, f'{base_path}/_100sats')
        self._10btc: _0satsPattern2 = _0satsPattern2(client, f'{base_path}/_10btc')
        self._10k_btc: _0satsPattern2 = _0satsPattern2(client, f'{base_path}/_10k_btc')
        self._10k_sats: _0satsPattern2 = _0satsPattern2(client, f'{base_path}/_10k_sats')
        self._10m_sats: _0satsPattern2 = _0satsPattern2(client, f'{base_path}/_10m_sats')
        self._10sats: _0satsPattern2 = _0satsPattern2(client, f'{base_path}/_10sats')
        self._1btc: _0satsPattern2 = _0satsPattern2(client, f'{base_path}/_1btc')
        self._1k_btc: _0satsPattern2 = _0satsPattern2(client, f'{base_path}/_1k_btc')
        self._1k_sats: _0satsPattern2 = _0satsPattern2(client, f'{base_path}/_1k_sats')
        self._1m_sats: _0satsPattern2 = _0satsPattern2(client, f'{base_path}/_1m_sats')
        self._1sat: _0satsPattern2 = _0satsPattern2(client, f'{base_path}/_1sat')

class CatalogTree_Computed_Stateful_UtxoCohorts_LtAmount:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._100btc: _0satsPattern2 = _0satsPattern2(client, f'{base_path}/_100btc')
        self._100k_btc: _0satsPattern2 = _0satsPattern2(client, f'{base_path}/_100k_btc')
        self._100k_sats: _0satsPattern2 = _0satsPattern2(client, f'{base_path}/_100k_sats')
        self._100sats: _0satsPattern2 = _0satsPattern2(client, f'{base_path}/_100sats')
        self._10btc: _0satsPattern2 = _0satsPattern2(client, f'{base_path}/_10btc')
        self._10k_btc: _0satsPattern2 = _0satsPattern2(client, f'{base_path}/_10k_btc')
        self._10k_sats: _0satsPattern2 = _0satsPattern2(client, f'{base_path}/_10k_sats')
        self._10m_sats: _0satsPattern2 = _0satsPattern2(client, f'{base_path}/_10m_sats')
        self._10sats: _0satsPattern2 = _0satsPattern2(client, f'{base_path}/_10sats')
        self._1btc: _0satsPattern2 = _0satsPattern2(client, f'{base_path}/_1btc')
        self._1k_btc: _0satsPattern2 = _0satsPattern2(client, f'{base_path}/_1k_btc')
        self._1k_sats: _0satsPattern2 = _0satsPattern2(client, f'{base_path}/_1k_sats')
        self._1m_sats: _0satsPattern2 = _0satsPattern2(client, f'{base_path}/_1m_sats')

class CatalogTree_Computed_Stateful_UtxoCohorts_MaxAge:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._10y: UpTo1dPattern = UpTo1dPattern(client, f'{base_path}/_10y')
        self._12y: UpTo1dPattern = UpTo1dPattern(client, f'{base_path}/_12y')
        self._15y: UpTo1dPattern = UpTo1dPattern(client, f'{base_path}/_15y')
        self._1m: UpTo1dPattern = UpTo1dPattern(client, f'{base_path}/_1m')
        self._1w: UpTo1dPattern = UpTo1dPattern(client, f'{base_path}/_1w')
        self._1y: UpTo1dPattern = UpTo1dPattern(client, f'{base_path}/_1y')
        self._2m: UpTo1dPattern = UpTo1dPattern(client, f'{base_path}/_2m')
        self._2y: UpTo1dPattern = UpTo1dPattern(client, f'{base_path}/_2y')
        self._3m: UpTo1dPattern = UpTo1dPattern(client, f'{base_path}/_3m')
        self._3y: UpTo1dPattern = UpTo1dPattern(client, f'{base_path}/_3y')
        self._4m: UpTo1dPattern = UpTo1dPattern(client, f'{base_path}/_4m')
        self._4y: UpTo1dPattern = UpTo1dPattern(client, f'{base_path}/_4y')
        self._5m: UpTo1dPattern = UpTo1dPattern(client, f'{base_path}/_5m')
        self._5y: UpTo1dPattern = UpTo1dPattern(client, f'{base_path}/_5y')
        self._6m: UpTo1dPattern = UpTo1dPattern(client, f'{base_path}/_6m')
        self._6y: UpTo1dPattern = UpTo1dPattern(client, f'{base_path}/_6y')
        self._7y: UpTo1dPattern = UpTo1dPattern(client, f'{base_path}/_7y')
        self._8y: UpTo1dPattern = UpTo1dPattern(client, f'{base_path}/_8y')

class CatalogTree_Computed_Stateful_UtxoCohorts_MinAge:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._10y: _10yTo12yPattern = _10yTo12yPattern(client, f'{base_path}/_10y')
        self._12y: _10yTo12yPattern = _10yTo12yPattern(client, f'{base_path}/_12y')
        self._1d: _10yTo12yPattern = _10yTo12yPattern(client, f'{base_path}/_1d')
        self._1m: _10yTo12yPattern = _10yTo12yPattern(client, f'{base_path}/_1m')
        self._1w: _10yTo12yPattern = _10yTo12yPattern(client, f'{base_path}/_1w')
        self._1y: _10yTo12yPattern = _10yTo12yPattern(client, f'{base_path}/_1y')
        self._2m: _10yTo12yPattern = _10yTo12yPattern(client, f'{base_path}/_2m')
        self._2y: _10yTo12yPattern = _10yTo12yPattern(client, f'{base_path}/_2y')
        self._3m: _10yTo12yPattern = _10yTo12yPattern(client, f'{base_path}/_3m')
        self._3y: _10yTo12yPattern = _10yTo12yPattern(client, f'{base_path}/_3y')
        self._4m: _10yTo12yPattern = _10yTo12yPattern(client, f'{base_path}/_4m')
        self._4y: _10yTo12yPattern = _10yTo12yPattern(client, f'{base_path}/_4y')
        self._5m: _10yTo12yPattern = _10yTo12yPattern(client, f'{base_path}/_5m')
        self._5y: _10yTo12yPattern = _10yTo12yPattern(client, f'{base_path}/_5y')
        self._6m: _10yTo12yPattern = _10yTo12yPattern(client, f'{base_path}/_6m')
        self._6y: _10yTo12yPattern = _10yTo12yPattern(client, f'{base_path}/_6y')
        self._7y: _10yTo12yPattern = _10yTo12yPattern(client, f'{base_path}/_7y')
        self._8y: _10yTo12yPattern = _10yTo12yPattern(client, f'{base_path}/_8y')

class CatalogTree_Computed_Stateful_UtxoCohorts_Term:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.long: UpTo1dPattern = UpTo1dPattern(client, f'{base_path}/long')
        self.short: UpTo1dPattern = UpTo1dPattern(client, f'{base_path}/short')

class CatalogTree_Computed_Stateful_UtxoCohorts_Type:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.empty: _0satsPattern2 = _0satsPattern2(client, f'{base_path}/empty')
        self.p2a: _0satsPattern2 = _0satsPattern2(client, f'{base_path}/p2a')
        self.p2ms: _0satsPattern2 = _0satsPattern2(client, f'{base_path}/p2ms')
        self.p2pk33: _0satsPattern2 = _0satsPattern2(client, f'{base_path}/p2pk33')
        self.p2pk65: _0satsPattern2 = _0satsPattern2(client, f'{base_path}/p2pk65')
        self.p2pkh: _0satsPattern2 = _0satsPattern2(client, f'{base_path}/p2pkh')
        self.p2sh: _0satsPattern2 = _0satsPattern2(client, f'{base_path}/p2sh')
        self.p2tr: _0satsPattern2 = _0satsPattern2(client, f'{base_path}/p2tr')
        self.p2wpkh: _0satsPattern2 = _0satsPattern2(client, f'{base_path}/p2wpkh')
        self.p2wsh: _0satsPattern2 = _0satsPattern2(client, f'{base_path}/p2wsh')
        self.unknown: _0satsPattern2 = _0satsPattern2(client, f'{base_path}/unknown')

class CatalogTree_Computed_Stateful_UtxoCohorts_Year:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._2009: _10yTo12yPattern = _10yTo12yPattern(client, f'{base_path}/_2009')
        self._2010: _10yTo12yPattern = _10yTo12yPattern(client, f'{base_path}/_2010')
        self._2011: _10yTo12yPattern = _10yTo12yPattern(client, f'{base_path}/_2011')
        self._2012: _10yTo12yPattern = _10yTo12yPattern(client, f'{base_path}/_2012')
        self._2013: _10yTo12yPattern = _10yTo12yPattern(client, f'{base_path}/_2013')
        self._2014: _10yTo12yPattern = _10yTo12yPattern(client, f'{base_path}/_2014')
        self._2015: _10yTo12yPattern = _10yTo12yPattern(client, f'{base_path}/_2015')
        self._2016: _10yTo12yPattern = _10yTo12yPattern(client, f'{base_path}/_2016')
        self._2017: _10yTo12yPattern = _10yTo12yPattern(client, f'{base_path}/_2017')
        self._2018: _10yTo12yPattern = _10yTo12yPattern(client, f'{base_path}/_2018')
        self._2019: _10yTo12yPattern = _10yTo12yPattern(client, f'{base_path}/_2019')
        self._2020: _10yTo12yPattern = _10yTo12yPattern(client, f'{base_path}/_2020')
        self._2021: _10yTo12yPattern = _10yTo12yPattern(client, f'{base_path}/_2021')
        self._2022: _10yTo12yPattern = _10yTo12yPattern(client, f'{base_path}/_2022')
        self._2023: _10yTo12yPattern = _10yTo12yPattern(client, f'{base_path}/_2023')
        self._2024: _10yTo12yPattern = _10yTo12yPattern(client, f'{base_path}/_2024')
        self._2025: _10yTo12yPattern = _10yTo12yPattern(client, f'{base_path}/_2025')
        self._2026: _10yTo12yPattern = _10yTo12yPattern(client, f'{base_path}/_2026')

class CatalogTree_Computed_Txins:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.txoutindex: Indexes24[TxOutIndex] = Indexes24(client, f'{base_path}/txoutindex')
        self.value: Indexes24[Sats] = Indexes24(client, f'{base_path}/value')

class CatalogTree_Computed_Txouts:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.txinindex: Indexes25[TxInIndex] = Indexes25(client, f'{base_path}/txinindex')

class CatalogTree_Indexed:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.address: CatalogTree_Indexed_Address = CatalogTree_Indexed_Address(client, f'{base_path}/address')
        self.block: CatalogTree_Indexed_Block = CatalogTree_Indexed_Block(client, f'{base_path}/block')
        self.output: CatalogTree_Indexed_Output = CatalogTree_Indexed_Output(client, f'{base_path}/output')
        self.tx: CatalogTree_Indexed_Tx = CatalogTree_Indexed_Tx(client, f'{base_path}/tx')
        self.txin: CatalogTree_Indexed_Txin = CatalogTree_Indexed_Txin(client, f'{base_path}/txin')
        self.txout: CatalogTree_Indexed_Txout = CatalogTree_Indexed_Txout(client, f'{base_path}/txout')

class CatalogTree_Indexed_Address:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.first_p2aaddressindex: Indexes2[P2AAddressIndex] = Indexes2(client, f'{base_path}/first_p2aaddressindex')
        self.first_p2pk33addressindex: Indexes2[P2PK33AddressIndex] = Indexes2(client, f'{base_path}/first_p2pk33addressindex')
        self.first_p2pk65addressindex: Indexes2[P2PK65AddressIndex] = Indexes2(client, f'{base_path}/first_p2pk65addressindex')
        self.first_p2pkhaddressindex: Indexes2[P2PKHAddressIndex] = Indexes2(client, f'{base_path}/first_p2pkhaddressindex')
        self.first_p2shaddressindex: Indexes2[P2SHAddressIndex] = Indexes2(client, f'{base_path}/first_p2shaddressindex')
        self.first_p2traddressindex: Indexes2[P2TRAddressIndex] = Indexes2(client, f'{base_path}/first_p2traddressindex')
        self.first_p2wpkhaddressindex: Indexes2[P2WPKHAddressIndex] = Indexes2(client, f'{base_path}/first_p2wpkhaddressindex')
        self.first_p2wshaddressindex: Indexes2[P2WSHAddressIndex] = Indexes2(client, f'{base_path}/first_p2wshaddressindex')
        self.p2abytes: Indexes16[P2ABytes] = Indexes16(client, f'{base_path}/p2abytes')
        self.p2pk33bytes: Indexes17[P2PK33Bytes] = Indexes17(client, f'{base_path}/p2pk33bytes')
        self.p2pk65bytes: Indexes18[P2PK65Bytes] = Indexes18(client, f'{base_path}/p2pk65bytes')
        self.p2pkhbytes: Indexes19[P2PKHBytes] = Indexes19(client, f'{base_path}/p2pkhbytes')
        self.p2shbytes: Indexes20[P2SHBytes] = Indexes20(client, f'{base_path}/p2shbytes')
        self.p2trbytes: Indexes21[P2TRBytes] = Indexes21(client, f'{base_path}/p2trbytes')
        self.p2wpkhbytes: Indexes22[P2WPKHBytes] = Indexes22(client, f'{base_path}/p2wpkhbytes')
        self.p2wshbytes: Indexes23[P2WSHBytes] = Indexes23(client, f'{base_path}/p2wshbytes')

class CatalogTree_Indexed_Block:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.blockhash: Indexes2[BlockHash] = Indexes2(client, f'{base_path}/blockhash')
        self.difficulty: Indexes2[StoredF64] = Indexes2(client, f'{base_path}/difficulty')
        self.timestamp: Indexes2[Timestamp] = Indexes2(client, f'{base_path}/timestamp')
        self.total_size: Indexes2[StoredU64] = Indexes2(client, f'{base_path}/total_size')
        self.weight: Indexes2[Weight] = Indexes2(client, f'{base_path}/weight')

class CatalogTree_Indexed_Output:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.first_emptyoutputindex: Indexes2[EmptyOutputIndex] = Indexes2(client, f'{base_path}/first_emptyoutputindex')
        self.first_opreturnindex: Indexes2[OpReturnIndex] = Indexes2(client, f'{base_path}/first_opreturnindex')
        self.first_p2msoutputindex: Indexes2[P2MSOutputIndex] = Indexes2(client, f'{base_path}/first_p2msoutputindex')
        self.first_unknownoutputindex: Indexes2[UnknownOutputIndex] = Indexes2(client, f'{base_path}/first_unknownoutputindex')
        self.txindex: MetricNode[TxIndex] = MetricNode(client, f'{base_path}/txindex')

class CatalogTree_Indexed_Tx:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.base_size: Indexes6[StoredU32] = Indexes6(client, f'{base_path}/base_size')
        self.first_txindex: Indexes2[TxIndex] = Indexes2(client, f'{base_path}/first_txindex')
        self.first_txinindex: Indexes6[TxInIndex] = Indexes6(client, f'{base_path}/first_txinindex')
        self.first_txoutindex: Indexes6[TxOutIndex] = Indexes6(client, f'{base_path}/first_txoutindex')
        self.height: Indexes6[Height] = Indexes6(client, f'{base_path}/height')
        self.is_explicitly_rbf: Indexes6[StoredBool] = Indexes6(client, f'{base_path}/is_explicitly_rbf')
        self.rawlocktime: Indexes6[RawLockTime] = Indexes6(client, f'{base_path}/rawlocktime')
        self.total_size: Indexes6[StoredU32] = Indexes6(client, f'{base_path}/total_size')
        self.txid: Indexes6[Txid] = Indexes6(client, f'{base_path}/txid')
        self.txversion: Indexes6[TxVersion] = Indexes6(client, f'{base_path}/txversion')

class CatalogTree_Indexed_Txin:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.first_txinindex: Indexes2[TxInIndex] = Indexes2(client, f'{base_path}/first_txinindex')
        self.outpoint: Indexes24[OutPoint] = Indexes24(client, f'{base_path}/outpoint')
        self.outputtype: Indexes24[OutputType] = Indexes24(client, f'{base_path}/outputtype')
        self.txindex: Indexes24[TxIndex] = Indexes24(client, f'{base_path}/txindex')
        self.typeindex: Indexes24[TypeIndex] = Indexes24(client, f'{base_path}/typeindex')

class CatalogTree_Indexed_Txout:
    """Catalog tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.first_txoutindex: Indexes2[TxOutIndex] = Indexes2(client, f'{base_path}/first_txoutindex')
        self.outputtype: Indexes25[OutputType] = Indexes25(client, f'{base_path}/outputtype')
        self.txindex: Indexes25[TxIndex] = Indexes25(client, f'{base_path}/txindex')
        self.typeindex: Indexes25[TypeIndex] = Indexes25(client, f'{base_path}/typeindex')
        self.value: Indexes25[Sats] = Indexes25(client, f'{base_path}/value')

class BrkClient(BrkClientBase):
    """Main BRK client with catalog tree and API methods."""
    
    def __init__(self, base_url: str = 'http://localhost:3000', timeout: float = 30.0):
        super().__init__(base_url, timeout)
        self.tree = CatalogTree(self)

    def get_api_address_by_address(self, address: str) -> AddressStats:
        """Address information.

        Retrieve comprehensive information about a Bitcoin address including balance, transaction history, UTXOs, and estimated investment metrics. Supports all standard Bitcoin address types (P2PKH, P2SH, P2WPKH, P2WSH, P2TR, etc.)."""
        return self.get(f'/api/address/{address}')

    def get_api_address_by_address_txs(self, address: str, after_txid: Optional[str] = None, limit: Optional[str] = None) -> List[Txid]:
        """Address transaction IDs.

        Get transaction IDs for an address, newest first. Use after_txid for pagination."""
        params = []
        if after_txid is not None: params.append(f'after_txid={after_txid}')
        if limit is not None: params.append(f'limit={limit}')
        query = '&'.join(params)
        return self.get(f'/api/address/{address}/txs{"?" + query if query else ""}')

    def get_api_address_by_address_txs_chain(self, address: str, after_txid: Optional[str] = None, limit: Optional[str] = None) -> List[Txid]:
        """Address confirmed transactions.

        Get confirmed transaction IDs for an address, 25 per page. Use ?after_txid=<txid> for pagination."""
        params = []
        if after_txid is not None: params.append(f'after_txid={after_txid}')
        if limit is not None: params.append(f'limit={limit}')
        query = '&'.join(params)
        return self.get(f'/api/address/{address}/txs/chain{"?" + query if query else ""}')

    def get_api_address_by_address_txs_mempool(self, address: str) -> List[Txid]:
        """Address mempool transactions.

        Get unconfirmed transaction IDs for an address from the mempool (up to 50)."""
        return self.get(f'/api/address/{address}/txs/mempool')

    def get_api_address_by_address_utxo(self, address: str) -> List[Utxo]:
        """Address UTXOs.

        Get unspent transaction outputs for an address."""
        return self.get(f'/api/address/{address}/utxo')

    def get_api_block_height_by_height(self, height: str) -> BlockInfo:
        """Block by height.

        Retrieve block information by block height. Returns block metadata including hash, timestamp, difficulty, size, weight, and transaction count."""
        return self.get(f'/api/block-height/{height}')

    def get_api_block_by_hash(self, hash: str) -> BlockInfo:
        """Block information.

        Retrieve block information by block hash. Returns block metadata including height, timestamp, difficulty, size, weight, and transaction count."""
        return self.get(f'/api/block/{hash}')

    def get_api_block_by_hash_raw(self, hash: str) -> List[int]:
        """Raw block.

        Returns the raw block data in binary format."""
        return self.get(f'/api/block/{hash}/raw')

    def get_api_block_by_hash_status(self, hash: str) -> BlockStatus:
        """Block status.

        Retrieve the status of a block. Returns whether the block is in the best chain and, if so, its height and the hash of the next block."""
        return self.get(f'/api/block/{hash}/status')

    def get_api_block_by_hash_txid_by_index(self, hash: str, index: str) -> Txid:
        """Transaction ID at index.

        Retrieve a single transaction ID at a specific index within a block. Returns plain text txid."""
        return self.get(f'/api/block/{hash}/txid/{index}')

    def get_api_block_by_hash_txids(self, hash: str) -> List[Txid]:
        """Block transaction IDs.

        Retrieve all transaction IDs in a block by block hash."""
        return self.get(f'/api/block/{hash}/txids')

    def get_api_block_by_hash_txs_by_start_index(self, hash: str, start_index: str) -> List[Transaction]:
        """Block transactions (paginated).

        Retrieve transactions in a block by block hash, starting from the specified index. Returns up to 25 transactions at a time."""
        return self.get(f'/api/block/{hash}/txs/{start_index}')

    def get_api_blocks(self) -> List[BlockInfo]:
        """Recent blocks.

        Retrieve the last 10 blocks. Returns block metadata for each block."""
        return self.get('/api/blocks')

    def get_api_blocks_by_height(self, height: str) -> List[BlockInfo]:
        """Blocks from height.

        Retrieve up to 10 blocks going backwards from the given height. For example, height=100 returns blocks 100, 99, 98, ..., 91. Height=0 returns only block 0."""
        return self.get(f'/api/blocks/{height}')

    def get_api_mempool_info(self) -> MempoolInfo:
        """Mempool statistics.

        Get current mempool statistics including transaction count, total vsize, and total fees."""
        return self.get('/api/mempool/info')

    def get_api_mempool_txids(self) -> List[Txid]:
        """Mempool transaction IDs.

        Get all transaction IDs currently in the mempool."""
        return self.get('/api/mempool/txids')

    def get_api_metric_by_metric(self, metric: str) -> List[Index]:
        """Get supported indexes for a metric.

        Returns the list of indexes are supported by the specified metric. For example, `realized_price` might be available on dateindex, weekindex, and monthindex."""
        return self.get(f'/api/metric/{metric}')

    def get_api_metric_by_metric_by_index(self, metric: str, index: str, from_: Optional[str] = None, to: Optional[str] = None, count: Optional[str] = None, format: Optional[str] = None) -> MetricData:
        """Get metric data.

        Fetch data for a specific metric at the given index. Use query parameters to filter by date range and format (json/csv)."""
        params = []
        if from_ is not None: params.append(f'from={from_}')
        if to is not None: params.append(f'to={to}')
        if count is not None: params.append(f'count={count}')
        if format is not None: params.append(f'format={format}')
        query = '&'.join(params)
        return self.get(f'/api/metric/{metric}/{index}{"?" + query if query else ""}')

    def get_api_metrics_bulk(self, metrics: str, index: str, from_: Optional[str] = None, to: Optional[str] = None, count: Optional[str] = None, format: Optional[str] = None) -> List[MetricData]:
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

    def get_api_metrics_catalog(self) -> TreeNode:
        """Metrics catalog.

        Returns the complete hierarchical catalog of available metrics organized as a tree structure. Metrics are grouped by categories and subcategories. Best viewed in an interactive JSON viewer (e.g., Firefox's built-in JSON viewer) for easy navigation of the nested structure."""
        return self.get('/api/metrics/catalog')

    def get_api_metrics_count(self) -> List[MetricCount]:
        """Metric count.

        Current metric count"""
        return self.get('/api/metrics/count')

    def get_api_metrics_indexes(self) -> List[IndexInfo]:
        """List available indexes.

        Returns all available indexes with their accepted query aliases. Use any alias when querying metrics."""
        return self.get('/api/metrics/indexes')

    def get_api_metrics_list(self, page: Optional[str] = None) -> PaginatedMetrics:
        """Metrics list.

        Paginated list of available metrics"""
        params = []
        if page is not None: params.append(f'page={page}')
        query = '&'.join(params)
        return self.get(f'/api/metrics/list{"?" + query if query else ""}')

    def get_api_metrics_search_by_metric(self, metric: str, limit: Optional[str] = None) -> List[Metric]:
        """Search metrics.

        Fuzzy search for metrics by name. Supports partial matches and typos."""
        params = []
        if limit is not None: params.append(f'limit={limit}')
        query = '&'.join(params)
        return self.get(f'/api/metrics/search/{metric}{"?" + query if query else ""}')

    def get_api_tx_by_txid(self, txid: str) -> Transaction:
        """Transaction information.

        Retrieve complete transaction data by transaction ID (txid). Returns the full transaction details including inputs, outputs, and metadata. The transaction data is read directly from the blockchain data files."""
        return self.get(f'/api/tx/{txid}')

    def get_api_tx_by_txid_hex(self, txid: str) -> Hex:
        """Transaction hex.

        Retrieve the raw transaction as a hex-encoded string. Returns the serialized transaction in hexadecimal format."""
        return self.get(f'/api/tx/{txid}/hex')

    def get_api_tx_by_txid_outspend_by_vout(self, txid: str, vout: str) -> TxOutspend:
        """Output spend status.

        Get the spending status of a transaction output. Returns whether the output has been spent and, if so, the spending transaction details."""
        return self.get(f'/api/tx/{txid}/outspend/{vout}')

    def get_api_tx_by_txid_outspends(self, txid: str) -> List[TxOutspend]:
        """All output spend statuses.

        Get the spending status of all outputs in a transaction. Returns an array with the spend status for each output."""
        return self.get(f'/api/tx/{txid}/outspends')

    def get_api_tx_by_txid_status(self, txid: str) -> TxStatus:
        """Transaction status.

        Retrieve the confirmation status of a transaction. Returns whether the transaction is confirmed and, if so, the block height, hash, and timestamp."""
        return self.get(f'/api/tx/{txid}/status')

    def get_api_v1_difficulty_adjustment(self) -> DifficultyAdjustment:
        """Difficulty adjustment.

        Get current difficulty adjustment information including progress through the current epoch, estimated retarget date, and difficulty change prediction."""
        return self.get('/api/v1/difficulty-adjustment')

    def get_api_v1_fees_mempool_blocks(self) -> List[MempoolBlock]:
        """Projected mempool blocks.

        Get projected blocks from the mempool for fee estimation. Each block contains statistics about transactions that would be included if a block were mined now."""
        return self.get('/api/v1/fees/mempool-blocks')

    def get_api_v1_fees_recommended(self) -> RecommendedFees:
        """Recommended fees.

        Get recommended fee rates for different confirmation targets based on current mempool state."""
        return self.get('/api/v1/fees/recommended')

    def get_api_v1_mining_blocks_fees_by_time_period(self, time_period: str) -> List[BlockFeesEntry]:
        """Block fees.

        Get average block fees for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y"""
        return self.get(f'/api/v1/mining/blocks/fees/{time_period}')

    def get_api_v1_mining_blocks_rewards_by_time_period(self, time_period: str) -> List[BlockRewardsEntry]:
        """Block rewards.

        Get average block rewards (coinbase = subsidy + fees) for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y"""
        return self.get(f'/api/v1/mining/blocks/rewards/{time_period}')

    def get_api_v1_mining_blocks_sizes_weights_by_time_period(self, time_period: str) -> BlockSizesWeights:
        """Block sizes and weights.

        Get average block sizes and weights for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y"""
        return self.get(f'/api/v1/mining/blocks/sizes-weights/{time_period}')

    def get_api_v1_mining_blocks_timestamp_by_timestamp(self, timestamp: str) -> BlockTimestamp:
        """Block by timestamp.

        Find the block closest to a given UNIX timestamp."""
        return self.get(f'/api/v1/mining/blocks/timestamp/{timestamp}')

    def get_api_v1_mining_difficulty_adjustments(self) -> List[DifficultyAdjustmentEntry]:
        """Difficulty adjustments (all time).

        Get historical difficulty adjustments. Returns array of [timestamp, height, difficulty, change_percent]."""
        return self.get('/api/v1/mining/difficulty-adjustments')

    def get_api_v1_mining_difficulty_adjustments_by_time_period(self, time_period: str) -> List[DifficultyAdjustmentEntry]:
        """Difficulty adjustments.

        Get historical difficulty adjustments for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y. Returns array of [timestamp, height, difficulty, change_percent]."""
        return self.get(f'/api/v1/mining/difficulty-adjustments/{time_period}')

    def get_api_v1_mining_hashrate(self) -> HashrateSummary:
        """Network hashrate (all time).

        Get network hashrate and difficulty data for all time."""
        return self.get('/api/v1/mining/hashrate')

    def get_api_v1_mining_hashrate_by_time_period(self, time_period: str) -> HashrateSummary:
        """Network hashrate.

        Get network hashrate and difficulty data for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y"""
        return self.get(f'/api/v1/mining/hashrate/{time_period}')

    def get_api_v1_mining_pool_by_slug(self, slug: str) -> PoolDetail:
        """Mining pool details.

        Get detailed information about a specific mining pool including block counts and shares for different time periods."""
        return self.get(f'/api/v1/mining/pool/{slug}')

    def get_api_v1_mining_pools(self) -> List[PoolInfo]:
        """List all mining pools.

        Get list of all known mining pools with their identifiers."""
        return self.get('/api/v1/mining/pools')

    def get_api_v1_mining_pools_by_time_period(self, time_period: str) -> PoolsSummary:
        """Mining pool statistics.

        Get mining pool statistics for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y"""
        return self.get(f'/api/v1/mining/pools/{time_period}')

    def get_api_v1_mining_reward_stats_by_block_count(self, block_count: str) -> RewardStats:
        """Mining reward statistics.

        Get mining reward statistics for the last N blocks including total rewards, fees, and transaction count."""
        return self.get(f'/api/v1/mining/reward-stats/{block_count}')

    def get_api_v1_validate_address_by_address(self, address: str) -> AddressValidation:
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

