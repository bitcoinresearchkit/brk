// Auto-generated BRK JavaScript client
// Do not edit manually

// Type definitions

/** @typedef {string} Address */
/**
 * @typedef {Object} AddressChainStats
 * @property {number} fundedTxoCount
 * @property {Sats} fundedTxoSum
 * @property {number} spentTxoCount
 * @property {Sats} spentTxoSum
 * @property {number} txCount
 * @property {TypeIndex} typeIndex
 */
/**
 * @typedef {Object} AddressMempoolStats
 * @property {number} fundedTxoCount
 * @property {Sats} fundedTxoSum
 * @property {number} spentTxoCount
 * @property {Sats} spentTxoSum
 * @property {number} txCount
 */
/**
 * @typedef {Object} AddressParam
 * @property {Address} address
 */
/**
 * @typedef {Object} AddressStats
 * @property {Address} address
 * @property {AddressChainStats} chainStats
 * @property {(AddressMempoolStats|null)=} mempoolStats
 */
/**
 * @typedef {Object} AddressTxidsParam
 * @property {(Txid|null)=} afterTxid
 * @property {number=} limit
 */
/**
 * @typedef {Object} AddressValidation
 * @property {boolean} isvalid
 * @property {?string=} address
 * @property {?string=} scriptPubKey
 * @property {?boolean=} isscript
 * @property {?boolean=} iswitness
 * @property {?number=} witnessVersion
 * @property {?string=} witnessProgram
 */
/** @typedef {TypeIndex} AnyAddressIndex */
/** @typedef {number} Bitcoin */
/** @typedef {number} BlkPosition */
/**
 * @typedef {Object} BlockCountParam
 * @property {number} blockCount
 */
/**
 * @typedef {Object} BlockFeesEntry
 * @property {Height} avgHeight
 * @property {Timestamp} timestamp
 * @property {Sats} avgFees
 */
/** @typedef {string} BlockHash */
/**
 * @typedef {Object} BlockHashParam
 * @property {BlockHash} hash
 */
/**
 * @typedef {Object} BlockHashStartIndex
 * @property {BlockHash} hash
 * @property {TxIndex} startIndex
 */
/**
 * @typedef {Object} BlockHashTxIndex
 * @property {BlockHash} hash
 * @property {TxIndex} index
 */
/**
 * @typedef {Object} BlockInfo
 * @property {BlockHash} id
 * @property {Height} height
 * @property {number} txCount
 * @property {number} size
 * @property {Weight} weight
 * @property {Timestamp} timestamp
 * @property {number} difficulty
 */
/**
 * @typedef {Object} BlockRewardsEntry
 * @property {number} avgHeight
 * @property {number} timestamp
 * @property {number} avgRewards
 */
/**
 * @typedef {Object} BlockSizeEntry
 * @property {number} avgHeight
 * @property {number} timestamp
 * @property {number} avgSize
 */
/**
 * @typedef {Object} BlockSizesWeights
 * @property {BlockSizeEntry[]} sizes
 * @property {BlockWeightEntry[]} weights
 */
/**
 * @typedef {Object} BlockStatus
 * @property {boolean} inBestChain
 * @property {(Height|null)=} height
 * @property {(BlockHash|null)=} nextBest
 */
/**
 * @typedef {Object} BlockTimestamp
 * @property {Height} height
 * @property {BlockHash} hash
 * @property {string} timestamp
 */
/**
 * @typedef {Object} BlockWeightEntry
 * @property {number} avgHeight
 * @property {number} timestamp
 * @property {number} avgWeight
 */
/** @typedef {number} Cents */
/** @typedef {Cents} Close */
/**
 * @typedef {Object} DataRangeFormat
 * @property {?number=} from
 * @property {?number=} to
 * @property {?number=} count
 * @property {Format=} format
 */
/** @typedef {number} Date */
/** @typedef {number} DateIndex */
/** @typedef {number} DecadeIndex */
/**
 * @typedef {Object} DifficultyAdjustment
 * @property {number} progressPercent
 * @property {number} difficultyChange
 * @property {number} estimatedRetargetDate
 * @property {number} remainingBlocks
 * @property {number} remainingTime
 * @property {number} previousRetarget
 * @property {Height} nextRetargetHeight
 * @property {number} timeAvg
 * @property {number} adjustedTimeAvg
 * @property {number} timeOffset
 */
/**
 * @typedef {Object} DifficultyAdjustmentEntry
 * @property {Timestamp} timestamp
 * @property {Height} height
 * @property {number} difficulty
 * @property {number} changePercent
 */
/**
 * @typedef {Object} DifficultyEntry
 * @property {Timestamp} timestamp
 * @property {number} difficulty
 * @property {Height} height
 */
/** @typedef {number} DifficultyEpoch */
/** @typedef {number} Dollars */
/**
 * @typedef {Object} EmptyAddressData
 * @property {number} txCount
 * @property {number} fundedTxoCount
 * @property {Sats} transfered
 */
/** @typedef {TypeIndex} EmptyAddressIndex */
/** @typedef {TypeIndex} EmptyOutputIndex */
/** @typedef {number} FeeRate */
/** @typedef {("json"|"csv")} Format */
/** @typedef {number} HalvingEpoch */
/**
 * @typedef {Object} HashrateEntry
 * @property {Timestamp} timestamp
 * @property {number} avgHashrate
 */
/**
 * @typedef {Object} HashrateSummary
 * @property {HashrateEntry[]} hashrates
 * @property {DifficultyEntry[]} difficulty
 * @property {number} currentHashrate
 * @property {number} currentDifficulty
 */
/**
 * @typedef {Object} Health
 * @property {string} status
 * @property {string} service
 * @property {string} timestamp
 */
/** @typedef {number} Height */
/**
 * @typedef {Object} HeightParam
 * @property {Height} height
 */
/** @typedef {string} Hex */
/** @typedef {Cents} High */
/** @typedef {("dateindex"|"decadeindex"|"difficultyepoch"|"emptyoutputindex"|"halvingepoch"|"height"|"txinindex"|"monthindex"|"opreturnindex"|"txoutindex"|"p2aaddressindex"|"p2msoutputindex"|"p2pk33addressindex"|"p2pk65addressindex"|"p2pkhaddressindex"|"p2shaddressindex"|"p2traddressindex"|"p2wpkhaddressindex"|"p2wshaddressindex"|"quarterindex"|"semesterindex"|"txindex"|"unknownoutputindex"|"weekindex"|"yearindex"|"loadedaddressindex"|"emptyaddressindex")} Index */
/**
 * @typedef {Object} IndexInfo
 * @property {Index} index
 * @property {string[]} aliases
 */
/** @typedef {number} Limit */
/**
 * @typedef {Object} LimitParam
 * @property {Limit=} limit
 */
/**
 * @typedef {Object} LoadedAddressData
 * @property {number} txCount
 * @property {number} fundedTxoCount
 * @property {number} spentTxoCount
 * @property {Sats} received
 * @property {Sats} sent
 * @property {Dollars} realizedCap
 */
/** @typedef {TypeIndex} LoadedAddressIndex */
/** @typedef {Cents} Low */
/**
 * @typedef {Object} MempoolBlock
 * @property {number} blockSize
 * @property {number} blockVSize
 * @property {number} nTx
 * @property {Sats} totalFees
 * @property {FeeRate} medianFee
 * @property {FeeRate[]} feeRange
 */
/**
 * @typedef {Object} MempoolInfo
 * @property {number} count
 * @property {VSize} vsize
 * @property {Sats} totalFee
 */
/** @typedef {string} Metric */
/**
 * @typedef {Object} MetricCount
 * @property {number} distinctMetrics
 * @property {number} totalEndpoints
 * @property {number} lazyEndpoints
 * @property {number} storedEndpoints
 */
/**
 * @typedef {Object} MetricLeafWithSchema
 * @property {string} name
 * @property {string} valueType
 * @property {Index[]} indexes
 */
/**
 * @typedef {Object} MetricParam
 * @property {Metric} metric
 */
/**
 * @typedef {Object} MetricSelection
 * @property {Metrics} metrics
 * @property {Index} index
 * @property {?number=} from
 * @property {?number=} to
 * @property {?number=} count
 * @property {Format=} format
 */
/**
 * @typedef {Object} MetricSelectionLegacy
 * @property {Index} index
 * @property {Metrics} ids
 * @property {?number=} from
 * @property {?number=} to
 * @property {?number=} count
 * @property {Format=} format
 */
/**
 * @typedef {Object} MetricWithIndex
 * @property {Metric} metric
 * @property {Index} index
 */
/** @typedef {string} Metrics */
/** @typedef {number} MonthIndex */
/**
 * @typedef {Object} OHLCCents
 * @property {Open} open
 * @property {High} high
 * @property {Low} low
 * @property {Close} close
 */
/**
 * @typedef {Object} OHLCDollars
 * @property {Open} open
 * @property {High} high
 * @property {Low} low
 * @property {Close} close
 */
/**
 * @typedef {Object} OHLCSats
 * @property {Open} open
 * @property {High} high
 * @property {Low} low
 * @property {Close} close
 */
/** @typedef {TypeIndex} OpReturnIndex */
/** @typedef {Cents} Open */
/** @typedef {number} OutPoint */
/** @typedef {("p2pk65"|"p2pk33"|"p2pkh"|"p2ms"|"p2sh"|"opreturn"|"p2wpkh"|"p2wsh"|"p2tr"|"p2a"|"empty"|"unknown")} OutputType */
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
 * @typedef {Object} PaginatedMetrics
 * @property {number} currentPage
 * @property {number} maxPage
 * @property {string[]} metrics
 */
/**
 * @typedef {Object} Pagination
 * @property {?number=} page
 */
/**
 * @typedef {Object} PoolBlockCounts
 * @property {number} all
 * @property {number} _24h
 * @property {number} _1w
 */
/**
 * @typedef {Object} PoolBlockShares
 * @property {number} all
 * @property {number} _24h
 * @property {number} _1w
 */
/**
 * @typedef {Object} PoolDetail
 * @property {PoolDetailInfo} pool
 * @property {PoolBlockCounts} blockCount
 * @property {PoolBlockShares} blockShare
 * @property {number} estimatedHashrate
 * @property {?number=} reportedHashrate
 */
/**
 * @typedef {Object} PoolDetailInfo
 * @property {number} id
 * @property {string} name
 * @property {string} link
 * @property {string[]} addresses
 * @property {string[]} regexes
 * @property {PoolSlug} slug
 */
/**
 * @typedef {Object} PoolInfo
 * @property {string} name
 * @property {PoolSlug} slug
 * @property {number} uniqueId
 */
/** @typedef {("unknown"|"blockfills"|"ultimuspool"|"terrapool"|"luxor"|"onethash"|"btccom"|"bitfarms"|"huobipool"|"wayicn"|"canoepool"|"btctop"|"bitcoincom"|"pool175btc"|"gbminers"|"axbt"|"asicminer"|"bitminter"|"bitcoinrussia"|"btcserv"|"simplecoinus"|"btcguild"|"eligius"|"ozcoin"|"eclipsemc"|"maxbtc"|"triplemining"|"coinlab"|"pool50btc"|"ghashio"|"stminingcorp"|"bitparking"|"mmpool"|"polmine"|"kncminer"|"bitalo"|"f2pool"|"hhtt"|"megabigpower"|"mtred"|"nmcbit"|"yourbtcnet"|"givemecoins"|"braiinspool"|"antpool"|"multicoinco"|"bcpoolio"|"cointerra"|"kanopool"|"solock"|"ckpool"|"nicehash"|"bitclub"|"bitcoinaffiliatenetwork"|"btcc"|"bwpool"|"exxbw"|"bitsolo"|"bitfury"|"twentyoneinc"|"digitalbtc"|"eightbaochi"|"mybtccoinpool"|"tbdice"|"hashpool"|"nexious"|"bravomining"|"hotpool"|"okexpool"|"bcmonster"|"onehash"|"bixin"|"tatmaspool"|"viabtc"|"connectbtc"|"batpool"|"waterhole"|"dcexploration"|"dcex"|"btpool"|"fiftyeightcoin"|"bitcoinindia"|"shawnp0wers"|"phashio"|"rigpool"|"haozhuzhu"|"sevenpool"|"miningkings"|"hashbx"|"dpool"|"rawpool"|"haominer"|"helix"|"bitcoinukraine"|"poolin"|"secretsuperstar"|"tigerpoolnet"|"sigmapoolcom"|"okpooltop"|"hummerpool"|"tangpool"|"bytepool"|"spiderpool"|"novablock"|"miningcity"|"binancepool"|"minerium"|"lubiancom"|"okkong"|"aaopool"|"emcdpool"|"foundryusa"|"sbicrypto"|"arkpool"|"purebtccom"|"marapool"|"kucoinpool"|"entrustcharitypool"|"okminer"|"titan"|"pegapool"|"btcnuggets"|"cloudhashing"|"digitalxmintsy"|"telco214"|"btcpoolparty"|"multipool"|"transactioncoinmining"|"btcdig"|"trickysbtcpool"|"btcmp"|"eobot"|"unomp"|"patels"|"gogreenlight"|"ekanembtc"|"canoe"|"tiger"|"onem1x"|"zulupool"|"secpool"|"ocean"|"whitepool"|"wk057"|"futurebitapollosolo"|"carbonnegative"|"portlandhodl"|"phoenix"|"neopool"|"maxipool"|"bitfufupool"|"luckypool"|"miningdutch"|"publicpool"|"miningsquared"|"innopolistech"|"btclab"|"parasite")} PoolSlug */
/**
 * @typedef {Object} PoolSlugParam
 * @property {PoolSlug} slug
 */
/**
 * @typedef {Object} PoolStats
 * @property {number} poolId
 * @property {string} name
 * @property {string} link
 * @property {number} blockCount
 * @property {number} rank
 * @property {number} emptyBlocks
 * @property {PoolSlug} slug
 * @property {number} share
 */
/**
 * @typedef {Object} PoolsSummary
 * @property {PoolStats[]} pools
 * @property {number} blockCount
 * @property {number} lastEstimatedHashrate
 */
/** @typedef {number} QuarterIndex */
/** @typedef {number} RawLockTime */
/**
 * @typedef {Object} RecommendedFees
 * @property {FeeRate} fastestFee
 * @property {FeeRate} halfHourFee
 * @property {FeeRate} hourFee
 * @property {FeeRate} economyFee
 * @property {FeeRate} minimumFee
 */
/**
 * @typedef {Object} RewardStats
 * @property {Height} startBlock
 * @property {Height} endBlock
 * @property {Sats} totalReward
 * @property {Sats} totalFee
 * @property {number} totalTx
 */
/** @typedef {number} Sats */
/** @typedef {number} SemesterIndex */
/** @typedef {number} StoredBool */
/** @typedef {number} StoredF32 */
/** @typedef {number} StoredF64 */
/** @typedef {number} StoredI16 */
/** @typedef {number} StoredU16 */
/** @typedef {number} StoredU32 */
/** @typedef {number} StoredU64 */
/**
 * @typedef {Object} SupplyState
 * @property {number} utxoCount
 * @property {Sats} value
 */
/** @typedef {("24h"|"3d"|"1w"|"1m"|"3m"|"6m"|"1y"|"2y"|"3y")} TimePeriod */
/**
 * @typedef {Object} TimePeriodParam
 * @property {TimePeriod} timePeriod
 */
/** @typedef {number} Timestamp */
/**
 * @typedef {Object} TimestampParam
 * @property {Timestamp} timestamp
 */
/**
 * @typedef {Object} Transaction
 * @property {(TxIndex|null)=} index
 * @property {Txid} txid
 * @property {TxVersion} version
 * @property {RawLockTime} locktime
 * @property {number} size
 * @property {Weight} weight
 * @property {number} sigops
 * @property {Sats} fee
 * @property {TxIn[]} vin
 * @property {TxOut[]} vout
 * @property {TxStatus} status
 */
/** @typedef {({ [key: string]: TreeNode }|MetricLeafWithSchema)} TreeNode */
/**
 * @typedef {Object} TxIn
 * @property {Txid} txid
 * @property {Vout} vout
 * @property {(TxOut|null)=} prevout
 * @property {string} scriptsig
 * @property {string} scriptsigAsm
 * @property {boolean} isCoinbase
 * @property {number} sequence
 * @property {?string=} innerRedeemscriptAsm
 */
/** @typedef {number} TxInIndex */
/** @typedef {number} TxIndex */
/**
 * @typedef {Object} TxOut
 * @property {string} scriptpubkey
 * @property {Sats} value
 */
/** @typedef {number} TxOutIndex */
/**
 * @typedef {Object} TxOutspend
 * @property {boolean} spent
 * @property {(Txid|null)=} txid
 * @property {(Vin|null)=} vin
 * @property {(TxStatus|null)=} status
 */
/**
 * @typedef {Object} TxStatus
 * @property {boolean} confirmed
 * @property {(Height|null)=} blockHeight
 * @property {(BlockHash|null)=} blockHash
 * @property {(Timestamp|null)=} blockTime
 */
/** @typedef {number} TxVersion */
/** @typedef {string} Txid */
/**
 * @typedef {Object} TxidParam
 * @property {Txid} txid
 */
/**
 * @typedef {Object} TxidVout
 * @property {Txid} txid
 * @property {Vout} vout
 */
/** @typedef {number} TypeIndex */
/** @typedef {number[]} U8x2 */
/** @typedef {number[]} U8x20 */
/** @typedef {number[]} U8x32 */
/** @typedef {string} U8x33 */
/** @typedef {string} U8x65 */
/** @typedef {TypeIndex} UnknownOutputIndex */
/**
 * @typedef {Object} Utxo
 * @property {Txid} txid
 * @property {Vout} vout
 * @property {TxStatus} status
 * @property {Sats} value
 */
/** @typedef {number} VSize */
/**
 * @typedef {Object} ValidateAddressParam
 * @property {string} address
 */
/** @typedef {number} Vin */
/** @typedef {number} Vout */
/** @typedef {number} WeekIndex */
/** @typedef {number} Weight */
/** @typedef {number} YearIndex */

/**
 * @typedef {Object} BrkClientOptions
 * @property {string} baseUrl - Base URL for the API
 * @property {number} [timeout] - Request timeout in milliseconds
 */

const _isBrowser = typeof window !== 'undefined' && 'caches' in window;
const _runIdle = (/** @type {VoidFunction} */ fn) => (globalThis.requestIdleCallback ?? setTimeout)(fn);

/** @type {Promise<Cache | null>} */
const _cachePromise = _isBrowser
  ? caches.open('__BRK_CLIENT__').catch(() => null)
  : Promise.resolve(null);

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

/**
 * @template T
 * @typedef {Object} MetricData
 * @property {number} total - Total number of data points
 * @property {number} from - Start index (inclusive)
 * @property {number} to - End index (exclusive)
 * @property {T[]} data - The metric data
 */
/** @typedef {MetricData<unknown>} AnyMetricData */

/**
 * @template T
 * @typedef {Object} MetricEndpoint
 * @property {(onUpdate?: (value: MetricData<T>) => void) => Promise<MetricData<T>>} get - Fetch all data points
 * @property {(from?: number, to?: number, onUpdate?: (value: MetricData<T>) => void) => Promise<MetricData<T>>} range - Fetch data in range
 * @property {string} path - The endpoint path
 */
/** @typedef {MetricEndpoint<unknown>} AnyMetricEndpoint */

/**
 * @template T
 * @typedef {Object} MetricPattern
 * @property {string} name - The metric name
 * @property {Partial<Record<Index, MetricEndpoint<T>>>} by - Index endpoints (lazy getters)
 * @property {() => Index[]} indexes - Get the list of available indexes
 * @property {(index: Index) => MetricEndpoint<T>|undefined} get - Get an endpoint for a specific index
 */

/** @typedef {MetricPattern<unknown>} AnyMetricPattern */

/**
 * Create an endpoint for a metric index.
 * @template T
 * @param {BrkClientBase} client
 * @param {string} name - The metric vec name
 * @param {Index} index - The index name
 * @returns {MetricEndpoint<T>}
 */
function _endpoint(client, name, index) {
  const p = `/api/metric/${name}/${index}`;
  return {
    get: (onUpdate) => client.get(p, onUpdate),
    range: (from, to, onUpdate) => {
      const params = new URLSearchParams();
      if (from !== undefined) params.set('from', String(from));
      if (to !== undefined) params.set('to', String(to));
      const query = params.toString();
      return client.get(query ? `${p}?${query}` : p, onUpdate);
    },
    get path() { return p; },
  };
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
  }

  /**
   * Make a GET request with stale-while-revalidate caching
   * @template T
   * @param {string} path
   * @param {(value: T) => void} [onUpdate] - Called when data is available
   * @returns {Promise<T>}
   */
  async get(path, onUpdate) {
    const base = this.baseUrl.endsWith('/') ? this.baseUrl.slice(0, -1) : this.baseUrl;
    const url = `${base}${path}`;
    const cache = await _cachePromise;
    const cachedRes = await cache?.match(url);
    const cachedJson = cachedRes ? await cachedRes.json() : null;

    if (cachedJson) onUpdate?.(cachedJson);
    if (!globalThis.navigator?.onLine) {
      if (cachedJson) return cachedJson;
      throw new BrkError('Offline and no cached data available');
    }

    try {
      const res = await fetch(url, { signal: AbortSignal.timeout(this.timeout) });
      if (!res.ok) throw new BrkError(`HTTP ${res.status}`, res.status);
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
}

/**
 * Build metric name with optional prefix.
 * @param {string} acc - Accumulated prefix
 * @param {string} s - Metric suffix
 * @returns {string}
 */
const _m = (acc, s) => acc ? `${acc}_${s}` : s;


// Index accessor factory functions

/**
 * @template T
 * @typedef {{ name: string, by: { dateindex: MetricEndpoint<T>, decadeindex: MetricEndpoint<T>, difficultyepoch: MetricEndpoint<T>, height: MetricEndpoint<T>, monthindex: MetricEndpoint<T>, quarterindex: MetricEndpoint<T>, semesterindex: MetricEndpoint<T>, weekindex: MetricEndpoint<T>, yearindex: MetricEndpoint<T> }, indexes: () => Index[], get: (index: Index) => MetricEndpoint<T>|undefined }} MetricPattern1
 */

/**
 * Create a MetricPattern1 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} name - The metric vec name
 * @returns {MetricPattern1<T>}
 */
function createMetricPattern1(client, name) {
  return {
    name,
    by: {
      get dateindex() { return _endpoint(client, name, 'dateindex'); },
      get decadeindex() { return _endpoint(client, name, 'decadeindex'); },
      get difficultyepoch() { return _endpoint(client, name, 'difficultyepoch'); },
      get height() { return _endpoint(client, name, 'height'); },
      get monthindex() { return _endpoint(client, name, 'monthindex'); },
      get quarterindex() { return _endpoint(client, name, 'quarterindex'); },
      get semesterindex() { return _endpoint(client, name, 'semesterindex'); },
      get weekindex() { return _endpoint(client, name, 'weekindex'); },
      get yearindex() { return _endpoint(client, name, 'yearindex'); }
    },
    indexes() {
      return ['dateindex', 'decadeindex', 'difficultyepoch', 'height', 'monthindex', 'quarterindex', 'semesterindex', 'weekindex', 'yearindex'];
    },
    get(index) {
      if (this.indexes().includes(index)) {
        return _endpoint(client, name, index);
      }
    }
  };
}

/**
 * @template T
 * @typedef {{ name: string, by: { dateindex: MetricEndpoint<T>, decadeindex: MetricEndpoint<T>, difficultyepoch: MetricEndpoint<T>, monthindex: MetricEndpoint<T>, quarterindex: MetricEndpoint<T>, semesterindex: MetricEndpoint<T>, weekindex: MetricEndpoint<T>, yearindex: MetricEndpoint<T> }, indexes: () => Index[], get: (index: Index) => MetricEndpoint<T>|undefined }} MetricPattern2
 */

/**
 * Create a MetricPattern2 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} name - The metric vec name
 * @returns {MetricPattern2<T>}
 */
function createMetricPattern2(client, name) {
  return {
    name,
    by: {
      get dateindex() { return _endpoint(client, name, 'dateindex'); },
      get decadeindex() { return _endpoint(client, name, 'decadeindex'); },
      get difficultyepoch() { return _endpoint(client, name, 'difficultyepoch'); },
      get monthindex() { return _endpoint(client, name, 'monthindex'); },
      get quarterindex() { return _endpoint(client, name, 'quarterindex'); },
      get semesterindex() { return _endpoint(client, name, 'semesterindex'); },
      get weekindex() { return _endpoint(client, name, 'weekindex'); },
      get yearindex() { return _endpoint(client, name, 'yearindex'); }
    },
    indexes() {
      return ['dateindex', 'decadeindex', 'difficultyepoch', 'monthindex', 'quarterindex', 'semesterindex', 'weekindex', 'yearindex'];
    },
    get(index) {
      if (this.indexes().includes(index)) {
        return _endpoint(client, name, index);
      }
    }
  };
}

/**
 * @template T
 * @typedef {{ name: string, by: { dateindex: MetricEndpoint<T>, decadeindex: MetricEndpoint<T>, height: MetricEndpoint<T>, monthindex: MetricEndpoint<T>, quarterindex: MetricEndpoint<T>, semesterindex: MetricEndpoint<T>, weekindex: MetricEndpoint<T>, yearindex: MetricEndpoint<T> }, indexes: () => Index[], get: (index: Index) => MetricEndpoint<T>|undefined }} MetricPattern3
 */

/**
 * Create a MetricPattern3 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} name - The metric vec name
 * @returns {MetricPattern3<T>}
 */
function createMetricPattern3(client, name) {
  return {
    name,
    by: {
      get dateindex() { return _endpoint(client, name, 'dateindex'); },
      get decadeindex() { return _endpoint(client, name, 'decadeindex'); },
      get height() { return _endpoint(client, name, 'height'); },
      get monthindex() { return _endpoint(client, name, 'monthindex'); },
      get quarterindex() { return _endpoint(client, name, 'quarterindex'); },
      get semesterindex() { return _endpoint(client, name, 'semesterindex'); },
      get weekindex() { return _endpoint(client, name, 'weekindex'); },
      get yearindex() { return _endpoint(client, name, 'yearindex'); }
    },
    indexes() {
      return ['dateindex', 'decadeindex', 'height', 'monthindex', 'quarterindex', 'semesterindex', 'weekindex', 'yearindex'];
    },
    get(index) {
      if (this.indexes().includes(index)) {
        return _endpoint(client, name, index);
      }
    }
  };
}

/**
 * @template T
 * @typedef {{ name: string, by: { decadeindex: MetricEndpoint<T>, difficultyepoch: MetricEndpoint<T>, height: MetricEndpoint<T>, monthindex: MetricEndpoint<T>, quarterindex: MetricEndpoint<T>, semesterindex: MetricEndpoint<T>, weekindex: MetricEndpoint<T>, yearindex: MetricEndpoint<T> }, indexes: () => Index[], get: (index: Index) => MetricEndpoint<T>|undefined }} MetricPattern4
 */

/**
 * Create a MetricPattern4 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} name - The metric vec name
 * @returns {MetricPattern4<T>}
 */
function createMetricPattern4(client, name) {
  return {
    name,
    by: {
      get decadeindex() { return _endpoint(client, name, 'decadeindex'); },
      get difficultyepoch() { return _endpoint(client, name, 'difficultyepoch'); },
      get height() { return _endpoint(client, name, 'height'); },
      get monthindex() { return _endpoint(client, name, 'monthindex'); },
      get quarterindex() { return _endpoint(client, name, 'quarterindex'); },
      get semesterindex() { return _endpoint(client, name, 'semesterindex'); },
      get weekindex() { return _endpoint(client, name, 'weekindex'); },
      get yearindex() { return _endpoint(client, name, 'yearindex'); }
    },
    indexes() {
      return ['decadeindex', 'difficultyepoch', 'height', 'monthindex', 'quarterindex', 'semesterindex', 'weekindex', 'yearindex'];
    },
    get(index) {
      if (this.indexes().includes(index)) {
        return _endpoint(client, name, index);
      }
    }
  };
}

/**
 * @template T
 * @typedef {{ name: string, by: { dateindex: MetricEndpoint<T>, decadeindex: MetricEndpoint<T>, monthindex: MetricEndpoint<T>, quarterindex: MetricEndpoint<T>, semesterindex: MetricEndpoint<T>, weekindex: MetricEndpoint<T>, yearindex: MetricEndpoint<T> }, indexes: () => Index[], get: (index: Index) => MetricEndpoint<T>|undefined }} MetricPattern5
 */

/**
 * Create a MetricPattern5 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} name - The metric vec name
 * @returns {MetricPattern5<T>}
 */
function createMetricPattern5(client, name) {
  return {
    name,
    by: {
      get dateindex() { return _endpoint(client, name, 'dateindex'); },
      get decadeindex() { return _endpoint(client, name, 'decadeindex'); },
      get monthindex() { return _endpoint(client, name, 'monthindex'); },
      get quarterindex() { return _endpoint(client, name, 'quarterindex'); },
      get semesterindex() { return _endpoint(client, name, 'semesterindex'); },
      get weekindex() { return _endpoint(client, name, 'weekindex'); },
      get yearindex() { return _endpoint(client, name, 'yearindex'); }
    },
    indexes() {
      return ['dateindex', 'decadeindex', 'monthindex', 'quarterindex', 'semesterindex', 'weekindex', 'yearindex'];
    },
    get(index) {
      if (this.indexes().includes(index)) {
        return _endpoint(client, name, index);
      }
    }
  };
}

/**
 * @template T
 * @typedef {{ name: string, by: { decadeindex: MetricEndpoint<T>, difficultyepoch: MetricEndpoint<T>, monthindex: MetricEndpoint<T>, quarterindex: MetricEndpoint<T>, semesterindex: MetricEndpoint<T>, weekindex: MetricEndpoint<T>, yearindex: MetricEndpoint<T> }, indexes: () => Index[], get: (index: Index) => MetricEndpoint<T>|undefined }} MetricPattern6
 */

/**
 * Create a MetricPattern6 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} name - The metric vec name
 * @returns {MetricPattern6<T>}
 */
function createMetricPattern6(client, name) {
  return {
    name,
    by: {
      get decadeindex() { return _endpoint(client, name, 'decadeindex'); },
      get difficultyepoch() { return _endpoint(client, name, 'difficultyepoch'); },
      get monthindex() { return _endpoint(client, name, 'monthindex'); },
      get quarterindex() { return _endpoint(client, name, 'quarterindex'); },
      get semesterindex() { return _endpoint(client, name, 'semesterindex'); },
      get weekindex() { return _endpoint(client, name, 'weekindex'); },
      get yearindex() { return _endpoint(client, name, 'yearindex'); }
    },
    indexes() {
      return ['decadeindex', 'difficultyepoch', 'monthindex', 'quarterindex', 'semesterindex', 'weekindex', 'yearindex'];
    },
    get(index) {
      if (this.indexes().includes(index)) {
        return _endpoint(client, name, index);
      }
    }
  };
}

/**
 * @template T
 * @typedef {{ name: string, by: { decadeindex: MetricEndpoint<T>, monthindex: MetricEndpoint<T>, quarterindex: MetricEndpoint<T>, semesterindex: MetricEndpoint<T>, weekindex: MetricEndpoint<T>, yearindex: MetricEndpoint<T> }, indexes: () => Index[], get: (index: Index) => MetricEndpoint<T>|undefined }} MetricPattern7
 */

/**
 * Create a MetricPattern7 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} name - The metric vec name
 * @returns {MetricPattern7<T>}
 */
function createMetricPattern7(client, name) {
  return {
    name,
    by: {
      get decadeindex() { return _endpoint(client, name, 'decadeindex'); },
      get monthindex() { return _endpoint(client, name, 'monthindex'); },
      get quarterindex() { return _endpoint(client, name, 'quarterindex'); },
      get semesterindex() { return _endpoint(client, name, 'semesterindex'); },
      get weekindex() { return _endpoint(client, name, 'weekindex'); },
      get yearindex() { return _endpoint(client, name, 'yearindex'); }
    },
    indexes() {
      return ['decadeindex', 'monthindex', 'quarterindex', 'semesterindex', 'weekindex', 'yearindex'];
    },
    get(index) {
      if (this.indexes().includes(index)) {
        return _endpoint(client, name, index);
      }
    }
  };
}

/**
 * @template T
 * @typedef {{ name: string, by: { emptyoutputindex: MetricEndpoint<T>, opreturnindex: MetricEndpoint<T>, p2msoutputindex: MetricEndpoint<T>, unknownoutputindex: MetricEndpoint<T> }, indexes: () => Index[], get: (index: Index) => MetricEndpoint<T>|undefined }} MetricPattern8
 */

/**
 * Create a MetricPattern8 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} name - The metric vec name
 * @returns {MetricPattern8<T>}
 */
function createMetricPattern8(client, name) {
  return {
    name,
    by: {
      get emptyoutputindex() { return _endpoint(client, name, 'emptyoutputindex'); },
      get opreturnindex() { return _endpoint(client, name, 'opreturnindex'); },
      get p2msoutputindex() { return _endpoint(client, name, 'p2msoutputindex'); },
      get unknownoutputindex() { return _endpoint(client, name, 'unknownoutputindex'); }
    },
    indexes() {
      return ['emptyoutputindex', 'opreturnindex', 'p2msoutputindex', 'unknownoutputindex'];
    },
    get(index) {
      if (this.indexes().includes(index)) {
        return _endpoint(client, name, index);
      }
    }
  };
}

/**
 * @template T
 * @typedef {{ name: string, by: { quarterindex: MetricEndpoint<T>, semesterindex: MetricEndpoint<T>, yearindex: MetricEndpoint<T> }, indexes: () => Index[], get: (index: Index) => MetricEndpoint<T>|undefined }} MetricPattern9
 */

/**
 * Create a MetricPattern9 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} name - The metric vec name
 * @returns {MetricPattern9<T>}
 */
function createMetricPattern9(client, name) {
  return {
    name,
    by: {
      get quarterindex() { return _endpoint(client, name, 'quarterindex'); },
      get semesterindex() { return _endpoint(client, name, 'semesterindex'); },
      get yearindex() { return _endpoint(client, name, 'yearindex'); }
    },
    indexes() {
      return ['quarterindex', 'semesterindex', 'yearindex'];
    },
    get(index) {
      if (this.indexes().includes(index)) {
        return _endpoint(client, name, index);
      }
    }
  };
}

/**
 * @template T
 * @typedef {{ name: string, by: { dateindex: MetricEndpoint<T>, height: MetricEndpoint<T> }, indexes: () => Index[], get: (index: Index) => MetricEndpoint<T>|undefined }} MetricPattern10
 */

/**
 * Create a MetricPattern10 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} name - The metric vec name
 * @returns {MetricPattern10<T>}
 */
function createMetricPattern10(client, name) {
  return {
    name,
    by: {
      get dateindex() { return _endpoint(client, name, 'dateindex'); },
      get height() { return _endpoint(client, name, 'height'); }
    },
    indexes() {
      return ['dateindex', 'height'];
    },
    get(index) {
      if (this.indexes().includes(index)) {
        return _endpoint(client, name, index);
      }
    }
  };
}

/**
 * @template T
 * @typedef {{ name: string, by: { dateindex: MetricEndpoint<T>, monthindex: MetricEndpoint<T> }, indexes: () => Index[], get: (index: Index) => MetricEndpoint<T>|undefined }} MetricPattern11
 */

/**
 * Create a MetricPattern11 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} name - The metric vec name
 * @returns {MetricPattern11<T>}
 */
function createMetricPattern11(client, name) {
  return {
    name,
    by: {
      get dateindex() { return _endpoint(client, name, 'dateindex'); },
      get monthindex() { return _endpoint(client, name, 'monthindex'); }
    },
    indexes() {
      return ['dateindex', 'monthindex'];
    },
    get(index) {
      if (this.indexes().includes(index)) {
        return _endpoint(client, name, index);
      }
    }
  };
}

/**
 * @template T
 * @typedef {{ name: string, by: { dateindex: MetricEndpoint<T>, weekindex: MetricEndpoint<T> }, indexes: () => Index[], get: (index: Index) => MetricEndpoint<T>|undefined }} MetricPattern12
 */

/**
 * Create a MetricPattern12 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} name - The metric vec name
 * @returns {MetricPattern12<T>}
 */
function createMetricPattern12(client, name) {
  return {
    name,
    by: {
      get dateindex() { return _endpoint(client, name, 'dateindex'); },
      get weekindex() { return _endpoint(client, name, 'weekindex'); }
    },
    indexes() {
      return ['dateindex', 'weekindex'];
    },
    get(index) {
      if (this.indexes().includes(index)) {
        return _endpoint(client, name, index);
      }
    }
  };
}

/**
 * @template T
 * @typedef {{ name: string, by: { decadeindex: MetricEndpoint<T>, yearindex: MetricEndpoint<T> }, indexes: () => Index[], get: (index: Index) => MetricEndpoint<T>|undefined }} MetricPattern13
 */

/**
 * Create a MetricPattern13 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} name - The metric vec name
 * @returns {MetricPattern13<T>}
 */
function createMetricPattern13(client, name) {
  return {
    name,
    by: {
      get decadeindex() { return _endpoint(client, name, 'decadeindex'); },
      get yearindex() { return _endpoint(client, name, 'yearindex'); }
    },
    indexes() {
      return ['decadeindex', 'yearindex'];
    },
    get(index) {
      if (this.indexes().includes(index)) {
        return _endpoint(client, name, index);
      }
    }
  };
}

/**
 * @template T
 * @typedef {{ name: string, by: { difficultyepoch: MetricEndpoint<T>, halvingepoch: MetricEndpoint<T> }, indexes: () => Index[], get: (index: Index) => MetricEndpoint<T>|undefined }} MetricPattern14
 */

/**
 * Create a MetricPattern14 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} name - The metric vec name
 * @returns {MetricPattern14<T>}
 */
function createMetricPattern14(client, name) {
  return {
    name,
    by: {
      get difficultyepoch() { return _endpoint(client, name, 'difficultyepoch'); },
      get halvingepoch() { return _endpoint(client, name, 'halvingepoch'); }
    },
    indexes() {
      return ['difficultyepoch', 'halvingepoch'];
    },
    get(index) {
      if (this.indexes().includes(index)) {
        return _endpoint(client, name, index);
      }
    }
  };
}

/**
 * @template T
 * @typedef {{ name: string, by: { difficultyepoch: MetricEndpoint<T>, height: MetricEndpoint<T> }, indexes: () => Index[], get: (index: Index) => MetricEndpoint<T>|undefined }} MetricPattern15
 */

/**
 * Create a MetricPattern15 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} name - The metric vec name
 * @returns {MetricPattern15<T>}
 */
function createMetricPattern15(client, name) {
  return {
    name,
    by: {
      get difficultyepoch() { return _endpoint(client, name, 'difficultyepoch'); },
      get height() { return _endpoint(client, name, 'height'); }
    },
    indexes() {
      return ['difficultyepoch', 'height'];
    },
    get(index) {
      if (this.indexes().includes(index)) {
        return _endpoint(client, name, index);
      }
    }
  };
}

/**
 * @template T
 * @typedef {{ name: string, by: { halvingepoch: MetricEndpoint<T>, height: MetricEndpoint<T> }, indexes: () => Index[], get: (index: Index) => MetricEndpoint<T>|undefined }} MetricPattern16
 */

/**
 * Create a MetricPattern16 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} name - The metric vec name
 * @returns {MetricPattern16<T>}
 */
function createMetricPattern16(client, name) {
  return {
    name,
    by: {
      get halvingepoch() { return _endpoint(client, name, 'halvingepoch'); },
      get height() { return _endpoint(client, name, 'height'); }
    },
    indexes() {
      return ['halvingepoch', 'height'];
    },
    get(index) {
      if (this.indexes().includes(index)) {
        return _endpoint(client, name, index);
      }
    }
  };
}

/**
 * @template T
 * @typedef {{ name: string, by: { height: MetricEndpoint<T>, txindex: MetricEndpoint<T> }, indexes: () => Index[], get: (index: Index) => MetricEndpoint<T>|undefined }} MetricPattern17
 */

/**
 * Create a MetricPattern17 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} name - The metric vec name
 * @returns {MetricPattern17<T>}
 */
function createMetricPattern17(client, name) {
  return {
    name,
    by: {
      get height() { return _endpoint(client, name, 'height'); },
      get txindex() { return _endpoint(client, name, 'txindex'); }
    },
    indexes() {
      return ['height', 'txindex'];
    },
    get(index) {
      if (this.indexes().includes(index)) {
        return _endpoint(client, name, index);
      }
    }
  };
}

/**
 * @template T
 * @typedef {{ name: string, by: { monthindex: MetricEndpoint<T>, quarterindex: MetricEndpoint<T> }, indexes: () => Index[], get: (index: Index) => MetricEndpoint<T>|undefined }} MetricPattern18
 */

/**
 * Create a MetricPattern18 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} name - The metric vec name
 * @returns {MetricPattern18<T>}
 */
function createMetricPattern18(client, name) {
  return {
    name,
    by: {
      get monthindex() { return _endpoint(client, name, 'monthindex'); },
      get quarterindex() { return _endpoint(client, name, 'quarterindex'); }
    },
    indexes() {
      return ['monthindex', 'quarterindex'];
    },
    get(index) {
      if (this.indexes().includes(index)) {
        return _endpoint(client, name, index);
      }
    }
  };
}

/**
 * @template T
 * @typedef {{ name: string, by: { monthindex: MetricEndpoint<T>, semesterindex: MetricEndpoint<T> }, indexes: () => Index[], get: (index: Index) => MetricEndpoint<T>|undefined }} MetricPattern19
 */

/**
 * Create a MetricPattern19 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} name - The metric vec name
 * @returns {MetricPattern19<T>}
 */
function createMetricPattern19(client, name) {
  return {
    name,
    by: {
      get monthindex() { return _endpoint(client, name, 'monthindex'); },
      get semesterindex() { return _endpoint(client, name, 'semesterindex'); }
    },
    indexes() {
      return ['monthindex', 'semesterindex'];
    },
    get(index) {
      if (this.indexes().includes(index)) {
        return _endpoint(client, name, index);
      }
    }
  };
}

/**
 * @template T
 * @typedef {{ name: string, by: { monthindex: MetricEndpoint<T>, weekindex: MetricEndpoint<T> }, indexes: () => Index[], get: (index: Index) => MetricEndpoint<T>|undefined }} MetricPattern20
 */

/**
 * Create a MetricPattern20 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} name - The metric vec name
 * @returns {MetricPattern20<T>}
 */
function createMetricPattern20(client, name) {
  return {
    name,
    by: {
      get monthindex() { return _endpoint(client, name, 'monthindex'); },
      get weekindex() { return _endpoint(client, name, 'weekindex'); }
    },
    indexes() {
      return ['monthindex', 'weekindex'];
    },
    get(index) {
      if (this.indexes().includes(index)) {
        return _endpoint(client, name, index);
      }
    }
  };
}

/**
 * @template T
 * @typedef {{ name: string, by: { monthindex: MetricEndpoint<T>, yearindex: MetricEndpoint<T> }, indexes: () => Index[], get: (index: Index) => MetricEndpoint<T>|undefined }} MetricPattern21
 */

/**
 * Create a MetricPattern21 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} name - The metric vec name
 * @returns {MetricPattern21<T>}
 */
function createMetricPattern21(client, name) {
  return {
    name,
    by: {
      get monthindex() { return _endpoint(client, name, 'monthindex'); },
      get yearindex() { return _endpoint(client, name, 'yearindex'); }
    },
    indexes() {
      return ['monthindex', 'yearindex'];
    },
    get(index) {
      if (this.indexes().includes(index)) {
        return _endpoint(client, name, index);
      }
    }
  };
}

/**
 * @template T
 * @typedef {{ name: string, by: { dateindex: MetricEndpoint<T> }, indexes: () => Index[], get: (index: Index) => MetricEndpoint<T>|undefined }} MetricPattern22
 */

/**
 * Create a MetricPattern22 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} name - The metric vec name
 * @returns {MetricPattern22<T>}
 */
function createMetricPattern22(client, name) {
  return {
    name,
    by: {
      get dateindex() { return _endpoint(client, name, 'dateindex'); }
    },
    indexes() {
      return ['dateindex'];
    },
    get(index) {
      if (this.indexes().includes(index)) {
        return _endpoint(client, name, index);
      }
    }
  };
}

/**
 * @template T
 * @typedef {{ name: string, by: { decadeindex: MetricEndpoint<T> }, indexes: () => Index[], get: (index: Index) => MetricEndpoint<T>|undefined }} MetricPattern23
 */

/**
 * Create a MetricPattern23 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} name - The metric vec name
 * @returns {MetricPattern23<T>}
 */
function createMetricPattern23(client, name) {
  return {
    name,
    by: {
      get decadeindex() { return _endpoint(client, name, 'decadeindex'); }
    },
    indexes() {
      return ['decadeindex'];
    },
    get(index) {
      if (this.indexes().includes(index)) {
        return _endpoint(client, name, index);
      }
    }
  };
}

/**
 * @template T
 * @typedef {{ name: string, by: { difficultyepoch: MetricEndpoint<T> }, indexes: () => Index[], get: (index: Index) => MetricEndpoint<T>|undefined }} MetricPattern24
 */

/**
 * Create a MetricPattern24 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} name - The metric vec name
 * @returns {MetricPattern24<T>}
 */
function createMetricPattern24(client, name) {
  return {
    name,
    by: {
      get difficultyepoch() { return _endpoint(client, name, 'difficultyepoch'); }
    },
    indexes() {
      return ['difficultyepoch'];
    },
    get(index) {
      if (this.indexes().includes(index)) {
        return _endpoint(client, name, index);
      }
    }
  };
}

/**
 * @template T
 * @typedef {{ name: string, by: { emptyoutputindex: MetricEndpoint<T> }, indexes: () => Index[], get: (index: Index) => MetricEndpoint<T>|undefined }} MetricPattern25
 */

/**
 * Create a MetricPattern25 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} name - The metric vec name
 * @returns {MetricPattern25<T>}
 */
function createMetricPattern25(client, name) {
  return {
    name,
    by: {
      get emptyoutputindex() { return _endpoint(client, name, 'emptyoutputindex'); }
    },
    indexes() {
      return ['emptyoutputindex'];
    },
    get(index) {
      if (this.indexes().includes(index)) {
        return _endpoint(client, name, index);
      }
    }
  };
}

/**
 * @template T
 * @typedef {{ name: string, by: { height: MetricEndpoint<T> }, indexes: () => Index[], get: (index: Index) => MetricEndpoint<T>|undefined }} MetricPattern26
 */

/**
 * Create a MetricPattern26 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} name - The metric vec name
 * @returns {MetricPattern26<T>}
 */
function createMetricPattern26(client, name) {
  return {
    name,
    by: {
      get height() { return _endpoint(client, name, 'height'); }
    },
    indexes() {
      return ['height'];
    },
    get(index) {
      if (this.indexes().includes(index)) {
        return _endpoint(client, name, index);
      }
    }
  };
}

/**
 * @template T
 * @typedef {{ name: string, by: { txinindex: MetricEndpoint<T> }, indexes: () => Index[], get: (index: Index) => MetricEndpoint<T>|undefined }} MetricPattern27
 */

/**
 * Create a MetricPattern27 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} name - The metric vec name
 * @returns {MetricPattern27<T>}
 */
function createMetricPattern27(client, name) {
  return {
    name,
    by: {
      get txinindex() { return _endpoint(client, name, 'txinindex'); }
    },
    indexes() {
      return ['txinindex'];
    },
    get(index) {
      if (this.indexes().includes(index)) {
        return _endpoint(client, name, index);
      }
    }
  };
}

/**
 * @template T
 * @typedef {{ name: string, by: { opreturnindex: MetricEndpoint<T> }, indexes: () => Index[], get: (index: Index) => MetricEndpoint<T>|undefined }} MetricPattern28
 */

/**
 * Create a MetricPattern28 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} name - The metric vec name
 * @returns {MetricPattern28<T>}
 */
function createMetricPattern28(client, name) {
  return {
    name,
    by: {
      get opreturnindex() { return _endpoint(client, name, 'opreturnindex'); }
    },
    indexes() {
      return ['opreturnindex'];
    },
    get(index) {
      if (this.indexes().includes(index)) {
        return _endpoint(client, name, index);
      }
    }
  };
}

/**
 * @template T
 * @typedef {{ name: string, by: { txoutindex: MetricEndpoint<T> }, indexes: () => Index[], get: (index: Index) => MetricEndpoint<T>|undefined }} MetricPattern29
 */

/**
 * Create a MetricPattern29 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} name - The metric vec name
 * @returns {MetricPattern29<T>}
 */
function createMetricPattern29(client, name) {
  return {
    name,
    by: {
      get txoutindex() { return _endpoint(client, name, 'txoutindex'); }
    },
    indexes() {
      return ['txoutindex'];
    },
    get(index) {
      if (this.indexes().includes(index)) {
        return _endpoint(client, name, index);
      }
    }
  };
}

/**
 * @template T
 * @typedef {{ name: string, by: { p2aaddressindex: MetricEndpoint<T> }, indexes: () => Index[], get: (index: Index) => MetricEndpoint<T>|undefined }} MetricPattern30
 */

/**
 * Create a MetricPattern30 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} name - The metric vec name
 * @returns {MetricPattern30<T>}
 */
function createMetricPattern30(client, name) {
  return {
    name,
    by: {
      get p2aaddressindex() { return _endpoint(client, name, 'p2aaddressindex'); }
    },
    indexes() {
      return ['p2aaddressindex'];
    },
    get(index) {
      if (this.indexes().includes(index)) {
        return _endpoint(client, name, index);
      }
    }
  };
}

/**
 * @template T
 * @typedef {{ name: string, by: { p2msoutputindex: MetricEndpoint<T> }, indexes: () => Index[], get: (index: Index) => MetricEndpoint<T>|undefined }} MetricPattern31
 */

/**
 * Create a MetricPattern31 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} name - The metric vec name
 * @returns {MetricPattern31<T>}
 */
function createMetricPattern31(client, name) {
  return {
    name,
    by: {
      get p2msoutputindex() { return _endpoint(client, name, 'p2msoutputindex'); }
    },
    indexes() {
      return ['p2msoutputindex'];
    },
    get(index) {
      if (this.indexes().includes(index)) {
        return _endpoint(client, name, index);
      }
    }
  };
}

/**
 * @template T
 * @typedef {{ name: string, by: { p2pk33addressindex: MetricEndpoint<T> }, indexes: () => Index[], get: (index: Index) => MetricEndpoint<T>|undefined }} MetricPattern32
 */

/**
 * Create a MetricPattern32 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} name - The metric vec name
 * @returns {MetricPattern32<T>}
 */
function createMetricPattern32(client, name) {
  return {
    name,
    by: {
      get p2pk33addressindex() { return _endpoint(client, name, 'p2pk33addressindex'); }
    },
    indexes() {
      return ['p2pk33addressindex'];
    },
    get(index) {
      if (this.indexes().includes(index)) {
        return _endpoint(client, name, index);
      }
    }
  };
}

/**
 * @template T
 * @typedef {{ name: string, by: { p2pk65addressindex: MetricEndpoint<T> }, indexes: () => Index[], get: (index: Index) => MetricEndpoint<T>|undefined }} MetricPattern33
 */

/**
 * Create a MetricPattern33 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} name - The metric vec name
 * @returns {MetricPattern33<T>}
 */
function createMetricPattern33(client, name) {
  return {
    name,
    by: {
      get p2pk65addressindex() { return _endpoint(client, name, 'p2pk65addressindex'); }
    },
    indexes() {
      return ['p2pk65addressindex'];
    },
    get(index) {
      if (this.indexes().includes(index)) {
        return _endpoint(client, name, index);
      }
    }
  };
}

/**
 * @template T
 * @typedef {{ name: string, by: { p2pkhaddressindex: MetricEndpoint<T> }, indexes: () => Index[], get: (index: Index) => MetricEndpoint<T>|undefined }} MetricPattern34
 */

/**
 * Create a MetricPattern34 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} name - The metric vec name
 * @returns {MetricPattern34<T>}
 */
function createMetricPattern34(client, name) {
  return {
    name,
    by: {
      get p2pkhaddressindex() { return _endpoint(client, name, 'p2pkhaddressindex'); }
    },
    indexes() {
      return ['p2pkhaddressindex'];
    },
    get(index) {
      if (this.indexes().includes(index)) {
        return _endpoint(client, name, index);
      }
    }
  };
}

/**
 * @template T
 * @typedef {{ name: string, by: { p2shaddressindex: MetricEndpoint<T> }, indexes: () => Index[], get: (index: Index) => MetricEndpoint<T>|undefined }} MetricPattern35
 */

/**
 * Create a MetricPattern35 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} name - The metric vec name
 * @returns {MetricPattern35<T>}
 */
function createMetricPattern35(client, name) {
  return {
    name,
    by: {
      get p2shaddressindex() { return _endpoint(client, name, 'p2shaddressindex'); }
    },
    indexes() {
      return ['p2shaddressindex'];
    },
    get(index) {
      if (this.indexes().includes(index)) {
        return _endpoint(client, name, index);
      }
    }
  };
}

/**
 * @template T
 * @typedef {{ name: string, by: { p2traddressindex: MetricEndpoint<T> }, indexes: () => Index[], get: (index: Index) => MetricEndpoint<T>|undefined }} MetricPattern36
 */

/**
 * Create a MetricPattern36 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} name - The metric vec name
 * @returns {MetricPattern36<T>}
 */
function createMetricPattern36(client, name) {
  return {
    name,
    by: {
      get p2traddressindex() { return _endpoint(client, name, 'p2traddressindex'); }
    },
    indexes() {
      return ['p2traddressindex'];
    },
    get(index) {
      if (this.indexes().includes(index)) {
        return _endpoint(client, name, index);
      }
    }
  };
}

/**
 * @template T
 * @typedef {{ name: string, by: { p2wpkhaddressindex: MetricEndpoint<T> }, indexes: () => Index[], get: (index: Index) => MetricEndpoint<T>|undefined }} MetricPattern37
 */

/**
 * Create a MetricPattern37 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} name - The metric vec name
 * @returns {MetricPattern37<T>}
 */
function createMetricPattern37(client, name) {
  return {
    name,
    by: {
      get p2wpkhaddressindex() { return _endpoint(client, name, 'p2wpkhaddressindex'); }
    },
    indexes() {
      return ['p2wpkhaddressindex'];
    },
    get(index) {
      if (this.indexes().includes(index)) {
        return _endpoint(client, name, index);
      }
    }
  };
}

/**
 * @template T
 * @typedef {{ name: string, by: { p2wshaddressindex: MetricEndpoint<T> }, indexes: () => Index[], get: (index: Index) => MetricEndpoint<T>|undefined }} MetricPattern38
 */

/**
 * Create a MetricPattern38 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} name - The metric vec name
 * @returns {MetricPattern38<T>}
 */
function createMetricPattern38(client, name) {
  return {
    name,
    by: {
      get p2wshaddressindex() { return _endpoint(client, name, 'p2wshaddressindex'); }
    },
    indexes() {
      return ['p2wshaddressindex'];
    },
    get(index) {
      if (this.indexes().includes(index)) {
        return _endpoint(client, name, index);
      }
    }
  };
}

/**
 * @template T
 * @typedef {{ name: string, by: { txindex: MetricEndpoint<T> }, indexes: () => Index[], get: (index: Index) => MetricEndpoint<T>|undefined }} MetricPattern39
 */

/**
 * Create a MetricPattern39 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} name - The metric vec name
 * @returns {MetricPattern39<T>}
 */
function createMetricPattern39(client, name) {
  return {
    name,
    by: {
      get txindex() { return _endpoint(client, name, 'txindex'); }
    },
    indexes() {
      return ['txindex'];
    },
    get(index) {
      if (this.indexes().includes(index)) {
        return _endpoint(client, name, index);
      }
    }
  };
}

/**
 * @template T
 * @typedef {{ name: string, by: { unknownoutputindex: MetricEndpoint<T> }, indexes: () => Index[], get: (index: Index) => MetricEndpoint<T>|undefined }} MetricPattern40
 */

/**
 * Create a MetricPattern40 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} name - The metric vec name
 * @returns {MetricPattern40<T>}
 */
function createMetricPattern40(client, name) {
  return {
    name,
    by: {
      get unknownoutputindex() { return _endpoint(client, name, 'unknownoutputindex'); }
    },
    indexes() {
      return ['unknownoutputindex'];
    },
    get(index) {
      if (this.indexes().includes(index)) {
        return _endpoint(client, name, index);
      }
    }
  };
}

/**
 * @template T
 * @typedef {{ name: string, by: { loadedaddressindex: MetricEndpoint<T> }, indexes: () => Index[], get: (index: Index) => MetricEndpoint<T>|undefined }} MetricPattern41
 */

/**
 * Create a MetricPattern41 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} name - The metric vec name
 * @returns {MetricPattern41<T>}
 */
function createMetricPattern41(client, name) {
  return {
    name,
    by: {
      get loadedaddressindex() { return _endpoint(client, name, 'loadedaddressindex'); }
    },
    indexes() {
      return ['loadedaddressindex'];
    },
    get(index) {
      if (this.indexes().includes(index)) {
        return _endpoint(client, name, index);
      }
    }
  };
}

/**
 * @template T
 * @typedef {{ name: string, by: { emptyaddressindex: MetricEndpoint<T> }, indexes: () => Index[], get: (index: Index) => MetricEndpoint<T>|undefined }} MetricPattern42
 */

/**
 * Create a MetricPattern42 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} name - The metric vec name
 * @returns {MetricPattern42<T>}
 */
function createMetricPattern42(client, name) {
  return {
    name,
    by: {
      get emptyaddressindex() { return _endpoint(client, name, 'emptyaddressindex'); }
    },
    indexes() {
      return ['emptyaddressindex'];
    },
    get(index) {
      if (this.indexes().includes(index)) {
        return _endpoint(client, name, index);
      }
    }
  };
}

// Reusable structural pattern factories

/**
 * @typedef {Object} RealizedPattern3
 * @property {MetricPattern22<StoredF64>} adjustedSopr
 * @property {MetricPattern22<StoredF64>} adjustedSopr30dEma
 * @property {MetricPattern22<StoredF64>} adjustedSopr7dEma
 * @property {MetricPattern1<Dollars>} adjustedValueCreated
 * @property {MetricPattern1<Dollars>} adjustedValueDestroyed
 * @property {MetricPattern5<StoredF32>} mvrv
 * @property {BlockCountPattern<Dollars>} negRealizedLoss
 * @property {BlockCountPattern<Dollars>} netRealizedPnl
 * @property {MetricPattern5<Dollars>} netRealizedPnlCumulative30dDelta
 * @property {MetricPattern5<StoredF32>} netRealizedPnlCumulative30dDeltaRelToMarketCap
 * @property {MetricPattern5<StoredF32>} netRealizedPnlCumulative30dDeltaRelToRealizedCap
 * @property {BlockCountPattern<StoredF32>} netRealizedPnlRelToRealizedCap
 * @property {MetricPattern1<Dollars>} realizedCap
 * @property {MetricPattern5<Dollars>} realizedCap30dDelta
 * @property {MetricPattern1<StoredF32>} realizedCapRelToOwnMarketCap
 * @property {BlockCountPattern<Dollars>} realizedLoss
 * @property {BlockCountPattern<StoredF32>} realizedLossRelToRealizedCap
 * @property {MetricPattern1<Dollars>} realizedPrice
 * @property {ActivePriceRatioPattern} realizedPriceExtra
 * @property {BlockCountPattern<Dollars>} realizedProfit
 * @property {BlockCountPattern<StoredF32>} realizedProfitRelToRealizedCap
 * @property {MetricPattern22<StoredF64>} realizedProfitToLossRatio
 * @property {DifficultyAdjustmentPattern<Dollars>} realizedValue
 * @property {MetricPattern22<StoredF32>} sellSideRiskRatio
 * @property {MetricPattern22<StoredF32>} sellSideRiskRatio30dEma
 * @property {MetricPattern22<StoredF32>} sellSideRiskRatio7dEma
 * @property {MetricPattern22<StoredF64>} sopr
 * @property {MetricPattern22<StoredF64>} sopr30dEma
 * @property {MetricPattern22<StoredF64>} sopr7dEma
 * @property {MetricPattern1<Dollars>} totalRealizedPnl
 * @property {MetricPattern26<Dollars>} valueCreated
 * @property {MetricPattern2<Dollars>} valueCreatedSum
 * @property {MetricPattern26<Dollars>} valueDestroyed
 * @property {MetricPattern2<Dollars>} valueDestroyedSum
 */

/**
 * Create a RealizedPattern3 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {RealizedPattern3}
 */
function createRealizedPattern3(client, acc) {
  return {
    adjustedSopr: createMetricPattern22(client, _m(acc, 'adjusted_sopr')),
    adjustedSopr30dEma: createMetricPattern22(client, _m(acc, 'adjusted_sopr_30d_ema')),
    adjustedSopr7dEma: createMetricPattern22(client, _m(acc, 'adjusted_sopr_7d_ema')),
    adjustedValueCreated: createMetricPattern1(client, _m(acc, 'adjusted_value_created')),
    adjustedValueDestroyed: createMetricPattern1(client, _m(acc, 'adjusted_value_destroyed')),
    mvrv: createMetricPattern5(client, _m(acc, 'mvrv')),
    negRealizedLoss: createBlockCountPattern(client, _m(acc, 'neg_realized_loss')),
    netRealizedPnl: createBlockCountPattern(client, _m(acc, 'net_realized_pnl')),
    netRealizedPnlCumulative30dDelta: createMetricPattern5(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta')),
    netRealizedPnlCumulative30dDeltaRelToMarketCap: createMetricPattern5(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta_rel_to_market_cap')),
    netRealizedPnlCumulative30dDeltaRelToRealizedCap: createMetricPattern5(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap')),
    netRealizedPnlRelToRealizedCap: createBlockCountPattern(client, _m(acc, 'net_realized_pnl_rel_to_realized_cap')),
    realizedCap: createMetricPattern1(client, _m(acc, 'realized_cap')),
    realizedCap30dDelta: createMetricPattern5(client, _m(acc, 'realized_cap_30d_delta')),
    realizedCapRelToOwnMarketCap: createMetricPattern1(client, _m(acc, 'realized_cap_rel_to_own_market_cap')),
    realizedLoss: createBlockCountPattern(client, _m(acc, 'realized_loss')),
    realizedLossRelToRealizedCap: createBlockCountPattern(client, _m(acc, 'realized_loss_rel_to_realized_cap')),
    realizedPrice: createMetricPattern1(client, _m(acc, 'realized_price')),
    realizedPriceExtra: createActivePriceRatioPattern(client, _m(acc, 'realized_price_ratio')),
    realizedProfit: createBlockCountPattern(client, _m(acc, 'realized_profit')),
    realizedProfitRelToRealizedCap: createBlockCountPattern(client, _m(acc, 'realized_profit_rel_to_realized_cap')),
    realizedProfitToLossRatio: createMetricPattern22(client, _m(acc, 'realized_profit_to_loss_ratio')),
    realizedValue: createDifficultyAdjustmentPattern(client, _m(acc, 'realized_value')),
    sellSideRiskRatio: createMetricPattern22(client, _m(acc, 'sell_side_risk_ratio')),
    sellSideRiskRatio30dEma: createMetricPattern22(client, _m(acc, 'sell_side_risk_ratio_30d_ema')),
    sellSideRiskRatio7dEma: createMetricPattern22(client, _m(acc, 'sell_side_risk_ratio_7d_ema')),
    sopr: createMetricPattern22(client, _m(acc, 'sopr')),
    sopr30dEma: createMetricPattern22(client, _m(acc, 'sopr_30d_ema')),
    sopr7dEma: createMetricPattern22(client, _m(acc, 'sopr_7d_ema')),
    totalRealizedPnl: createMetricPattern1(client, _m(acc, 'total_realized_pnl')),
    valueCreated: createMetricPattern26(client, _m(acc, 'value_created')),
    valueCreatedSum: createMetricPattern2(client, _m(acc, 'value_created_sum')),
    valueDestroyed: createMetricPattern26(client, _m(acc, 'value_destroyed')),
    valueDestroyedSum: createMetricPattern2(client, _m(acc, 'value_destroyed_sum')),
  };
}

/**
 * @typedef {Object} RealizedPattern4
 * @property {MetricPattern22<StoredF64>} adjustedSopr
 * @property {MetricPattern22<StoredF64>} adjustedSopr30dEma
 * @property {MetricPattern22<StoredF64>} adjustedSopr7dEma
 * @property {MetricPattern1<Dollars>} adjustedValueCreated
 * @property {MetricPattern1<Dollars>} adjustedValueDestroyed
 * @property {MetricPattern5<StoredF32>} mvrv
 * @property {BlockCountPattern<Dollars>} negRealizedLoss
 * @property {BlockCountPattern<Dollars>} netRealizedPnl
 * @property {MetricPattern5<Dollars>} netRealizedPnlCumulative30dDelta
 * @property {MetricPattern5<StoredF32>} netRealizedPnlCumulative30dDeltaRelToMarketCap
 * @property {MetricPattern5<StoredF32>} netRealizedPnlCumulative30dDeltaRelToRealizedCap
 * @property {BlockCountPattern<StoredF32>} netRealizedPnlRelToRealizedCap
 * @property {MetricPattern1<Dollars>} realizedCap
 * @property {MetricPattern5<Dollars>} realizedCap30dDelta
 * @property {BlockCountPattern<Dollars>} realizedLoss
 * @property {BlockCountPattern<StoredF32>} realizedLossRelToRealizedCap
 * @property {MetricPattern1<Dollars>} realizedPrice
 * @property {RealizedPriceExtraPattern} realizedPriceExtra
 * @property {BlockCountPattern<Dollars>} realizedProfit
 * @property {BlockCountPattern<StoredF32>} realizedProfitRelToRealizedCap
 * @property {DifficultyAdjustmentPattern<Dollars>} realizedValue
 * @property {MetricPattern22<StoredF32>} sellSideRiskRatio
 * @property {MetricPattern22<StoredF32>} sellSideRiskRatio30dEma
 * @property {MetricPattern22<StoredF32>} sellSideRiskRatio7dEma
 * @property {MetricPattern22<StoredF64>} sopr
 * @property {MetricPattern22<StoredF64>} sopr30dEma
 * @property {MetricPattern22<StoredF64>} sopr7dEma
 * @property {MetricPattern1<Dollars>} totalRealizedPnl
 * @property {MetricPattern26<Dollars>} valueCreated
 * @property {MetricPattern2<Dollars>} valueCreatedSum
 * @property {MetricPattern26<Dollars>} valueDestroyed
 * @property {MetricPattern2<Dollars>} valueDestroyedSum
 */

/**
 * Create a RealizedPattern4 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {RealizedPattern4}
 */
function createRealizedPattern4(client, acc) {
  return {
    adjustedSopr: createMetricPattern22(client, _m(acc, 'adjusted_sopr')),
    adjustedSopr30dEma: createMetricPattern22(client, _m(acc, 'adjusted_sopr_30d_ema')),
    adjustedSopr7dEma: createMetricPattern22(client, _m(acc, 'adjusted_sopr_7d_ema')),
    adjustedValueCreated: createMetricPattern1(client, _m(acc, 'adjusted_value_created')),
    adjustedValueDestroyed: createMetricPattern1(client, _m(acc, 'adjusted_value_destroyed')),
    mvrv: createMetricPattern5(client, _m(acc, 'mvrv')),
    negRealizedLoss: createBlockCountPattern(client, _m(acc, 'neg_realized_loss')),
    netRealizedPnl: createBlockCountPattern(client, _m(acc, 'net_realized_pnl')),
    netRealizedPnlCumulative30dDelta: createMetricPattern5(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta')),
    netRealizedPnlCumulative30dDeltaRelToMarketCap: createMetricPattern5(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta_rel_to_market_cap')),
    netRealizedPnlCumulative30dDeltaRelToRealizedCap: createMetricPattern5(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap')),
    netRealizedPnlRelToRealizedCap: createBlockCountPattern(client, _m(acc, 'net_realized_pnl_rel_to_realized_cap')),
    realizedCap: createMetricPattern1(client, _m(acc, 'realized_cap')),
    realizedCap30dDelta: createMetricPattern5(client, _m(acc, 'realized_cap_30d_delta')),
    realizedLoss: createBlockCountPattern(client, _m(acc, 'realized_loss')),
    realizedLossRelToRealizedCap: createBlockCountPattern(client, _m(acc, 'realized_loss_rel_to_realized_cap')),
    realizedPrice: createMetricPattern1(client, _m(acc, 'realized_price')),
    realizedPriceExtra: createRealizedPriceExtraPattern(client, _m(acc, 'realized_price')),
    realizedProfit: createBlockCountPattern(client, _m(acc, 'realized_profit')),
    realizedProfitRelToRealizedCap: createBlockCountPattern(client, _m(acc, 'realized_profit_rel_to_realized_cap')),
    realizedValue: createDifficultyAdjustmentPattern(client, _m(acc, 'realized_value')),
    sellSideRiskRatio: createMetricPattern22(client, _m(acc, 'sell_side_risk_ratio')),
    sellSideRiskRatio30dEma: createMetricPattern22(client, _m(acc, 'sell_side_risk_ratio_30d_ema')),
    sellSideRiskRatio7dEma: createMetricPattern22(client, _m(acc, 'sell_side_risk_ratio_7d_ema')),
    sopr: createMetricPattern22(client, _m(acc, 'sopr')),
    sopr30dEma: createMetricPattern22(client, _m(acc, 'sopr_30d_ema')),
    sopr7dEma: createMetricPattern22(client, _m(acc, 'sopr_7d_ema')),
    totalRealizedPnl: createMetricPattern1(client, _m(acc, 'total_realized_pnl')),
    valueCreated: createMetricPattern26(client, _m(acc, 'value_created')),
    valueCreatedSum: createMetricPattern2(client, _m(acc, 'value_created_sum')),
    valueDestroyed: createMetricPattern26(client, _m(acc, 'value_destroyed')),
    valueDestroyedSum: createMetricPattern2(client, _m(acc, 'value_destroyed_sum')),
  };
}

/**
 * @typedef {Object} RealizedPattern2
 * @property {MetricPattern5<StoredF32>} mvrv
 * @property {BlockCountPattern<Dollars>} negRealizedLoss
 * @property {BlockCountPattern<Dollars>} netRealizedPnl
 * @property {MetricPattern5<Dollars>} netRealizedPnlCumulative30dDelta
 * @property {MetricPattern5<StoredF32>} netRealizedPnlCumulative30dDeltaRelToMarketCap
 * @property {MetricPattern5<StoredF32>} netRealizedPnlCumulative30dDeltaRelToRealizedCap
 * @property {BlockCountPattern<StoredF32>} netRealizedPnlRelToRealizedCap
 * @property {MetricPattern1<Dollars>} realizedCap
 * @property {MetricPattern5<Dollars>} realizedCap30dDelta
 * @property {MetricPattern1<StoredF32>} realizedCapRelToOwnMarketCap
 * @property {BlockCountPattern<Dollars>} realizedLoss
 * @property {BlockCountPattern<StoredF32>} realizedLossRelToRealizedCap
 * @property {MetricPattern1<Dollars>} realizedPrice
 * @property {ActivePriceRatioPattern} realizedPriceExtra
 * @property {BlockCountPattern<Dollars>} realizedProfit
 * @property {BlockCountPattern<StoredF32>} realizedProfitRelToRealizedCap
 * @property {MetricPattern22<StoredF64>} realizedProfitToLossRatio
 * @property {DifficultyAdjustmentPattern<Dollars>} realizedValue
 * @property {MetricPattern22<StoredF32>} sellSideRiskRatio
 * @property {MetricPattern22<StoredF32>} sellSideRiskRatio30dEma
 * @property {MetricPattern22<StoredF32>} sellSideRiskRatio7dEma
 * @property {MetricPattern22<StoredF64>} sopr
 * @property {MetricPattern22<StoredF64>} sopr30dEma
 * @property {MetricPattern22<StoredF64>} sopr7dEma
 * @property {MetricPattern1<Dollars>} totalRealizedPnl
 * @property {MetricPattern26<Dollars>} valueCreated
 * @property {MetricPattern2<Dollars>} valueCreatedSum
 * @property {MetricPattern26<Dollars>} valueDestroyed
 * @property {MetricPattern2<Dollars>} valueDestroyedSum
 */

/**
 * Create a RealizedPattern2 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {RealizedPattern2}
 */
function createRealizedPattern2(client, acc) {
  return {
    mvrv: createMetricPattern5(client, _m(acc, 'mvrv')),
    negRealizedLoss: createBlockCountPattern(client, _m(acc, 'neg_realized_loss')),
    netRealizedPnl: createBlockCountPattern(client, _m(acc, 'net_realized_pnl')),
    netRealizedPnlCumulative30dDelta: createMetricPattern5(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta')),
    netRealizedPnlCumulative30dDeltaRelToMarketCap: createMetricPattern5(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta_rel_to_market_cap')),
    netRealizedPnlCumulative30dDeltaRelToRealizedCap: createMetricPattern5(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap')),
    netRealizedPnlRelToRealizedCap: createBlockCountPattern(client, _m(acc, 'net_realized_pnl_rel_to_realized_cap')),
    realizedCap: createMetricPattern1(client, _m(acc, 'realized_cap')),
    realizedCap30dDelta: createMetricPattern5(client, _m(acc, 'realized_cap_30d_delta')),
    realizedCapRelToOwnMarketCap: createMetricPattern1(client, _m(acc, 'realized_cap_rel_to_own_market_cap')),
    realizedLoss: createBlockCountPattern(client, _m(acc, 'realized_loss')),
    realizedLossRelToRealizedCap: createBlockCountPattern(client, _m(acc, 'realized_loss_rel_to_realized_cap')),
    realizedPrice: createMetricPattern1(client, _m(acc, 'realized_price')),
    realizedPriceExtra: createActivePriceRatioPattern(client, _m(acc, 'realized_price_ratio')),
    realizedProfit: createBlockCountPattern(client, _m(acc, 'realized_profit')),
    realizedProfitRelToRealizedCap: createBlockCountPattern(client, _m(acc, 'realized_profit_rel_to_realized_cap')),
    realizedProfitToLossRatio: createMetricPattern22(client, _m(acc, 'realized_profit_to_loss_ratio')),
    realizedValue: createDifficultyAdjustmentPattern(client, _m(acc, 'realized_value')),
    sellSideRiskRatio: createMetricPattern22(client, _m(acc, 'sell_side_risk_ratio')),
    sellSideRiskRatio30dEma: createMetricPattern22(client, _m(acc, 'sell_side_risk_ratio_30d_ema')),
    sellSideRiskRatio7dEma: createMetricPattern22(client, _m(acc, 'sell_side_risk_ratio_7d_ema')),
    sopr: createMetricPattern22(client, _m(acc, 'sopr')),
    sopr30dEma: createMetricPattern22(client, _m(acc, 'sopr_30d_ema')),
    sopr7dEma: createMetricPattern22(client, _m(acc, 'sopr_7d_ema')),
    totalRealizedPnl: createMetricPattern1(client, _m(acc, 'total_realized_pnl')),
    valueCreated: createMetricPattern26(client, _m(acc, 'value_created')),
    valueCreatedSum: createMetricPattern2(client, _m(acc, 'value_created_sum')),
    valueDestroyed: createMetricPattern26(client, _m(acc, 'value_destroyed')),
    valueDestroyedSum: createMetricPattern2(client, _m(acc, 'value_destroyed_sum')),
  };
}

/**
 * @typedef {Object} Ratio1ySdPattern
 * @property {MetricPattern5<Dollars>} _0sdUsd
 * @property {MetricPattern5<StoredF32>} m05sd
 * @property {MetricPattern5<Dollars>} m05sdUsd
 * @property {MetricPattern5<StoredF32>} m15sd
 * @property {MetricPattern5<Dollars>} m15sdUsd
 * @property {MetricPattern5<StoredF32>} m1sd
 * @property {MetricPattern5<Dollars>} m1sdUsd
 * @property {MetricPattern5<StoredF32>} m25sd
 * @property {MetricPattern5<Dollars>} m25sdUsd
 * @property {MetricPattern5<StoredF32>} m2sd
 * @property {MetricPattern5<Dollars>} m2sdUsd
 * @property {MetricPattern5<StoredF32>} m3sd
 * @property {MetricPattern5<Dollars>} m3sdUsd
 * @property {MetricPattern5<StoredF32>} p05sd
 * @property {MetricPattern5<Dollars>} p05sdUsd
 * @property {MetricPattern5<StoredF32>} p15sd
 * @property {MetricPattern5<Dollars>} p15sdUsd
 * @property {MetricPattern5<StoredF32>} p1sd
 * @property {MetricPattern5<Dollars>} p1sdUsd
 * @property {MetricPattern5<StoredF32>} p25sd
 * @property {MetricPattern5<Dollars>} p25sdUsd
 * @property {MetricPattern5<StoredF32>} p2sd
 * @property {MetricPattern5<Dollars>} p2sdUsd
 * @property {MetricPattern5<StoredF32>} p3sd
 * @property {MetricPattern5<Dollars>} p3sdUsd
 * @property {MetricPattern5<StoredF32>} sd
 * @property {MetricPattern5<StoredF32>} sma
 * @property {MetricPattern5<StoredF32>} zscore
 */

/**
 * Create a Ratio1ySdPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {Ratio1ySdPattern}
 */
function createRatio1ySdPattern(client, acc) {
  return {
    _0sdUsd: createMetricPattern5(client, _m(acc, '0sd_usd')),
    m05sd: createMetricPattern5(client, _m(acc, 'm0_5sd')),
    m05sdUsd: createMetricPattern5(client, _m(acc, 'm0_5sd_usd')),
    m15sd: createMetricPattern5(client, _m(acc, 'm1_5sd')),
    m15sdUsd: createMetricPattern5(client, _m(acc, 'm1_5sd_usd')),
    m1sd: createMetricPattern5(client, _m(acc, 'm1sd')),
    m1sdUsd: createMetricPattern5(client, _m(acc, 'm1sd_usd')),
    m25sd: createMetricPattern5(client, _m(acc, 'm2_5sd')),
    m25sdUsd: createMetricPattern5(client, _m(acc, 'm2_5sd_usd')),
    m2sd: createMetricPattern5(client, _m(acc, 'm2sd')),
    m2sdUsd: createMetricPattern5(client, _m(acc, 'm2sd_usd')),
    m3sd: createMetricPattern5(client, _m(acc, 'm3sd')),
    m3sdUsd: createMetricPattern5(client, _m(acc, 'm3sd_usd')),
    p05sd: createMetricPattern5(client, _m(acc, 'p0_5sd')),
    p05sdUsd: createMetricPattern5(client, _m(acc, 'p0_5sd_usd')),
    p15sd: createMetricPattern5(client, _m(acc, 'p1_5sd')),
    p15sdUsd: createMetricPattern5(client, _m(acc, 'p1_5sd_usd')),
    p1sd: createMetricPattern5(client, _m(acc, 'p1sd')),
    p1sdUsd: createMetricPattern5(client, _m(acc, 'p1sd_usd')),
    p25sd: createMetricPattern5(client, _m(acc, 'p2_5sd')),
    p25sdUsd: createMetricPattern5(client, _m(acc, 'p2_5sd_usd')),
    p2sd: createMetricPattern5(client, _m(acc, 'p2sd')),
    p2sdUsd: createMetricPattern5(client, _m(acc, 'p2sd_usd')),
    p3sd: createMetricPattern5(client, _m(acc, 'p3sd')),
    p3sdUsd: createMetricPattern5(client, _m(acc, 'p3sd_usd')),
    sd: createMetricPattern5(client, _m(acc, 'sd')),
    sma: createMetricPattern5(client, _m(acc, 'sma')),
    zscore: createMetricPattern5(client, _m(acc, 'zscore')),
  };
}

/**
 * @typedef {Object} RealizedPattern
 * @property {MetricPattern5<StoredF32>} mvrv
 * @property {BlockCountPattern<Dollars>} negRealizedLoss
 * @property {BlockCountPattern<Dollars>} netRealizedPnl
 * @property {MetricPattern5<Dollars>} netRealizedPnlCumulative30dDelta
 * @property {MetricPattern5<StoredF32>} netRealizedPnlCumulative30dDeltaRelToMarketCap
 * @property {MetricPattern5<StoredF32>} netRealizedPnlCumulative30dDeltaRelToRealizedCap
 * @property {BlockCountPattern<StoredF32>} netRealizedPnlRelToRealizedCap
 * @property {MetricPattern1<Dollars>} realizedCap
 * @property {MetricPattern5<Dollars>} realizedCap30dDelta
 * @property {BlockCountPattern<Dollars>} realizedLoss
 * @property {BlockCountPattern<StoredF32>} realizedLossRelToRealizedCap
 * @property {MetricPattern1<Dollars>} realizedPrice
 * @property {RealizedPriceExtraPattern} realizedPriceExtra
 * @property {BlockCountPattern<Dollars>} realizedProfit
 * @property {BlockCountPattern<StoredF32>} realizedProfitRelToRealizedCap
 * @property {DifficultyAdjustmentPattern<Dollars>} realizedValue
 * @property {MetricPattern22<StoredF32>} sellSideRiskRatio
 * @property {MetricPattern22<StoredF32>} sellSideRiskRatio30dEma
 * @property {MetricPattern22<StoredF32>} sellSideRiskRatio7dEma
 * @property {MetricPattern22<StoredF64>} sopr
 * @property {MetricPattern22<StoredF64>} sopr30dEma
 * @property {MetricPattern22<StoredF64>} sopr7dEma
 * @property {MetricPattern1<Dollars>} totalRealizedPnl
 * @property {MetricPattern26<Dollars>} valueCreated
 * @property {MetricPattern2<Dollars>} valueCreatedSum
 * @property {MetricPattern26<Dollars>} valueDestroyed
 * @property {MetricPattern2<Dollars>} valueDestroyedSum
 */

/**
 * Create a RealizedPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {RealizedPattern}
 */
function createRealizedPattern(client, acc) {
  return {
    mvrv: createMetricPattern5(client, _m(acc, 'mvrv')),
    negRealizedLoss: createBlockCountPattern(client, _m(acc, 'neg_realized_loss')),
    netRealizedPnl: createBlockCountPattern(client, _m(acc, 'net_realized_pnl')),
    netRealizedPnlCumulative30dDelta: createMetricPattern5(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta')),
    netRealizedPnlCumulative30dDeltaRelToMarketCap: createMetricPattern5(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta_rel_to_market_cap')),
    netRealizedPnlCumulative30dDeltaRelToRealizedCap: createMetricPattern5(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap')),
    netRealizedPnlRelToRealizedCap: createBlockCountPattern(client, _m(acc, 'net_realized_pnl_rel_to_realized_cap')),
    realizedCap: createMetricPattern1(client, _m(acc, 'realized_cap')),
    realizedCap30dDelta: createMetricPattern5(client, _m(acc, 'realized_cap_30d_delta')),
    realizedLoss: createBlockCountPattern(client, _m(acc, 'realized_loss')),
    realizedLossRelToRealizedCap: createBlockCountPattern(client, _m(acc, 'realized_loss_rel_to_realized_cap')),
    realizedPrice: createMetricPattern1(client, _m(acc, 'realized_price')),
    realizedPriceExtra: createRealizedPriceExtraPattern(client, _m(acc, 'realized_price')),
    realizedProfit: createBlockCountPattern(client, _m(acc, 'realized_profit')),
    realizedProfitRelToRealizedCap: createBlockCountPattern(client, _m(acc, 'realized_profit_rel_to_realized_cap')),
    realizedValue: createDifficultyAdjustmentPattern(client, _m(acc, 'realized_value')),
    sellSideRiskRatio: createMetricPattern22(client, _m(acc, 'sell_side_risk_ratio')),
    sellSideRiskRatio30dEma: createMetricPattern22(client, _m(acc, 'sell_side_risk_ratio_30d_ema')),
    sellSideRiskRatio7dEma: createMetricPattern22(client, _m(acc, 'sell_side_risk_ratio_7d_ema')),
    sopr: createMetricPattern22(client, _m(acc, 'sopr')),
    sopr30dEma: createMetricPattern22(client, _m(acc, 'sopr_30d_ema')),
    sopr7dEma: createMetricPattern22(client, _m(acc, 'sopr_7d_ema')),
    totalRealizedPnl: createMetricPattern1(client, _m(acc, 'total_realized_pnl')),
    valueCreated: createMetricPattern26(client, _m(acc, 'value_created')),
    valueCreatedSum: createMetricPattern2(client, _m(acc, 'value_created_sum')),
    valueDestroyed: createMetricPattern26(client, _m(acc, 'value_destroyed')),
    valueDestroyedSum: createMetricPattern2(client, _m(acc, 'value_destroyed_sum')),
  };
}

/**
 * @typedef {Object} Price111dSmaPattern
 * @property {MetricPattern5<Dollars>} price
 * @property {MetricPattern5<StoredF32>} ratio
 * @property {MetricPattern5<StoredF32>} ratio1mSma
 * @property {MetricPattern5<StoredF32>} ratio1wSma
 * @property {Ratio1ySdPattern} ratio1ySd
 * @property {Ratio1ySdPattern} ratio2ySd
 * @property {Ratio1ySdPattern} ratio4ySd
 * @property {MetricPattern5<StoredF32>} ratioPct1
 * @property {MetricPattern5<Dollars>} ratioPct1Usd
 * @property {MetricPattern5<StoredF32>} ratioPct2
 * @property {MetricPattern5<Dollars>} ratioPct2Usd
 * @property {MetricPattern5<StoredF32>} ratioPct5
 * @property {MetricPattern5<Dollars>} ratioPct5Usd
 * @property {MetricPattern5<StoredF32>} ratioPct95
 * @property {MetricPattern5<Dollars>} ratioPct95Usd
 * @property {MetricPattern5<StoredF32>} ratioPct98
 * @property {MetricPattern5<Dollars>} ratioPct98Usd
 * @property {MetricPattern5<StoredF32>} ratioPct99
 * @property {MetricPattern5<Dollars>} ratioPct99Usd
 * @property {Ratio1ySdPattern} ratioSd
 */

/**
 * Create a Price111dSmaPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {Price111dSmaPattern}
 */
function createPrice111dSmaPattern(client, acc) {
  return {
    price: createMetricPattern5(client, acc),
    ratio: createMetricPattern5(client, _m(acc, 'ratio')),
    ratio1mSma: createMetricPattern5(client, _m(acc, 'ratio_1m_sma')),
    ratio1wSma: createMetricPattern5(client, _m(acc, 'ratio_1w_sma')),
    ratio1ySd: createRatio1ySdPattern(client, _m(acc, 'ratio_1y')),
    ratio2ySd: createRatio1ySdPattern(client, _m(acc, 'ratio_2y')),
    ratio4ySd: createRatio1ySdPattern(client, _m(acc, 'ratio_4y')),
    ratioPct1: createMetricPattern5(client, _m(acc, 'ratio_pct1')),
    ratioPct1Usd: createMetricPattern5(client, _m(acc, 'ratio_pct1_usd')),
    ratioPct2: createMetricPattern5(client, _m(acc, 'ratio_pct2')),
    ratioPct2Usd: createMetricPattern5(client, _m(acc, 'ratio_pct2_usd')),
    ratioPct5: createMetricPattern5(client, _m(acc, 'ratio_pct5')),
    ratioPct5Usd: createMetricPattern5(client, _m(acc, 'ratio_pct5_usd')),
    ratioPct95: createMetricPattern5(client, _m(acc, 'ratio_pct95')),
    ratioPct95Usd: createMetricPattern5(client, _m(acc, 'ratio_pct95_usd')),
    ratioPct98: createMetricPattern5(client, _m(acc, 'ratio_pct98')),
    ratioPct98Usd: createMetricPattern5(client, _m(acc, 'ratio_pct98_usd')),
    ratioPct99: createMetricPattern5(client, _m(acc, 'ratio_pct99')),
    ratioPct99Usd: createMetricPattern5(client, _m(acc, 'ratio_pct99_usd')),
    ratioSd: createRatio1ySdPattern(client, _m(acc, 'ratio')),
  };
}

/**
 * @typedef {Object} PercentilesPattern2
 * @property {MetricPattern5<Dollars>} costBasisPct05
 * @property {MetricPattern5<Dollars>} costBasisPct10
 * @property {MetricPattern5<Dollars>} costBasisPct15
 * @property {MetricPattern5<Dollars>} costBasisPct20
 * @property {MetricPattern5<Dollars>} costBasisPct25
 * @property {MetricPattern5<Dollars>} costBasisPct30
 * @property {MetricPattern5<Dollars>} costBasisPct35
 * @property {MetricPattern5<Dollars>} costBasisPct40
 * @property {MetricPattern5<Dollars>} costBasisPct45
 * @property {MetricPattern5<Dollars>} costBasisPct50
 * @property {MetricPattern5<Dollars>} costBasisPct55
 * @property {MetricPattern5<Dollars>} costBasisPct60
 * @property {MetricPattern5<Dollars>} costBasisPct65
 * @property {MetricPattern5<Dollars>} costBasisPct70
 * @property {MetricPattern5<Dollars>} costBasisPct75
 * @property {MetricPattern5<Dollars>} costBasisPct80
 * @property {MetricPattern5<Dollars>} costBasisPct85
 * @property {MetricPattern5<Dollars>} costBasisPct90
 * @property {MetricPattern5<Dollars>} costBasisPct95
 */

/**
 * Create a PercentilesPattern2 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {PercentilesPattern2}
 */
function createPercentilesPattern2(client, acc) {
  return {
    costBasisPct05: createMetricPattern5(client, _m(acc, 'pct05')),
    costBasisPct10: createMetricPattern5(client, _m(acc, 'pct10')),
    costBasisPct15: createMetricPattern5(client, _m(acc, 'pct15')),
    costBasisPct20: createMetricPattern5(client, _m(acc, 'pct20')),
    costBasisPct25: createMetricPattern5(client, _m(acc, 'pct25')),
    costBasisPct30: createMetricPattern5(client, _m(acc, 'pct30')),
    costBasisPct35: createMetricPattern5(client, _m(acc, 'pct35')),
    costBasisPct40: createMetricPattern5(client, _m(acc, 'pct40')),
    costBasisPct45: createMetricPattern5(client, _m(acc, 'pct45')),
    costBasisPct50: createMetricPattern5(client, _m(acc, 'pct50')),
    costBasisPct55: createMetricPattern5(client, _m(acc, 'pct55')),
    costBasisPct60: createMetricPattern5(client, _m(acc, 'pct60')),
    costBasisPct65: createMetricPattern5(client, _m(acc, 'pct65')),
    costBasisPct70: createMetricPattern5(client, _m(acc, 'pct70')),
    costBasisPct75: createMetricPattern5(client, _m(acc, 'pct75')),
    costBasisPct80: createMetricPattern5(client, _m(acc, 'pct80')),
    costBasisPct85: createMetricPattern5(client, _m(acc, 'pct85')),
    costBasisPct90: createMetricPattern5(client, _m(acc, 'pct90')),
    costBasisPct95: createMetricPattern5(client, _m(acc, 'pct95')),
  };
}

/**
 * @typedef {Object} ActivePriceRatioPattern
 * @property {MetricPattern5<StoredF32>} ratio
 * @property {MetricPattern5<StoredF32>} ratio1mSma
 * @property {MetricPattern5<StoredF32>} ratio1wSma
 * @property {Ratio1ySdPattern} ratio1ySd
 * @property {Ratio1ySdPattern} ratio2ySd
 * @property {Ratio1ySdPattern} ratio4ySd
 * @property {MetricPattern5<StoredF32>} ratioPct1
 * @property {MetricPattern5<Dollars>} ratioPct1Usd
 * @property {MetricPattern5<StoredF32>} ratioPct2
 * @property {MetricPattern5<Dollars>} ratioPct2Usd
 * @property {MetricPattern5<StoredF32>} ratioPct5
 * @property {MetricPattern5<Dollars>} ratioPct5Usd
 * @property {MetricPattern5<StoredF32>} ratioPct95
 * @property {MetricPattern5<Dollars>} ratioPct95Usd
 * @property {MetricPattern5<StoredF32>} ratioPct98
 * @property {MetricPattern5<Dollars>} ratioPct98Usd
 * @property {MetricPattern5<StoredF32>} ratioPct99
 * @property {MetricPattern5<Dollars>} ratioPct99Usd
 * @property {Ratio1ySdPattern} ratioSd
 */

/**
 * Create a ActivePriceRatioPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {ActivePriceRatioPattern}
 */
function createActivePriceRatioPattern(client, acc) {
  return {
    ratio: createMetricPattern5(client, acc),
    ratio1mSma: createMetricPattern5(client, _m(acc, '1m_sma')),
    ratio1wSma: createMetricPattern5(client, _m(acc, '1w_sma')),
    ratio1ySd: createRatio1ySdPattern(client, _m(acc, '1y')),
    ratio2ySd: createRatio1ySdPattern(client, _m(acc, '2y')),
    ratio4ySd: createRatio1ySdPattern(client, _m(acc, '4y')),
    ratioPct1: createMetricPattern5(client, _m(acc, 'pct1')),
    ratioPct1Usd: createMetricPattern5(client, _m(acc, 'pct1_usd')),
    ratioPct2: createMetricPattern5(client, _m(acc, 'pct2')),
    ratioPct2Usd: createMetricPattern5(client, _m(acc, 'pct2_usd')),
    ratioPct5: createMetricPattern5(client, _m(acc, 'pct5')),
    ratioPct5Usd: createMetricPattern5(client, _m(acc, 'pct5_usd')),
    ratioPct95: createMetricPattern5(client, _m(acc, 'pct95')),
    ratioPct95Usd: createMetricPattern5(client, _m(acc, 'pct95_usd')),
    ratioPct98: createMetricPattern5(client, _m(acc, 'pct98')),
    ratioPct98Usd: createMetricPattern5(client, _m(acc, 'pct98_usd')),
    ratioPct99: createMetricPattern5(client, _m(acc, 'pct99')),
    ratioPct99Usd: createMetricPattern5(client, _m(acc, 'pct99_usd')),
    ratioSd: createRatio1ySdPattern(client, acc),
  };
}

/**
 * @typedef {Object} RelativePattern5
 * @property {MetricPattern3<StoredF32>} negUnrealizedLossRelToMarketCap
 * @property {MetricPattern3<StoredF32>} negUnrealizedLossRelToOwnMarketCap
 * @property {MetricPattern3<StoredF32>} negUnrealizedLossRelToOwnTotalUnrealizedPnl
 * @property {MetricPattern3<StoredF32>} netUnrealizedPnlRelToMarketCap
 * @property {MetricPattern3<StoredF32>} netUnrealizedPnlRelToOwnMarketCap
 * @property {MetricPattern3<StoredF32>} netUnrealizedPnlRelToOwnTotalUnrealizedPnl
 * @property {MetricPattern5<StoredF32>} nupl
 * @property {MetricPattern3<StoredF64>} supplyInLossRelToCirculatingSupply
 * @property {MetricPattern3<StoredF64>} supplyInLossRelToOwnSupply
 * @property {MetricPattern3<StoredF64>} supplyInProfitRelToCirculatingSupply
 * @property {MetricPattern3<StoredF64>} supplyInProfitRelToOwnSupply
 * @property {MetricPattern5<StoredF64>} supplyRelToCirculatingSupply
 * @property {MetricPattern3<StoredF32>} unrealizedLossRelToMarketCap
 * @property {MetricPattern3<StoredF32>} unrealizedLossRelToOwnMarketCap
 * @property {MetricPattern3<StoredF32>} unrealizedLossRelToOwnTotalUnrealizedPnl
 * @property {MetricPattern3<StoredF32>} unrealizedProfitRelToMarketCap
 * @property {MetricPattern3<StoredF32>} unrealizedProfitRelToOwnMarketCap
 * @property {MetricPattern3<StoredF32>} unrealizedProfitRelToOwnTotalUnrealizedPnl
 */

/**
 * Create a RelativePattern5 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {RelativePattern5}
 */
function createRelativePattern5(client, acc) {
  return {
    negUnrealizedLossRelToMarketCap: createMetricPattern3(client, _m(acc, 'neg_unrealized_loss_rel_to_market_cap')),
    negUnrealizedLossRelToOwnMarketCap: createMetricPattern3(client, _m(acc, 'neg_unrealized_loss_rel_to_own_market_cap')),
    negUnrealizedLossRelToOwnTotalUnrealizedPnl: createMetricPattern3(client, _m(acc, 'neg_unrealized_loss_rel_to_own_total_unrealized_pnl')),
    netUnrealizedPnlRelToMarketCap: createMetricPattern3(client, _m(acc, 'net_unrealized_pnl_rel_to_market_cap')),
    netUnrealizedPnlRelToOwnMarketCap: createMetricPattern3(client, _m(acc, 'net_unrealized_pnl_rel_to_own_market_cap')),
    netUnrealizedPnlRelToOwnTotalUnrealizedPnl: createMetricPattern3(client, _m(acc, 'net_unrealized_pnl_rel_to_own_total_unrealized_pnl')),
    nupl: createMetricPattern5(client, _m(acc, 'nupl')),
    supplyInLossRelToCirculatingSupply: createMetricPattern3(client, _m(acc, 'supply_in_loss_rel_to_circulating_supply')),
    supplyInLossRelToOwnSupply: createMetricPattern3(client, _m(acc, 'supply_in_loss_rel_to_own_supply')),
    supplyInProfitRelToCirculatingSupply: createMetricPattern3(client, _m(acc, 'supply_in_profit_rel_to_circulating_supply')),
    supplyInProfitRelToOwnSupply: createMetricPattern3(client, _m(acc, 'supply_in_profit_rel_to_own_supply')),
    supplyRelToCirculatingSupply: createMetricPattern5(client, _m(acc, 'supply_rel_to_circulating_supply')),
    unrealizedLossRelToMarketCap: createMetricPattern3(client, _m(acc, 'unrealized_loss_rel_to_market_cap')),
    unrealizedLossRelToOwnMarketCap: createMetricPattern3(client, _m(acc, 'unrealized_loss_rel_to_own_market_cap')),
    unrealizedLossRelToOwnTotalUnrealizedPnl: createMetricPattern3(client, _m(acc, 'unrealized_loss_rel_to_own_total_unrealized_pnl')),
    unrealizedProfitRelToMarketCap: createMetricPattern3(client, _m(acc, 'unrealized_profit_rel_to_market_cap')),
    unrealizedProfitRelToOwnMarketCap: createMetricPattern3(client, _m(acc, 'unrealized_profit_rel_to_own_market_cap')),
    unrealizedProfitRelToOwnTotalUnrealizedPnl: createMetricPattern3(client, _m(acc, 'unrealized_profit_rel_to_own_total_unrealized_pnl')),
  };
}

/**
 * @typedef {Object} AXbtPattern
 * @property {BlockCountPattern<StoredF32>} _1dDominance
 * @property {MetricPattern5<StoredU32>} _1mBlocksMined
 * @property {MetricPattern5<StoredF32>} _1mDominance
 * @property {MetricPattern5<StoredU32>} _1wBlocksMined
 * @property {MetricPattern5<StoredF32>} _1wDominance
 * @property {MetricPattern5<StoredU32>} _1yBlocksMined
 * @property {MetricPattern5<StoredF32>} _1yDominance
 * @property {BlockCountPattern<StoredU32>} blocksMined
 * @property {UnclaimedRewardsPattern} coinbase
 * @property {MetricPattern5<StoredU16>} daysSinceBlock
 * @property {BlockCountPattern<StoredF32>} dominance
 * @property {SentPattern} fee
 * @property {SentPattern} subsidy
 */

/**
 * Create a AXbtPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {AXbtPattern}
 */
function createAXbtPattern(client, acc) {
  return {
    _1dDominance: createBlockCountPattern(client, _m(acc, '1d_dominance')),
    _1mBlocksMined: createMetricPattern5(client, _m(acc, '1m_blocks_mined')),
    _1mDominance: createMetricPattern5(client, _m(acc, '1m_dominance')),
    _1wBlocksMined: createMetricPattern5(client, _m(acc, '1w_blocks_mined')),
    _1wDominance: createMetricPattern5(client, _m(acc, '1w_dominance')),
    _1yBlocksMined: createMetricPattern5(client, _m(acc, '1y_blocks_mined')),
    _1yDominance: createMetricPattern5(client, _m(acc, '1y_dominance')),
    blocksMined: createBlockCountPattern(client, _m(acc, 'blocks_mined')),
    coinbase: createUnclaimedRewardsPattern(client, _m(acc, 'coinbase')),
    daysSinceBlock: createMetricPattern5(client, _m(acc, 'days_since_block')),
    dominance: createBlockCountPattern(client, _m(acc, 'dominance')),
    fee: createSentPattern(client, acc),
    subsidy: createSentPattern(client, acc),
  };
}

/**
 * @template T
 * @typedef {Object} PriceAgoPattern
 * @property {MetricPattern5<T>} _10y
 * @property {MetricPattern5<T>} _1d
 * @property {MetricPattern5<T>} _1m
 * @property {MetricPattern5<T>} _1w
 * @property {MetricPattern5<T>} _1y
 * @property {MetricPattern5<T>} _2y
 * @property {MetricPattern5<T>} _3m
 * @property {MetricPattern5<T>} _3y
 * @property {MetricPattern5<T>} _4y
 * @property {MetricPattern5<T>} _5y
 * @property {MetricPattern5<T>} _6m
 * @property {MetricPattern5<T>} _6y
 * @property {MetricPattern5<T>} _8y
 */

/**
 * Create a PriceAgoPattern pattern node
 * @template T
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {PriceAgoPattern<T>}
 */
function createPriceAgoPattern(client, acc) {
  return {
    _10y: createMetricPattern5(client, _m(acc, '10y_ago')),
    _1d: createMetricPattern5(client, _m(acc, '1d_ago')),
    _1m: createMetricPattern5(client, _m(acc, '1m_ago')),
    _1w: createMetricPattern5(client, _m(acc, '1w_ago')),
    _1y: createMetricPattern5(client, _m(acc, '1y_ago')),
    _2y: createMetricPattern5(client, _m(acc, '2y_ago')),
    _3m: createMetricPattern5(client, _m(acc, '3m_ago')),
    _3y: createMetricPattern5(client, _m(acc, '3y_ago')),
    _4y: createMetricPattern5(client, _m(acc, '4y_ago')),
    _5y: createMetricPattern5(client, _m(acc, '5y_ago')),
    _6m: createMetricPattern5(client, _m(acc, '6m_ago')),
    _6y: createMetricPattern5(client, _m(acc, '6y_ago')),
    _8y: createMetricPattern5(client, _m(acc, '8y_ago')),
  };
}

/**
 * @typedef {Object} PeriodLumpSumStackPattern
 * @property {ActiveSupplyPattern} _10y
 * @property {ActiveSupplyPattern} _1m
 * @property {ActiveSupplyPattern} _1w
 * @property {ActiveSupplyPattern} _1y
 * @property {ActiveSupplyPattern} _2y
 * @property {ActiveSupplyPattern} _3m
 * @property {ActiveSupplyPattern} _3y
 * @property {ActiveSupplyPattern} _4y
 * @property {ActiveSupplyPattern} _5y
 * @property {ActiveSupplyPattern} _6m
 * @property {ActiveSupplyPattern} _6y
 * @property {ActiveSupplyPattern} _8y
 */

/**
 * Create a PeriodLumpSumStackPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {PeriodLumpSumStackPattern}
 */
function createPeriodLumpSumStackPattern(client, acc) {
  return {
    _10y: createActiveSupplyPattern(client, (acc ? `10y_${acc}` : '10y')),
    _1m: createActiveSupplyPattern(client, (acc ? `1m_${acc}` : '1m')),
    _1w: createActiveSupplyPattern(client, (acc ? `1w_${acc}` : '1w')),
    _1y: createActiveSupplyPattern(client, (acc ? `1y_${acc}` : '1y')),
    _2y: createActiveSupplyPattern(client, (acc ? `2y_${acc}` : '2y')),
    _3m: createActiveSupplyPattern(client, (acc ? `3m_${acc}` : '3m')),
    _3y: createActiveSupplyPattern(client, (acc ? `3y_${acc}` : '3y')),
    _4y: createActiveSupplyPattern(client, (acc ? `4y_${acc}` : '4y')),
    _5y: createActiveSupplyPattern(client, (acc ? `5y_${acc}` : '5y')),
    _6m: createActiveSupplyPattern(client, (acc ? `6m_${acc}` : '6m')),
    _6y: createActiveSupplyPattern(client, (acc ? `6y_${acc}` : '6y')),
    _8y: createActiveSupplyPattern(client, (acc ? `8y_${acc}` : '8y')),
  };
}

/**
 * @template T
 * @typedef {Object} PeriodAveragePricePattern
 * @property {MetricPattern5<T>} _10y
 * @property {MetricPattern5<T>} _1m
 * @property {MetricPattern5<T>} _1w
 * @property {MetricPattern5<T>} _1y
 * @property {MetricPattern5<T>} _2y
 * @property {MetricPattern5<T>} _3m
 * @property {MetricPattern5<T>} _3y
 * @property {MetricPattern5<T>} _4y
 * @property {MetricPattern5<T>} _5y
 * @property {MetricPattern5<T>} _6m
 * @property {MetricPattern5<T>} _6y
 * @property {MetricPattern5<T>} _8y
 */

/**
 * Create a PeriodAveragePricePattern pattern node
 * @template T
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {PeriodAveragePricePattern<T>}
 */
function createPeriodAveragePricePattern(client, acc) {
  return {
    _10y: createMetricPattern5(client, (acc ? `10y_${acc}` : '10y')),
    _1m: createMetricPattern5(client, (acc ? `1m_${acc}` : '1m')),
    _1w: createMetricPattern5(client, (acc ? `1w_${acc}` : '1w')),
    _1y: createMetricPattern5(client, (acc ? `1y_${acc}` : '1y')),
    _2y: createMetricPattern5(client, (acc ? `2y_${acc}` : '2y')),
    _3m: createMetricPattern5(client, (acc ? `3m_${acc}` : '3m')),
    _3y: createMetricPattern5(client, (acc ? `3y_${acc}` : '3y')),
    _4y: createMetricPattern5(client, (acc ? `4y_${acc}` : '4y')),
    _5y: createMetricPattern5(client, (acc ? `5y_${acc}` : '5y')),
    _6m: createMetricPattern5(client, (acc ? `6m_${acc}` : '6m')),
    _6y: createMetricPattern5(client, (acc ? `6y_${acc}` : '6y')),
    _8y: createMetricPattern5(client, (acc ? `8y_${acc}` : '8y')),
  };
}

/**
 * @template T
 * @typedef {Object} ClassAveragePricePattern
 * @property {MetricPattern5<T>} _2015
 * @property {MetricPattern5<T>} _2016
 * @property {MetricPattern5<T>} _2017
 * @property {MetricPattern5<T>} _2018
 * @property {MetricPattern5<T>} _2019
 * @property {MetricPattern5<T>} _2020
 * @property {MetricPattern5<T>} _2021
 * @property {MetricPattern5<T>} _2022
 * @property {MetricPattern5<T>} _2023
 * @property {MetricPattern5<T>} _2024
 * @property {MetricPattern5<T>} _2025
 */

/**
 * Create a ClassAveragePricePattern pattern node
 * @template T
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {ClassAveragePricePattern<T>}
 */
function createClassAveragePricePattern(client, acc) {
  return {
    _2015: createMetricPattern5(client, _m(acc, '2015_average_price')),
    _2016: createMetricPattern5(client, _m(acc, '2016_average_price')),
    _2017: createMetricPattern5(client, _m(acc, '2017_average_price')),
    _2018: createMetricPattern5(client, _m(acc, '2018_average_price')),
    _2019: createMetricPattern5(client, _m(acc, '2019_average_price')),
    _2020: createMetricPattern5(client, _m(acc, '2020_average_price')),
    _2021: createMetricPattern5(client, _m(acc, '2021_average_price')),
    _2022: createMetricPattern5(client, _m(acc, '2022_average_price')),
    _2023: createMetricPattern5(client, _m(acc, '2023_average_price')),
    _2024: createMetricPattern5(client, _m(acc, '2024_average_price')),
    _2025: createMetricPattern5(client, _m(acc, '2025_average_price')),
  };
}

/**
 * @typedef {Object} RelativePattern2
 * @property {MetricPattern3<StoredF32>} negUnrealizedLossRelToOwnMarketCap
 * @property {MetricPattern3<StoredF32>} negUnrealizedLossRelToOwnTotalUnrealizedPnl
 * @property {MetricPattern3<StoredF32>} netUnrealizedPnlRelToOwnMarketCap
 * @property {MetricPattern3<StoredF32>} netUnrealizedPnlRelToOwnTotalUnrealizedPnl
 * @property {MetricPattern3<StoredF64>} supplyInLossRelToOwnSupply
 * @property {MetricPattern3<StoredF64>} supplyInProfitRelToOwnSupply
 * @property {MetricPattern3<StoredF32>} unrealizedLossRelToOwnMarketCap
 * @property {MetricPattern3<StoredF32>} unrealizedLossRelToOwnTotalUnrealizedPnl
 * @property {MetricPattern3<StoredF32>} unrealizedProfitRelToOwnMarketCap
 * @property {MetricPattern3<StoredF32>} unrealizedProfitRelToOwnTotalUnrealizedPnl
 */

/**
 * Create a RelativePattern2 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {RelativePattern2}
 */
function createRelativePattern2(client, acc) {
  return {
    negUnrealizedLossRelToOwnMarketCap: createMetricPattern3(client, _m(acc, 'neg_unrealized_loss_rel_to_own_market_cap')),
    negUnrealizedLossRelToOwnTotalUnrealizedPnl: createMetricPattern3(client, _m(acc, 'neg_unrealized_loss_rel_to_own_total_unrealized_pnl')),
    netUnrealizedPnlRelToOwnMarketCap: createMetricPattern3(client, _m(acc, 'net_unrealized_pnl_rel_to_own_market_cap')),
    netUnrealizedPnlRelToOwnTotalUnrealizedPnl: createMetricPattern3(client, _m(acc, 'net_unrealized_pnl_rel_to_own_total_unrealized_pnl')),
    supplyInLossRelToOwnSupply: createMetricPattern3(client, _m(acc, 'supply_in_loss_rel_to_own_supply')),
    supplyInProfitRelToOwnSupply: createMetricPattern3(client, _m(acc, 'supply_in_profit_rel_to_own_supply')),
    unrealizedLossRelToOwnMarketCap: createMetricPattern3(client, _m(acc, 'unrealized_loss_rel_to_own_market_cap')),
    unrealizedLossRelToOwnTotalUnrealizedPnl: createMetricPattern3(client, _m(acc, 'unrealized_loss_rel_to_own_total_unrealized_pnl')),
    unrealizedProfitRelToOwnMarketCap: createMetricPattern3(client, _m(acc, 'unrealized_profit_rel_to_own_market_cap')),
    unrealizedProfitRelToOwnTotalUnrealizedPnl: createMetricPattern3(client, _m(acc, 'unrealized_profit_rel_to_own_total_unrealized_pnl')),
  };
}

/**
 * @typedef {Object} RelativePattern
 * @property {MetricPattern3<StoredF32>} negUnrealizedLossRelToMarketCap
 * @property {MetricPattern3<StoredF32>} netUnrealizedPnlRelToMarketCap
 * @property {MetricPattern5<StoredF32>} nupl
 * @property {MetricPattern3<StoredF64>} supplyInLossRelToCirculatingSupply
 * @property {MetricPattern3<StoredF64>} supplyInLossRelToOwnSupply
 * @property {MetricPattern3<StoredF64>} supplyInProfitRelToCirculatingSupply
 * @property {MetricPattern3<StoredF64>} supplyInProfitRelToOwnSupply
 * @property {MetricPattern5<StoredF64>} supplyRelToCirculatingSupply
 * @property {MetricPattern3<StoredF32>} unrealizedLossRelToMarketCap
 * @property {MetricPattern3<StoredF32>} unrealizedProfitRelToMarketCap
 */

/**
 * Create a RelativePattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {RelativePattern}
 */
function createRelativePattern(client, acc) {
  return {
    negUnrealizedLossRelToMarketCap: createMetricPattern3(client, _m(acc, 'neg_unrealized_loss_rel_to_market_cap')),
    netUnrealizedPnlRelToMarketCap: createMetricPattern3(client, _m(acc, 'net_unrealized_pnl_rel_to_market_cap')),
    nupl: createMetricPattern5(client, _m(acc, 'nupl')),
    supplyInLossRelToCirculatingSupply: createMetricPattern3(client, _m(acc, 'supply_in_loss_rel_to_circulating_supply')),
    supplyInLossRelToOwnSupply: createMetricPattern3(client, _m(acc, 'supply_in_loss_rel_to_own_supply')),
    supplyInProfitRelToCirculatingSupply: createMetricPattern3(client, _m(acc, 'supply_in_profit_rel_to_circulating_supply')),
    supplyInProfitRelToOwnSupply: createMetricPattern3(client, _m(acc, 'supply_in_profit_rel_to_own_supply')),
    supplyRelToCirculatingSupply: createMetricPattern5(client, _m(acc, 'supply_rel_to_circulating_supply')),
    unrealizedLossRelToMarketCap: createMetricPattern3(client, _m(acc, 'unrealized_loss_rel_to_market_cap')),
    unrealizedProfitRelToMarketCap: createMetricPattern3(client, _m(acc, 'unrealized_profit_rel_to_market_cap')),
  };
}

/**
 * @typedef {Object} UnrealizedPattern
 * @property {MetricPattern3<Dollars>} negUnrealizedLoss
 * @property {MetricPattern3<Dollars>} netUnrealizedPnl
 * @property {SupplyPattern2} supplyInLoss
 * @property {SupplyValuePattern} supplyInLossValue
 * @property {SupplyPattern2} supplyInProfit
 * @property {SupplyValuePattern} supplyInProfitValue
 * @property {MetricPattern3<Dollars>} totalUnrealizedPnl
 * @property {MetricPattern3<Dollars>} unrealizedLoss
 * @property {MetricPattern3<Dollars>} unrealizedProfit
 */

/**
 * Create a UnrealizedPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {UnrealizedPattern}
 */
function createUnrealizedPattern(client, acc) {
  return {
    negUnrealizedLoss: createMetricPattern3(client, _m(acc, 'neg_unrealized_loss')),
    netUnrealizedPnl: createMetricPattern3(client, _m(acc, 'net_unrealized_pnl')),
    supplyInLoss: createSupplyPattern2(client, _m(acc, 'supply_in_loss')),
    supplyInLossValue: createSupplyValuePattern(client, _m(acc, 'supply_in_loss')),
    supplyInProfit: createSupplyPattern2(client, _m(acc, 'supply_in_profit')),
    supplyInProfitValue: createSupplyValuePattern(client, _m(acc, 'supply_in_profit')),
    totalUnrealizedPnl: createMetricPattern3(client, _m(acc, 'total_unrealized_pnl')),
    unrealizedLoss: createMetricPattern3(client, _m(acc, 'unrealized_loss')),
    unrealizedProfit: createMetricPattern3(client, _m(acc, 'unrealized_profit')),
  };
}

/**
 * @template T
 * @typedef {Object} AddresstypeToHeightToAddrCountPattern
 * @property {MetricPattern26<T>} p2a
 * @property {MetricPattern26<T>} p2pk33
 * @property {MetricPattern26<T>} p2pk65
 * @property {MetricPattern26<T>} p2pkh
 * @property {MetricPattern26<T>} p2sh
 * @property {MetricPattern26<T>} p2tr
 * @property {MetricPattern26<T>} p2wpkh
 * @property {MetricPattern26<T>} p2wsh
 */

/**
 * Create a AddresstypeToHeightToAddrCountPattern pattern node
 * @template T
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {AddresstypeToHeightToAddrCountPattern<T>}
 */
function createAddresstypeToHeightToAddrCountPattern(client, acc) {
  return {
    p2a: createMetricPattern26(client, (acc ? `p2a_${acc}` : 'p2a')),
    p2pk33: createMetricPattern26(client, (acc ? `p2pk33_${acc}` : 'p2pk33')),
    p2pk65: createMetricPattern26(client, (acc ? `p2pk65_${acc}` : 'p2pk65')),
    p2pkh: createMetricPattern26(client, (acc ? `p2pkh_${acc}` : 'p2pkh')),
    p2sh: createMetricPattern26(client, (acc ? `p2sh_${acc}` : 'p2sh')),
    p2tr: createMetricPattern26(client, (acc ? `p2tr_${acc}` : 'p2tr')),
    p2wpkh: createMetricPattern26(client, (acc ? `p2wpkh_${acc}` : 'p2wpkh')),
    p2wsh: createMetricPattern26(client, (acc ? `p2wsh_${acc}` : 'p2wsh')),
  };
}

/**
 * @template T
 * @typedef {Object} CountPattern2
 * @property {MetricPattern2<T>} average
 * @property {MetricPattern6<T>} cumulative
 * @property {BlockIntervalPattern<T>} distribution
 * @property {MetricPattern6<T>} max
 * @property {MetricPattern6<T>} min
 * @property {MinmaxPattern<T>} minmax
 * @property {MetricPattern6<T>} sum
 * @property {SumCumPattern<T>} sumCum
 */

/**
 * Create a CountPattern2 pattern node
 * @template T
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {CountPattern2<T>}
 */
function createCountPattern2(client, acc) {
  return {
    average: createMetricPattern2(client, _m(acc, 'average')),
    cumulative: createMetricPattern6(client, _m(acc, 'cumulative')),
    distribution: createBlockIntervalPattern(client, acc),
    max: createMetricPattern6(client, _m(acc, 'max')),
    min: createMetricPattern6(client, _m(acc, 'min')),
    minmax: createMinmaxPattern(client, acc),
    sum: createMetricPattern6(client, _m(acc, 'sum')),
    sumCum: createSumCumPattern(client, acc),
  };
}

/**
 * @typedef {Object} _0satsPattern
 * @property {ActivityPattern2} activity
 * @property {MetricPattern1<StoredU64>} addrCount
 * @property {CostBasisPattern} costBasis
 * @property {RealizedPattern} realized
 * @property {RelativePattern} relative
 * @property {SupplyPattern3} supply
 * @property {UnrealizedPattern} unrealized
 */

/**
 * Create a _0satsPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {_0satsPattern}
 */
function create_0satsPattern(client, acc) {
  return {
    activity: createActivityPattern2(client, acc),
    addrCount: createMetricPattern1(client, _m(acc, 'addr_count')),
    costBasis: createCostBasisPattern(client, acc),
    realized: createRealizedPattern(client, acc),
    relative: createRelativePattern(client, acc),
    supply: createSupplyPattern3(client, acc),
    unrealized: createUnrealizedPattern(client, acc),
  };
}

/**
 * @typedef {Object} PeriodCagrPattern
 * @property {MetricPattern5<StoredF32>} _10y
 * @property {MetricPattern5<StoredF32>} _2y
 * @property {MetricPattern5<StoredF32>} _3y
 * @property {MetricPattern5<StoredF32>} _4y
 * @property {MetricPattern5<StoredF32>} _5y
 * @property {MetricPattern5<StoredF32>} _6y
 * @property {MetricPattern5<StoredF32>} _8y
 */

/**
 * Create a PeriodCagrPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {PeriodCagrPattern}
 */
function createPeriodCagrPattern(client, acc) {
  return {
    _10y: createMetricPattern5(client, (acc ? `10y_${acc}` : '10y')),
    _2y: createMetricPattern5(client, (acc ? `2y_${acc}` : '2y')),
    _3y: createMetricPattern5(client, (acc ? `3y_${acc}` : '3y')),
    _4y: createMetricPattern5(client, (acc ? `4y_${acc}` : '4y')),
    _5y: createMetricPattern5(client, (acc ? `5y_${acc}` : '5y')),
    _6y: createMetricPattern5(client, (acc ? `6y_${acc}` : '6y')),
    _8y: createMetricPattern5(client, (acc ? `8y_${acc}` : '8y')),
  };
}

/**
 * @template T
 * @typedef {Object} BlockSizePattern
 * @property {MetricPattern6<T>} average
 * @property {MetricPattern4<T>} cumulative
 * @property {BlockIntervalPattern<T>} distribution
 * @property {MetricPattern6<T>} max
 * @property {MetricPattern6<T>} min
 * @property {MetricPattern6<T>} sum
 * @property {SumCumPattern<T>} sumCum
 */

/**
 * Create a BlockSizePattern pattern node
 * @template T
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {BlockSizePattern<T>}
 */
function createBlockSizePattern(client, acc) {
  return {
    average: createMetricPattern6(client, _m(acc, 'average')),
    cumulative: createMetricPattern4(client, _m(acc, 'cumulative')),
    distribution: createBlockIntervalPattern(client, acc),
    max: createMetricPattern6(client, _m(acc, 'max')),
    min: createMetricPattern6(client, _m(acc, 'min')),
    sum: createMetricPattern6(client, _m(acc, 'sum')),
    sumCum: createSumCumPattern(client, acc),
  };
}

/**
 * @template T
 * @typedef {Object} DollarsPattern
 * @property {MetricPattern2<T>} average
 * @property {MetricPattern26<T>} base
 * @property {MetricPattern1<T>} cumulative
 * @property {MetricPattern2<T>} max
 * @property {MetricPattern2<T>} min
 * @property {PercentilesPattern<T>} percentiles
 * @property {MetricPattern2<T>} sum
 */

/**
 * Create a DollarsPattern pattern node
 * @template T
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {DollarsPattern<T>}
 */
function createDollarsPattern(client, acc) {
  return {
    average: createMetricPattern2(client, _m(acc, 'average')),
    base: createMetricPattern26(client, acc),
    cumulative: createMetricPattern1(client, _m(acc, 'cumulative')),
    max: createMetricPattern2(client, _m(acc, 'max')),
    min: createMetricPattern2(client, _m(acc, 'min')),
    percentiles: createPercentilesPattern(client, acc),
    sum: createMetricPattern2(client, _m(acc, 'sum')),
  };
}

/**
 * @typedef {Object} _10yPattern
 * @property {ActivityPattern2} activity
 * @property {CostBasisPattern} costBasis
 * @property {RealizedPattern4} realized
 * @property {RelativePattern} relative
 * @property {SupplyPattern3} supply
 * @property {UnrealizedPattern} unrealized
 */

/**
 * Create a _10yPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {_10yPattern}
 */
function create_10yPattern(client, acc) {
  return {
    activity: createActivityPattern2(client, acc),
    costBasis: createCostBasisPattern(client, acc),
    realized: createRealizedPattern4(client, acc),
    relative: createRelativePattern(client, acc),
    supply: createSupplyPattern3(client, acc),
    unrealized: createUnrealizedPattern(client, acc),
  };
}

/**
 * @typedef {Object} _10yTo12yPattern
 * @property {ActivityPattern2} activity
 * @property {CostBasisPattern2} costBasis
 * @property {RealizedPattern2} realized
 * @property {RelativePattern2} relative
 * @property {SupplyPattern3} supply
 * @property {UnrealizedPattern} unrealized
 */

/**
 * Create a _10yTo12yPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {_10yTo12yPattern}
 */
function create_10yTo12yPattern(client, acc) {
  return {
    activity: createActivityPattern2(client, acc),
    costBasis: createCostBasisPattern2(client, acc),
    realized: createRealizedPattern2(client, acc),
    relative: createRelativePattern2(client, acc),
    supply: createSupplyPattern3(client, acc),
    unrealized: createUnrealizedPattern(client, acc),
  };
}

/**
 * @typedef {Object} _100btcPattern
 * @property {ActivityPattern2} activity
 * @property {CostBasisPattern} costBasis
 * @property {RealizedPattern} realized
 * @property {RelativePattern} relative
 * @property {SupplyPattern3} supply
 * @property {UnrealizedPattern} unrealized
 */

/**
 * Create a _100btcPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {_100btcPattern}
 */
function create_100btcPattern(client, acc) {
  return {
    activity: createActivityPattern2(client, acc),
    costBasis: createCostBasisPattern(client, acc),
    realized: createRealizedPattern(client, acc),
    relative: createRelativePattern(client, acc),
    supply: createSupplyPattern3(client, acc),
    unrealized: createUnrealizedPattern(client, acc),
  };
}

/**
 * @typedef {Object} _0satsPattern2
 * @property {ActivityPattern2} activity
 * @property {CostBasisPattern} costBasis
 * @property {RealizedPattern} realized
 * @property {RelativePattern4} relative
 * @property {SupplyPattern3} supply
 * @property {UnrealizedPattern} unrealized
 */

/**
 * Create a _0satsPattern2 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {_0satsPattern2}
 */
function create_0satsPattern2(client, acc) {
  return {
    activity: createActivityPattern2(client, acc),
    costBasis: createCostBasisPattern(client, acc),
    realized: createRealizedPattern(client, acc),
    relative: createRelativePattern4(client, _m(acc, 'supply_in')),
    supply: createSupplyPattern3(client, acc),
    unrealized: createUnrealizedPattern(client, acc),
  };
}

/**
 * @template T
 * @typedef {Object} BitcoinPattern
 * @property {MetricPattern2<T>} average
 * @property {MetricPattern26<T>} base
 * @property {MetricPattern2<T>} cumulative
 * @property {MetricPattern2<T>} max
 * @property {MetricPattern2<T>} min
 * @property {MetricPattern2<T>} sum
 */

/**
 * Create a BitcoinPattern pattern node
 * @template T
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {BitcoinPattern<T>}
 */
function createBitcoinPattern(client, acc) {
  return {
    average: createMetricPattern2(client, _m(acc, 'average')),
    base: createMetricPattern26(client, acc),
    cumulative: createMetricPattern2(client, _m(acc, 'cum')),
    max: createMetricPattern2(client, _m(acc, 'max')),
    min: createMetricPattern2(client, _m(acc, 'min')),
    sum: createMetricPattern2(client, _m(acc, 'sum')),
  };
}

/**
 * @typedef {Object} ActivityPattern2
 * @property {BlockCountPattern<StoredF64>} coinblocksDestroyed
 * @property {BlockCountPattern<StoredF64>} coindaysDestroyed
 * @property {MetricPattern26<Sats>} satblocksDestroyed
 * @property {MetricPattern26<Sats>} satdaysDestroyed
 * @property {SentPattern} sent
 */

/**
 * Create a ActivityPattern2 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {ActivityPattern2}
 */
function createActivityPattern2(client, acc) {
  return {
    coinblocksDestroyed: createBlockCountPattern(client, _m(acc, 'coinblocks_destroyed')),
    coindaysDestroyed: createBlockCountPattern(client, _m(acc, 'coindays_destroyed')),
    satblocksDestroyed: createMetricPattern26(client, _m(acc, 'satblocks_destroyed')),
    satdaysDestroyed: createMetricPattern26(client, _m(acc, 'satdays_destroyed')),
    sent: createSentPattern(client, _m(acc, 'sent')),
  };
}

/**
 * @typedef {Object} SentPattern
 * @property {MetricPattern26<Sats>} base
 * @property {BlockCountPattern<Bitcoin>} bitcoin
 * @property {SumCumPattern<Dollars>} dollars
 * @property {MetricPattern26<Dollars>} dollarsSource
 * @property {SumCumPattern<Sats>} sats
 */

/**
 * Create a SentPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {SentPattern}
 */
function createSentPattern(client, acc) {
  return {
    base: createMetricPattern26(client, _m(acc, 'height_fee')),
    bitcoin: createBlockCountPattern(client, _m(acc, 'btc')),
    dollars: createSumCumPattern(client, _m(acc, 'usd')),
    dollarsSource: createMetricPattern26(client, _m(acc, 'usd')),
    sats: createSumCumPattern(client, _m(acc, 'fee')),
  };
}

/**
 * @typedef {Object} SupplyPattern3
 * @property {SupplyPattern2} supply
 * @property {ActiveSupplyPattern} supplyHalf
 * @property {ActiveSupplyPattern} supplyHalfValue
 * @property {SupplyValuePattern} supplyValue
 * @property {MetricPattern1<StoredU64>} utxoCount
 */

/**
 * Create a SupplyPattern3 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {SupplyPattern3}
 */
function createSupplyPattern3(client, acc) {
  return {
    supply: createSupplyPattern2(client, _m(acc, 'supply')),
    supplyHalf: createActiveSupplyPattern(client, _m(acc, 'supply_half')),
    supplyHalfValue: createActiveSupplyPattern(client, _m(acc, 'supply_half')),
    supplyValue: createSupplyValuePattern(client, _m(acc, 'supply')),
    utxoCount: createMetricPattern1(client, _m(acc, 'utxo_count')),
  };
}

/**
 * @template T
 * @typedef {Object} PercentilesPattern
 * @property {MetricPattern22<T>} median
 * @property {MetricPattern22<T>} pct10
 * @property {MetricPattern22<T>} pct25
 * @property {MetricPattern22<T>} pct75
 * @property {MetricPattern22<T>} pct90
 */

/**
 * Create a PercentilesPattern pattern node
 * @template T
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {PercentilesPattern<T>}
 */
function createPercentilesPattern(client, acc) {
  return {
    median: createMetricPattern22(client, _m(acc, 'median')),
    pct10: createMetricPattern22(client, _m(acc, 'pct10')),
    pct25: createMetricPattern22(client, _m(acc, 'pct25')),
    pct75: createMetricPattern22(client, _m(acc, 'pct75')),
    pct90: createMetricPattern22(client, _m(acc, 'pct90')),
  };
}

/**
 * @typedef {Object} SupplyPattern2
 * @property {MetricPattern26<Sats>} base
 * @property {MetricPattern5<Bitcoin>} bitcoin
 * @property {MetricPattern5<Dollars>} dollars
 * @property {MetricPattern7<Sats>} sats
 */

/**
 * Create a SupplyPattern2 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {SupplyPattern2}
 */
function createSupplyPattern2(client, acc) {
  return {
    base: createMetricPattern26(client, acc),
    bitcoin: createMetricPattern5(client, _m(acc, 'btc')),
    dollars: createMetricPattern5(client, _m(acc, 'usd')),
    sats: createMetricPattern7(client, acc),
  };
}

/**
 * @template T
 * @typedef {Object} PriceHighInSatsPattern
 * @property {MetricPattern22<T>} dateindex
 * @property {MetricPattern26<T>} height
 * @property {MetricPattern24<T>} max
 * @property {MetricPattern7<T>} rest
 */

/**
 * Create a PriceHighInSatsPattern pattern node
 * @template T
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {PriceHighInSatsPattern<T>}
 */
function createPriceHighInSatsPattern(client, acc) {
  return {
    dateindex: createMetricPattern22(client, acc),
    height: createMetricPattern26(client, acc),
    max: createMetricPattern24(client, _m(acc, 'max')),
    rest: createMetricPattern7(client, _m(acc, 'max')),
  };
}

/**
 * @template T
 * @typedef {Object} PriceLowInSatsPattern
 * @property {MetricPattern22<T>} dateindex
 * @property {MetricPattern26<T>} height
 * @property {MetricPattern24<T>} min
 * @property {MetricPattern7<T>} rest
 */

/**
 * Create a PriceLowInSatsPattern pattern node
 * @template T
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {PriceLowInSatsPattern<T>}
 */
function createPriceLowInSatsPattern(client, acc) {
  return {
    dateindex: createMetricPattern22(client, acc),
    height: createMetricPattern26(client, acc),
    min: createMetricPattern24(client, _m(acc, 'min')),
    rest: createMetricPattern7(client, _m(acc, 'min')),
  };
}

/**
 * @template T
 * @typedef {Object} BlockIntervalPattern
 * @property {MetricPattern22<T>} average
 * @property {MetricPattern22<T>} max
 * @property {MetricPattern22<T>} min
 * @property {PercentilesPattern<T>} percentiles
 */

/**
 * Create a BlockIntervalPattern pattern node
 * @template T
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {BlockIntervalPattern<T>}
 */
function createBlockIntervalPattern(client, acc) {
  return {
    average: createMetricPattern22(client, _m(acc, 'average')),
    max: createMetricPattern22(client, _m(acc, 'max')),
    min: createMetricPattern22(client, _m(acc, 'min')),
    percentiles: createPercentilesPattern(client, acc),
  };
}

/**
 * @typedef {Object} ActiveSupplyPattern
 * @property {MetricPattern1<Bitcoin>} bitcoin
 * @property {MetricPattern1<Dollars>} dollars
 * @property {MetricPattern1<Sats>} sats
 */

/**
 * Create a ActiveSupplyPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {ActiveSupplyPattern}
 */
function createActiveSupplyPattern(client, acc) {
  return {
    bitcoin: createMetricPattern1(client, _m(acc, 'btc')),
    dollars: createMetricPattern1(client, _m(acc, 'usd')),
    sats: createMetricPattern1(client, acc),
  };
}

/**
 * @typedef {Object} CostBasisPattern2
 * @property {MetricPattern1<Dollars>} maxCostBasis
 * @property {MetricPattern1<Dollars>} minCostBasis
 * @property {PercentilesPattern2} percentiles
 */

/**
 * Create a CostBasisPattern2 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {CostBasisPattern2}
 */
function createCostBasisPattern2(client, acc) {
  return {
    maxCostBasis: createMetricPattern1(client, _m(acc, 'max_cost_basis')),
    minCostBasis: createMetricPattern1(client, _m(acc, 'min_cost_basis')),
    percentiles: createPercentilesPattern2(client, _m(acc, 'cost_basis')),
  };
}

/**
 * @typedef {Object} CoinbasePattern
 * @property {BitcoinPattern<Bitcoin>} bitcoin
 * @property {DollarsPattern<Dollars>} dollars
 * @property {DollarsPattern<Sats>} sats
 */

/**
 * Create a CoinbasePattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {CoinbasePattern}
 */
function createCoinbasePattern(client, acc) {
  return {
    bitcoin: createBitcoinPattern(client, _m(acc, 'btc')),
    dollars: createDollarsPattern(client, _m(acc, 'usd')),
    sats: createDollarsPattern(client, acc),
  };
}

/**
 * @typedef {Object} UnclaimedRewardsPattern
 * @property {BlockCountPattern<Bitcoin>} bitcoin
 * @property {BlockCountPattern<Dollars>} dollars
 * @property {BlockCountPattern<Sats>} sats
 */

/**
 * Create a UnclaimedRewardsPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {UnclaimedRewardsPattern}
 */
function createUnclaimedRewardsPattern(client, acc) {
  return {
    bitcoin: createBlockCountPattern(client, _m(acc, 'btc')),
    dollars: createBlockCountPattern(client, _m(acc, 'usd')),
    sats: createBlockCountPattern(client, acc),
  };
}

/**
 * @template T
 * @typedef {Object} BlockCountPattern
 * @property {MetricPattern26<T>} base
 * @property {MetricPattern2<T>} cumulative
 * @property {MetricPattern2<T>} sum
 */

/**
 * Create a BlockCountPattern pattern node
 * @template T
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {BlockCountPattern<T>}
 */
function createBlockCountPattern(client, acc) {
  return {
    base: createMetricPattern26(client, acc),
    cumulative: createMetricPattern2(client, _m(acc, 'cumulative')),
    sum: createMetricPattern2(client, _m(acc, 'sum')),
  };
}

/**
 * @typedef {Object} SupplyValuePattern
 * @property {MetricPattern26<Bitcoin>} bitcoin
 * @property {MetricPattern26<Dollars>} dollars
 */

/**
 * Create a SupplyValuePattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {SupplyValuePattern}
 */
function createSupplyValuePattern(client, acc) {
  return {
    bitcoin: createMetricPattern26(client, _m(acc, 'btc')),
    dollars: createMetricPattern26(client, _m(acc, 'usd')),
  };
}

/**
 * @typedef {Object} RelativePattern4
 * @property {MetricPattern3<StoredF64>} supplyInLossRelToOwnSupply
 * @property {MetricPattern3<StoredF64>} supplyInProfitRelToOwnSupply
 */

/**
 * Create a RelativePattern4 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {RelativePattern4}
 */
function createRelativePattern4(client, acc) {
  return {
    supplyInLossRelToOwnSupply: createMetricPattern3(client, _m(acc, 'loss_rel_to_own_supply')),
    supplyInProfitRelToOwnSupply: createMetricPattern3(client, _m(acc, 'profit_rel_to_own_supply')),
  };
}

/**
 * @typedef {Object} CostBasisPattern
 * @property {MetricPattern1<Dollars>} maxCostBasis
 * @property {MetricPattern1<Dollars>} minCostBasis
 */

/**
 * Create a CostBasisPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {CostBasisPattern}
 */
function createCostBasisPattern(client, acc) {
  return {
    maxCostBasis: createMetricPattern1(client, _m(acc, 'max_cost_basis')),
    minCostBasis: createMetricPattern1(client, _m(acc, 'min_cost_basis')),
  };
}

/**
 * @typedef {Object} _1dReturns1mSdPattern
 * @property {MetricPattern5<StoredF32>} sd
 * @property {MetricPattern5<StoredF32>} sma
 */

/**
 * Create a _1dReturns1mSdPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {_1dReturns1mSdPattern}
 */
function create_1dReturns1mSdPattern(client, acc) {
  return {
    sd: createMetricPattern5(client, _m(acc, 'sd')),
    sma: createMetricPattern5(client, _m(acc, 'sma')),
  };
}

/**
 * @template T
 * @typedef {Object} MinmaxPattern
 * @property {MetricPattern22<T>} max
 * @property {MetricPattern22<T>} min
 */

/**
 * Create a MinmaxPattern pattern node
 * @template T
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {MinmaxPattern<T>}
 */
function createMinmaxPattern(client, acc) {
  return {
    max: createMetricPattern22(client, _m(acc, 'max')),
    min: createMetricPattern22(client, _m(acc, 'min')),
  };
}

/**
 * @template T
 * @typedef {Object} SumCumPattern
 * @property {MetricPattern1<T>} cumulative
 * @property {MetricPattern2<T>} sum
 */

/**
 * Create a SumCumPattern pattern node
 * @template T
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {SumCumPattern<T>}
 */
function createSumCumPattern(client, acc) {
  return {
    cumulative: createMetricPattern1(client, _m(acc, 'cumulative')),
    sum: createMetricPattern2(client, _m(acc, 'sum')),
  };
}

/**
 * @template T
 * @typedef {Object} IndexesPattern2
 * @property {MetricPattern22<T>} dateindex
 * @property {MetricPattern7<T>} rest
 */

/**
 * Create a IndexesPattern2 pattern node
 * @template T
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {IndexesPattern2<T>}
 */
function createIndexesPattern2(client, acc) {
  return {
    dateindex: createMetricPattern22(client, acc),
    rest: createMetricPattern7(client, _m(acc, 'average')),
  };
}

/**
 * @template T
 * @typedef {Object} DifficultyAdjustmentPattern
 * @property {MetricPattern26<T>} base
 * @property {MetricPattern2<T>} rest
 */

/**
 * Create a DifficultyAdjustmentPattern pattern node
 * @template T
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {DifficultyAdjustmentPattern<T>}
 */
function createDifficultyAdjustmentPattern(client, acc) {
  return {
    base: createMetricPattern26(client, acc),
    rest: createMetricPattern2(client, _m(acc, 'sum')),
  };
}

/**
 * @typedef {Object} RealizedPriceExtraPattern
 * @property {MetricPattern5<StoredF32>} ratio
 */

/**
 * Create a RealizedPriceExtraPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {RealizedPriceExtraPattern}
 */
function createRealizedPriceExtraPattern(client, acc) {
  return {
    ratio: createMetricPattern5(client, _m(acc, 'ratio')),
  };
}

// Catalog tree typedefs

/**
 * @typedef {Object} CatalogTree
 * @property {CatalogTree_Computed} computed
 * @property {CatalogTree_Indexed} indexed
 */

/**
 * @typedef {Object} CatalogTree_Computed
 * @property {CatalogTree_Computed_Blocks} blocks
 * @property {CatalogTree_Computed_Cointime} cointime
 * @property {CatalogTree_Computed_Constants} constants
 * @property {CatalogTree_Computed_Distribution} distribution
 * @property {CatalogTree_Computed_Indexes} indexes
 * @property {CatalogTree_Computed_Inputs} inputs
 * @property {CatalogTree_Computed_Market} market
 * @property {CatalogTree_Computed_Outputs} outputs
 * @property {CatalogTree_Computed_Pools} pools
 * @property {CatalogTree_Computed_Positions} positions
 * @property {CatalogTree_Computed_Price} price
 * @property {CatalogTree_Computed_Scripts} scripts
 * @property {CatalogTree_Computed_Supply} supply
 * @property {CatalogTree_Computed_Transactions} transactions
 */

/**
 * @typedef {Object} CatalogTree_Computed_Blocks
 * @property {CatalogTree_Computed_Blocks_Count} count
 * @property {CatalogTree_Computed_Blocks_Difficulty} difficulty
 * @property {CatalogTree_Computed_Blocks_Halving} halving
 * @property {CatalogTree_Computed_Blocks_Interval} interval
 * @property {CatalogTree_Computed_Blocks_Mining} mining
 * @property {CatalogTree_Computed_Blocks_Rewards} rewards
 * @property {CatalogTree_Computed_Blocks_Size} size
 * @property {CatalogTree_Computed_Blocks_Time} time
 * @property {CatalogTree_Computed_Blocks_Weight} weight
 */

/**
 * @typedef {Object} CatalogTree_Computed_Blocks_Count
 * @property {MetricPattern5<StoredU32>} _1mBlockCount
 * @property {MetricPattern5<StoredU32>} _1wBlockCount
 * @property {MetricPattern5<StoredU32>} _1yBlockCount
 * @property {MetricPattern26<StoredU32>} _24hBlockCount
 * @property {BlockCountPattern<StoredU32>} blockCount
 * @property {MetricPattern5<StoredU64>} blockCountTarget
 */

/**
 * @typedef {Object} CatalogTree_Computed_Blocks_Difficulty
 * @property {MetricPattern1<StoredU32>} blocksBeforeNextDifficultyAdjustment
 * @property {MetricPattern1<StoredF32>} daysBeforeNextDifficultyAdjustment
 * @property {MetricPattern5<DifficultyEpoch>} difficultyepoch
 */

/**
 * @typedef {Object} CatalogTree_Computed_Blocks_Halving
 * @property {MetricPattern1<StoredU32>} blocksBeforeNextHalving
 * @property {MetricPattern1<StoredF32>} daysBeforeNextHalving
 * @property {MetricPattern5<HalvingEpoch>} halvingepoch
 */

/**
 * @typedef {Object} CatalogTree_Computed_Blocks_Interval
 * @property {BlockIntervalPattern<Timestamp>} blockInterval
 * @property {MetricPattern26<Timestamp>} interval
 */

/**
 * @typedef {Object} CatalogTree_Computed_Blocks_Mining
 * @property {MetricPattern2<StoredF64>} difficulty
 * @property {DifficultyAdjustmentPattern<StoredF32>} difficultyAdjustment
 * @property {MetricPattern1<StoredF32>} difficultyAsHash
 * @property {MetricPattern1<StoredF32>} hashPricePhs
 * @property {MetricPattern1<StoredF32>} hashPricePhsMin
 * @property {MetricPattern1<StoredF32>} hashPriceRebound
 * @property {MetricPattern1<StoredF32>} hashPriceThs
 * @property {MetricPattern1<StoredF32>} hashPriceThsMin
 * @property {MetricPattern1<StoredF64>} hashRate
 * @property {MetricPattern5<StoredF32>} hashRate1mSma
 * @property {MetricPattern5<StoredF64>} hashRate1wSma
 * @property {MetricPattern5<StoredF32>} hashRate1ySma
 * @property {MetricPattern5<StoredF32>} hashRate2mSma
 * @property {MetricPattern1<StoredF32>} hashValuePhs
 * @property {MetricPattern1<StoredF32>} hashValuePhsMin
 * @property {MetricPattern1<StoredF32>} hashValueRebound
 * @property {MetricPattern1<StoredF32>} hashValueThs
 * @property {MetricPattern1<StoredF32>} hashValueThsMin
 */

/**
 * @typedef {Object} CatalogTree_Computed_Blocks_Rewards
 * @property {MetricPattern26<Sats>} _24hCoinbaseSum
 * @property {MetricPattern26<Dollars>} _24hCoinbaseUsdSum
 * @property {CoinbasePattern} coinbase
 * @property {MetricPattern22<StoredF32>} feeDominance
 * @property {CoinbasePattern} subsidy
 * @property {MetricPattern22<StoredF32>} subsidyDominance
 * @property {MetricPattern5<Dollars>} subsidyUsd1ySma
 * @property {UnclaimedRewardsPattern} unclaimedRewards
 */

/**
 * @typedef {Object} CatalogTree_Computed_Blocks_Size
 * @property {BlockSizePattern<StoredU64>} blockSize
 * @property {BlockSizePattern<StoredU64>} blockVbytes
 * @property {MetricPattern26<StoredU64>} vbytes
 */

/**
 * @typedef {Object} CatalogTree_Computed_Blocks_Time
 * @property {MetricPattern26<Date>} date
 * @property {MetricPattern26<Date>} dateFixed
 * @property {MetricPattern2<Timestamp>} timestamp
 * @property {MetricPattern26<Timestamp>} timestampFixed
 */

/**
 * @typedef {Object} CatalogTree_Computed_Blocks_Weight
 * @property {BitcoinPattern<StoredF32>} blockFullness
 * @property {BlockSizePattern<Weight>} blockWeight
 */

/**
 * @typedef {Object} CatalogTree_Computed_Cointime
 * @property {CatalogTree_Computed_Cointime_Activity} activity
 * @property {CatalogTree_Computed_Cointime_Adjusted} adjusted
 * @property {CatalogTree_Computed_Cointime_Cap} cap
 * @property {CatalogTree_Computed_Cointime_Pricing} pricing
 * @property {CatalogTree_Computed_Cointime_Supply} supply
 * @property {CatalogTree_Computed_Cointime_Value} value
 */

/**
 * @typedef {Object} CatalogTree_Computed_Cointime_Activity
 * @property {MetricPattern1<StoredF64>} activityToVaultednessRatio
 * @property {BlockCountPattern<StoredF64>} coinblocksCreated
 * @property {BlockCountPattern<StoredF64>} coinblocksStored
 * @property {MetricPattern1<StoredF64>} liveliness
 * @property {MetricPattern1<StoredF64>} vaultedness
 */

/**
 * @typedef {Object} CatalogTree_Computed_Cointime_Adjusted
 * @property {MetricPattern5<StoredF32>} cointimeAdjInflationRate
 * @property {MetricPattern5<StoredF64>} cointimeAdjTxBtcVelocity
 * @property {MetricPattern5<StoredF64>} cointimeAdjTxUsdVelocity
 */

/**
 * @typedef {Object} CatalogTree_Computed_Cointime_Cap
 * @property {MetricPattern1<Dollars>} activeCap
 * @property {MetricPattern1<Dollars>} cointimeCap
 * @property {MetricPattern1<Dollars>} investorCap
 * @property {MetricPattern1<Dollars>} thermoCap
 * @property {MetricPattern1<Dollars>} vaultedCap
 */

/**
 * @typedef {Object} CatalogTree_Computed_Cointime_Pricing
 * @property {MetricPattern1<Dollars>} activePrice
 * @property {ActivePriceRatioPattern} activePriceRatio
 * @property {MetricPattern1<Dollars>} cointimePrice
 * @property {ActivePriceRatioPattern} cointimePriceRatio
 * @property {MetricPattern1<Dollars>} trueMarketMean
 * @property {ActivePriceRatioPattern} trueMarketMeanRatio
 * @property {MetricPattern1<Dollars>} vaultedPrice
 * @property {ActivePriceRatioPattern} vaultedPriceRatio
 */

/**
 * @typedef {Object} CatalogTree_Computed_Cointime_Supply
 * @property {ActiveSupplyPattern} activeSupply
 * @property {ActiveSupplyPattern} vaultedSupply
 */

/**
 * @typedef {Object} CatalogTree_Computed_Cointime_Value
 * @property {BlockCountPattern<StoredF64>} cointimeValueCreated
 * @property {BlockCountPattern<StoredF64>} cointimeValueDestroyed
 * @property {BlockCountPattern<StoredF64>} cointimeValueStored
 */

/**
 * @typedef {Object} CatalogTree_Computed_Constants
 * @property {MetricPattern3<StoredU16>} constant0
 * @property {MetricPattern3<StoredU16>} constant1
 * @property {MetricPattern3<StoredU16>} constant100
 * @property {MetricPattern3<StoredU16>} constant2
 * @property {MetricPattern3<StoredU16>} constant20
 * @property {MetricPattern3<StoredU16>} constant3
 * @property {MetricPattern3<StoredU16>} constant30
 * @property {MetricPattern3<StoredF32>} constant382
 * @property {MetricPattern3<StoredU16>} constant4
 * @property {MetricPattern3<StoredU16>} constant50
 * @property {MetricPattern3<StoredU16>} constant600
 * @property {MetricPattern3<StoredF32>} constant618
 * @property {MetricPattern3<StoredU16>} constant70
 * @property {MetricPattern3<StoredU16>} constant80
 * @property {MetricPattern3<StoredI16>} constantMinus1
 * @property {MetricPattern3<StoredI16>} constantMinus2
 * @property {MetricPattern3<StoredI16>} constantMinus3
 * @property {MetricPattern3<StoredI16>} constantMinus4
 */

/**
 * @typedef {Object} CatalogTree_Computed_Distribution
 * @property {MetricPattern1<StoredU64>} addrCount
 * @property {CatalogTree_Computed_Distribution_AddressCohorts} addressCohorts
 * @property {CatalogTree_Computed_Distribution_AddressesData} addressesData
 * @property {AddresstypeToHeightToAddrCountPattern<StoredU64>} addresstypeToHeightToAddrCount
 * @property {AddresstypeToHeightToAddrCountPattern<StoredU64>} addresstypeToHeightToEmptyAddrCount
 * @property {AddresstypeToHeightToAddrCountPattern<StoredU64>} addresstypeToIndexesToAddrCount
 * @property {AddresstypeToHeightToAddrCountPattern<StoredU64>} addresstypeToIndexesToEmptyAddrCount
 * @property {AddresstypeToHeightToAddrCountPattern<AnyAddressIndex>} anyAddressIndexes
 * @property {MetricPattern26<SupplyState>} chainState
 * @property {MetricPattern1<StoredU64>} emptyAddrCount
 * @property {MetricPattern42<EmptyAddressIndex>} emptyaddressindex
 * @property {MetricPattern41<LoadedAddressIndex>} loadedaddressindex
 * @property {CatalogTree_Computed_Distribution_UtxoCohorts} utxoCohorts
 */

/**
 * @typedef {Object} CatalogTree_Computed_Distribution_AddressCohorts
 * @property {CatalogTree_Computed_Distribution_AddressCohorts_AmountRange} amountRange
 * @property {CatalogTree_Computed_Distribution_AddressCohorts_GeAmount} geAmount
 * @property {CatalogTree_Computed_Distribution_AddressCohorts_LtAmount} ltAmount
 */

/**
 * @typedef {Object} CatalogTree_Computed_Distribution_AddressCohorts_AmountRange
 * @property {_0satsPattern} _0sats
 * @property {_0satsPattern} _100btcTo1kBtc
 * @property {_0satsPattern} _100kBtcOrMore
 * @property {_0satsPattern} _100kSatsTo1mSats
 * @property {_0satsPattern} _100satsTo1kSats
 * @property {_0satsPattern} _10btcTo100btc
 * @property {_0satsPattern} _10kBtcTo100kBtc
 * @property {_0satsPattern} _10kSatsTo100kSats
 * @property {_0satsPattern} _10mSatsTo1btc
 * @property {_0satsPattern} _10satsTo100sats
 * @property {_0satsPattern} _1btcTo10btc
 * @property {_0satsPattern} _1kBtcTo10kBtc
 * @property {_0satsPattern} _1kSatsTo10kSats
 * @property {_0satsPattern} _1mSatsTo10mSats
 * @property {_0satsPattern} _1satTo10sats
 */

/**
 * @typedef {Object} CatalogTree_Computed_Distribution_AddressCohorts_GeAmount
 * @property {_0satsPattern} _100btc
 * @property {_0satsPattern} _100kSats
 * @property {_0satsPattern} _100sats
 * @property {_0satsPattern} _10btc
 * @property {_0satsPattern} _10kBtc
 * @property {_0satsPattern} _10kSats
 * @property {_0satsPattern} _10mSats
 * @property {_0satsPattern} _10sats
 * @property {_0satsPattern} _1btc
 * @property {_0satsPattern} _1kBtc
 * @property {_0satsPattern} _1kSats
 * @property {_0satsPattern} _1mSats
 * @property {_0satsPattern} _1sat
 */

/**
 * @typedef {Object} CatalogTree_Computed_Distribution_AddressCohorts_LtAmount
 * @property {_0satsPattern} _100btc
 * @property {_0satsPattern} _100kBtc
 * @property {_0satsPattern} _100kSats
 * @property {_0satsPattern} _100sats
 * @property {_0satsPattern} _10btc
 * @property {_0satsPattern} _10kBtc
 * @property {_0satsPattern} _10kSats
 * @property {_0satsPattern} _10mSats
 * @property {_0satsPattern} _10sats
 * @property {_0satsPattern} _1btc
 * @property {_0satsPattern} _1kBtc
 * @property {_0satsPattern} _1kSats
 * @property {_0satsPattern} _1mSats
 */

/**
 * @typedef {Object} CatalogTree_Computed_Distribution_AddressesData
 * @property {MetricPattern42<EmptyAddressData>} empty
 * @property {MetricPattern41<LoadedAddressData>} loaded
 */

/**
 * @typedef {Object} CatalogTree_Computed_Distribution_UtxoCohorts
 * @property {CatalogTree_Computed_Distribution_UtxoCohorts_AgeRange} ageRange
 * @property {CatalogTree_Computed_Distribution_UtxoCohorts_All} all
 * @property {CatalogTree_Computed_Distribution_UtxoCohorts_AmountRange} amountRange
 * @property {CatalogTree_Computed_Distribution_UtxoCohorts_Epoch} epoch
 * @property {CatalogTree_Computed_Distribution_UtxoCohorts_GeAmount} geAmount
 * @property {CatalogTree_Computed_Distribution_UtxoCohorts_LtAmount} ltAmount
 * @property {CatalogTree_Computed_Distribution_UtxoCohorts_MaxAge} maxAge
 * @property {CatalogTree_Computed_Distribution_UtxoCohorts_MinAge} minAge
 * @property {CatalogTree_Computed_Distribution_UtxoCohorts_Term} term
 * @property {CatalogTree_Computed_Distribution_UtxoCohorts_Type} type
 * @property {CatalogTree_Computed_Distribution_UtxoCohorts_Year} year
 */

/**
 * @typedef {Object} CatalogTree_Computed_Distribution_UtxoCohorts_AgeRange
 * @property {_10yTo12yPattern} _10yTo12y
 * @property {_10yTo12yPattern} _12yTo15y
 * @property {_10yTo12yPattern} _1dTo1w
 * @property {_10yTo12yPattern} _1hTo1d
 * @property {_10yTo12yPattern} _1mTo2m
 * @property {_10yTo12yPattern} _1wTo1m
 * @property {_10yTo12yPattern} _1yTo2y
 * @property {_10yTo12yPattern} _2mTo3m
 * @property {_10yTo12yPattern} _2yTo3y
 * @property {_10yTo12yPattern} _3mTo4m
 * @property {_10yTo12yPattern} _3yTo4y
 * @property {_10yTo12yPattern} _4mTo5m
 * @property {_10yTo12yPattern} _4yTo5y
 * @property {_10yTo12yPattern} _5mTo6m
 * @property {_10yTo12yPattern} _5yTo6y
 * @property {_10yTo12yPattern} _6mTo1y
 * @property {_10yTo12yPattern} _6yTo7y
 * @property {_10yTo12yPattern} _7yTo8y
 * @property {_10yTo12yPattern} _8yTo10y
 * @property {_10yTo12yPattern} from15y
 * @property {_10yTo12yPattern} upTo1h
 */

/**
 * @typedef {Object} CatalogTree_Computed_Distribution_UtxoCohorts_All
 * @property {ActivityPattern2} activity
 * @property {CostBasisPattern2} costBasis
 * @property {RealizedPattern3} realized
 * @property {CatalogTree_Computed_Distribution_UtxoCohorts_All_Relative} relative
 * @property {SupplyPattern3} supply
 * @property {UnrealizedPattern} unrealized
 */

/**
 * @typedef {Object} CatalogTree_Computed_Distribution_UtxoCohorts_All_Relative
 * @property {MetricPattern3<StoredF32>} negUnrealizedLossRelToOwnTotalUnrealizedPnl
 * @property {MetricPattern3<StoredF32>} netUnrealizedPnlRelToOwnTotalUnrealizedPnl
 * @property {MetricPattern3<StoredF64>} supplyInLossRelToOwnSupply
 * @property {MetricPattern3<StoredF64>} supplyInProfitRelToOwnSupply
 * @property {MetricPattern3<StoredF32>} unrealizedLossRelToOwnTotalUnrealizedPnl
 * @property {MetricPattern3<StoredF32>} unrealizedProfitRelToOwnTotalUnrealizedPnl
 */

/**
 * @typedef {Object} CatalogTree_Computed_Distribution_UtxoCohorts_AmountRange
 * @property {_0satsPattern2} _0sats
 * @property {_0satsPattern2} _100btcTo1kBtc
 * @property {_0satsPattern2} _100kBtcOrMore
 * @property {_0satsPattern2} _100kSatsTo1mSats
 * @property {_0satsPattern2} _100satsTo1kSats
 * @property {_0satsPattern2} _10btcTo100btc
 * @property {_0satsPattern2} _10kBtcTo100kBtc
 * @property {_0satsPattern2} _10kSatsTo100kSats
 * @property {_0satsPattern2} _10mSatsTo1btc
 * @property {_0satsPattern2} _10satsTo100sats
 * @property {_0satsPattern2} _1btcTo10btc
 * @property {_0satsPattern2} _1kBtcTo10kBtc
 * @property {_0satsPattern2} _1kSatsTo10kSats
 * @property {_0satsPattern2} _1mSatsTo10mSats
 * @property {_0satsPattern2} _1satTo10sats
 */

/**
 * @typedef {Object} CatalogTree_Computed_Distribution_UtxoCohorts_Epoch
 * @property {_0satsPattern2} _0
 * @property {_0satsPattern2} _1
 * @property {_0satsPattern2} _2
 * @property {_0satsPattern2} _3
 * @property {_0satsPattern2} _4
 */

/**
 * @typedef {Object} CatalogTree_Computed_Distribution_UtxoCohorts_GeAmount
 * @property {_100btcPattern} _100btc
 * @property {_100btcPattern} _100kSats
 * @property {_100btcPattern} _100sats
 * @property {_100btcPattern} _10btc
 * @property {_100btcPattern} _10kBtc
 * @property {_100btcPattern} _10kSats
 * @property {_100btcPattern} _10mSats
 * @property {_100btcPattern} _10sats
 * @property {_100btcPattern} _1btc
 * @property {_100btcPattern} _1kBtc
 * @property {_100btcPattern} _1kSats
 * @property {_100btcPattern} _1mSats
 * @property {_100btcPattern} _1sat
 */

/**
 * @typedef {Object} CatalogTree_Computed_Distribution_UtxoCohorts_LtAmount
 * @property {_100btcPattern} _100btc
 * @property {_100btcPattern} _100kBtc
 * @property {_100btcPattern} _100kSats
 * @property {_100btcPattern} _100sats
 * @property {_100btcPattern} _10btc
 * @property {_100btcPattern} _10kBtc
 * @property {_100btcPattern} _10kSats
 * @property {_100btcPattern} _10mSats
 * @property {_100btcPattern} _10sats
 * @property {_100btcPattern} _1btc
 * @property {_100btcPattern} _1kBtc
 * @property {_100btcPattern} _1kSats
 * @property {_100btcPattern} _1mSats
 */

/**
 * @typedef {Object} CatalogTree_Computed_Distribution_UtxoCohorts_MaxAge
 * @property {_10yPattern} _10y
 * @property {_10yPattern} _12y
 * @property {_10yPattern} _15y
 * @property {_10yPattern} _1m
 * @property {_10yPattern} _1w
 * @property {_10yPattern} _1y
 * @property {_10yPattern} _2m
 * @property {_10yPattern} _2y
 * @property {_10yPattern} _3m
 * @property {_10yPattern} _3y
 * @property {_10yPattern} _4m
 * @property {_10yPattern} _4y
 * @property {_10yPattern} _5m
 * @property {_10yPattern} _5y
 * @property {_10yPattern} _6m
 * @property {_10yPattern} _6y
 * @property {_10yPattern} _7y
 * @property {_10yPattern} _8y
 */

/**
 * @typedef {Object} CatalogTree_Computed_Distribution_UtxoCohorts_MinAge
 * @property {_100btcPattern} _10y
 * @property {_100btcPattern} _12y
 * @property {_100btcPattern} _1d
 * @property {_100btcPattern} _1m
 * @property {_100btcPattern} _1w
 * @property {_100btcPattern} _1y
 * @property {_100btcPattern} _2m
 * @property {_100btcPattern} _2y
 * @property {_100btcPattern} _3m
 * @property {_100btcPattern} _3y
 * @property {_100btcPattern} _4m
 * @property {_100btcPattern} _4y
 * @property {_100btcPattern} _5m
 * @property {_100btcPattern} _5y
 * @property {_100btcPattern} _6m
 * @property {_100btcPattern} _6y
 * @property {_100btcPattern} _7y
 * @property {_100btcPattern} _8y
 */

/**
 * @typedef {Object} CatalogTree_Computed_Distribution_UtxoCohorts_Term
 * @property {CatalogTree_Computed_Distribution_UtxoCohorts_Term_Long} long
 * @property {CatalogTree_Computed_Distribution_UtxoCohorts_Term_Short} short
 */

/**
 * @typedef {Object} CatalogTree_Computed_Distribution_UtxoCohorts_Term_Long
 * @property {ActivityPattern2} activity
 * @property {CostBasisPattern2} costBasis
 * @property {RealizedPattern2} realized
 * @property {RelativePattern5} relative
 * @property {SupplyPattern3} supply
 * @property {UnrealizedPattern} unrealized
 */

/**
 * @typedef {Object} CatalogTree_Computed_Distribution_UtxoCohorts_Term_Short
 * @property {ActivityPattern2} activity
 * @property {CostBasisPattern2} costBasis
 * @property {RealizedPattern3} realized
 * @property {RelativePattern5} relative
 * @property {SupplyPattern3} supply
 * @property {UnrealizedPattern} unrealized
 */

/**
 * @typedef {Object} CatalogTree_Computed_Distribution_UtxoCohorts_Type
 * @property {_0satsPattern2} empty
 * @property {_0satsPattern2} p2a
 * @property {_0satsPattern2} p2ms
 * @property {_0satsPattern2} p2pk33
 * @property {_0satsPattern2} p2pk65
 * @property {_0satsPattern2} p2pkh
 * @property {_0satsPattern2} p2sh
 * @property {_0satsPattern2} p2tr
 * @property {_0satsPattern2} p2wpkh
 * @property {_0satsPattern2} p2wsh
 * @property {_0satsPattern2} unknown
 */

/**
 * @typedef {Object} CatalogTree_Computed_Distribution_UtxoCohorts_Year
 * @property {_0satsPattern2} _2009
 * @property {_0satsPattern2} _2010
 * @property {_0satsPattern2} _2011
 * @property {_0satsPattern2} _2012
 * @property {_0satsPattern2} _2013
 * @property {_0satsPattern2} _2014
 * @property {_0satsPattern2} _2015
 * @property {_0satsPattern2} _2016
 * @property {_0satsPattern2} _2017
 * @property {_0satsPattern2} _2018
 * @property {_0satsPattern2} _2019
 * @property {_0satsPattern2} _2020
 * @property {_0satsPattern2} _2021
 * @property {_0satsPattern2} _2022
 * @property {_0satsPattern2} _2023
 * @property {_0satsPattern2} _2024
 * @property {_0satsPattern2} _2025
 * @property {_0satsPattern2} _2026
 */

/**
 * @typedef {Object} CatalogTree_Computed_Indexes
 * @property {CatalogTree_Computed_Indexes_Address} address
 * @property {CatalogTree_Computed_Indexes_Block} block
 * @property {CatalogTree_Computed_Indexes_Time} time
 * @property {CatalogTree_Computed_Indexes_Transaction} transaction
 */

/**
 * @typedef {Object} CatalogTree_Computed_Indexes_Address
 * @property {MetricPattern25<EmptyOutputIndex>} emptyoutputindex
 * @property {MetricPattern28<OpReturnIndex>} opreturnindex
 * @property {MetricPattern30<P2AAddressIndex>} p2aaddressindex
 * @property {MetricPattern31<P2MSOutputIndex>} p2msoutputindex
 * @property {MetricPattern32<P2PK33AddressIndex>} p2pk33addressindex
 * @property {MetricPattern33<P2PK65AddressIndex>} p2pk65addressindex
 * @property {MetricPattern34<P2PKHAddressIndex>} p2pkhaddressindex
 * @property {MetricPattern35<P2SHAddressIndex>} p2shaddressindex
 * @property {MetricPattern36<P2TRAddressIndex>} p2traddressindex
 * @property {MetricPattern37<P2WPKHAddressIndex>} p2wpkhaddressindex
 * @property {MetricPattern38<P2WSHAddressIndex>} p2wshaddressindex
 * @property {MetricPattern40<UnknownOutputIndex>} unknownoutputindex
 */

/**
 * @typedef {Object} CatalogTree_Computed_Indexes_Block
 * @property {MetricPattern26<DateIndex>} dateindex
 * @property {MetricPattern15<DifficultyEpoch>} difficultyepoch
 * @property {MetricPattern14<Height>} firstHeight
 * @property {MetricPattern16<HalvingEpoch>} halvingepoch
 * @property {MetricPattern26<Height>} height
 * @property {MetricPattern24<StoredU64>} heightCount
 * @property {MetricPattern26<StoredU64>} txindexCount
 */

/**
 * @typedef {Object} CatalogTree_Computed_Indexes_Time
 * @property {MetricPattern22<Date>} date
 * @property {MetricPattern22<DateIndex>} dateindex
 * @property {MetricPattern20<StoredU64>} dateindexCount
 * @property {MetricPattern13<DecadeIndex>} decadeindex
 * @property {MetricPattern20<DateIndex>} firstDateindex
 * @property {MetricPattern22<Height>} firstHeight
 * @property {MetricPattern9<MonthIndex>} firstMonthindex
 * @property {MetricPattern23<YearIndex>} firstYearindex
 * @property {MetricPattern22<StoredU64>} heightCount
 * @property {MetricPattern11<MonthIndex>} monthindex
 * @property {MetricPattern9<StoredU64>} monthindexCount
 * @property {MetricPattern18<QuarterIndex>} quarterindex
 * @property {MetricPattern19<SemesterIndex>} semesterindex
 * @property {MetricPattern12<WeekIndex>} weekindex
 * @property {MetricPattern21<YearIndex>} yearindex
 * @property {MetricPattern23<StoredU64>} yearindexCount
 */

/**
 * @typedef {Object} CatalogTree_Computed_Indexes_Transaction
 * @property {MetricPattern39<StoredU64>} inputCount
 * @property {MetricPattern39<StoredU64>} outputCount
 * @property {MetricPattern39<TxIndex>} txindex
 * @property {MetricPattern27<TxInIndex>} txinindex
 * @property {MetricPattern29<TxOutIndex>} txoutindex
 */

/**
 * @typedef {Object} CatalogTree_Computed_Inputs
 * @property {CatalogTree_Computed_Inputs_Count} count
 * @property {CatalogTree_Computed_Inputs_Spent} spent
 */

/**
 * @typedef {Object} CatalogTree_Computed_Inputs_Count
 * @property {CountPattern2<StoredU64>} count
 */

/**
 * @typedef {Object} CatalogTree_Computed_Inputs_Spent
 * @property {MetricPattern27<TxOutIndex>} txoutindex
 * @property {MetricPattern27<Sats>} value
 */

/**
 * @typedef {Object} CatalogTree_Computed_Market
 * @property {CatalogTree_Computed_Market_Ath} ath
 * @property {CatalogTree_Computed_Market_Dca} dca
 * @property {CatalogTree_Computed_Market_Indicators} indicators
 * @property {CatalogTree_Computed_Market_Lookback} lookback
 * @property {CatalogTree_Computed_Market_MovingAverage} movingAverage
 * @property {CatalogTree_Computed_Market_Range} range
 * @property {CatalogTree_Computed_Market_Returns} returns
 * @property {CatalogTree_Computed_Market_Volatility} volatility
 */

/**
 * @typedef {Object} CatalogTree_Computed_Market_Ath
 * @property {MetricPattern5<StoredU16>} daysSincePriceAth
 * @property {MetricPattern5<StoredU16>} maxDaysBetweenPriceAths
 * @property {MetricPattern5<StoredF32>} maxYearsBetweenPriceAths
 * @property {MetricPattern3<Dollars>} priceAth
 * @property {MetricPattern3<StoredF32>} priceDrawdown
 * @property {MetricPattern5<StoredF32>} yearsSincePriceAth
 */

/**
 * @typedef {Object} CatalogTree_Computed_Market_Dca
 * @property {ClassAveragePricePattern<Dollars>} classAveragePrice
 * @property {ClassAveragePricePattern<StoredF32>} classReturns
 * @property {CatalogTree_Computed_Market_Dca_ClassStack} classStack
 * @property {PeriodAveragePricePattern<Dollars>} periodAveragePrice
 * @property {PeriodCagrPattern} periodCagr
 * @property {PeriodLumpSumStackPattern} periodLumpSumStack
 * @property {PeriodAveragePricePattern<StoredF32>} periodReturns
 * @property {PeriodLumpSumStackPattern} periodStack
 */

/**
 * @typedef {Object} CatalogTree_Computed_Market_Dca_ClassStack
 * @property {ActiveSupplyPattern} _2015
 * @property {ActiveSupplyPattern} _2016
 * @property {ActiveSupplyPattern} _2017
 * @property {ActiveSupplyPattern} _2018
 * @property {ActiveSupplyPattern} _2019
 * @property {ActiveSupplyPattern} _2020
 * @property {ActiveSupplyPattern} _2021
 * @property {ActiveSupplyPattern} _2022
 * @property {ActiveSupplyPattern} _2023
 * @property {ActiveSupplyPattern} _2024
 * @property {ActiveSupplyPattern} _2025
 */

/**
 * @typedef {Object} CatalogTree_Computed_Market_Indicators
 * @property {MetricPattern22<StoredF32>} gini
 * @property {MetricPattern22<StoredF32>} macdHistogram
 * @property {MetricPattern22<StoredF32>} macdLine
 * @property {MetricPattern22<StoredF32>} macdSignal
 * @property {MetricPattern5<StoredF32>} nvt
 * @property {MetricPattern22<StoredF32>} piCycle
 * @property {MetricPattern5<StoredF32>} puellMultiple
 * @property {MetricPattern22<StoredF32>} rsi14d
 * @property {MetricPattern22<StoredF32>} rsi14dMax
 * @property {MetricPattern22<StoredF32>} rsi14dMin
 * @property {MetricPattern22<StoredF32>} rsiAverageGain14d
 * @property {MetricPattern22<StoredF32>} rsiAverageLoss14d
 * @property {MetricPattern22<StoredF32>} rsiGains
 * @property {MetricPattern22<StoredF32>} rsiLosses
 * @property {MetricPattern22<StoredF32>} stochD
 * @property {MetricPattern22<StoredF32>} stochK
 * @property {MetricPattern22<StoredF32>} stochRsi
 * @property {MetricPattern22<StoredF32>} stochRsiD
 * @property {MetricPattern22<StoredF32>} stochRsiK
 */

/**
 * @typedef {Object} CatalogTree_Computed_Market_Lookback
 * @property {PriceAgoPattern<Dollars>} priceAgo
 */

/**
 * @typedef {Object} CatalogTree_Computed_Market_MovingAverage
 * @property {Price111dSmaPattern} price111dSma
 * @property {Price111dSmaPattern} price12dEma
 * @property {Price111dSmaPattern} price13dEma
 * @property {Price111dSmaPattern} price13dSma
 * @property {Price111dSmaPattern} price144dEma
 * @property {Price111dSmaPattern} price144dSma
 * @property {Price111dSmaPattern} price1mEma
 * @property {Price111dSmaPattern} price1mSma
 * @property {Price111dSmaPattern} price1wEma
 * @property {Price111dSmaPattern} price1wSma
 * @property {Price111dSmaPattern} price1yEma
 * @property {Price111dSmaPattern} price1ySma
 * @property {Price111dSmaPattern} price200dEma
 * @property {Price111dSmaPattern} price200dSma
 * @property {MetricPattern5<Dollars>} price200dSmaX08
 * @property {MetricPattern5<Dollars>} price200dSmaX24
 * @property {Price111dSmaPattern} price200wEma
 * @property {Price111dSmaPattern} price200wSma
 * @property {Price111dSmaPattern} price21dEma
 * @property {Price111dSmaPattern} price21dSma
 * @property {Price111dSmaPattern} price26dEma
 * @property {Price111dSmaPattern} price2yEma
 * @property {Price111dSmaPattern} price2ySma
 * @property {Price111dSmaPattern} price34dEma
 * @property {Price111dSmaPattern} price34dSma
 * @property {Price111dSmaPattern} price350dSma
 * @property {MetricPattern5<Dollars>} price350dSmaX2
 * @property {Price111dSmaPattern} price4yEma
 * @property {Price111dSmaPattern} price4ySma
 * @property {Price111dSmaPattern} price55dEma
 * @property {Price111dSmaPattern} price55dSma
 * @property {Price111dSmaPattern} price89dEma
 * @property {Price111dSmaPattern} price89dSma
 * @property {Price111dSmaPattern} price8dEma
 * @property {Price111dSmaPattern} price8dSma
 */

/**
 * @typedef {Object} CatalogTree_Computed_Market_Range
 * @property {MetricPattern5<Dollars>} price1mMax
 * @property {MetricPattern5<Dollars>} price1mMin
 * @property {MetricPattern5<Dollars>} price1wMax
 * @property {MetricPattern5<Dollars>} price1wMin
 * @property {MetricPattern5<Dollars>} price1yMax
 * @property {MetricPattern5<Dollars>} price1yMin
 * @property {MetricPattern5<StoredF32>} price2wChoppinessIndex
 * @property {MetricPattern5<Dollars>} price2wMax
 * @property {MetricPattern5<Dollars>} price2wMin
 * @property {MetricPattern22<StoredF32>} priceTrueRange
 * @property {MetricPattern22<StoredF32>} priceTrueRange2wSum
 */

/**
 * @typedef {Object} CatalogTree_Computed_Market_Returns
 * @property {_1dReturns1mSdPattern} _1dReturns1mSd
 * @property {_1dReturns1mSdPattern} _1dReturns1wSd
 * @property {_1dReturns1mSdPattern} _1dReturns1ySd
 * @property {PeriodCagrPattern} cagr
 * @property {_1dReturns1mSdPattern} downside1mSd
 * @property {_1dReturns1mSdPattern} downside1wSd
 * @property {_1dReturns1mSdPattern} downside1ySd
 * @property {MetricPattern22<StoredF32>} downsideReturns
 * @property {PriceAgoPattern<StoredF32>} priceReturns
 */

/**
 * @typedef {Object} CatalogTree_Computed_Market_Volatility
 * @property {MetricPattern5<StoredF32>} price1mVolatility
 * @property {MetricPattern5<StoredF32>} price1wVolatility
 * @property {MetricPattern5<StoredF32>} price1yVolatility
 * @property {MetricPattern22<StoredF32>} sharpe1m
 * @property {MetricPattern22<StoredF32>} sharpe1w
 * @property {MetricPattern22<StoredF32>} sharpe1y
 * @property {MetricPattern22<StoredF32>} sortino1m
 * @property {MetricPattern22<StoredF32>} sortino1w
 * @property {MetricPattern22<StoredF32>} sortino1y
 */

/**
 * @typedef {Object} CatalogTree_Computed_Outputs
 * @property {CatalogTree_Computed_Outputs_Count} count
 * @property {CatalogTree_Computed_Outputs_Spent} spent
 */

/**
 * @typedef {Object} CatalogTree_Computed_Outputs_Count
 * @property {CountPattern2<StoredU64>} count
 * @property {DollarsPattern<StoredU64>} utxoCount
 */

/**
 * @typedef {Object} CatalogTree_Computed_Outputs_Spent
 * @property {MetricPattern29<TxInIndex>} txinindex
 */

/**
 * @typedef {Object} CatalogTree_Computed_Pools
 * @property {MetricPattern26<PoolSlug>} pool
 * @property {CatalogTree_Computed_Pools_Vecs} vecs
 */

/**
 * @typedef {Object} CatalogTree_Computed_Pools_Vecs
 * @property {AXbtPattern} aXbt
 * @property {AXbtPattern} aaoPool
 * @property {AXbtPattern} antPool
 * @property {AXbtPattern} arkPool
 * @property {AXbtPattern} asicMiner
 * @property {AXbtPattern} batPool
 * @property {AXbtPattern} bcMonster
 * @property {AXbtPattern} bcpoolIo
 * @property {AXbtPattern} binancePool
 * @property {AXbtPattern} bitClub
 * @property {AXbtPattern} bitFuFuPool
 * @property {AXbtPattern} bitFury
 * @property {AXbtPattern} bitMinter
 * @property {AXbtPattern} bitalo
 * @property {AXbtPattern} bitcoinAffiliateNetwork
 * @property {AXbtPattern} bitcoinCom
 * @property {AXbtPattern} bitcoinIndia
 * @property {AXbtPattern} bitcoinRussia
 * @property {AXbtPattern} bitcoinUkraine
 * @property {AXbtPattern} bitfarms
 * @property {AXbtPattern} bitparking
 * @property {AXbtPattern} bitsolo
 * @property {AXbtPattern} bixin
 * @property {AXbtPattern} blockFills
 * @property {AXbtPattern} braiinsPool
 * @property {AXbtPattern} bravoMining
 * @property {AXbtPattern} btPool
 * @property {AXbtPattern} btcCom
 * @property {AXbtPattern} btcDig
 * @property {AXbtPattern} btcGuild
 * @property {AXbtPattern} btcLab
 * @property {AXbtPattern} btcMp
 * @property {AXbtPattern} btcNuggets
 * @property {AXbtPattern} btcPoolParty
 * @property {AXbtPattern} btcServ
 * @property {AXbtPattern} btcTop
 * @property {AXbtPattern} btcc
 * @property {AXbtPattern} bwPool
 * @property {AXbtPattern} bytePool
 * @property {AXbtPattern} canoe
 * @property {AXbtPattern} canoePool
 * @property {AXbtPattern} carbonNegative
 * @property {AXbtPattern} ckPool
 * @property {AXbtPattern} cloudHashing
 * @property {AXbtPattern} coinLab
 * @property {AXbtPattern} cointerra
 * @property {AXbtPattern} connectBtc
 * @property {AXbtPattern} dPool
 * @property {AXbtPattern} dcExploration
 * @property {AXbtPattern} dcex
 * @property {AXbtPattern} digitalBtc
 * @property {AXbtPattern} digitalXMintsy
 * @property {AXbtPattern} eclipseMc
 * @property {AXbtPattern} eightBaochi
 * @property {AXbtPattern} ekanemBtc
 * @property {AXbtPattern} eligius
 * @property {AXbtPattern} emcdPool
 * @property {AXbtPattern} entrustCharityPool
 * @property {AXbtPattern} eobot
 * @property {AXbtPattern} exxBw
 * @property {AXbtPattern} f2Pool
 * @property {AXbtPattern} fiftyEightCoin
 * @property {AXbtPattern} foundryUsa
 * @property {AXbtPattern} futureBitApolloSolo
 * @property {AXbtPattern} gbMiners
 * @property {AXbtPattern} ghashIo
 * @property {AXbtPattern} giveMeCoins
 * @property {AXbtPattern} goGreenLight
 * @property {AXbtPattern} haoZhuZhu
 * @property {AXbtPattern} haominer
 * @property {AXbtPattern} hashBx
 * @property {AXbtPattern} hashPool
 * @property {AXbtPattern} helix
 * @property {AXbtPattern} hhtt
 * @property {AXbtPattern} hotPool
 * @property {AXbtPattern} hummerpool
 * @property {AXbtPattern} huobiPool
 * @property {AXbtPattern} innopolisTech
 * @property {AXbtPattern} kanoPool
 * @property {AXbtPattern} kncMiner
 * @property {AXbtPattern} kuCoinPool
 * @property {AXbtPattern} lubianCom
 * @property {AXbtPattern} luckyPool
 * @property {AXbtPattern} luxor
 * @property {AXbtPattern} maraPool
 * @property {AXbtPattern} maxBtc
 * @property {AXbtPattern} maxiPool
 * @property {AXbtPattern} megaBigPower
 * @property {AXbtPattern} minerium
 * @property {AXbtPattern} miningCity
 * @property {AXbtPattern} miningDutch
 * @property {AXbtPattern} miningKings
 * @property {AXbtPattern} miningSquared
 * @property {AXbtPattern} mmpool
 * @property {AXbtPattern} mtRed
 * @property {AXbtPattern} multiCoinCo
 * @property {AXbtPattern} multipool
 * @property {AXbtPattern} myBtcCoinPool
 * @property {AXbtPattern} neopool
 * @property {AXbtPattern} nexious
 * @property {AXbtPattern} niceHash
 * @property {AXbtPattern} nmcBit
 * @property {AXbtPattern} novaBlock
 * @property {AXbtPattern} ocean
 * @property {AXbtPattern} okExPool
 * @property {AXbtPattern} okMiner
 * @property {AXbtPattern} okkong
 * @property {AXbtPattern} okpoolTop
 * @property {AXbtPattern} oneHash
 * @property {AXbtPattern} oneM1x
 * @property {AXbtPattern} oneThash
 * @property {AXbtPattern} ozCoin
 * @property {AXbtPattern} pHashIo
 * @property {AXbtPattern} parasite
 * @property {AXbtPattern} patels
 * @property {AXbtPattern} pegaPool
 * @property {AXbtPattern} phoenix
 * @property {AXbtPattern} polmine
 * @property {AXbtPattern} pool175btc
 * @property {AXbtPattern} pool50btc
 * @property {AXbtPattern} poolin
 * @property {AXbtPattern} portlandHodl
 * @property {AXbtPattern} publicPool
 * @property {AXbtPattern} pureBtcCom
 * @property {AXbtPattern} rawpool
 * @property {AXbtPattern} rigPool
 * @property {AXbtPattern} sbiCrypto
 * @property {AXbtPattern} secPool
 * @property {AXbtPattern} secretSuperstar
 * @property {AXbtPattern} sevenPool
 * @property {AXbtPattern} shawnP0wers
 * @property {AXbtPattern} sigmapoolCom
 * @property {AXbtPattern} simplecoinUs
 * @property {AXbtPattern} soloCk
 * @property {AXbtPattern} spiderPool
 * @property {AXbtPattern} stMiningCorp
 * @property {AXbtPattern} tangpool
 * @property {AXbtPattern} tatmasPool
 * @property {AXbtPattern} tbDice
 * @property {AXbtPattern} telco214
 * @property {AXbtPattern} terraPool
 * @property {AXbtPattern} tiger
 * @property {AXbtPattern} tigerpoolNet
 * @property {AXbtPattern} titan
 * @property {AXbtPattern} transactionCoinMining
 * @property {AXbtPattern} trickysBtcPool
 * @property {AXbtPattern} tripleMining
 * @property {AXbtPattern} twentyOneInc
 * @property {AXbtPattern} ultimusPool
 * @property {AXbtPattern} unknown
 * @property {AXbtPattern} unomp
 * @property {AXbtPattern} viaBtc
 * @property {AXbtPattern} waterhole
 * @property {AXbtPattern} wayiCn
 * @property {AXbtPattern} whitePool
 * @property {AXbtPattern} wk057
 * @property {AXbtPattern} yourbtcNet
 * @property {AXbtPattern} zulupool
 */

/**
 * @typedef {Object} CatalogTree_Computed_Positions
 * @property {MetricPattern17<BlkPosition>} position
 */

/**
 * @typedef {Object} CatalogTree_Computed_Price
 * @property {CatalogTree_Computed_Price_Ohlc} ohlc
 * @property {CatalogTree_Computed_Price_Sats} sats
 * @property {CatalogTree_Computed_Price_Usd} usd
 */

/**
 * @typedef {Object} CatalogTree_Computed_Price_Ohlc
 * @property {MetricPattern10<OHLCCents>} ohlcInCents
 */

/**
 * @typedef {Object} CatalogTree_Computed_Price_Sats
 * @property {MetricPattern1<Sats>} priceCloseInSats
 * @property {PriceHighInSatsPattern<Sats>} priceHighInSats
 * @property {PriceLowInSatsPattern<Sats>} priceLowInSats
 * @property {MetricPattern1<OHLCSats>} priceOhlcInSats
 * @property {MetricPattern1<Sats>} priceOpenInSats
 */

/**
 * @typedef {Object} CatalogTree_Computed_Price_Usd
 * @property {MetricPattern1<Dollars>} priceClose
 * @property {MetricPattern10<Cents>} priceCloseInCents
 * @property {PriceHighInSatsPattern<Dollars>} priceHigh
 * @property {MetricPattern10<Cents>} priceHighInCents
 * @property {PriceLowInSatsPattern<Dollars>} priceLow
 * @property {MetricPattern10<Cents>} priceLowInCents
 * @property {MetricPattern1<OHLCDollars>} priceOhlc
 * @property {MetricPattern1<Dollars>} priceOpen
 * @property {MetricPattern10<Cents>} priceOpenInCents
 */

/**
 * @typedef {Object} CatalogTree_Computed_Scripts
 * @property {CatalogTree_Computed_Scripts_Count} count
 * @property {CatalogTree_Computed_Scripts_Value} value
 */

/**
 * @typedef {Object} CatalogTree_Computed_Scripts_Count
 * @property {DollarsPattern<StoredU64>} emptyoutputCount
 * @property {DollarsPattern<StoredU64>} opreturnCount
 * @property {DollarsPattern<StoredU64>} p2aCount
 * @property {DollarsPattern<StoredU64>} p2msCount
 * @property {DollarsPattern<StoredU64>} p2pk33Count
 * @property {DollarsPattern<StoredU64>} p2pk65Count
 * @property {DollarsPattern<StoredU64>} p2pkhCount
 * @property {DollarsPattern<StoredU64>} p2shCount
 * @property {DollarsPattern<StoredU64>} p2trCount
 * @property {DollarsPattern<StoredU64>} p2wpkhCount
 * @property {DollarsPattern<StoredU64>} p2wshCount
 * @property {BlockCountPattern<StoredF32>} segwitAdoption
 * @property {DollarsPattern<StoredU64>} segwitCount
 * @property {BlockCountPattern<StoredF32>} taprootAdoption
 * @property {DollarsPattern<StoredU64>} unknownoutputCount
 */

/**
 * @typedef {Object} CatalogTree_Computed_Scripts_Value
 * @property {CoinbasePattern} opreturnValue
 */

/**
 * @typedef {Object} CatalogTree_Computed_Supply
 * @property {CatalogTree_Computed_Supply_Burned} burned
 * @property {CatalogTree_Computed_Supply_Circulating} circulating
 * @property {CatalogTree_Computed_Supply_Inflation} inflation
 * @property {CatalogTree_Computed_Supply_MarketCap} marketCap
 * @property {CatalogTree_Computed_Supply_Velocity} velocity
 */

/**
 * @typedef {Object} CatalogTree_Computed_Supply_Burned
 * @property {UnclaimedRewardsPattern} opreturn
 * @property {UnclaimedRewardsPattern} unspendable
 */

/**
 * @typedef {Object} CatalogTree_Computed_Supply_Circulating
 * @property {MetricPattern26<Bitcoin>} btc
 * @property {ActiveSupplyPattern} indexes
 * @property {MetricPattern26<Sats>} sats
 * @property {MetricPattern26<Dollars>} usd
 */

/**
 * @typedef {Object} CatalogTree_Computed_Supply_Inflation
 * @property {IndexesPattern2<StoredF32>} indexes
 */

/**
 * @typedef {Object} CatalogTree_Computed_Supply_MarketCap
 * @property {MetricPattern26<Dollars>} height
 * @property {MetricPattern5<Dollars>} indexes
 */

/**
 * @typedef {Object} CatalogTree_Computed_Supply_Velocity
 * @property {IndexesPattern2<StoredF64>} btc
 * @property {IndexesPattern2<StoredF64>} usd
 */

/**
 * @typedef {Object} CatalogTree_Computed_Transactions
 * @property {CatalogTree_Computed_Transactions_Count} count
 * @property {CatalogTree_Computed_Transactions_Fees} fees
 * @property {CatalogTree_Computed_Transactions_Size} size
 * @property {CatalogTree_Computed_Transactions_Versions} versions
 * @property {CatalogTree_Computed_Transactions_Volume} volume
 */

/**
 * @typedef {Object} CatalogTree_Computed_Transactions_Count
 * @property {MetricPattern39<StoredBool>} isCoinbase
 * @property {DollarsPattern<StoredU64>} txCount
 */

/**
 * @typedef {Object} CatalogTree_Computed_Transactions_Fees
 * @property {CatalogTree_Computed_Transactions_Fees_Fee} fee
 * @property {CatalogTree_Computed_Transactions_Fees_FeeRate} feeRate
 * @property {MetricPattern39<Sats>} inputValue
 * @property {MetricPattern39<Sats>} outputValue
 */

/**
 * @typedef {Object} CatalogTree_Computed_Transactions_Fees_Fee
 * @property {MetricPattern39<Sats>} base
 * @property {CatalogTree_Computed_Transactions_Fees_Fee_Bitcoin} bitcoin
 * @property {CountPattern2<Dollars>} dollars
 * @property {CountPattern2<Sats>} sats
 */

/**
 * @typedef {Object} CatalogTree_Computed_Transactions_Fees_Fee_Bitcoin
 * @property {MetricPattern1<Bitcoin>} average
 * @property {MetricPattern1<Bitcoin>} cumulative
 * @property {MetricPattern1<Bitcoin>} max
 * @property {MetricPattern1<Bitcoin>} min
 * @property {MetricPattern1<Bitcoin>} sum
 */

/**
 * @typedef {Object} CatalogTree_Computed_Transactions_Fees_FeeRate
 * @property {MetricPattern1<FeeRate>} average
 * @property {MetricPattern39<FeeRate>} base
 * @property {MetricPattern1<FeeRate>} max
 * @property {MetricPattern1<FeeRate>} min
 * @property {PercentilesPattern<FeeRate>} percentiles
 */

/**
 * @typedef {Object} CatalogTree_Computed_Transactions_Size
 * @property {BlockIntervalPattern<VSize>} txVsize
 * @property {BlockIntervalPattern<Weight>} txWeight
 * @property {MetricPattern39<VSize>} vsize
 * @property {MetricPattern39<Weight>} weight
 */

/**
 * @typedef {Object} CatalogTree_Computed_Transactions_Versions
 * @property {BlockCountPattern<StoredU64>} txV1
 * @property {BlockCountPattern<StoredU64>} txV2
 * @property {BlockCountPattern<StoredU64>} txV3
 */

/**
 * @typedef {Object} CatalogTree_Computed_Transactions_Volume
 * @property {MetricPattern5<Sats>} annualizedVolume
 * @property {MetricPattern5<Bitcoin>} annualizedVolumeBtc
 * @property {MetricPattern5<Dollars>} annualizedVolumeUsd
 * @property {MetricPattern5<StoredF32>} inputsPerSec
 * @property {MetricPattern5<StoredF32>} outputsPerSec
 * @property {CatalogTree_Computed_Transactions_Volume_SentSum} sentSum
 * @property {MetricPattern5<StoredF32>} txPerSec
 */

/**
 * @typedef {Object} CatalogTree_Computed_Transactions_Volume_SentSum
 * @property {MetricPattern1<Bitcoin>} bitcoin
 * @property {DifficultyAdjustmentPattern<Dollars>} dollars
 * @property {DifficultyAdjustmentPattern<Sats>} sats
 */

/**
 * @typedef {Object} CatalogTree_Indexed
 * @property {CatalogTree_Indexed_Address} address
 * @property {CatalogTree_Indexed_Block} block
 * @property {CatalogTree_Indexed_Output} output
 * @property {CatalogTree_Indexed_Tx} tx
 * @property {CatalogTree_Indexed_Txin} txin
 * @property {CatalogTree_Indexed_Txout} txout
 */

/**
 * @typedef {Object} CatalogTree_Indexed_Address
 * @property {MetricPattern26<P2AAddressIndex>} firstP2aaddressindex
 * @property {MetricPattern26<P2PK33AddressIndex>} firstP2pk33addressindex
 * @property {MetricPattern26<P2PK65AddressIndex>} firstP2pk65addressindex
 * @property {MetricPattern26<P2PKHAddressIndex>} firstP2pkhaddressindex
 * @property {MetricPattern26<P2SHAddressIndex>} firstP2shaddressindex
 * @property {MetricPattern26<P2TRAddressIndex>} firstP2traddressindex
 * @property {MetricPattern26<P2WPKHAddressIndex>} firstP2wpkhaddressindex
 * @property {MetricPattern26<P2WSHAddressIndex>} firstP2wshaddressindex
 * @property {MetricPattern30<P2ABytes>} p2abytes
 * @property {MetricPattern32<P2PK33Bytes>} p2pk33bytes
 * @property {MetricPattern33<P2PK65Bytes>} p2pk65bytes
 * @property {MetricPattern34<P2PKHBytes>} p2pkhbytes
 * @property {MetricPattern35<P2SHBytes>} p2shbytes
 * @property {MetricPattern36<P2TRBytes>} p2trbytes
 * @property {MetricPattern37<P2WPKHBytes>} p2wpkhbytes
 * @property {MetricPattern38<P2WSHBytes>} p2wshbytes
 */

/**
 * @typedef {Object} CatalogTree_Indexed_Block
 * @property {MetricPattern26<BlockHash>} blockhash
 * @property {MetricPattern26<StoredF64>} difficulty
 * @property {MetricPattern26<Timestamp>} timestamp
 * @property {MetricPattern26<StoredU64>} totalSize
 * @property {MetricPattern26<Weight>} weight
 */

/**
 * @typedef {Object} CatalogTree_Indexed_Output
 * @property {MetricPattern26<EmptyOutputIndex>} firstEmptyoutputindex
 * @property {MetricPattern26<OpReturnIndex>} firstOpreturnindex
 * @property {MetricPattern26<P2MSOutputIndex>} firstP2msoutputindex
 * @property {MetricPattern26<UnknownOutputIndex>} firstUnknownoutputindex
 * @property {MetricPattern8<TxIndex>} txindex
 */

/**
 * @typedef {Object} CatalogTree_Indexed_Tx
 * @property {MetricPattern39<StoredU32>} baseSize
 * @property {MetricPattern26<TxIndex>} firstTxindex
 * @property {MetricPattern39<TxInIndex>} firstTxinindex
 * @property {MetricPattern39<TxOutIndex>} firstTxoutindex
 * @property {MetricPattern39<Height>} height
 * @property {MetricPattern39<StoredBool>} isExplicitlyRbf
 * @property {MetricPattern39<RawLockTime>} rawlocktime
 * @property {MetricPattern39<StoredU32>} totalSize
 * @property {MetricPattern39<Txid>} txid
 * @property {MetricPattern39<TxVersion>} txversion
 */

/**
 * @typedef {Object} CatalogTree_Indexed_Txin
 * @property {MetricPattern26<TxInIndex>} firstTxinindex
 * @property {MetricPattern27<OutPoint>} outpoint
 * @property {MetricPattern27<OutputType>} outputtype
 * @property {MetricPattern27<TxIndex>} txindex
 * @property {MetricPattern27<TypeIndex>} typeindex
 */

/**
 * @typedef {Object} CatalogTree_Indexed_Txout
 * @property {MetricPattern26<TxOutIndex>} firstTxoutindex
 * @property {MetricPattern29<OutputType>} outputtype
 * @property {MetricPattern29<TxIndex>} txindex
 * @property {MetricPattern29<TypeIndex>} typeindex
 * @property {MetricPattern29<Sats>} value
 */

/**
 * Main BRK client with catalog tree and API methods
 * @extends BrkClientBase
 */
class BrkClient extends BrkClientBase {
  VERSION = "v0.1.0-alpha.2";

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
    "loadedaddressindex",
    "emptyaddressindex"
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
      "id": "up_to_1h_old",
      "short": "<1h",
      "long": "Up to 1 Hour Old"
    },
    "_1hTo1d": {
      "id": "at_least_1h_up_to_1d_old",
      "short": "1h-1d",
      "long": "1 Hour to 1 Day Old"
    },
    "_1dTo1w": {
      "id": "at_least_1d_up_to_1w_old",
      "short": "1d-1w",
      "long": "1 Day to 1 Week Old"
    },
    "_1wTo1m": {
      "id": "at_least_1w_up_to_1m_old",
      "short": "1w-1m",
      "long": "1 Week to 1 Month Old"
    },
    "_1mTo2m": {
      "id": "at_least_1m_up_to_2m_old",
      "short": "1m-2m",
      "long": "1 to 2 Months Old"
    },
    "_2mTo3m": {
      "id": "at_least_2m_up_to_3m_old",
      "short": "2m-3m",
      "long": "2 to 3 Months Old"
    },
    "_3mTo4m": {
      "id": "at_least_3m_up_to_4m_old",
      "short": "3m-4m",
      "long": "3 to 4 Months Old"
    },
    "_4mTo5m": {
      "id": "at_least_4m_up_to_5m_old",
      "short": "4m-5m",
      "long": "4 to 5 Months Old"
    },
    "_5mTo6m": {
      "id": "at_least_5m_up_to_6m_old",
      "short": "5m-6m",
      "long": "5 to 6 Months Old"
    },
    "_6mTo1y": {
      "id": "at_least_6m_up_to_1y_old",
      "short": "6m-1y",
      "long": "6 Months to 1 Year Old"
    },
    "_1yTo2y": {
      "id": "at_least_1y_up_to_2y_old",
      "short": "1y-2y",
      "long": "1 to 2 Years Old"
    },
    "_2yTo3y": {
      "id": "at_least_2y_up_to_3y_old",
      "short": "2y-3y",
      "long": "2 to 3 Years Old"
    },
    "_3yTo4y": {
      "id": "at_least_3y_up_to_4y_old",
      "short": "3y-4y",
      "long": "3 to 4 Years Old"
    },
    "_4yTo5y": {
      "id": "at_least_4y_up_to_5y_old",
      "short": "4y-5y",
      "long": "4 to 5 Years Old"
    },
    "_5yTo6y": {
      "id": "at_least_5y_up_to_6y_old",
      "short": "5y-6y",
      "long": "5 to 6 Years Old"
    },
    "_6yTo7y": {
      "id": "at_least_6y_up_to_7y_old",
      "short": "6y-7y",
      "long": "6 to 7 Years Old"
    },
    "_7yTo8y": {
      "id": "at_least_7y_up_to_8y_old",
      "short": "7y-8y",
      "long": "7 to 8 Years Old"
    },
    "_8yTo10y": {
      "id": "at_least_8y_up_to_10y_old",
      "short": "8y-10y",
      "long": "8 to 10 Years Old"
    },
    "_10yTo12y": {
      "id": "at_least_10y_up_to_12y_old",
      "short": "10y-12y",
      "long": "10 to 12 Years Old"
    },
    "_12yTo15y": {
      "id": "at_least_12y_up_to_15y_old",
      "short": "12y-15y",
      "long": "12 to 15 Years Old"
    },
    "from15y": {
      "id": "at_least_15y_old",
      "short": "15y+",
      "long": "15+ Years Old"
    }
  });

  MAX_AGE_NAMES = /** @type {const} */ ({
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
  });

  MIN_AGE_NAMES = /** @type {const} */ ({
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
      "long": "1 to 10 Sats"
    },
    "_10satsTo100sats": {
      "id": "above_10sats_under_100sats",
      "short": "10-100 sats",
      "long": "10 to 100 Sats"
    },
    "_100satsTo1kSats": {
      "id": "above_100sats_under_1k_sats",
      "short": "100-1k sats",
      "long": "100 to 1K Sats"
    },
    "_1kSatsTo10kSats": {
      "id": "above_1k_sats_under_10k_sats",
      "short": "1k-10k sats",
      "long": "1K to 10K Sats"
    },
    "_10kSatsTo100kSats": {
      "id": "above_10k_sats_under_100k_sats",
      "short": "10k-100k sats",
      "long": "10K to 100K Sats"
    },
    "_100kSatsTo1mSats": {
      "id": "above_100k_sats_under_1m_sats",
      "short": "100k-1M sats",
      "long": "100K to 1M Sats"
    },
    "_1mSatsTo10mSats": {
      "id": "above_1m_sats_under_10m_sats",
      "short": "1M-10M sats",
      "long": "1M to 10M Sats"
    },
    "_10mSatsTo1btc": {
      "id": "above_10m_sats_under_1btc",
      "short": "0.1-1 BTC",
      "long": "0.1 to 1 BTC"
    },
    "_1btcTo10btc": {
      "id": "above_1btc_under_10btc",
      "short": "1-10 BTC",
      "long": "1 to 10 BTC"
    },
    "_10btcTo100btc": {
      "id": "above_10btc_under_100btc",
      "short": "10-100 BTC",
      "long": "10 to 100 BTC"
    },
    "_100btcTo1kBtc": {
      "id": "above_100btc_under_1k_btc",
      "short": "100-1k BTC",
      "long": "100 to 1K BTC"
    },
    "_1kBtcTo10kBtc": {
      "id": "above_1k_btc_under_10k_btc",
      "short": "1k-10k BTC",
      "long": "1K to 10K BTC"
    },
    "_10kBtcTo100kBtc": {
      "id": "above_10k_btc_under_100k_btc",
      "short": "10k-100k BTC",
      "long": "10K to 100K BTC"
    },
    "_100kBtcOrMore": {
      "id": "above_100k_btc",
      "short": "100k+ BTC",
      "long": "100K+ BTC"
    }
  });

  GE_AMOUNT_NAMES = /** @type {const} */ ({
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
    "_1kSats": {
      "id": "above_1k_sats",
      "short": "1k+ sats",
      "long": "Above 1K Sats"
    },
    "_10kSats": {
      "id": "above_10k_sats",
      "short": "10k+ sats",
      "long": "Above 10K Sats"
    },
    "_100kSats": {
      "id": "above_100k_sats",
      "short": "100k+ sats",
      "long": "Above 100K Sats"
    },
    "_1mSats": {
      "id": "above_1m_sats",
      "short": "1M+ sats",
      "long": "Above 1M Sats"
    },
    "_10mSats": {
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
    "_1kBtc": {
      "id": "above_1k_btc",
      "short": "1k+ BTC",
      "long": "Above 1K BTC"
    },
    "_10kBtc": {
      "id": "above_10k_btc",
      "short": "10k+ BTC",
      "long": "Above 10K BTC"
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
   * @param {BrkClientOptions|string} options
   */
  constructor(options) {
    super(options);
    /** @type {CatalogTree} */
    this.tree = this._buildTree('');
  }

  /**
   * @private
   * @param {string} basePath
   * @returns {CatalogTree}
   */
  _buildTree(basePath) {
    return {
      computed: {
        blocks: {
          count: {
            _1mBlockCount: createMetricPattern5(this, '1m_block_count'),
            _1wBlockCount: createMetricPattern5(this, '1w_block_count'),
            _1yBlockCount: createMetricPattern5(this, '1y_block_count'),
            _24hBlockCount: createMetricPattern26(this, '24h_block_count'),
            blockCount: createBlockCountPattern(this, 'block_count'),
            blockCountTarget: createMetricPattern5(this, 'block_count_target')
          },
          difficulty: {
            blocksBeforeNextDifficultyAdjustment: createMetricPattern1(this, 'blocks_before_next_difficulty_adjustment'),
            daysBeforeNextDifficultyAdjustment: createMetricPattern1(this, 'days_before_next_difficulty_adjustment'),
            difficultyepoch: createMetricPattern5(this, 'difficultyepoch')
          },
          halving: {
            blocksBeforeNextHalving: createMetricPattern1(this, 'blocks_before_next_halving'),
            daysBeforeNextHalving: createMetricPattern1(this, 'days_before_next_halving'),
            halvingepoch: createMetricPattern5(this, 'halvingepoch')
          },
          interval: {
            blockInterval: createBlockIntervalPattern(this, 'block_interval'),
            interval: createMetricPattern26(this, 'interval')
          },
          mining: {
            difficulty: createMetricPattern2(this, 'difficulty'),
            difficultyAdjustment: createDifficultyAdjustmentPattern(this, 'difficulty_adjustment'),
            difficultyAsHash: createMetricPattern1(this, 'difficulty_as_hash'),
            hashPricePhs: createMetricPattern1(this, 'hash_price_phs'),
            hashPricePhsMin: createMetricPattern1(this, 'hash_price_phs_min'),
            hashPriceRebound: createMetricPattern1(this, 'hash_price_rebound'),
            hashPriceThs: createMetricPattern1(this, 'hash_price_ths'),
            hashPriceThsMin: createMetricPattern1(this, 'hash_price_ths_min'),
            hashRate: createMetricPattern1(this, 'hash_rate'),
            hashRate1mSma: createMetricPattern5(this, 'hash_rate_1m_sma'),
            hashRate1wSma: createMetricPattern5(this, 'hash_rate_1w_sma'),
            hashRate1ySma: createMetricPattern5(this, 'hash_rate_1y_sma'),
            hashRate2mSma: createMetricPattern5(this, 'hash_rate_2m_sma'),
            hashValuePhs: createMetricPattern1(this, 'hash_value_phs'),
            hashValuePhsMin: createMetricPattern1(this, 'hash_value_phs_min'),
            hashValueRebound: createMetricPattern1(this, 'hash_value_rebound'),
            hashValueThs: createMetricPattern1(this, 'hash_value_ths'),
            hashValueThsMin: createMetricPattern1(this, 'hash_value_ths_min')
          },
          rewards: {
            _24hCoinbaseSum: createMetricPattern26(this, '24h_coinbase_sum'),
            _24hCoinbaseUsdSum: createMetricPattern26(this, '24h_coinbase_usd_sum'),
            coinbase: createCoinbasePattern(this, 'coinbase'),
            feeDominance: createMetricPattern22(this, 'fee_dominance'),
            subsidy: createCoinbasePattern(this, 'subsidy'),
            subsidyDominance: createMetricPattern22(this, 'subsidy_dominance'),
            subsidyUsd1ySma: createMetricPattern5(this, 'subsidy_usd_1y_sma'),
            unclaimedRewards: createUnclaimedRewardsPattern(this, 'unclaimed_rewards')
          },
          size: {
            blockSize: createBlockSizePattern(this, 'block_size'),
            blockVbytes: createBlockSizePattern(this, 'block_vbytes'),
            vbytes: createMetricPattern26(this, 'vbytes')
          },
          time: {
            date: createMetricPattern26(this, 'date'),
            dateFixed: createMetricPattern26(this, 'date_fixed'),
            timestamp: createMetricPattern2(this, 'timestamp'),
            timestampFixed: createMetricPattern26(this, 'timestamp_fixed')
          },
          weight: {
            blockFullness: createBitcoinPattern(this, 'block_fullness'),
            blockWeight: createBlockSizePattern(this, 'block_weight')
          }
        },
        cointime: {
          activity: {
            activityToVaultednessRatio: createMetricPattern1(this, 'activity_to_vaultedness_ratio'),
            coinblocksCreated: createBlockCountPattern(this, 'coinblocks_created'),
            coinblocksStored: createBlockCountPattern(this, 'coinblocks_stored'),
            liveliness: createMetricPattern1(this, 'liveliness'),
            vaultedness: createMetricPattern1(this, 'vaultedness')
          },
          adjusted: {
            cointimeAdjInflationRate: createMetricPattern5(this, 'cointime_adj_inflation_rate'),
            cointimeAdjTxBtcVelocity: createMetricPattern5(this, 'cointime_adj_tx_btc_velocity'),
            cointimeAdjTxUsdVelocity: createMetricPattern5(this, 'cointime_adj_tx_usd_velocity')
          },
          cap: {
            activeCap: createMetricPattern1(this, 'active_cap'),
            cointimeCap: createMetricPattern1(this, 'cointime_cap'),
            investorCap: createMetricPattern1(this, 'investor_cap'),
            thermoCap: createMetricPattern1(this, 'thermo_cap'),
            vaultedCap: createMetricPattern1(this, 'vaulted_cap')
          },
          pricing: {
            activePrice: createMetricPattern1(this, 'active_price'),
            activePriceRatio: createActivePriceRatioPattern(this, 'active_price_ratio'),
            cointimePrice: createMetricPattern1(this, 'cointime_price'),
            cointimePriceRatio: createActivePriceRatioPattern(this, 'cointime_price_ratio'),
            trueMarketMean: createMetricPattern1(this, 'true_market_mean'),
            trueMarketMeanRatio: createActivePriceRatioPattern(this, 'true_market_mean_ratio'),
            vaultedPrice: createMetricPattern1(this, 'vaulted_price'),
            vaultedPriceRatio: createActivePriceRatioPattern(this, 'vaulted_price_ratio')
          },
          supply: {
            activeSupply: createActiveSupplyPattern(this, 'active_supply'),
            vaultedSupply: createActiveSupplyPattern(this, 'vaulted_supply')
          },
          value: {
            cointimeValueCreated: createBlockCountPattern(this, 'cointime_value_created'),
            cointimeValueDestroyed: createBlockCountPattern(this, 'cointime_value_destroyed'),
            cointimeValueStored: createBlockCountPattern(this, 'cointime_value_stored')
          }
        },
        constants: {
          constant0: createMetricPattern3(this, 'constant_0'),
          constant1: createMetricPattern3(this, 'constant_1'),
          constant100: createMetricPattern3(this, 'constant_100'),
          constant2: createMetricPattern3(this, 'constant_2'),
          constant20: createMetricPattern3(this, 'constant_20'),
          constant3: createMetricPattern3(this, 'constant_3'),
          constant30: createMetricPattern3(this, 'constant_30'),
          constant382: createMetricPattern3(this, 'constant_38_2'),
          constant4: createMetricPattern3(this, 'constant_4'),
          constant50: createMetricPattern3(this, 'constant_50'),
          constant600: createMetricPattern3(this, 'constant_600'),
          constant618: createMetricPattern3(this, 'constant_61_8'),
          constant70: createMetricPattern3(this, 'constant_70'),
          constant80: createMetricPattern3(this, 'constant_80'),
          constantMinus1: createMetricPattern3(this, 'constant_minus_1'),
          constantMinus2: createMetricPattern3(this, 'constant_minus_2'),
          constantMinus3: createMetricPattern3(this, 'constant_minus_3'),
          constantMinus4: createMetricPattern3(this, 'constant_minus_4')
        },
        distribution: {
          addrCount: createMetricPattern1(this, 'addr_count'),
          addressCohorts: {
            amountRange: {
              _0sats: create_0satsPattern(this, 'addrs_with_0sats'),
              _100btcTo1kBtc: create_0satsPattern(this, 'addrs_above_100btc_under_1k_btc'),
              _100kBtcOrMore: create_0satsPattern(this, 'addrs_above_100k_btc'),
              _100kSatsTo1mSats: create_0satsPattern(this, 'addrs_above_100k_sats_under_1m_sats'),
              _100satsTo1kSats: create_0satsPattern(this, 'addrs_above_100sats_under_1k_sats'),
              _10btcTo100btc: create_0satsPattern(this, 'addrs_above_10btc_under_100btc'),
              _10kBtcTo100kBtc: create_0satsPattern(this, 'addrs_above_10k_btc_under_100k_btc'),
              _10kSatsTo100kSats: create_0satsPattern(this, 'addrs_above_10k_sats_under_100k_sats'),
              _10mSatsTo1btc: create_0satsPattern(this, 'addrs_above_10m_sats_under_1btc'),
              _10satsTo100sats: create_0satsPattern(this, 'addrs_above_10sats_under_100sats'),
              _1btcTo10btc: create_0satsPattern(this, 'addrs_above_1btc_under_10btc'),
              _1kBtcTo10kBtc: create_0satsPattern(this, 'addrs_above_1k_btc_under_10k_btc'),
              _1kSatsTo10kSats: create_0satsPattern(this, 'addrs_above_1k_sats_under_10k_sats'),
              _1mSatsTo10mSats: create_0satsPattern(this, 'addrs_above_1m_sats_under_10m_sats'),
              _1satTo10sats: create_0satsPattern(this, 'addrs_above_1sat_under_10sats')
            },
            geAmount: {
              _100btc: create_0satsPattern(this, 'addrs_above_100btc'),
              _100kSats: create_0satsPattern(this, 'addrs_above_100k_sats'),
              _100sats: create_0satsPattern(this, 'addrs_above_100sats'),
              _10btc: create_0satsPattern(this, 'addrs_above_10btc'),
              _10kBtc: create_0satsPattern(this, 'addrs_above_10k_btc'),
              _10kSats: create_0satsPattern(this, 'addrs_above_10k_sats'),
              _10mSats: create_0satsPattern(this, 'addrs_above_10m_sats'),
              _10sats: create_0satsPattern(this, 'addrs_above_10sats'),
              _1btc: create_0satsPattern(this, 'addrs_above_1btc'),
              _1kBtc: create_0satsPattern(this, 'addrs_above_1k_btc'),
              _1kSats: create_0satsPattern(this, 'addrs_above_1k_sats'),
              _1mSats: create_0satsPattern(this, 'addrs_above_1m_sats'),
              _1sat: create_0satsPattern(this, 'addrs_above_1sat')
            },
            ltAmount: {
              _100btc: create_0satsPattern(this, 'addrs_under_100btc'),
              _100kBtc: create_0satsPattern(this, 'addrs_under_100k_btc'),
              _100kSats: create_0satsPattern(this, 'addrs_under_100k_sats'),
              _100sats: create_0satsPattern(this, 'addrs_under_100sats'),
              _10btc: create_0satsPattern(this, 'addrs_under_10btc'),
              _10kBtc: create_0satsPattern(this, 'addrs_under_10k_btc'),
              _10kSats: create_0satsPattern(this, 'addrs_under_10k_sats'),
              _10mSats: create_0satsPattern(this, 'addrs_under_10m_sats'),
              _10sats: create_0satsPattern(this, 'addrs_under_10sats'),
              _1btc: create_0satsPattern(this, 'addrs_under_1btc'),
              _1kBtc: create_0satsPattern(this, 'addrs_under_1k_btc'),
              _1kSats: create_0satsPattern(this, 'addrs_under_1k_sats'),
              _1mSats: create_0satsPattern(this, 'addrs_under_1m_sats')
            }
          },
          addressesData: {
            empty: createMetricPattern42(this, 'emptyaddressdata'),
            loaded: createMetricPattern41(this, 'loadedaddressdata')
          },
          addresstypeToHeightToAddrCount: createAddresstypeToHeightToAddrCountPattern(this, 'addr_count'),
          addresstypeToHeightToEmptyAddrCount: createAddresstypeToHeightToAddrCountPattern(this, 'empty_addr_count'),
          addresstypeToIndexesToAddrCount: createAddresstypeToHeightToAddrCountPattern(this, 'addr_count'),
          addresstypeToIndexesToEmptyAddrCount: createAddresstypeToHeightToAddrCountPattern(this, 'empty_addr_count'),
          anyAddressIndexes: createAddresstypeToHeightToAddrCountPattern(this, 'anyaddressindex'),
          chainState: createMetricPattern26(this, 'chain'),
          emptyAddrCount: createMetricPattern1(this, 'empty_addr_count'),
          emptyaddressindex: createMetricPattern42(this, 'emptyaddressindex'),
          loadedaddressindex: createMetricPattern41(this, 'loadedaddressindex'),
          utxoCohorts: {
            ageRange: {
              _10yTo12y: create_10yTo12yPattern(this, 'utxos_at_least_10y_up_to_12y_old'),
              _12yTo15y: create_10yTo12yPattern(this, 'utxos_at_least_12y_up_to_15y_old'),
              _1dTo1w: create_10yTo12yPattern(this, 'utxos_at_least_1d_up_to_1w_old'),
              _1hTo1d: create_10yTo12yPattern(this, 'utxos_at_least_1h_up_to_1d_old'),
              _1mTo2m: create_10yTo12yPattern(this, 'utxos_at_least_1m_up_to_2m_old'),
              _1wTo1m: create_10yTo12yPattern(this, 'utxos_at_least_1w_up_to_1m_old'),
              _1yTo2y: create_10yTo12yPattern(this, 'utxos_at_least_1y_up_to_2y_old'),
              _2mTo3m: create_10yTo12yPattern(this, 'utxos_at_least_2m_up_to_3m_old'),
              _2yTo3y: create_10yTo12yPattern(this, 'utxos_at_least_2y_up_to_3y_old'),
              _3mTo4m: create_10yTo12yPattern(this, 'utxos_at_least_3m_up_to_4m_old'),
              _3yTo4y: create_10yTo12yPattern(this, 'utxos_at_least_3y_up_to_4y_old'),
              _4mTo5m: create_10yTo12yPattern(this, 'utxos_at_least_4m_up_to_5m_old'),
              _4yTo5y: create_10yTo12yPattern(this, 'utxos_at_least_4y_up_to_5y_old'),
              _5mTo6m: create_10yTo12yPattern(this, 'utxos_at_least_5m_up_to_6m_old'),
              _5yTo6y: create_10yTo12yPattern(this, 'utxos_at_least_5y_up_to_6y_old'),
              _6mTo1y: create_10yTo12yPattern(this, 'utxos_at_least_6m_up_to_1y_old'),
              _6yTo7y: create_10yTo12yPattern(this, 'utxos_at_least_6y_up_to_7y_old'),
              _7yTo8y: create_10yTo12yPattern(this, 'utxos_at_least_7y_up_to_8y_old'),
              _8yTo10y: create_10yTo12yPattern(this, 'utxos_at_least_8y_up_to_10y_old'),
              from15y: create_10yTo12yPattern(this, 'utxos_at_least_15y_old'),
              upTo1h: create_10yTo12yPattern(this, 'utxos_up_to_1h_old')
            },
            all: {
              activity: createActivityPattern2(this, ''),
              costBasis: createCostBasisPattern2(this, ''),
              realized: createRealizedPattern3(this, ''),
              relative: {
                negUnrealizedLossRelToOwnTotalUnrealizedPnl: createMetricPattern3(this, 'neg_unrealized_loss_rel_to_own_total_unrealized_pnl'),
                netUnrealizedPnlRelToOwnTotalUnrealizedPnl: createMetricPattern3(this, 'net_unrealized_pnl_rel_to_own_total_unrealized_pnl'),
                supplyInLossRelToOwnSupply: createMetricPattern3(this, 'supply_in_loss_rel_to_own_supply'),
                supplyInProfitRelToOwnSupply: createMetricPattern3(this, 'supply_in_profit_rel_to_own_supply'),
                unrealizedLossRelToOwnTotalUnrealizedPnl: createMetricPattern3(this, 'unrealized_loss_rel_to_own_total_unrealized_pnl'),
                unrealizedProfitRelToOwnTotalUnrealizedPnl: createMetricPattern3(this, 'unrealized_profit_rel_to_own_total_unrealized_pnl')
              },
              supply: createSupplyPattern3(this, ''),
              unrealized: createUnrealizedPattern(this, '')
            },
            amountRange: {
              _0sats: create_0satsPattern2(this, 'utxos_with_0sats'),
              _100btcTo1kBtc: create_0satsPattern2(this, 'utxos_above_100btc_under_1k_btc'),
              _100kBtcOrMore: create_0satsPattern2(this, 'utxos_above_100k_btc'),
              _100kSatsTo1mSats: create_0satsPattern2(this, 'utxos_above_100k_sats_under_1m_sats'),
              _100satsTo1kSats: create_0satsPattern2(this, 'utxos_above_100sats_under_1k_sats'),
              _10btcTo100btc: create_0satsPattern2(this, 'utxos_above_10btc_under_100btc'),
              _10kBtcTo100kBtc: create_0satsPattern2(this, 'utxos_above_10k_btc_under_100k_btc'),
              _10kSatsTo100kSats: create_0satsPattern2(this, 'utxos_above_10k_sats_under_100k_sats'),
              _10mSatsTo1btc: create_0satsPattern2(this, 'utxos_above_10m_sats_under_1btc'),
              _10satsTo100sats: create_0satsPattern2(this, 'utxos_above_10sats_under_100sats'),
              _1btcTo10btc: create_0satsPattern2(this, 'utxos_above_1btc_under_10btc'),
              _1kBtcTo10kBtc: create_0satsPattern2(this, 'utxos_above_1k_btc_under_10k_btc'),
              _1kSatsTo10kSats: create_0satsPattern2(this, 'utxos_above_1k_sats_under_10k_sats'),
              _1mSatsTo10mSats: create_0satsPattern2(this, 'utxos_above_1m_sats_under_10m_sats'),
              _1satTo10sats: create_0satsPattern2(this, 'utxos_above_1sat_under_10sats')
            },
            epoch: {
              _0: create_0satsPattern2(this, 'epoch_0'),
              _1: create_0satsPattern2(this, 'epoch_1'),
              _2: create_0satsPattern2(this, 'epoch_2'),
              _3: create_0satsPattern2(this, 'epoch_3'),
              _4: create_0satsPattern2(this, 'epoch_4')
            },
            geAmount: {
              _100btc: create_100btcPattern(this, 'utxos_above_100btc'),
              _100kSats: create_100btcPattern(this, 'utxos_above_100k_sats'),
              _100sats: create_100btcPattern(this, 'utxos_above_100sats'),
              _10btc: create_100btcPattern(this, 'utxos_above_10btc'),
              _10kBtc: create_100btcPattern(this, 'utxos_above_10k_btc'),
              _10kSats: create_100btcPattern(this, 'utxos_above_10k_sats'),
              _10mSats: create_100btcPattern(this, 'utxos_above_10m_sats'),
              _10sats: create_100btcPattern(this, 'utxos_above_10sats'),
              _1btc: create_100btcPattern(this, 'utxos_above_1btc'),
              _1kBtc: create_100btcPattern(this, 'utxos_above_1k_btc'),
              _1kSats: create_100btcPattern(this, 'utxos_above_1k_sats'),
              _1mSats: create_100btcPattern(this, 'utxos_above_1m_sats'),
              _1sat: create_100btcPattern(this, 'utxos_above_1sat')
            },
            ltAmount: {
              _100btc: create_100btcPattern(this, 'utxos_under_100btc'),
              _100kBtc: create_100btcPattern(this, 'utxos_under_100k_btc'),
              _100kSats: create_100btcPattern(this, 'utxos_under_100k_sats'),
              _100sats: create_100btcPattern(this, 'utxos_under_100sats'),
              _10btc: create_100btcPattern(this, 'utxos_under_10btc'),
              _10kBtc: create_100btcPattern(this, 'utxos_under_10k_btc'),
              _10kSats: create_100btcPattern(this, 'utxos_under_10k_sats'),
              _10mSats: create_100btcPattern(this, 'utxos_under_10m_sats'),
              _10sats: create_100btcPattern(this, 'utxos_under_10sats'),
              _1btc: create_100btcPattern(this, 'utxos_under_1btc'),
              _1kBtc: create_100btcPattern(this, 'utxos_under_1k_btc'),
              _1kSats: create_100btcPattern(this, 'utxos_under_1k_sats'),
              _1mSats: create_100btcPattern(this, 'utxos_under_1m_sats')
            },
            maxAge: {
              _10y: create_10yPattern(this, 'utxos_up_to_10y_old'),
              _12y: create_10yPattern(this, 'utxos_up_to_12y_old'),
              _15y: create_10yPattern(this, 'utxos_up_to_15y_old'),
              _1m: create_10yPattern(this, 'utxos_up_to_1m_old'),
              _1w: create_10yPattern(this, 'utxos_up_to_1w_old'),
              _1y: create_10yPattern(this, 'utxos_up_to_1y_old'),
              _2m: create_10yPattern(this, 'utxos_up_to_2m_old'),
              _2y: create_10yPattern(this, 'utxos_up_to_2y_old'),
              _3m: create_10yPattern(this, 'utxos_up_to_3m_old'),
              _3y: create_10yPattern(this, 'utxos_up_to_3y_old'),
              _4m: create_10yPattern(this, 'utxos_up_to_4m_old'),
              _4y: create_10yPattern(this, 'utxos_up_to_4y_old'),
              _5m: create_10yPattern(this, 'utxos_up_to_5m_old'),
              _5y: create_10yPattern(this, 'utxos_up_to_5y_old'),
              _6m: create_10yPattern(this, 'utxos_up_to_6m_old'),
              _6y: create_10yPattern(this, 'utxos_up_to_6y_old'),
              _7y: create_10yPattern(this, 'utxos_up_to_7y_old'),
              _8y: create_10yPattern(this, 'utxos_up_to_8y_old')
            },
            minAge: {
              _10y: create_100btcPattern(this, 'utxos_at_least_10y_old'),
              _12y: create_100btcPattern(this, 'utxos_at_least_12y_old'),
              _1d: create_100btcPattern(this, 'utxos_at_least_1d_old'),
              _1m: create_100btcPattern(this, 'utxos_at_least_1m_old'),
              _1w: create_100btcPattern(this, 'utxos_at_least_1w_old'),
              _1y: create_100btcPattern(this, 'utxos_at_least_1y_old'),
              _2m: create_100btcPattern(this, 'utxos_at_least_2m_old'),
              _2y: create_100btcPattern(this, 'utxos_at_least_2y_old'),
              _3m: create_100btcPattern(this, 'utxos_at_least_3m_old'),
              _3y: create_100btcPattern(this, 'utxos_at_least_3y_old'),
              _4m: create_100btcPattern(this, 'utxos_at_least_4m_old'),
              _4y: create_100btcPattern(this, 'utxos_at_least_4y_old'),
              _5m: create_100btcPattern(this, 'utxos_at_least_5m_old'),
              _5y: create_100btcPattern(this, 'utxos_at_least_5y_old'),
              _6m: create_100btcPattern(this, 'utxos_at_least_6m_old'),
              _6y: create_100btcPattern(this, 'utxos_at_least_6y_old'),
              _7y: create_100btcPattern(this, 'utxos_at_least_7y_old'),
              _8y: create_100btcPattern(this, 'utxos_at_least_8y_old')
            },
            term: {
              long: {
                activity: createActivityPattern2(this, 'lth'),
                costBasis: createCostBasisPattern2(this, 'lth'),
                realized: createRealizedPattern2(this, 'lth'),
                relative: createRelativePattern5(this, 'lth'),
                supply: createSupplyPattern3(this, 'lth'),
                unrealized: createUnrealizedPattern(this, 'lth')
              },
              short: {
                activity: createActivityPattern2(this, 'sth'),
                costBasis: createCostBasisPattern2(this, 'sth'),
                realized: createRealizedPattern3(this, 'sth'),
                relative: createRelativePattern5(this, 'sth'),
                supply: createSupplyPattern3(this, 'sth'),
                unrealized: createUnrealizedPattern(this, 'sth')
              }
            },
            type: {
              empty: create_0satsPattern2(this, 'empty_outputs'),
              p2a: create_0satsPattern2(this, 'p2a'),
              p2ms: create_0satsPattern2(this, 'p2ms'),
              p2pk33: create_0satsPattern2(this, 'p2pk33'),
              p2pk65: create_0satsPattern2(this, 'p2pk65'),
              p2pkh: create_0satsPattern2(this, 'p2pkh'),
              p2sh: create_0satsPattern2(this, 'p2sh'),
              p2tr: create_0satsPattern2(this, 'p2tr'),
              p2wpkh: create_0satsPattern2(this, 'p2wpkh'),
              p2wsh: create_0satsPattern2(this, 'p2wsh'),
              unknown: create_0satsPattern2(this, 'unknown_outputs')
            },
            year: {
              _2009: create_0satsPattern2(this, 'year_2009'),
              _2010: create_0satsPattern2(this, 'year_2010'),
              _2011: create_0satsPattern2(this, 'year_2011'),
              _2012: create_0satsPattern2(this, 'year_2012'),
              _2013: create_0satsPattern2(this, 'year_2013'),
              _2014: create_0satsPattern2(this, 'year_2014'),
              _2015: create_0satsPattern2(this, 'year_2015'),
              _2016: create_0satsPattern2(this, 'year_2016'),
              _2017: create_0satsPattern2(this, 'year_2017'),
              _2018: create_0satsPattern2(this, 'year_2018'),
              _2019: create_0satsPattern2(this, 'year_2019'),
              _2020: create_0satsPattern2(this, 'year_2020'),
              _2021: create_0satsPattern2(this, 'year_2021'),
              _2022: create_0satsPattern2(this, 'year_2022'),
              _2023: create_0satsPattern2(this, 'year_2023'),
              _2024: create_0satsPattern2(this, 'year_2024'),
              _2025: create_0satsPattern2(this, 'year_2025'),
              _2026: create_0satsPattern2(this, 'year_2026')
            }
          }
        },
        indexes: {
          address: {
            emptyoutputindex: createMetricPattern25(this, 'emptyoutputindex'),
            opreturnindex: createMetricPattern28(this, 'opreturnindex'),
            p2aaddressindex: createMetricPattern30(this, 'p2aaddressindex'),
            p2msoutputindex: createMetricPattern31(this, 'p2msoutputindex'),
            p2pk33addressindex: createMetricPattern32(this, 'p2pk33addressindex'),
            p2pk65addressindex: createMetricPattern33(this, 'p2pk65addressindex'),
            p2pkhaddressindex: createMetricPattern34(this, 'p2pkhaddressindex'),
            p2shaddressindex: createMetricPattern35(this, 'p2shaddressindex'),
            p2traddressindex: createMetricPattern36(this, 'p2traddressindex'),
            p2wpkhaddressindex: createMetricPattern37(this, 'p2wpkhaddressindex'),
            p2wshaddressindex: createMetricPattern38(this, 'p2wshaddressindex'),
            unknownoutputindex: createMetricPattern40(this, 'unknownoutputindex')
          },
          block: {
            dateindex: createMetricPattern26(this, 'dateindex'),
            difficultyepoch: createMetricPattern15(this, 'difficultyepoch'),
            firstHeight: createMetricPattern14(this, 'first_height'),
            halvingepoch: createMetricPattern16(this, 'halvingepoch'),
            height: createMetricPattern26(this, 'height'),
            heightCount: createMetricPattern24(this, 'height_count'),
            txindexCount: createMetricPattern26(this, 'txindex_count')
          },
          time: {
            date: createMetricPattern22(this, 'date'),
            dateindex: createMetricPattern22(this, 'dateindex'),
            dateindexCount: createMetricPattern20(this, 'dateindex_count'),
            decadeindex: createMetricPattern13(this, 'decadeindex'),
            firstDateindex: createMetricPattern20(this, 'first_dateindex'),
            firstHeight: createMetricPattern22(this, 'first_height'),
            firstMonthindex: createMetricPattern9(this, 'first_monthindex'),
            firstYearindex: createMetricPattern23(this, 'first_yearindex'),
            heightCount: createMetricPattern22(this, 'height_count'),
            monthindex: createMetricPattern11(this, 'monthindex'),
            monthindexCount: createMetricPattern9(this, 'monthindex_count'),
            quarterindex: createMetricPattern18(this, 'quarterindex'),
            semesterindex: createMetricPattern19(this, 'semesterindex'),
            weekindex: createMetricPattern12(this, 'weekindex'),
            yearindex: createMetricPattern21(this, 'yearindex'),
            yearindexCount: createMetricPattern23(this, 'yearindex_count')
          },
          transaction: {
            inputCount: createMetricPattern39(this, 'input_count'),
            outputCount: createMetricPattern39(this, 'output_count'),
            txindex: createMetricPattern39(this, 'txindex'),
            txinindex: createMetricPattern27(this, 'txinindex'),
            txoutindex: createMetricPattern29(this, 'txoutindex')
          }
        },
        inputs: {
          count: {
            count: createCountPattern2(this, 'input_count')
          },
          spent: {
            txoutindex: createMetricPattern27(this, 'txoutindex'),
            value: createMetricPattern27(this, 'value')
          }
        },
        market: {
          ath: {
            daysSincePriceAth: createMetricPattern5(this, 'days_since_price_ath'),
            maxDaysBetweenPriceAths: createMetricPattern5(this, 'max_days_between_price_aths'),
            maxYearsBetweenPriceAths: createMetricPattern5(this, 'max_years_between_price_aths'),
            priceAth: createMetricPattern3(this, 'price_ath'),
            priceDrawdown: createMetricPattern3(this, 'price_drawdown'),
            yearsSincePriceAth: createMetricPattern5(this, 'years_since_price_ath')
          },
          dca: {
            classAveragePrice: createClassAveragePricePattern(this, 'dca_class'),
            classReturns: createClassAveragePricePattern(this, 'dca_class'),
            classStack: {
              _2015: createActiveSupplyPattern(this, 'dca_class_2015_stack'),
              _2016: createActiveSupplyPattern(this, 'dca_class_2016_stack'),
              _2017: createActiveSupplyPattern(this, 'dca_class_2017_stack'),
              _2018: createActiveSupplyPattern(this, 'dca_class_2018_stack'),
              _2019: createActiveSupplyPattern(this, 'dca_class_2019_stack'),
              _2020: createActiveSupplyPattern(this, 'dca_class_2020_stack'),
              _2021: createActiveSupplyPattern(this, 'dca_class_2021_stack'),
              _2022: createActiveSupplyPattern(this, 'dca_class_2022_stack'),
              _2023: createActiveSupplyPattern(this, 'dca_class_2023_stack'),
              _2024: createActiveSupplyPattern(this, 'dca_class_2024_stack'),
              _2025: createActiveSupplyPattern(this, 'dca_class_2025_stack')
            },
            periodAveragePrice: createPeriodAveragePricePattern(this, 'dca_average_price'),
            periodCagr: createPeriodCagrPattern(this, 'dca_cagr'),
            periodLumpSumStack: createPeriodLumpSumStackPattern(this, ''),
            periodReturns: createPeriodAveragePricePattern(this, 'dca_returns'),
            periodStack: createPeriodLumpSumStackPattern(this, '')
          },
          indicators: {
            gini: createMetricPattern22(this, 'gini'),
            macdHistogram: createMetricPattern22(this, 'macd_histogram'),
            macdLine: createMetricPattern22(this, 'macd_line'),
            macdSignal: createMetricPattern22(this, 'macd_signal'),
            nvt: createMetricPattern5(this, 'nvt'),
            piCycle: createMetricPattern22(this, 'pi_cycle'),
            puellMultiple: createMetricPattern5(this, 'puell_multiple'),
            rsi14d: createMetricPattern22(this, 'rsi_14d'),
            rsi14dMax: createMetricPattern22(this, 'rsi_14d_max'),
            rsi14dMin: createMetricPattern22(this, 'rsi_14d_min'),
            rsiAverageGain14d: createMetricPattern22(this, 'rsi_average_gain_14d'),
            rsiAverageLoss14d: createMetricPattern22(this, 'rsi_average_loss_14d'),
            rsiGains: createMetricPattern22(this, 'rsi_gains'),
            rsiLosses: createMetricPattern22(this, 'rsi_losses'),
            stochD: createMetricPattern22(this, 'stoch_d'),
            stochK: createMetricPattern22(this, 'stoch_k'),
            stochRsi: createMetricPattern22(this, 'stoch_rsi'),
            stochRsiD: createMetricPattern22(this, 'stoch_rsi_d'),
            stochRsiK: createMetricPattern22(this, 'stoch_rsi_k')
          },
          lookback: {
            priceAgo: createPriceAgoPattern(this, 'price')
          },
          movingAverage: {
            price111dSma: createPrice111dSmaPattern(this, 'price_111d_sma'),
            price12dEma: createPrice111dSmaPattern(this, 'price_12d_ema'),
            price13dEma: createPrice111dSmaPattern(this, 'price_13d_ema'),
            price13dSma: createPrice111dSmaPattern(this, 'price_13d_sma'),
            price144dEma: createPrice111dSmaPattern(this, 'price_144d_ema'),
            price144dSma: createPrice111dSmaPattern(this, 'price_144d_sma'),
            price1mEma: createPrice111dSmaPattern(this, 'price_1m_ema'),
            price1mSma: createPrice111dSmaPattern(this, 'price_1m_sma'),
            price1wEma: createPrice111dSmaPattern(this, 'price_1w_ema'),
            price1wSma: createPrice111dSmaPattern(this, 'price_1w_sma'),
            price1yEma: createPrice111dSmaPattern(this, 'price_1y_ema'),
            price1ySma: createPrice111dSmaPattern(this, 'price_1y_sma'),
            price200dEma: createPrice111dSmaPattern(this, 'price_200d_ema'),
            price200dSma: createPrice111dSmaPattern(this, 'price_200d_sma'),
            price200dSmaX08: createMetricPattern5(this, 'price_200d_sma_x0_8'),
            price200dSmaX24: createMetricPattern5(this, 'price_200d_sma_x2_4'),
            price200wEma: createPrice111dSmaPattern(this, 'price_200w_ema'),
            price200wSma: createPrice111dSmaPattern(this, 'price_200w_sma'),
            price21dEma: createPrice111dSmaPattern(this, 'price_21d_ema'),
            price21dSma: createPrice111dSmaPattern(this, 'price_21d_sma'),
            price26dEma: createPrice111dSmaPattern(this, 'price_26d_ema'),
            price2yEma: createPrice111dSmaPattern(this, 'price_2y_ema'),
            price2ySma: createPrice111dSmaPattern(this, 'price_2y_sma'),
            price34dEma: createPrice111dSmaPattern(this, 'price_34d_ema'),
            price34dSma: createPrice111dSmaPattern(this, 'price_34d_sma'),
            price350dSma: createPrice111dSmaPattern(this, 'price_350d_sma'),
            price350dSmaX2: createMetricPattern5(this, 'price_350d_sma_x2'),
            price4yEma: createPrice111dSmaPattern(this, 'price_4y_ema'),
            price4ySma: createPrice111dSmaPattern(this, 'price_4y_sma'),
            price55dEma: createPrice111dSmaPattern(this, 'price_55d_ema'),
            price55dSma: createPrice111dSmaPattern(this, 'price_55d_sma'),
            price89dEma: createPrice111dSmaPattern(this, 'price_89d_ema'),
            price89dSma: createPrice111dSmaPattern(this, 'price_89d_sma'),
            price8dEma: createPrice111dSmaPattern(this, 'price_8d_ema'),
            price8dSma: createPrice111dSmaPattern(this, 'price_8d_sma')
          },
          range: {
            price1mMax: createMetricPattern5(this, 'price_1m_max'),
            price1mMin: createMetricPattern5(this, 'price_1m_min'),
            price1wMax: createMetricPattern5(this, 'price_1w_max'),
            price1wMin: createMetricPattern5(this, 'price_1w_min'),
            price1yMax: createMetricPattern5(this, 'price_1y_max'),
            price1yMin: createMetricPattern5(this, 'price_1y_min'),
            price2wChoppinessIndex: createMetricPattern5(this, 'price_2w_choppiness_index'),
            price2wMax: createMetricPattern5(this, 'price_2w_max'),
            price2wMin: createMetricPattern5(this, 'price_2w_min'),
            priceTrueRange: createMetricPattern22(this, 'price_true_range'),
            priceTrueRange2wSum: createMetricPattern22(this, 'price_true_range_2w_sum')
          },
          returns: {
            _1dReturns1mSd: create_1dReturns1mSdPattern(this, '1d_returns_1m_sd'),
            _1dReturns1wSd: create_1dReturns1mSdPattern(this, '1d_returns_1w_sd'),
            _1dReturns1ySd: create_1dReturns1mSdPattern(this, '1d_returns_1y_sd'),
            cagr: createPeriodCagrPattern(this, 'cagr'),
            downside1mSd: create_1dReturns1mSdPattern(this, 'downside_1m_sd'),
            downside1wSd: create_1dReturns1mSdPattern(this, 'downside_1w_sd'),
            downside1ySd: create_1dReturns1mSdPattern(this, 'downside_1y_sd'),
            downsideReturns: createMetricPattern22(this, 'downside_returns'),
            priceReturns: createPriceAgoPattern(this, 'price_returns')
          },
          volatility: {
            price1mVolatility: createMetricPattern5(this, 'price_1m_volatility'),
            price1wVolatility: createMetricPattern5(this, 'price_1w_volatility'),
            price1yVolatility: createMetricPattern5(this, 'price_1y_volatility'),
            sharpe1m: createMetricPattern22(this, 'sharpe_1m'),
            sharpe1w: createMetricPattern22(this, 'sharpe_1w'),
            sharpe1y: createMetricPattern22(this, 'sharpe_1y'),
            sortino1m: createMetricPattern22(this, 'sortino_1m'),
            sortino1w: createMetricPattern22(this, 'sortino_1w'),
            sortino1y: createMetricPattern22(this, 'sortino_1y')
          }
        },
        outputs: {
          count: {
            count: createCountPattern2(this, 'output_count'),
            utxoCount: createDollarsPattern(this, 'exact_utxo_count')
          },
          spent: {
            txinindex: createMetricPattern29(this, 'txinindex')
          }
        },
        pools: {
          pool: createMetricPattern26(this, 'pool'),
          vecs: {
            aXbt: createAXbtPattern(this, 'axbt'),
            aaoPool: createAXbtPattern(this, 'aaopool'),
            antPool: createAXbtPattern(this, 'antpool'),
            arkPool: createAXbtPattern(this, 'arkpool'),
            asicMiner: createAXbtPattern(this, 'asicminer'),
            batPool: createAXbtPattern(this, 'batpool'),
            bcMonster: createAXbtPattern(this, 'bcmonster'),
            bcpoolIo: createAXbtPattern(this, 'bcpoolio'),
            binancePool: createAXbtPattern(this, 'binancepool'),
            bitClub: createAXbtPattern(this, 'bitclub'),
            bitFuFuPool: createAXbtPattern(this, 'bitfufupool'),
            bitFury: createAXbtPattern(this, 'bitfury'),
            bitMinter: createAXbtPattern(this, 'bitminter'),
            bitalo: createAXbtPattern(this, 'bitalo'),
            bitcoinAffiliateNetwork: createAXbtPattern(this, 'bitcoinaffiliatenetwork'),
            bitcoinCom: createAXbtPattern(this, 'bitcoincom'),
            bitcoinIndia: createAXbtPattern(this, 'bitcoinindia'),
            bitcoinRussia: createAXbtPattern(this, 'bitcoinrussia'),
            bitcoinUkraine: createAXbtPattern(this, 'bitcoinukraine'),
            bitfarms: createAXbtPattern(this, 'bitfarms'),
            bitparking: createAXbtPattern(this, 'bitparking'),
            bitsolo: createAXbtPattern(this, 'bitsolo'),
            bixin: createAXbtPattern(this, 'bixin'),
            blockFills: createAXbtPattern(this, 'blockfills'),
            braiinsPool: createAXbtPattern(this, 'braiinspool'),
            bravoMining: createAXbtPattern(this, 'bravomining'),
            btPool: createAXbtPattern(this, 'btpool'),
            btcCom: createAXbtPattern(this, 'btccom'),
            btcDig: createAXbtPattern(this, 'btcdig'),
            btcGuild: createAXbtPattern(this, 'btcguild'),
            btcLab: createAXbtPattern(this, 'btclab'),
            btcMp: createAXbtPattern(this, 'btcmp'),
            btcNuggets: createAXbtPattern(this, 'btcnuggets'),
            btcPoolParty: createAXbtPattern(this, 'btcpoolparty'),
            btcServ: createAXbtPattern(this, 'btcserv'),
            btcTop: createAXbtPattern(this, 'btctop'),
            btcc: createAXbtPattern(this, 'btcc'),
            bwPool: createAXbtPattern(this, 'bwpool'),
            bytePool: createAXbtPattern(this, 'bytepool'),
            canoe: createAXbtPattern(this, 'canoe'),
            canoePool: createAXbtPattern(this, 'canoepool'),
            carbonNegative: createAXbtPattern(this, 'carbonnegative'),
            ckPool: createAXbtPattern(this, 'ckpool'),
            cloudHashing: createAXbtPattern(this, 'cloudhashing'),
            coinLab: createAXbtPattern(this, 'coinlab'),
            cointerra: createAXbtPattern(this, 'cointerra'),
            connectBtc: createAXbtPattern(this, 'connectbtc'),
            dPool: createAXbtPattern(this, 'dpool'),
            dcExploration: createAXbtPattern(this, 'dcexploration'),
            dcex: createAXbtPattern(this, 'dcex'),
            digitalBtc: createAXbtPattern(this, 'digitalbtc'),
            digitalXMintsy: createAXbtPattern(this, 'digitalxmintsy'),
            eclipseMc: createAXbtPattern(this, 'eclipsemc'),
            eightBaochi: createAXbtPattern(this, 'eightbaochi'),
            ekanemBtc: createAXbtPattern(this, 'ekanembtc'),
            eligius: createAXbtPattern(this, 'eligius'),
            emcdPool: createAXbtPattern(this, 'emcdpool'),
            entrustCharityPool: createAXbtPattern(this, 'entrustcharitypool'),
            eobot: createAXbtPattern(this, 'eobot'),
            exxBw: createAXbtPattern(this, 'exxbw'),
            f2Pool: createAXbtPattern(this, 'f2pool'),
            fiftyEightCoin: createAXbtPattern(this, 'fiftyeightcoin'),
            foundryUsa: createAXbtPattern(this, 'foundryusa'),
            futureBitApolloSolo: createAXbtPattern(this, 'futurebitapollosolo'),
            gbMiners: createAXbtPattern(this, 'gbminers'),
            ghashIo: createAXbtPattern(this, 'ghashio'),
            giveMeCoins: createAXbtPattern(this, 'givemecoins'),
            goGreenLight: createAXbtPattern(this, 'gogreenlight'),
            haoZhuZhu: createAXbtPattern(this, 'haozhuzhu'),
            haominer: createAXbtPattern(this, 'haominer'),
            hashBx: createAXbtPattern(this, 'hashbx'),
            hashPool: createAXbtPattern(this, 'hashpool'),
            helix: createAXbtPattern(this, 'helix'),
            hhtt: createAXbtPattern(this, 'hhtt'),
            hotPool: createAXbtPattern(this, 'hotpool'),
            hummerpool: createAXbtPattern(this, 'hummerpool'),
            huobiPool: createAXbtPattern(this, 'huobipool'),
            innopolisTech: createAXbtPattern(this, 'innopolistech'),
            kanoPool: createAXbtPattern(this, 'kanopool'),
            kncMiner: createAXbtPattern(this, 'kncminer'),
            kuCoinPool: createAXbtPattern(this, 'kucoinpool'),
            lubianCom: createAXbtPattern(this, 'lubiancom'),
            luckyPool: createAXbtPattern(this, 'luckypool'),
            luxor: createAXbtPattern(this, 'luxor'),
            maraPool: createAXbtPattern(this, 'marapool'),
            maxBtc: createAXbtPattern(this, 'maxbtc'),
            maxiPool: createAXbtPattern(this, 'maxipool'),
            megaBigPower: createAXbtPattern(this, 'megabigpower'),
            minerium: createAXbtPattern(this, 'minerium'),
            miningCity: createAXbtPattern(this, 'miningcity'),
            miningDutch: createAXbtPattern(this, 'miningdutch'),
            miningKings: createAXbtPattern(this, 'miningkings'),
            miningSquared: createAXbtPattern(this, 'miningsquared'),
            mmpool: createAXbtPattern(this, 'mmpool'),
            mtRed: createAXbtPattern(this, 'mtred'),
            multiCoinCo: createAXbtPattern(this, 'multicoinco'),
            multipool: createAXbtPattern(this, 'multipool'),
            myBtcCoinPool: createAXbtPattern(this, 'mybtccoinpool'),
            neopool: createAXbtPattern(this, 'neopool'),
            nexious: createAXbtPattern(this, 'nexious'),
            niceHash: createAXbtPattern(this, 'nicehash'),
            nmcBit: createAXbtPattern(this, 'nmcbit'),
            novaBlock: createAXbtPattern(this, 'novablock'),
            ocean: createAXbtPattern(this, 'ocean'),
            okExPool: createAXbtPattern(this, 'okexpool'),
            okMiner: createAXbtPattern(this, 'okminer'),
            okkong: createAXbtPattern(this, 'okkong'),
            okpoolTop: createAXbtPattern(this, 'okpooltop'),
            oneHash: createAXbtPattern(this, 'onehash'),
            oneM1x: createAXbtPattern(this, 'onem1x'),
            oneThash: createAXbtPattern(this, 'onethash'),
            ozCoin: createAXbtPattern(this, 'ozcoin'),
            pHashIo: createAXbtPattern(this, 'phashio'),
            parasite: createAXbtPattern(this, 'parasite'),
            patels: createAXbtPattern(this, 'patels'),
            pegaPool: createAXbtPattern(this, 'pegapool'),
            phoenix: createAXbtPattern(this, 'phoenix'),
            polmine: createAXbtPattern(this, 'polmine'),
            pool175btc: createAXbtPattern(this, 'pool175btc'),
            pool50btc: createAXbtPattern(this, 'pool50btc'),
            poolin: createAXbtPattern(this, 'poolin'),
            portlandHodl: createAXbtPattern(this, 'portlandhodl'),
            publicPool: createAXbtPattern(this, 'publicpool'),
            pureBtcCom: createAXbtPattern(this, 'purebtccom'),
            rawpool: createAXbtPattern(this, 'rawpool'),
            rigPool: createAXbtPattern(this, 'rigpool'),
            sbiCrypto: createAXbtPattern(this, 'sbicrypto'),
            secPool: createAXbtPattern(this, 'secpool'),
            secretSuperstar: createAXbtPattern(this, 'secretsuperstar'),
            sevenPool: createAXbtPattern(this, 'sevenpool'),
            shawnP0wers: createAXbtPattern(this, 'shawnp0wers'),
            sigmapoolCom: createAXbtPattern(this, 'sigmapoolcom'),
            simplecoinUs: createAXbtPattern(this, 'simplecoinus'),
            soloCk: createAXbtPattern(this, 'solock'),
            spiderPool: createAXbtPattern(this, 'spiderpool'),
            stMiningCorp: createAXbtPattern(this, 'stminingcorp'),
            tangpool: createAXbtPattern(this, 'tangpool'),
            tatmasPool: createAXbtPattern(this, 'tatmaspool'),
            tbDice: createAXbtPattern(this, 'tbdice'),
            telco214: createAXbtPattern(this, 'telco214'),
            terraPool: createAXbtPattern(this, 'terrapool'),
            tiger: createAXbtPattern(this, 'tiger'),
            tigerpoolNet: createAXbtPattern(this, 'tigerpoolnet'),
            titan: createAXbtPattern(this, 'titan'),
            transactionCoinMining: createAXbtPattern(this, 'transactioncoinmining'),
            trickysBtcPool: createAXbtPattern(this, 'trickysbtcpool'),
            tripleMining: createAXbtPattern(this, 'triplemining'),
            twentyOneInc: createAXbtPattern(this, 'twentyoneinc'),
            ultimusPool: createAXbtPattern(this, 'ultimuspool'),
            unknown: createAXbtPattern(this, 'unknown'),
            unomp: createAXbtPattern(this, 'unomp'),
            viaBtc: createAXbtPattern(this, 'viabtc'),
            waterhole: createAXbtPattern(this, 'waterhole'),
            wayiCn: createAXbtPattern(this, 'wayicn'),
            whitePool: createAXbtPattern(this, 'whitepool'),
            wk057: createAXbtPattern(this, 'wk057'),
            yourbtcNet: createAXbtPattern(this, 'yourbtcnet'),
            zulupool: createAXbtPattern(this, 'zulupool')
          }
        },
        positions: {
          position: createMetricPattern17(this, 'position')
        },
        price: {
          ohlc: {
            ohlcInCents: createMetricPattern10(this, 'ohlc_in_cents')
          },
          sats: {
            priceCloseInSats: createMetricPattern1(this, 'price_close_in_sats'),
            priceHighInSats: createPriceHighInSatsPattern(this, 'price_high_in_sats'),
            priceLowInSats: createPriceLowInSatsPattern(this, 'price_low_in_sats'),
            priceOhlcInSats: createMetricPattern1(this, 'price_ohlc_in_sats'),
            priceOpenInSats: createMetricPattern1(this, 'price_open_in_sats')
          },
          usd: {
            priceClose: createMetricPattern1(this, 'price_close'),
            priceCloseInCents: createMetricPattern10(this, 'price_close_in_cents'),
            priceHigh: createPriceHighInSatsPattern(this, 'price_high'),
            priceHighInCents: createMetricPattern10(this, 'price_high_in_cents'),
            priceLow: createPriceLowInSatsPattern(this, 'price_low'),
            priceLowInCents: createMetricPattern10(this, 'price_low_in_cents'),
            priceOhlc: createMetricPattern1(this, 'price_ohlc'),
            priceOpen: createMetricPattern1(this, 'price_open'),
            priceOpenInCents: createMetricPattern10(this, 'price_open_in_cents')
          }
        },
        scripts: {
          count: {
            emptyoutputCount: createDollarsPattern(this, 'emptyoutput_count'),
            opreturnCount: createDollarsPattern(this, 'opreturn_count'),
            p2aCount: createDollarsPattern(this, 'p2a_count'),
            p2msCount: createDollarsPattern(this, 'p2ms_count'),
            p2pk33Count: createDollarsPattern(this, 'p2pk33_count'),
            p2pk65Count: createDollarsPattern(this, 'p2pk65_count'),
            p2pkhCount: createDollarsPattern(this, 'p2pkh_count'),
            p2shCount: createDollarsPattern(this, 'p2sh_count'),
            p2trCount: createDollarsPattern(this, 'p2tr_count'),
            p2wpkhCount: createDollarsPattern(this, 'p2wpkh_count'),
            p2wshCount: createDollarsPattern(this, 'p2wsh_count'),
            segwitAdoption: createBlockCountPattern(this, 'segwit_adoption'),
            segwitCount: createDollarsPattern(this, 'segwit_count'),
            taprootAdoption: createBlockCountPattern(this, 'taproot_adoption'),
            unknownoutputCount: createDollarsPattern(this, 'unknownoutput_count')
          },
          value: {
            opreturnValue: createCoinbasePattern(this, 'opreturn_value')
          }
        },
        supply: {
          burned: {
            opreturn: createUnclaimedRewardsPattern(this, 'opreturn_supply'),
            unspendable: createUnclaimedRewardsPattern(this, 'unspendable_supply')
          },
          circulating: {
            btc: createMetricPattern26(this, 'circulating_btc'),
            indexes: createActiveSupplyPattern(this, 'circulating'),
            sats: createMetricPattern26(this, 'circulating_sats'),
            usd: createMetricPattern26(this, 'circulating_usd')
          },
          inflation: {
            indexes: createIndexesPattern2(this, 'inflation_rate')
          },
          marketCap: {
            height: createMetricPattern26(this, 'market_cap'),
            indexes: createMetricPattern5(this, 'market_cap')
          },
          velocity: {
            btc: createIndexesPattern2(this, 'btc_velocity'),
            usd: createIndexesPattern2(this, 'usd_velocity')
          }
        },
        transactions: {
          count: {
            isCoinbase: createMetricPattern39(this, 'is_coinbase'),
            txCount: createDollarsPattern(this, 'tx_count')
          },
          fees: {
            fee: {
              base: createMetricPattern39(this, 'fee'),
              bitcoin: {
                average: createMetricPattern1(this, 'fee_btc_average'),
                cumulative: createMetricPattern1(this, 'fee_btc_cum'),
                max: createMetricPattern1(this, 'fee_btc_max'),
                min: createMetricPattern1(this, 'fee_btc_min'),
                sum: createMetricPattern1(this, 'fee_btc_sum')
              },
              dollars: createCountPattern2(this, 'fee_usd'),
              sats: createCountPattern2(this, 'fee')
            },
            feeRate: {
              average: createMetricPattern1(this, 'fee_rate_average'),
              base: createMetricPattern39(this, 'fee_rate'),
              max: createMetricPattern1(this, 'fee_rate_max'),
              min: createMetricPattern1(this, 'fee_rate_min'),
              percentiles: createPercentilesPattern(this, 'fee_rate')
            },
            inputValue: createMetricPattern39(this, 'input_value'),
            outputValue: createMetricPattern39(this, 'output_value')
          },
          size: {
            txVsize: createBlockIntervalPattern(this, 'tx_vsize'),
            txWeight: createBlockIntervalPattern(this, 'tx_weight'),
            vsize: createMetricPattern39(this, 'vsize'),
            weight: createMetricPattern39(this, 'weight')
          },
          versions: {
            txV1: createBlockCountPattern(this, 'tx_v1'),
            txV2: createBlockCountPattern(this, 'tx_v2'),
            txV3: createBlockCountPattern(this, 'tx_v3')
          },
          volume: {
            annualizedVolume: createMetricPattern5(this, 'annualized_volume'),
            annualizedVolumeBtc: createMetricPattern5(this, 'annualized_volume_btc'),
            annualizedVolumeUsd: createMetricPattern5(this, 'annualized_volume_usd'),
            inputsPerSec: createMetricPattern5(this, 'inputs_per_sec'),
            outputsPerSec: createMetricPattern5(this, 'outputs_per_sec'),
            sentSum: {
              bitcoin: createMetricPattern1(this, 'sent_sum_btc'),
              dollars: createDifficultyAdjustmentPattern(this, 'sent_sum_usd'),
              sats: createDifficultyAdjustmentPattern(this, 'sent_sum')
            },
            txPerSec: createMetricPattern5(this, 'tx_per_sec')
          }
        }
      },
      indexed: {
        address: {
          firstP2aaddressindex: createMetricPattern26(this, 'first_p2aaddressindex'),
          firstP2pk33addressindex: createMetricPattern26(this, 'first_p2pk33addressindex'),
          firstP2pk65addressindex: createMetricPattern26(this, 'first_p2pk65addressindex'),
          firstP2pkhaddressindex: createMetricPattern26(this, 'first_p2pkhaddressindex'),
          firstP2shaddressindex: createMetricPattern26(this, 'first_p2shaddressindex'),
          firstP2traddressindex: createMetricPattern26(this, 'first_p2traddressindex'),
          firstP2wpkhaddressindex: createMetricPattern26(this, 'first_p2wpkhaddressindex'),
          firstP2wshaddressindex: createMetricPattern26(this, 'first_p2wshaddressindex'),
          p2abytes: createMetricPattern30(this, 'p2abytes'),
          p2pk33bytes: createMetricPattern32(this, 'p2pk33bytes'),
          p2pk65bytes: createMetricPattern33(this, 'p2pk65bytes'),
          p2pkhbytes: createMetricPattern34(this, 'p2pkhbytes'),
          p2shbytes: createMetricPattern35(this, 'p2shbytes'),
          p2trbytes: createMetricPattern36(this, 'p2trbytes'),
          p2wpkhbytes: createMetricPattern37(this, 'p2wpkhbytes'),
          p2wshbytes: createMetricPattern38(this, 'p2wshbytes')
        },
        block: {
          blockhash: createMetricPattern26(this, 'blockhash'),
          difficulty: createMetricPattern26(this, 'difficulty'),
          timestamp: createMetricPattern26(this, 'timestamp'),
          totalSize: createMetricPattern26(this, 'total_size'),
          weight: createMetricPattern26(this, 'weight')
        },
        output: {
          firstEmptyoutputindex: createMetricPattern26(this, 'first_emptyoutputindex'),
          firstOpreturnindex: createMetricPattern26(this, 'first_opreturnindex'),
          firstP2msoutputindex: createMetricPattern26(this, 'first_p2msoutputindex'),
          firstUnknownoutputindex: createMetricPattern26(this, 'first_unknownoutputindex'),
          txindex: createMetricPattern8(this, 'txindex')
        },
        tx: {
          baseSize: createMetricPattern39(this, 'base_size'),
          firstTxindex: createMetricPattern26(this, 'first_txindex'),
          firstTxinindex: createMetricPattern39(this, 'first_txinindex'),
          firstTxoutindex: createMetricPattern39(this, 'first_txoutindex'),
          height: createMetricPattern39(this, 'height'),
          isExplicitlyRbf: createMetricPattern39(this, 'is_explicitly_rbf'),
          rawlocktime: createMetricPattern39(this, 'rawlocktime'),
          totalSize: createMetricPattern39(this, 'total_size'),
          txid: createMetricPattern39(this, 'txid'),
          txversion: createMetricPattern39(this, 'txversion')
        },
        txin: {
          firstTxinindex: createMetricPattern26(this, 'first_txinindex'),
          outpoint: createMetricPattern27(this, 'outpoint'),
          outputtype: createMetricPattern27(this, 'outputtype'),
          txindex: createMetricPattern27(this, 'txindex'),
          typeindex: createMetricPattern27(this, 'typeindex')
        },
        txout: {
          firstTxoutindex: createMetricPattern26(this, 'first_txoutindex'),
          outputtype: createMetricPattern29(this, 'outputtype'),
          txindex: createMetricPattern29(this, 'txindex'),
          typeindex: createMetricPattern29(this, 'typeindex'),
          value: createMetricPattern29(this, 'value')
        }
      }
    };
  }

  /**
   * Address information
   * @description Retrieve comprehensive information about a Bitcoin address including balance, transaction history, UTXOs, and estimated investment metrics. Supports all standard Bitcoin address types (P2PKH, P2SH, P2WPKH, P2WSH, P2TR, etc.).
   * @param {Address} address 
   * @returns {Promise<AddressStats>}
   */
  async getAddress(address) {
    return this.get(`/api/address/${address}`);
  }

  /**
   * Address transaction IDs
   * @description Get transaction IDs for an address, newest first. Use after_txid for pagination.
   * @param {Address} address 
   * @param {string=} [after_txid] Txid to paginate from (return transactions before this one)
   * @param {number=} [limit] Maximum number of results to return. Defaults to 25 if not specified.
   * @returns {Promise<Txid[]>}
   */
  async getAddressTxs(address, after_txid, limit) {
    const params = new URLSearchParams();
    if (after_txid !== undefined) params.set('after_txid', String(after_txid));
    if (limit !== undefined) params.set('limit', String(limit));
    const query = params.toString();
    return this.get(`/api/address/${address}/txs${query ? '?' + query : ''}`);
  }

  /**
   * Address confirmed transactions
   * @description Get confirmed transaction IDs for an address, 25 per page. Use ?after_txid=<txid> for pagination.
   * @param {Address} address 
   * @param {string=} [after_txid] Txid to paginate from (return transactions before this one)
   * @param {number=} [limit] Maximum number of results to return. Defaults to 25 if not specified.
   * @returns {Promise<Txid[]>}
   */
  async getAddressTxsChain(address, after_txid, limit) {
    const params = new URLSearchParams();
    if (after_txid !== undefined) params.set('after_txid', String(after_txid));
    if (limit !== undefined) params.set('limit', String(limit));
    const query = params.toString();
    return this.get(`/api/address/${address}/txs/chain${query ? '?' + query : ''}`);
  }

  /**
   * Address mempool transactions
   * @description Get unconfirmed transaction IDs for an address from the mempool (up to 50).
   * @param {Address} address 
   * @returns {Promise<Txid[]>}
   */
  async getAddressTxsMempool(address) {
    return this.get(`/api/address/${address}/txs/mempool`);
  }

  /**
   * Address UTXOs
   * @description Get unspent transaction outputs for an address.
   * @param {Address} address 
   * @returns {Promise<Utxo[]>}
   */
  async getAddressUtxo(address) {
    return this.get(`/api/address/${address}/utxo`);
  }

  /**
   * Block by height
   * @description Retrieve block information by block height. Returns block metadata including hash, timestamp, difficulty, size, weight, and transaction count.
   * @param {Height} height 
   * @returns {Promise<BlockInfo>}
   */
  async getBlockHeight(height) {
    return this.get(`/api/block-height/${height}`);
  }

  /**
   * Block information
   * @description Retrieve block information by block hash. Returns block metadata including height, timestamp, difficulty, size, weight, and transaction count.
   * @param {BlockHash} hash 
   * @returns {Promise<BlockInfo>}
   */
  async getBlockByHash(hash) {
    return this.get(`/api/block/${hash}`);
  }

  /**
   * Raw block
   * @description Returns the raw block data in binary format.
   * @param {BlockHash} hash 
   * @returns {Promise<number[]>}
   */
  async getBlockByHashRaw(hash) {
    return this.get(`/api/block/${hash}/raw`);
  }

  /**
   * Block status
   * @description Retrieve the status of a block. Returns whether the block is in the best chain and, if so, its height and the hash of the next block.
   * @param {BlockHash} hash 
   * @returns {Promise<BlockStatus>}
   */
  async getBlockByHashStatus(hash) {
    return this.get(`/api/block/${hash}/status`);
  }

  /**
   * Transaction ID at index
   * @description Retrieve a single transaction ID at a specific index within a block. Returns plain text txid.
   * @param {BlockHash} hash Bitcoin block hash
   * @param {TxIndex} index Transaction index within the block (0-based)
   * @returns {Promise<Txid>}
   */
  async getBlockByHashTxidByIndex(hash, index) {
    return this.get(`/api/block/${hash}/txid/${index}`);
  }

  /**
   * Block transaction IDs
   * @description Retrieve all transaction IDs in a block by block hash.
   * @param {BlockHash} hash 
   * @returns {Promise<Txid[]>}
   */
  async getBlockByHashTxids(hash) {
    return this.get(`/api/block/${hash}/txids`);
  }

  /**
   * Block transactions (paginated)
   * @description Retrieve transactions in a block by block hash, starting from the specified index. Returns up to 25 transactions at a time.
   * @param {BlockHash} hash Bitcoin block hash
   * @param {TxIndex} start_index Starting transaction index within the block (0-based)
   * @returns {Promise<Transaction[]>}
   */
  async getBlockByHashTxsByStartIndex(hash, start_index) {
    return this.get(`/api/block/${hash}/txs/${start_index}`);
  }

  /**
   * Recent blocks
   * @description Retrieve the last 10 blocks. Returns block metadata for each block.
   * @returns {Promise<BlockInfo[]>}
   */
  async getBlocks() {
    return this.get(`/api/blocks`);
  }

  /**
   * Blocks from height
   * @description Retrieve up to 10 blocks going backwards from the given height. For example, height=100 returns blocks 100, 99, 98, ..., 91. Height=0 returns only block 0.
   * @param {Height} height 
   * @returns {Promise<BlockInfo[]>}
   */
  async getBlocksByHeight(height) {
    return this.get(`/api/blocks/${height}`);
  }

  /**
   * Mempool statistics
   * @description Get current mempool statistics including transaction count, total vsize, and total fees.
   * @returns {Promise<MempoolInfo>}
   */
  async getMempoolInfo() {
    return this.get(`/api/mempool/info`);
  }

  /**
   * Mempool transaction IDs
   * @description Get all transaction IDs currently in the mempool.
   * @returns {Promise<Txid[]>}
   */
  async getMempoolTxids() {
    return this.get(`/api/mempool/txids`);
  }

  /**
   * Get supported indexes for a metric
   * @description Returns the list of indexes are supported by the specified metric. For example, `realized_price` might be available on dateindex, weekindex, and monthindex.
   * @param {Metric} metric 
   * @returns {Promise<Index[]>}
   */
  async getMetric(metric) {
    return this.get(`/api/metric/${metric}`);
  }

  /**
   * Get metric data
   * @description Fetch data for a specific metric at the given index. Use query parameters to filter by date range and format (json/csv).
   * @param {Metric} metric Metric name
   * @param {Index} index Aggregation index
   * @param {*=} [from] Inclusive starting index, if negative counts from end
   * @param {*=} [to] Exclusive ending index, if negative counts from end
   * @param {*=} [count] Number of values to return (ignored if `to` is set)
   * @param {Format=} [format] Format of the output
   * @returns {Promise<AnyMetricData>}
   */
  async getMetricByIndex(metric, index, from, to, count, format) {
    const params = new URLSearchParams();
    if (from !== undefined) params.set('from', String(from));
    if (to !== undefined) params.set('to', String(to));
    if (count !== undefined) params.set('count', String(count));
    if (format !== undefined) params.set('format', String(format));
    const query = params.toString();
    return this.get(`/api/metric/${metric}/${index}${query ? '?' + query : ''}`);
  }

  /**
   * Bulk metric data
   * @description Fetch multiple metrics in a single request. Supports filtering by index and date range. Returns an array of MetricData objects.
   * @param {Metrics} [metrics] Requested metrics
   * @param {Index} [index] Index to query
   * @param {*=} [from] Inclusive starting index, if negative counts from end
   * @param {*=} [to] Exclusive ending index, if negative counts from end
   * @param {*=} [count] Number of values to return (ignored if `to` is set)
   * @param {Format=} [format] Format of the output
   * @returns {Promise<AnyMetricData[]>}
   */
  async getMetricsBulk(metrics, index, from, to, count, format) {
    const params = new URLSearchParams();
    params.set('metrics', String(metrics));
    params.set('index', String(index));
    if (from !== undefined) params.set('from', String(from));
    if (to !== undefined) params.set('to', String(to));
    if (count !== undefined) params.set('count', String(count));
    if (format !== undefined) params.set('format', String(format));
    const query = params.toString();
    return this.get(`/api/metrics/bulk${query ? '?' + query : ''}`);
  }

  /**
   * Metrics catalog
   * @description Returns the complete hierarchical catalog of available metrics organized as a tree structure. Metrics are grouped by categories and subcategories. Best viewed in an interactive JSON viewer (e.g., Firefox's built-in JSON viewer) for easy navigation of the nested structure.
   * @returns {Promise<TreeNode>}
   */
  async getMetricsCatalog() {
    return this.get(`/api/metrics/catalog`);
  }

  /**
   * Metric count
   * @description Current metric count
   * @returns {Promise<MetricCount[]>}
   */
  async getMetricsCount() {
    return this.get(`/api/metrics/count`);
  }

  /**
   * List available indexes
   * @description Returns all available indexes with their accepted query aliases. Use any alias when querying metrics.
   * @returns {Promise<IndexInfo[]>}
   */
  async getMetricsIndexes() {
    return this.get(`/api/metrics/indexes`);
  }

  /**
   * Metrics list
   * @description Paginated list of available metrics
   * @param {*=} [page] Pagination index
   * @returns {Promise<PaginatedMetrics>}
   */
  async getMetricsList(page) {
    const params = new URLSearchParams();
    if (page !== undefined) params.set('page', String(page));
    const query = params.toString();
    return this.get(`/api/metrics/list${query ? '?' + query : ''}`);
  }

  /**
   * Search metrics
   * @description Fuzzy search for metrics by name. Supports partial matches and typos.
   * @param {Metric} metric 
   * @param {Limit=} [limit] 
   * @returns {Promise<Metric[]>}
   */
  async getMetricsSearchByMetric(metric, limit) {
    const params = new URLSearchParams();
    if (limit !== undefined) params.set('limit', String(limit));
    const query = params.toString();
    return this.get(`/api/metrics/search/${metric}${query ? '?' + query : ''}`);
  }

  /**
   * Transaction information
   * @description Retrieve complete transaction data by transaction ID (txid). Returns the full transaction details including inputs, outputs, and metadata. The transaction data is read directly from the blockchain data files.
   * @param {Txid} txid 
   * @returns {Promise<Transaction>}
   */
  async getTxByTxid(txid) {
    return this.get(`/api/tx/${txid}`);
  }

  /**
   * Transaction hex
   * @description Retrieve the raw transaction as a hex-encoded string. Returns the serialized transaction in hexadecimal format.
   * @param {Txid} txid 
   * @returns {Promise<Hex>}
   */
  async getTxByTxidHex(txid) {
    return this.get(`/api/tx/${txid}/hex`);
  }

  /**
   * Output spend status
   * @description Get the spending status of a transaction output. Returns whether the output has been spent and, if so, the spending transaction details.
   * @param {Txid} txid Transaction ID
   * @param {Vout} vout Output index
   * @returns {Promise<TxOutspend>}
   */
  async getTxByTxidOutspendByVout(txid, vout) {
    return this.get(`/api/tx/${txid}/outspend/${vout}`);
  }

  /**
   * All output spend statuses
   * @description Get the spending status of all outputs in a transaction. Returns an array with the spend status for each output.
   * @param {Txid} txid 
   * @returns {Promise<TxOutspend[]>}
   */
  async getTxByTxidOutspends(txid) {
    return this.get(`/api/tx/${txid}/outspends`);
  }

  /**
   * Transaction status
   * @description Retrieve the confirmation status of a transaction. Returns whether the transaction is confirmed and, if so, the block height, hash, and timestamp.
   * @param {Txid} txid 
   * @returns {Promise<TxStatus>}
   */
  async getTxByTxidStatus(txid) {
    return this.get(`/api/tx/${txid}/status`);
  }

  /**
   * Difficulty adjustment
   * @description Get current difficulty adjustment information including progress through the current epoch, estimated retarget date, and difficulty change prediction.
   * @returns {Promise<DifficultyAdjustment>}
   */
  async getV1DifficultyAdjustment() {
    return this.get(`/api/v1/difficulty-adjustment`);
  }

  /**
   * Projected mempool blocks
   * @description Get projected blocks from the mempool for fee estimation. Each block contains statistics about transactions that would be included if a block were mined now.
   * @returns {Promise<MempoolBlock[]>}
   */
  async getV1FeesMempoolBlocks() {
    return this.get(`/api/v1/fees/mempool-blocks`);
  }

  /**
   * Recommended fees
   * @description Get recommended fee rates for different confirmation targets based on current mempool state.
   * @returns {Promise<RecommendedFees>}
   */
  async getV1FeesRecommended() {
    return this.get(`/api/v1/fees/recommended`);
  }

  /**
   * Block fees
   * @description Get average block fees for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y
   * @param {TimePeriod} time_period 
   * @returns {Promise<BlockFeesEntry[]>}
   */
  async getV1MiningBlocksFeesByTimePeriod(time_period) {
    return this.get(`/api/v1/mining/blocks/fees/${time_period}`);
  }

  /**
   * Block rewards
   * @description Get average block rewards (coinbase = subsidy + fees) for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y
   * @param {TimePeriod} time_period 
   * @returns {Promise<BlockRewardsEntry[]>}
   */
  async getV1MiningBlocksRewardsByTimePeriod(time_period) {
    return this.get(`/api/v1/mining/blocks/rewards/${time_period}`);
  }

  /**
   * Block sizes and weights
   * @description Get average block sizes and weights for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y
   * @param {TimePeriod} time_period 
   * @returns {Promise<BlockSizesWeights>}
   */
  async getV1MiningBlocksSizesWeightsByTimePeriod(time_period) {
    return this.get(`/api/v1/mining/blocks/sizes-weights/${time_period}`);
  }

  /**
   * Block by timestamp
   * @description Find the block closest to a given UNIX timestamp.
   * @param {Timestamp} timestamp 
   * @returns {Promise<BlockTimestamp>}
   */
  async getV1MiningBlocksTimestamp(timestamp) {
    return this.get(`/api/v1/mining/blocks/timestamp/${timestamp}`);
  }

  /**
   * Difficulty adjustments (all time)
   * @description Get historical difficulty adjustments. Returns array of [timestamp, height, difficulty, change_percent].
   * @returns {Promise<DifficultyAdjustmentEntry[]>}
   */
  async getV1MiningDifficultyAdjustments() {
    return this.get(`/api/v1/mining/difficulty-adjustments`);
  }

  /**
   * Difficulty adjustments
   * @description Get historical difficulty adjustments for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y. Returns array of [timestamp, height, difficulty, change_percent].
   * @param {TimePeriod} time_period 
   * @returns {Promise<DifficultyAdjustmentEntry[]>}
   */
  async getV1MiningDifficultyAdjustmentsByTimePeriod(time_period) {
    return this.get(`/api/v1/mining/difficulty-adjustments/${time_period}`);
  }

  /**
   * Network hashrate (all time)
   * @description Get network hashrate and difficulty data for all time.
   * @returns {Promise<HashrateSummary>}
   */
  async getV1MiningHashrate() {
    return this.get(`/api/v1/mining/hashrate`);
  }

  /**
   * Network hashrate
   * @description Get network hashrate and difficulty data for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y
   * @param {TimePeriod} time_period 
   * @returns {Promise<HashrateSummary>}
   */
  async getV1MiningHashrateByTimePeriod(time_period) {
    return this.get(`/api/v1/mining/hashrate/${time_period}`);
  }

  /**
   * Mining pool details
   * @description Get detailed information about a specific mining pool including block counts and shares for different time periods.
   * @param {PoolSlug} slug 
   * @returns {Promise<PoolDetail>}
   */
  async getV1MiningPoolBySlug(slug) {
    return this.get(`/api/v1/mining/pool/${slug}`);
  }

  /**
   * List all mining pools
   * @description Get list of all known mining pools with their identifiers.
   * @returns {Promise<PoolInfo[]>}
   */
  async getV1MiningPools() {
    return this.get(`/api/v1/mining/pools`);
  }

  /**
   * Mining pool statistics
   * @description Get mining pool statistics for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y
   * @param {TimePeriod} time_period 
   * @returns {Promise<PoolsSummary>}
   */
  async getV1MiningPoolsByTimePeriod(time_period) {
    return this.get(`/api/v1/mining/pools/${time_period}`);
  }

  /**
   * Mining reward statistics
   * @description Get mining reward statistics for the last N blocks including total rewards, fees, and transaction count.
   * @param {number} block_count Number of recent blocks to include
   * @returns {Promise<RewardStats>}
   */
  async getV1MiningRewardStatsByBlockCount(block_count) {
    return this.get(`/api/v1/mining/reward-stats/${block_count}`);
  }

  /**
   * Validate address
   * @description Validate a Bitcoin address and get information about its type and scriptPubKey.
   * @param {string} address Bitcoin address to validate (can be any string)
   * @returns {Promise<AddressValidation>}
   */
  async getV1ValidateAddress(address) {
    return this.get(`/api/v1/validate-address/${address}`);
  }

  /**
   * Health check
   * @description Returns the health status of the API server
   * @returns {Promise<Health>}
   */
  async getHealth() {
    return this.get(`/health`);
  }

  /**
   * API version
   * @description Returns the current version of the API server
   * @returns {Promise<string>}
   */
  async getVersion() {
    return this.get(`/version`);
  }


  /**
   * Merge multiple MetricPatterns into a single pattern.
   * Throws if any two patterns have overlapping indexes.
   * @template T
   * @param {...MetricPattern<T>} patterns - The patterns to merge
   * @returns {MetricPattern<T>} A new merged pattern
   */
  mergeMetricPatterns(...patterns) {
    if (patterns.length === 0) {
      throw new BrkError('mergeMetricPatterns requires at least one pattern');
    }
    if (patterns.length === 1) {
      return patterns[0];
    }

    const seenIndexes = /** @type {Map<Index, string>} */ (new Map());
    const mergedBy = /** @type {Partial<Record<Index, MetricEndpoint<T>>>} */ ({});

    for (const pattern of patterns) {
      for (const index of pattern.indexes()) {
        const existing = seenIndexes.get(index);
        if (existing !== undefined) {
          throw new BrkError(`Index '${index}' exists in both '${existing}' and '${pattern.name}'`);
        }
        seenIndexes.set(index, pattern.name);
        Object.defineProperty(mergedBy, index, {
          get() { return pattern.get(index); },
          enumerable: true,
          configurable: true,
        });
      }
    }

    const allIndexes = /** @type {Index[]} */ ([...seenIndexes.keys()]);
    const firstName = patterns[0].name;

    return {
      name: firstName,
      by: mergedBy,
      indexes() { return allIndexes; },
      get(index) { return mergedBy[index]; },
    };
  }

}

export { BrkClient, BrkError };
