// Auto-generated BRK JavaScript client
// Do not edit manually

// Type definitions

/** @typedef {string} Address */
/**
 * @typedef {Object} AddressChainStats
 * @property {number} funded_txo_count
 * @property {Sats} funded_txo_sum
 * @property {number} spent_txo_count
 * @property {Sats} spent_txo_sum
 * @property {number} tx_count
 * @property {TypeIndex} type_index
 */
/**
 * @typedef {Object} AddressMempoolStats
 * @property {number} funded_txo_count
 * @property {Sats} funded_txo_sum
 * @property {number} spent_txo_count
 * @property {Sats} spent_txo_sum
 * @property {number} tx_count
 */
/**
 * @typedef {Object} AddressParam
 * @property {Address} address
 */
/**
 * @typedef {Object} AddressStats
 * @property {Address} address
 * @property {AddressChainStats} chain_stats
 * @property {(AddressMempoolStats|null)=} mempool_stats
 */
/**
 * @typedef {Object} AddressTxidsParam
 * @property {(Txid|null)=} after_txid
 * @property {number=} limit
 */
/**
 * @typedef {Object} AddressValidation
 * @property {boolean} isvalid
 * @property {?string=} address
 * @property {?string=} scriptPubKey
 * @property {?boolean=} isscript
 * @property {?boolean=} iswitness
 * @property {?number=} witness_version
 * @property {?string=} witness_program
 */
/** @typedef {TypeIndex} AnyAddressIndex */
/** @typedef {number} Bitcoin */
/** @typedef {number} BlkPosition */
/**
 * @typedef {Object} BlockCountParam
 * @property {number} block_count
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
 * @property {TxIndex} start_index
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
 * @property {number} tx_count
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
 * @property {boolean} in_best_chain
 * @property {(Height|null)=} height
 * @property {(BlockHash|null)=} next_best
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
 * @property {number} change_percent
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
 * @property {number} tx_count
 * @property {number} funded_txo_count
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
 * @property {number} tx_count
 * @property {number} funded_txo_count
 * @property {number} spent_txo_count
 * @property {Sats} received
 * @property {Sats} sent
 * @property {Dollars} realized_cap
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
 * @property {Sats} total_fee
 */
/** @typedef {string} Metric */
/**
 * @typedef {Object} MetricCount
 * @property {number} distinct_metrics
 * @property {number} total_endpoints
 */
/**
 * @typedef {Object} MetricData
 * @property {number} total
 * @property {number} from
 * @property {number} to
 * @property {*[]} data
 */
/**
 * @typedef {Object} MetricLeafWithSchema
 * @property {string} name
 * @property {string} value_type
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
 * @property {number} current_page
 * @property {number} max_page
 * @property {string[]} metrics
 */
/**
 * @typedef {Object} Pagination
 * @property {?number=} page
 */
/**
 * @typedef {Object} PoolBlockCounts
 * @property {number} all
 * @property {number} 24h
 * @property {number} 1w
 */
/**
 * @typedef {Object} PoolBlockShares
 * @property {number} all
 * @property {number} 24h
 * @property {number} 1w
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
 * @property {number} unique_id
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
 * @property {number} utxo_count
 * @property {Sats} value
 */
/** @typedef {("24h"|"3d"|"1w"|"1m"|"3m"|"6m"|"1y"|"2y"|"3y")} TimePeriod */
/**
 * @typedef {Object} TimePeriodParam
 * @property {TimePeriod} time_period
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
 * @property {string} scriptsig_asm
 * @property {boolean} is_coinbase
 * @property {number} sequence
 * @property {?string=} inner_redeemscript_asm
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
 * @property {(Height|null)=} block_height
 * @property {(BlockHash|null)=} block_hash
 * @property {(Timestamp|null)=} block_time
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
const _runIdle = (fn) => (globalThis.requestIdleCallback ?? setTimeout)(fn);

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
 * A metric node that can fetch data for different indexes.
 * @template T
 */
class MetricNode {
  /**
   * @param {BrkClientBase} client
   * @param {string} path
   */
  constructor(client, path) {
    this._client = client;
    this._path = path;
  }

  /**
   * Fetch all data points for this metric.
   * @param {(value: T[]) => void} [onUpdate] - Called when data is available (may be called twice: cache then fresh)
   * @returns {Promise<T[] | null>}
   */
  get(onUpdate) {
    return this._client.get(this._path, onUpdate);
  }

  /**
   * Fetch data points within a range.
   * @param {string | number} from
   * @param {string | number} to
   * @param {(value: T[]) => void} [onUpdate] - Called when data is available (may be called twice: cache then fresh)
   * @returns {Promise<T[] | null>}
   */
  getRange(from, to, onUpdate) {
    return this._client.get(`${this._path}?from=${from}&to=${to}`, onUpdate);
  }
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
    const url = `${this.baseUrl}${path}`;
    const cache = await _cachePromise;
    const cachedRes = await cache?.match(url);
    const cachedJson = cachedRes ? await cachedRes.json() : null;

    if (cachedJson) onUpdate?.(cachedJson);
    if (!globalThis.navigator?.onLine) {
      if (cachedJson) return cachedJson;
      throw new BrkError('Offline and no cached data', 0);
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


// Index accessor factory functions

/**
 * @template T
 * @typedef {Object} Indexes3
 * @property {MetricNode<T>} byDateindex
 * @property {MetricNode<T>} byDecadeindex
 * @property {MetricNode<T>} byDifficultyepoch
 * @property {MetricNode<T>} byHeight
 * @property {MetricNode<T>} byMonthindex
 * @property {MetricNode<T>} byQuarterindex
 * @property {MetricNode<T>} bySemesterindex
 * @property {MetricNode<T>} byWeekindex
 * @property {MetricNode<T>} byYearindex
 */

/**
 * Create a Indexes3 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {Indexes3<T>}
 */
function createIndexes3(client, basePath) {
  return {
    byDateindex: new MetricNode(client, `${basePath}/dateindex`),
    byDecadeindex: new MetricNode(client, `${basePath}/decadeindex`),
    byDifficultyepoch: new MetricNode(client, `${basePath}/difficultyepoch`),
    byHeight: new MetricNode(client, `${basePath}/height`),
    byMonthindex: new MetricNode(client, `${basePath}/monthindex`),
    byQuarterindex: new MetricNode(client, `${basePath}/quarterindex`),
    bySemesterindex: new MetricNode(client, `${basePath}/semesterindex`),
    byWeekindex: new MetricNode(client, `${basePath}/weekindex`),
    byYearindex: new MetricNode(client, `${basePath}/yearindex`)
  };
}

/**
 * @template T
 * @typedef {Object} Indexes4
 * @property {MetricNode<T>} byDateindex
 * @property {MetricNode<T>} byDecadeindex
 * @property {MetricNode<T>} byDifficultyepoch
 * @property {MetricNode<T>} byMonthindex
 * @property {MetricNode<T>} byQuarterindex
 * @property {MetricNode<T>} bySemesterindex
 * @property {MetricNode<T>} byWeekindex
 * @property {MetricNode<T>} byYearindex
 */

/**
 * Create a Indexes4 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {Indexes4<T>}
 */
function createIndexes4(client, basePath) {
  return {
    byDateindex: new MetricNode(client, `${basePath}/dateindex`),
    byDecadeindex: new MetricNode(client, `${basePath}/decadeindex`),
    byDifficultyepoch: new MetricNode(client, `${basePath}/difficultyepoch`),
    byMonthindex: new MetricNode(client, `${basePath}/monthindex`),
    byQuarterindex: new MetricNode(client, `${basePath}/quarterindex`),
    bySemesterindex: new MetricNode(client, `${basePath}/semesterindex`),
    byWeekindex: new MetricNode(client, `${basePath}/weekindex`),
    byYearindex: new MetricNode(client, `${basePath}/yearindex`)
  };
}

/**
 * @template T
 * @typedef {Object} Indexes21
 * @property {MetricNode<T>} byDateindex
 * @property {MetricNode<T>} byDecadeindex
 * @property {MetricNode<T>} byHeight
 * @property {MetricNode<T>} byMonthindex
 * @property {MetricNode<T>} byQuarterindex
 * @property {MetricNode<T>} bySemesterindex
 * @property {MetricNode<T>} byWeekindex
 * @property {MetricNode<T>} byYearindex
 */

/**
 * Create a Indexes21 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {Indexes21<T>}
 */
function createIndexes21(client, basePath) {
  return {
    byDateindex: new MetricNode(client, `${basePath}/dateindex`),
    byDecadeindex: new MetricNode(client, `${basePath}/decadeindex`),
    byHeight: new MetricNode(client, `${basePath}/height`),
    byMonthindex: new MetricNode(client, `${basePath}/monthindex`),
    byQuarterindex: new MetricNode(client, `${basePath}/quarterindex`),
    bySemesterindex: new MetricNode(client, `${basePath}/semesterindex`),
    byWeekindex: new MetricNode(client, `${basePath}/weekindex`),
    byYearindex: new MetricNode(client, `${basePath}/yearindex`)
  };
}

/**
 * @template T
 * @typedef {Object} Indexes
 * @property {MetricNode<T>} byDateindex
 * @property {MetricNode<T>} byDecadeindex
 * @property {MetricNode<T>} byMonthindex
 * @property {MetricNode<T>} byQuarterindex
 * @property {MetricNode<T>} bySemesterindex
 * @property {MetricNode<T>} byWeekindex
 * @property {MetricNode<T>} byYearindex
 */

/**
 * Create a Indexes accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {Indexes<T>}
 */
function createIndexes(client, basePath) {
  return {
    byDateindex: new MetricNode(client, `${basePath}/dateindex`),
    byDecadeindex: new MetricNode(client, `${basePath}/decadeindex`),
    byMonthindex: new MetricNode(client, `${basePath}/monthindex`),
    byQuarterindex: new MetricNode(client, `${basePath}/quarterindex`),
    bySemesterindex: new MetricNode(client, `${basePath}/semesterindex`),
    byWeekindex: new MetricNode(client, `${basePath}/weekindex`),
    byYearindex: new MetricNode(client, `${basePath}/yearindex`)
  };
}

/**
 * @template T
 * @typedef {Object} Indexes22
 * @property {MetricNode<T>} byDecadeindex
 * @property {MetricNode<T>} byMonthindex
 * @property {MetricNode<T>} byQuarterindex
 * @property {MetricNode<T>} bySemesterindex
 * @property {MetricNode<T>} byWeekindex
 * @property {MetricNode<T>} byYearindex
 */

/**
 * Create a Indexes22 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {Indexes22<T>}
 */
function createIndexes22(client, basePath) {
  return {
    byDecadeindex: new MetricNode(client, `${basePath}/decadeindex`),
    byMonthindex: new MetricNode(client, `${basePath}/monthindex`),
    byQuarterindex: new MetricNode(client, `${basePath}/quarterindex`),
    bySemesterindex: new MetricNode(client, `${basePath}/semesterindex`),
    byWeekindex: new MetricNode(client, `${basePath}/weekindex`),
    byYearindex: new MetricNode(client, `${basePath}/yearindex`)
  };
}

/**
 * @template T
 * @typedef {Object} Indexes10
 * @property {MetricNode<T>} byQuarterindex
 * @property {MetricNode<T>} bySemesterindex
 * @property {MetricNode<T>} byYearindex
 */

/**
 * Create a Indexes10 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {Indexes10<T>}
 */
function createIndexes10(client, basePath) {
  return {
    byQuarterindex: new MetricNode(client, `${basePath}/quarterindex`),
    bySemesterindex: new MetricNode(client, `${basePath}/semesterindex`),
    byYearindex: new MetricNode(client, `${basePath}/yearindex`)
  };
}

/**
 * @template T
 * @typedef {Object} Indexes8
 * @property {MetricNode<T>} byDateindex
 * @property {MetricNode<T>} byHeight
 */

/**
 * Create a Indexes8 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {Indexes8<T>}
 */
function createIndexes8(client, basePath) {
  return {
    byDateindex: new MetricNode(client, `${basePath}/dateindex`),
    byHeight: new MetricNode(client, `${basePath}/height`)
  };
}

/**
 * @template T
 * @typedef {Object} Indexes9
 * @property {MetricNode<T>} byMonthindex
 * @property {MetricNode<T>} byWeekindex
 */

/**
 * Create a Indexes9 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {Indexes9<T>}
 */
function createIndexes9(client, basePath) {
  return {
    byMonthindex: new MetricNode(client, `${basePath}/monthindex`),
    byWeekindex: new MetricNode(client, `${basePath}/weekindex`)
  };
}

/**
 * @template T
 * @typedef {Object} Indexes2
 * @property {MetricNode<T>} byHeight
 */

/**
 * Create a Indexes2 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {Indexes2<T>}
 */
function createIndexes2(client, basePath) {
  return {
    byHeight: new MetricNode(client, `${basePath}/height`)
  };
}

/**
 * @template T
 * @typedef {Object} Indexes5
 * @property {MetricNode<T>} byDateindex
 */

/**
 * Create a Indexes5 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {Indexes5<T>}
 */
function createIndexes5(client, basePath) {
  return {
    byDateindex: new MetricNode(client, `${basePath}/dateindex`)
  };
}

/**
 * @template T
 * @typedef {Object} Indexes6
 * @property {MetricNode<T>} byTxindex
 */

/**
 * Create a Indexes6 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {Indexes6<T>}
 */
function createIndexes6(client, basePath) {
  return {
    byTxindex: new MetricNode(client, `${basePath}/txindex`)
  };
}

/**
 * @template T
 * @typedef {Object} Indexes7
 * @property {MetricNode<T>} byTxinindex
 */

/**
 * Create a Indexes7 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {Indexes7<T>}
 */
function createIndexes7(client, basePath) {
  return {
    byTxinindex: new MetricNode(client, `${basePath}/txinindex`)
  };
}

/**
 * @template T
 * @typedef {Object} Indexes11
 * @property {MetricNode<T>} byDecadeindex
 */

/**
 * Create a Indexes11 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {Indexes11<T>}
 */
function createIndexes11(client, basePath) {
  return {
    byDecadeindex: new MetricNode(client, `${basePath}/decadeindex`)
  };
}

/**
 * @template T
 * @typedef {Object} Indexes12
 * @property {MetricNode<T>} byP2aaddressindex
 */

/**
 * Create a Indexes12 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {Indexes12<T>}
 */
function createIndexes12(client, basePath) {
  return {
    byP2aaddressindex: new MetricNode(client, `${basePath}/p2aaddressindex`)
  };
}

/**
 * @template T
 * @typedef {Object} Indexes13
 * @property {MetricNode<T>} byP2pk33addressindex
 */

/**
 * Create a Indexes13 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {Indexes13<T>}
 */
function createIndexes13(client, basePath) {
  return {
    byP2pk33addressindex: new MetricNode(client, `${basePath}/p2pk33addressindex`)
  };
}

/**
 * @template T
 * @typedef {Object} Indexes14
 * @property {MetricNode<T>} byP2pk65addressindex
 */

/**
 * Create a Indexes14 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {Indexes14<T>}
 */
function createIndexes14(client, basePath) {
  return {
    byP2pk65addressindex: new MetricNode(client, `${basePath}/p2pk65addressindex`)
  };
}

/**
 * @template T
 * @typedef {Object} Indexes15
 * @property {MetricNode<T>} byP2pkhaddressindex
 */

/**
 * Create a Indexes15 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {Indexes15<T>}
 */
function createIndexes15(client, basePath) {
  return {
    byP2pkhaddressindex: new MetricNode(client, `${basePath}/p2pkhaddressindex`)
  };
}

/**
 * @template T
 * @typedef {Object} Indexes16
 * @property {MetricNode<T>} byP2shaddressindex
 */

/**
 * Create a Indexes16 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {Indexes16<T>}
 */
function createIndexes16(client, basePath) {
  return {
    byP2shaddressindex: new MetricNode(client, `${basePath}/p2shaddressindex`)
  };
}

/**
 * @template T
 * @typedef {Object} Indexes17
 * @property {MetricNode<T>} byP2traddressindex
 */

/**
 * Create a Indexes17 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {Indexes17<T>}
 */
function createIndexes17(client, basePath) {
  return {
    byP2traddressindex: new MetricNode(client, `${basePath}/p2traddressindex`)
  };
}

/**
 * @template T
 * @typedef {Object} Indexes18
 * @property {MetricNode<T>} byP2wpkhaddressindex
 */

/**
 * Create a Indexes18 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {Indexes18<T>}
 */
function createIndexes18(client, basePath) {
  return {
    byP2wpkhaddressindex: new MetricNode(client, `${basePath}/p2wpkhaddressindex`)
  };
}

/**
 * @template T
 * @typedef {Object} Indexes19
 * @property {MetricNode<T>} byP2wshaddressindex
 */

/**
 * Create a Indexes19 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {Indexes19<T>}
 */
function createIndexes19(client, basePath) {
  return {
    byP2wshaddressindex: new MetricNode(client, `${basePath}/p2wshaddressindex`)
  };
}

/**
 * @template T
 * @typedef {Object} Indexes20
 * @property {MetricNode<T>} byTxoutindex
 */

/**
 * Create a Indexes20 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {Indexes20<T>}
 */
function createIndexes20(client, basePath) {
  return {
    byTxoutindex: new MetricNode(client, `${basePath}/txoutindex`)
  };
}

/**
 * @template T
 * @typedef {Object} Indexes23
 * @property {MetricNode<T>} byEmptyaddressindex
 */

/**
 * Create a Indexes23 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {Indexes23<T>}
 */
function createIndexes23(client, basePath) {
  return {
    byEmptyaddressindex: new MetricNode(client, `${basePath}/emptyaddressindex`)
  };
}

/**
 * @template T
 * @typedef {Object} Indexes24
 * @property {MetricNode<T>} byLoadedaddressindex
 */

/**
 * Create a Indexes24 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {Indexes24<T>}
 */
function createIndexes24(client, basePath) {
  return {
    byLoadedaddressindex: new MetricNode(client, `${basePath}/loadedaddressindex`)
  };
}

// Reusable structural pattern factories

/**
 * @typedef {Object} RealizedPattern3
 * @property {Indexes5<StoredF64>} adjustedSopr
 * @property {Indexes5<StoredF64>} adjustedSopr30dEma
 * @property {Indexes5<StoredF64>} adjustedSopr7dEma
 * @property {Indexes3<Dollars>} adjustedValueCreated
 * @property {Indexes3<Dollars>} adjustedValueDestroyed
 * @property {BlockCountPattern} negRealizedLoss
 * @property {BlockCountPattern} netRealizedPnl
 * @property {Indexes<Dollars>} netRealizedPnlCumulative30dDelta
 * @property {Indexes<StoredF32>} netRealizedPnlCumulative30dDeltaRelToMarketCap
 * @property {Indexes<StoredF32>} netRealizedPnlCumulative30dDeltaRelToRealizedCap
 * @property {Indexes3<StoredF32>} netRealizedPnlRelToRealizedCap
 * @property {Indexes3<Dollars>} realizedCap
 * @property {Indexes<Dollars>} realizedCap30dDelta
 * @property {Indexes3<StoredF32>} realizedCapRelToOwnMarketCap
 * @property {BlockCountPattern} realizedLoss
 * @property {Indexes3<StoredF32>} realizedLossRelToRealizedCap
 * @property {Indexes3<Dollars>} realizedPrice
 * @property {ActivePriceRatioPattern} realizedPriceExtra
 * @property {BlockCountPattern} realizedProfit
 * @property {Indexes3<StoredF32>} realizedProfitRelToRealizedCap
 * @property {Indexes5<StoredF64>} realizedProfitToLossRatio
 * @property {Indexes3<Dollars>} realizedValue
 * @property {Indexes5<StoredF32>} sellSideRiskRatio
 * @property {Indexes5<StoredF32>} sellSideRiskRatio30dEma
 * @property {Indexes5<StoredF32>} sellSideRiskRatio7dEma
 * @property {Indexes5<StoredF64>} sopr
 * @property {Indexes5<StoredF64>} sopr30dEma
 * @property {Indexes5<StoredF64>} sopr7dEma
 * @property {Indexes21<Dollars>} totalRealizedPnl
 * @property {Indexes3<Dollars>} valueCreated
 * @property {Indexes3<Dollars>} valueDestroyed
 */

/**
 * Create a RealizedPattern3 pattern node
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {RealizedPattern3}
 */
function createRealizedPattern3(client, basePath) {
  return {
    adjustedSopr: createIndexes5(client, `${basePath}/adjusted_sopr`),
    adjustedSopr30dEma: createIndexes5(client, `${basePath}/adjusted_sopr_30d_ema`),
    adjustedSopr7dEma: createIndexes5(client, `${basePath}/adjusted_sopr_7d_ema`),
    adjustedValueCreated: createIndexes3(client, `${basePath}/adjusted_value_created`),
    adjustedValueDestroyed: createIndexes3(client, `${basePath}/adjusted_value_destroyed`),
    negRealizedLoss: createBlockCountPattern(client, `${basePath}/neg_realized_loss`),
    netRealizedPnl: createBlockCountPattern(client, `${basePath}/net_realized_pnl`),
    netRealizedPnlCumulative30dDelta: createIndexes(client, `${basePath}/net_realized_pnl_cumulative_30d_delta`),
    netRealizedPnlCumulative30dDeltaRelToMarketCap: createIndexes(client, `${basePath}/net_realized_pnl_cumulative_30d_delta_rel_to_market_cap`),
    netRealizedPnlCumulative30dDeltaRelToRealizedCap: createIndexes(client, `${basePath}/net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap`),
    netRealizedPnlRelToRealizedCap: createIndexes3(client, `${basePath}/net_realized_pnl_rel_to_realized_cap`),
    realizedCap: createIndexes3(client, `${basePath}/realized_cap`),
    realizedCap30dDelta: createIndexes(client, `${basePath}/realized_cap_30d_delta`),
    realizedCapRelToOwnMarketCap: createIndexes3(client, `${basePath}/realized_cap_rel_to_own_market_cap`),
    realizedLoss: createBlockCountPattern(client, `${basePath}/realized_loss`),
    realizedLossRelToRealizedCap: createIndexes3(client, `${basePath}/realized_loss_rel_to_realized_cap`),
    realizedPrice: createIndexes3(client, `${basePath}/realized_price`),
    realizedPriceExtra: createActivePriceRatioPattern(client, `${basePath}/realized_price_extra`),
    realizedProfit: createBlockCountPattern(client, `${basePath}/realized_profit`),
    realizedProfitRelToRealizedCap: createIndexes3(client, `${basePath}/realized_profit_rel_to_realized_cap`),
    realizedProfitToLossRatio: createIndexes5(client, `${basePath}/realized_profit_to_loss_ratio`),
    realizedValue: createIndexes3(client, `${basePath}/realized_value`),
    sellSideRiskRatio: createIndexes5(client, `${basePath}/sell_side_risk_ratio`),
    sellSideRiskRatio30dEma: createIndexes5(client, `${basePath}/sell_side_risk_ratio_30d_ema`),
    sellSideRiskRatio7dEma: createIndexes5(client, `${basePath}/sell_side_risk_ratio_7d_ema`),
    sopr: createIndexes5(client, `${basePath}/sopr`),
    sopr30dEma: createIndexes5(client, `${basePath}/sopr_30d_ema`),
    sopr7dEma: createIndexes5(client, `${basePath}/sopr_7d_ema`),
    totalRealizedPnl: createIndexes21(client, `${basePath}/total_realized_pnl`),
    valueCreated: createIndexes3(client, `${basePath}/value_created`),
    valueDestroyed: createIndexes3(client, `${basePath}/value_destroyed`)
  };
}

/**
 * @typedef {Object} Ratio1ySdPattern
 * @property {Indexes<Dollars>} _0sdUsd
 * @property {Indexes<StoredF32>} m05sd
 * @property {Indexes<Dollars>} m05sdUsd
 * @property {Indexes<StoredF32>} m15sd
 * @property {Indexes<Dollars>} m15sdUsd
 * @property {Indexes<StoredF32>} m1sd
 * @property {Indexes<Dollars>} m1sdUsd
 * @property {Indexes<StoredF32>} m25sd
 * @property {Indexes<Dollars>} m25sdUsd
 * @property {Indexes<StoredF32>} m2sd
 * @property {Indexes<Dollars>} m2sdUsd
 * @property {Indexes<StoredF32>} m3sd
 * @property {Indexes<Dollars>} m3sdUsd
 * @property {Indexes<StoredF32>} p05sd
 * @property {Indexes<Dollars>} p05sdUsd
 * @property {Indexes<StoredF32>} p15sd
 * @property {Indexes<Dollars>} p15sdUsd
 * @property {Indexes<StoredF32>} p1sd
 * @property {Indexes<Dollars>} p1sdUsd
 * @property {Indexes<StoredF32>} p25sd
 * @property {Indexes<Dollars>} p25sdUsd
 * @property {Indexes<StoredF32>} p2sd
 * @property {Indexes<Dollars>} p2sdUsd
 * @property {Indexes<StoredF32>} p3sd
 * @property {Indexes<Dollars>} p3sdUsd
 * @property {Indexes<StoredF32>} sd
 * @property {Indexes<StoredF32>} sma
 * @property {Indexes<StoredF32>} zscore
 */

/**
 * Create a Ratio1ySdPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {Ratio1ySdPattern}
 */
function createRatio1ySdPattern(client, basePath) {
  return {
    _0sdUsd: createIndexes(client, `${basePath}/_0sd_usd`),
    m05sd: createIndexes(client, `${basePath}/m0_5sd`),
    m05sdUsd: createIndexes(client, `${basePath}/m0_5sd_usd`),
    m15sd: createIndexes(client, `${basePath}/m1_5sd`),
    m15sdUsd: createIndexes(client, `${basePath}/m1_5sd_usd`),
    m1sd: createIndexes(client, `${basePath}/m1sd`),
    m1sdUsd: createIndexes(client, `${basePath}/m1sd_usd`),
    m25sd: createIndexes(client, `${basePath}/m2_5sd`),
    m25sdUsd: createIndexes(client, `${basePath}/m2_5sd_usd`),
    m2sd: createIndexes(client, `${basePath}/m2sd`),
    m2sdUsd: createIndexes(client, `${basePath}/m2sd_usd`),
    m3sd: createIndexes(client, `${basePath}/m3sd`),
    m3sdUsd: createIndexes(client, `${basePath}/m3sd_usd`),
    p05sd: createIndexes(client, `${basePath}/p0_5sd`),
    p05sdUsd: createIndexes(client, `${basePath}/p0_5sd_usd`),
    p15sd: createIndexes(client, `${basePath}/p1_5sd`),
    p15sdUsd: createIndexes(client, `${basePath}/p1_5sd_usd`),
    p1sd: createIndexes(client, `${basePath}/p1sd`),
    p1sdUsd: createIndexes(client, `${basePath}/p1sd_usd`),
    p25sd: createIndexes(client, `${basePath}/p2_5sd`),
    p25sdUsd: createIndexes(client, `${basePath}/p2_5sd_usd`),
    p2sd: createIndexes(client, `${basePath}/p2sd`),
    p2sdUsd: createIndexes(client, `${basePath}/p2sd_usd`),
    p3sd: createIndexes(client, `${basePath}/p3sd`),
    p3sdUsd: createIndexes(client, `${basePath}/p3sd_usd`),
    sd: createIndexes(client, `${basePath}/sd`),
    sma: createIndexes(client, `${basePath}/sma`),
    zscore: createIndexes(client, `${basePath}/zscore`)
  };
}

/**
 * @typedef {Object} RealizedPattern2
 * @property {BlockCountPattern} negRealizedLoss
 * @property {BlockCountPattern} netRealizedPnl
 * @property {Indexes<Dollars>} netRealizedPnlCumulative30dDelta
 * @property {Indexes<StoredF32>} netRealizedPnlCumulative30dDeltaRelToMarketCap
 * @property {Indexes<StoredF32>} netRealizedPnlCumulative30dDeltaRelToRealizedCap
 * @property {Indexes3<StoredF32>} netRealizedPnlRelToRealizedCap
 * @property {Indexes3<Dollars>} realizedCap
 * @property {Indexes<Dollars>} realizedCap30dDelta
 * @property {Indexes3<StoredF32>} realizedCapRelToOwnMarketCap
 * @property {BlockCountPattern} realizedLoss
 * @property {Indexes3<StoredF32>} realizedLossRelToRealizedCap
 * @property {Indexes3<Dollars>} realizedPrice
 * @property {ActivePriceRatioPattern} realizedPriceExtra
 * @property {BlockCountPattern} realizedProfit
 * @property {Indexes3<StoredF32>} realizedProfitRelToRealizedCap
 * @property {Indexes5<StoredF64>} realizedProfitToLossRatio
 * @property {Indexes3<Dollars>} realizedValue
 * @property {Indexes5<StoredF32>} sellSideRiskRatio
 * @property {Indexes5<StoredF32>} sellSideRiskRatio30dEma
 * @property {Indexes5<StoredF32>} sellSideRiskRatio7dEma
 * @property {Indexes5<StoredF64>} sopr
 * @property {Indexes5<StoredF64>} sopr30dEma
 * @property {Indexes5<StoredF64>} sopr7dEma
 * @property {Indexes21<Dollars>} totalRealizedPnl
 * @property {Indexes3<Dollars>} valueCreated
 * @property {Indexes3<Dollars>} valueDestroyed
 */

/**
 * Create a RealizedPattern2 pattern node
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {RealizedPattern2}
 */
function createRealizedPattern2(client, basePath) {
  return {
    negRealizedLoss: createBlockCountPattern(client, `${basePath}/neg_realized_loss`),
    netRealizedPnl: createBlockCountPattern(client, `${basePath}/net_realized_pnl`),
    netRealizedPnlCumulative30dDelta: createIndexes(client, `${basePath}/net_realized_pnl_cumulative_30d_delta`),
    netRealizedPnlCumulative30dDeltaRelToMarketCap: createIndexes(client, `${basePath}/net_realized_pnl_cumulative_30d_delta_rel_to_market_cap`),
    netRealizedPnlCumulative30dDeltaRelToRealizedCap: createIndexes(client, `${basePath}/net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap`),
    netRealizedPnlRelToRealizedCap: createIndexes3(client, `${basePath}/net_realized_pnl_rel_to_realized_cap`),
    realizedCap: createIndexes3(client, `${basePath}/realized_cap`),
    realizedCap30dDelta: createIndexes(client, `${basePath}/realized_cap_30d_delta`),
    realizedCapRelToOwnMarketCap: createIndexes3(client, `${basePath}/realized_cap_rel_to_own_market_cap`),
    realizedLoss: createBlockCountPattern(client, `${basePath}/realized_loss`),
    realizedLossRelToRealizedCap: createIndexes3(client, `${basePath}/realized_loss_rel_to_realized_cap`),
    realizedPrice: createIndexes3(client, `${basePath}/realized_price`),
    realizedPriceExtra: createActivePriceRatioPattern(client, `${basePath}/realized_price_extra`),
    realizedProfit: createBlockCountPattern(client, `${basePath}/realized_profit`),
    realizedProfitRelToRealizedCap: createIndexes3(client, `${basePath}/realized_profit_rel_to_realized_cap`),
    realizedProfitToLossRatio: createIndexes5(client, `${basePath}/realized_profit_to_loss_ratio`),
    realizedValue: createIndexes3(client, `${basePath}/realized_value`),
    sellSideRiskRatio: createIndexes5(client, `${basePath}/sell_side_risk_ratio`),
    sellSideRiskRatio30dEma: createIndexes5(client, `${basePath}/sell_side_risk_ratio_30d_ema`),
    sellSideRiskRatio7dEma: createIndexes5(client, `${basePath}/sell_side_risk_ratio_7d_ema`),
    sopr: createIndexes5(client, `${basePath}/sopr`),
    sopr30dEma: createIndexes5(client, `${basePath}/sopr_30d_ema`),
    sopr7dEma: createIndexes5(client, `${basePath}/sopr_7d_ema`),
    totalRealizedPnl: createIndexes21(client, `${basePath}/total_realized_pnl`),
    valueCreated: createIndexes3(client, `${basePath}/value_created`),
    valueDestroyed: createIndexes3(client, `${basePath}/value_destroyed`)
  };
}

/**
 * @typedef {Object} RealizedPattern
 * @property {BlockCountPattern} negRealizedLoss
 * @property {BlockCountPattern} netRealizedPnl
 * @property {Indexes<Dollars>} netRealizedPnlCumulative30dDelta
 * @property {Indexes<StoredF32>} netRealizedPnlCumulative30dDeltaRelToMarketCap
 * @property {Indexes<StoredF32>} netRealizedPnlCumulative30dDeltaRelToRealizedCap
 * @property {Indexes3<StoredF32>} netRealizedPnlRelToRealizedCap
 * @property {Indexes3<Dollars>} realizedCap
 * @property {Indexes<Dollars>} realizedCap30dDelta
 * @property {BlockCountPattern} realizedLoss
 * @property {Indexes3<StoredF32>} realizedLossRelToRealizedCap
 * @property {Indexes3<Dollars>} realizedPrice
 * @property {RealizedPriceExtraPattern} realizedPriceExtra
 * @property {BlockCountPattern} realizedProfit
 * @property {Indexes3<StoredF32>} realizedProfitRelToRealizedCap
 * @property {Indexes3<Dollars>} realizedValue
 * @property {Indexes5<StoredF32>} sellSideRiskRatio
 * @property {Indexes5<StoredF32>} sellSideRiskRatio30dEma
 * @property {Indexes5<StoredF32>} sellSideRiskRatio7dEma
 * @property {Indexes5<StoredF64>} sopr
 * @property {Indexes5<StoredF64>} sopr30dEma
 * @property {Indexes5<StoredF64>} sopr7dEma
 * @property {Indexes21<Dollars>} totalRealizedPnl
 * @property {Indexes3<Dollars>} valueCreated
 * @property {Indexes3<Dollars>} valueDestroyed
 */

/**
 * Create a RealizedPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {RealizedPattern}
 */
function createRealizedPattern(client, basePath) {
  return {
    negRealizedLoss: createBlockCountPattern(client, `${basePath}/neg_realized_loss`),
    netRealizedPnl: createBlockCountPattern(client, `${basePath}/net_realized_pnl`),
    netRealizedPnlCumulative30dDelta: createIndexes(client, `${basePath}/net_realized_pnl_cumulative_30d_delta`),
    netRealizedPnlCumulative30dDeltaRelToMarketCap: createIndexes(client, `${basePath}/net_realized_pnl_cumulative_30d_delta_rel_to_market_cap`),
    netRealizedPnlCumulative30dDeltaRelToRealizedCap: createIndexes(client, `${basePath}/net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap`),
    netRealizedPnlRelToRealizedCap: createIndexes3(client, `${basePath}/net_realized_pnl_rel_to_realized_cap`),
    realizedCap: createIndexes3(client, `${basePath}/realized_cap`),
    realizedCap30dDelta: createIndexes(client, `${basePath}/realized_cap_30d_delta`),
    realizedLoss: createBlockCountPattern(client, `${basePath}/realized_loss`),
    realizedLossRelToRealizedCap: createIndexes3(client, `${basePath}/realized_loss_rel_to_realized_cap`),
    realizedPrice: createIndexes3(client, `${basePath}/realized_price`),
    realizedPriceExtra: createRealizedPriceExtraPattern(client, `${basePath}/realized_price_extra`),
    realizedProfit: createBlockCountPattern(client, `${basePath}/realized_profit`),
    realizedProfitRelToRealizedCap: createIndexes3(client, `${basePath}/realized_profit_rel_to_realized_cap`),
    realizedValue: createIndexes3(client, `${basePath}/realized_value`),
    sellSideRiskRatio: createIndexes5(client, `${basePath}/sell_side_risk_ratio`),
    sellSideRiskRatio30dEma: createIndexes5(client, `${basePath}/sell_side_risk_ratio_30d_ema`),
    sellSideRiskRatio7dEma: createIndexes5(client, `${basePath}/sell_side_risk_ratio_7d_ema`),
    sopr: createIndexes5(client, `${basePath}/sopr`),
    sopr30dEma: createIndexes5(client, `${basePath}/sopr_30d_ema`),
    sopr7dEma: createIndexes5(client, `${basePath}/sopr_7d_ema`),
    totalRealizedPnl: createIndexes21(client, `${basePath}/total_realized_pnl`),
    valueCreated: createIndexes3(client, `${basePath}/value_created`),
    valueDestroyed: createIndexes3(client, `${basePath}/value_destroyed`)
  };
}

/**
 * @typedef {Object} Price13dEmaPattern
 * @property {Indexes<Dollars>} price
 * @property {Indexes<StoredF32>} ratio
 * @property {Indexes<StoredF32>} ratio1mSma
 * @property {Indexes<StoredF32>} ratio1wSma
 * @property {Ratio1ySdPattern} ratio1ySd
 * @property {Ratio1ySdPattern} ratio2ySd
 * @property {Ratio1ySdPattern} ratio4ySd
 * @property {Indexes<StoredF32>} ratioPct1
 * @property {Indexes<Dollars>} ratioPct1Usd
 * @property {Indexes<StoredF32>} ratioPct2
 * @property {Indexes<Dollars>} ratioPct2Usd
 * @property {Indexes<StoredF32>} ratioPct5
 * @property {Indexes<Dollars>} ratioPct5Usd
 * @property {Indexes<StoredF32>} ratioPct95
 * @property {Indexes<Dollars>} ratioPct95Usd
 * @property {Indexes<StoredF32>} ratioPct98
 * @property {Indexes<Dollars>} ratioPct98Usd
 * @property {Indexes<StoredF32>} ratioPct99
 * @property {Indexes<Dollars>} ratioPct99Usd
 * @property {Ratio1ySdPattern} ratioSd
 */

/**
 * Create a Price13dEmaPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {Price13dEmaPattern}
 */
function createPrice13dEmaPattern(client, acc) {
  return {
    price: createIndexes(client, `/${acc}`),
    ratio: createIndexes(client, `/${acc}_ratio`),
    ratio1mSma: createIndexes(client, `/${acc}_ratio_1m_sma`),
    ratio1wSma: createIndexes(client, `/${acc}_ratio_1w_sma`),
    ratio1ySd: createRatio1ySdPattern(client, `${acc}_ratio_1y_sd`),
    ratio2ySd: createRatio1ySdPattern(client, `${acc}_ratio_2y_sd`),
    ratio4ySd: createRatio1ySdPattern(client, `${acc}_ratio_4y_sd`),
    ratioPct1: createIndexes(client, `/${acc}_ratio_pct1`),
    ratioPct1Usd: createIndexes(client, `/${acc}_ratio_pct1_usd`),
    ratioPct2: createIndexes(client, `/${acc}_ratio_pct2`),
    ratioPct2Usd: createIndexes(client, `/${acc}_ratio_pct2_usd`),
    ratioPct5: createIndexes(client, `/${acc}_ratio_pct5`),
    ratioPct5Usd: createIndexes(client, `/${acc}_ratio_pct5_usd`),
    ratioPct95: createIndexes(client, `/${acc}_ratio_pct95`),
    ratioPct95Usd: createIndexes(client, `/${acc}_ratio_pct95_usd`),
    ratioPct98: createIndexes(client, `/${acc}_ratio_pct98`),
    ratioPct98Usd: createIndexes(client, `/${acc}_ratio_pct98_usd`),
    ratioPct99: createIndexes(client, `/${acc}_ratio_pct99`),
    ratioPct99Usd: createIndexes(client, `/${acc}_ratio_pct99_usd`),
    ratioSd: createRatio1ySdPattern(client, `${acc}_ratio_sd`)
  };
}

/**
 * @typedef {Object} PricePercentilesPattern
 * @property {Indexes<Dollars>} pct05
 * @property {Indexes<Dollars>} pct10
 * @property {Indexes<Dollars>} pct15
 * @property {Indexes<Dollars>} pct20
 * @property {Indexes<Dollars>} pct25
 * @property {Indexes<Dollars>} pct30
 * @property {Indexes<Dollars>} pct35
 * @property {Indexes<Dollars>} pct40
 * @property {Indexes<Dollars>} pct45
 * @property {Indexes<Dollars>} pct50
 * @property {Indexes<Dollars>} pct55
 * @property {Indexes<Dollars>} pct60
 * @property {Indexes<Dollars>} pct65
 * @property {Indexes<Dollars>} pct70
 * @property {Indexes<Dollars>} pct75
 * @property {Indexes<Dollars>} pct80
 * @property {Indexes<Dollars>} pct85
 * @property {Indexes<Dollars>} pct90
 * @property {Indexes<Dollars>} pct95
 */

/**
 * Create a PricePercentilesPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {PricePercentilesPattern}
 */
function createPricePercentilesPattern(client, basePath) {
  return {
    pct05: createIndexes(client, `${basePath}/pct05`),
    pct10: createIndexes(client, `${basePath}/pct10`),
    pct15: createIndexes(client, `${basePath}/pct15`),
    pct20: createIndexes(client, `${basePath}/pct20`),
    pct25: createIndexes(client, `${basePath}/pct25`),
    pct30: createIndexes(client, `${basePath}/pct30`),
    pct35: createIndexes(client, `${basePath}/pct35`),
    pct40: createIndexes(client, `${basePath}/pct40`),
    pct45: createIndexes(client, `${basePath}/pct45`),
    pct50: createIndexes(client, `${basePath}/pct50`),
    pct55: createIndexes(client, `${basePath}/pct55`),
    pct60: createIndexes(client, `${basePath}/pct60`),
    pct65: createIndexes(client, `${basePath}/pct65`),
    pct70: createIndexes(client, `${basePath}/pct70`),
    pct75: createIndexes(client, `${basePath}/pct75`),
    pct80: createIndexes(client, `${basePath}/pct80`),
    pct85: createIndexes(client, `${basePath}/pct85`),
    pct90: createIndexes(client, `${basePath}/pct90`),
    pct95: createIndexes(client, `${basePath}/pct95`)
  };
}

/**
 * @typedef {Object} ActivePriceRatioPattern
 * @property {Indexes<StoredF32>} ratio
 * @property {Indexes<StoredF32>} ratio1mSma
 * @property {Indexes<StoredF32>} ratio1wSma
 * @property {Ratio1ySdPattern} ratio1ySd
 * @property {Ratio1ySdPattern} ratio2ySd
 * @property {Ratio1ySdPattern} ratio4ySd
 * @property {Indexes<StoredF32>} ratioPct1
 * @property {Indexes<Dollars>} ratioPct1Usd
 * @property {Indexes<StoredF32>} ratioPct2
 * @property {Indexes<Dollars>} ratioPct2Usd
 * @property {Indexes<StoredF32>} ratioPct5
 * @property {Indexes<Dollars>} ratioPct5Usd
 * @property {Indexes<StoredF32>} ratioPct95
 * @property {Indexes<Dollars>} ratioPct95Usd
 * @property {Indexes<StoredF32>} ratioPct98
 * @property {Indexes<Dollars>} ratioPct98Usd
 * @property {Indexes<StoredF32>} ratioPct99
 * @property {Indexes<Dollars>} ratioPct99Usd
 * @property {Ratio1ySdPattern} ratioSd
 */

/**
 * Create a ActivePriceRatioPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {ActivePriceRatioPattern}
 */
function createActivePriceRatioPattern(client, basePath) {
  return {
    ratio: createIndexes(client, `${basePath}/ratio`),
    ratio1mSma: createIndexes(client, `${basePath}/ratio_1m_sma`),
    ratio1wSma: createIndexes(client, `${basePath}/ratio_1w_sma`),
    ratio1ySd: createRatio1ySdPattern(client, `${basePath}/ratio_1y_sd`),
    ratio2ySd: createRatio1ySdPattern(client, `${basePath}/ratio_2y_sd`),
    ratio4ySd: createRatio1ySdPattern(client, `${basePath}/ratio_4y_sd`),
    ratioPct1: createIndexes(client, `${basePath}/ratio_pct1`),
    ratioPct1Usd: createIndexes(client, `${basePath}/ratio_pct1_usd`),
    ratioPct2: createIndexes(client, `${basePath}/ratio_pct2`),
    ratioPct2Usd: createIndexes(client, `${basePath}/ratio_pct2_usd`),
    ratioPct5: createIndexes(client, `${basePath}/ratio_pct5`),
    ratioPct5Usd: createIndexes(client, `${basePath}/ratio_pct5_usd`),
    ratioPct95: createIndexes(client, `${basePath}/ratio_pct95`),
    ratioPct95Usd: createIndexes(client, `${basePath}/ratio_pct95_usd`),
    ratioPct98: createIndexes(client, `${basePath}/ratio_pct98`),
    ratioPct98Usd: createIndexes(client, `${basePath}/ratio_pct98_usd`),
    ratioPct99: createIndexes(client, `${basePath}/ratio_pct99`),
    ratioPct99Usd: createIndexes(client, `${basePath}/ratio_pct99_usd`),
    ratioSd: createRatio1ySdPattern(client, `${basePath}/ratio_sd`)
  };
}

/**
 * @typedef {Object} RelativePattern2
 * @property {Indexes21<StoredF32>} negUnrealizedLossRelToMarketCap
 * @property {Indexes21<StoredF32>} negUnrealizedLossRelToOwnMarketCap
 * @property {Indexes21<StoredF32>} negUnrealizedLossRelToOwnTotalUnrealizedPnl
 * @property {Indexes21<StoredF32>} netUnrealizedPnlRelToMarketCap
 * @property {Indexes21<StoredF32>} netUnrealizedPnlRelToOwnMarketCap
 * @property {Indexes21<StoredF32>} netUnrealizedPnlRelToOwnTotalUnrealizedPnl
 * @property {Indexes21<StoredF64>} supplyInLossRelToCirculatingSupply
 * @property {Indexes21<StoredF64>} supplyInLossRelToOwnSupply
 * @property {Indexes21<StoredF64>} supplyInProfitRelToCirculatingSupply
 * @property {Indexes21<StoredF64>} supplyInProfitRelToOwnSupply
 * @property {Indexes3<StoredF64>} supplyRelToCirculatingSupply
 * @property {Indexes21<StoredF32>} unrealizedLossRelToMarketCap
 * @property {Indexes21<StoredF32>} unrealizedLossRelToOwnMarketCap
 * @property {Indexes21<StoredF32>} unrealizedLossRelToOwnTotalUnrealizedPnl
 * @property {Indexes21<StoredF32>} unrealizedProfitRelToMarketCap
 * @property {Indexes21<StoredF32>} unrealizedProfitRelToOwnMarketCap
 * @property {Indexes21<StoredF32>} unrealizedProfitRelToOwnTotalUnrealizedPnl
 */

/**
 * Create a RelativePattern2 pattern node
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {RelativePattern2}
 */
function createRelativePattern2(client, basePath) {
  return {
    negUnrealizedLossRelToMarketCap: createIndexes21(client, `${basePath}/neg_unrealized_loss_rel_to_market_cap`),
    negUnrealizedLossRelToOwnMarketCap: createIndexes21(client, `${basePath}/neg_unrealized_loss_rel_to_own_market_cap`),
    negUnrealizedLossRelToOwnTotalUnrealizedPnl: createIndexes21(client, `${basePath}/neg_unrealized_loss_rel_to_own_total_unrealized_pnl`),
    netUnrealizedPnlRelToMarketCap: createIndexes21(client, `${basePath}/net_unrealized_pnl_rel_to_market_cap`),
    netUnrealizedPnlRelToOwnMarketCap: createIndexes21(client, `${basePath}/net_unrealized_pnl_rel_to_own_market_cap`),
    netUnrealizedPnlRelToOwnTotalUnrealizedPnl: createIndexes21(client, `${basePath}/net_unrealized_pnl_rel_to_own_total_unrealized_pnl`),
    supplyInLossRelToCirculatingSupply: createIndexes21(client, `${basePath}/supply_in_loss_rel_to_circulating_supply`),
    supplyInLossRelToOwnSupply: createIndexes21(client, `${basePath}/supply_in_loss_rel_to_own_supply`),
    supplyInProfitRelToCirculatingSupply: createIndexes21(client, `${basePath}/supply_in_profit_rel_to_circulating_supply`),
    supplyInProfitRelToOwnSupply: createIndexes21(client, `${basePath}/supply_in_profit_rel_to_own_supply`),
    supplyRelToCirculatingSupply: createIndexes3(client, `${basePath}/supply_rel_to_circulating_supply`),
    unrealizedLossRelToMarketCap: createIndexes21(client, `${basePath}/unrealized_loss_rel_to_market_cap`),
    unrealizedLossRelToOwnMarketCap: createIndexes21(client, `${basePath}/unrealized_loss_rel_to_own_market_cap`),
    unrealizedLossRelToOwnTotalUnrealizedPnl: createIndexes21(client, `${basePath}/unrealized_loss_rel_to_own_total_unrealized_pnl`),
    unrealizedProfitRelToMarketCap: createIndexes21(client, `${basePath}/unrealized_profit_rel_to_market_cap`),
    unrealizedProfitRelToOwnMarketCap: createIndexes21(client, `${basePath}/unrealized_profit_rel_to_own_market_cap`),
    unrealizedProfitRelToOwnTotalUnrealizedPnl: createIndexes21(client, `${basePath}/unrealized_profit_rel_to_own_total_unrealized_pnl`)
  };
}

/**
 * @typedef {Object} AXbtPattern
 * @property {Indexes<StoredF32>} _1dDominance
 * @property {Indexes<StoredU32>} _1mBlocksMined
 * @property {Indexes<StoredF32>} _1mDominance
 * @property {Indexes<StoredU32>} _1wBlocksMined
 * @property {Indexes<StoredF32>} _1wDominance
 * @property {Indexes<StoredU32>} _1yBlocksMined
 * @property {Indexes<StoredF32>} _1yDominance
 * @property {BlockCountPattern} blocksMined
 * @property {UnclaimedRewardsPattern} coinbase
 * @property {Indexes<StoredU16>} daysSinceBlock
 * @property {Indexes<StoredF32>} dominance
 * @property {UnclaimedRewardsPattern} fee
 * @property {UnclaimedRewardsPattern} subsidy
 */

/**
 * Create a AXbtPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {AXbtPattern}
 */
function createAXbtPattern(client, basePath) {
  return {
    _1dDominance: createIndexes(client, `${basePath}/1d_dominance`),
    _1mBlocksMined: createIndexes(client, `${basePath}/1m_blocks_mined`),
    _1mDominance: createIndexes(client, `${basePath}/1m_dominance`),
    _1wBlocksMined: createIndexes(client, `${basePath}/1w_blocks_mined`),
    _1wDominance: createIndexes(client, `${basePath}/1w_dominance`),
    _1yBlocksMined: createIndexes(client, `${basePath}/1y_blocks_mined`),
    _1yDominance: createIndexes(client, `${basePath}/1y_dominance`),
    blocksMined: createBlockCountPattern(client, `${basePath}/blocks_mined`),
    coinbase: createUnclaimedRewardsPattern(client, `${basePath}/coinbase`),
    daysSinceBlock: createIndexes(client, `${basePath}/days_since_block`),
    dominance: createIndexes(client, `${basePath}/dominance`),
    fee: createUnclaimedRewardsPattern(client, `${basePath}/fee`),
    subsidy: createUnclaimedRewardsPattern(client, `${basePath}/subsidy`)
  };
}

/**
 * @template T
 * @typedef {Object} BitcoinPattern
 * @property {Indexes4<T>} average
 * @property {Indexes2<T>} base
 * @property {Indexes3<T>} cumulative
 * @property {Indexes4<T>} max
 * @property {Indexes5<T>} median
 * @property {Indexes4<T>} min
 * @property {Indexes5<T>} pct10
 * @property {Indexes5<T>} pct25
 * @property {Indexes5<T>} pct75
 * @property {Indexes5<T>} pct90
 * @property {Indexes4<T>} sum
 */

/**
 * Create a BitcoinPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {BitcoinPattern}
 */
function createBitcoinPattern(client, basePath) {
  return {
    average: createIndexes4(client, `${basePath}/average`),
    base: createIndexes2(client, `${basePath}/base`),
    cumulative: createIndexes3(client, `${basePath}/cumulative`),
    max: createIndexes4(client, `${basePath}/max`),
    median: createIndexes5(client, `${basePath}/median`),
    min: createIndexes4(client, `${basePath}/min`),
    pct10: createIndexes5(client, `${basePath}/pct10`),
    pct25: createIndexes5(client, `${basePath}/pct25`),
    pct75: createIndexes5(client, `${basePath}/pct75`),
    pct90: createIndexes5(client, `${basePath}/pct90`),
    sum: createIndexes4(client, `${basePath}/sum`)
  };
}

/**
 * @template T
 * @typedef {Object} BlockSizePattern
 * @property {Indexes3<T>} average
 * @property {Indexes3<T>} cumulative
 * @property {Indexes3<T>} max
 * @property {Indexes2<T>} median
 * @property {Indexes3<T>} min
 * @property {Indexes2<T>} pct10
 * @property {Indexes2<T>} pct25
 * @property {Indexes2<T>} pct75
 * @property {Indexes2<T>} pct90
 * @property {Indexes3<T>} sum
 */

/**
 * Create a BlockSizePattern pattern node
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {BlockSizePattern}
 */
function createBlockSizePattern(client, basePath) {
  return {
    average: createIndexes3(client, `${basePath}/average`),
    cumulative: createIndexes3(client, `${basePath}/cumulative`),
    max: createIndexes3(client, `${basePath}/max`),
    median: createIndexes2(client, `${basePath}/median`),
    min: createIndexes3(client, `${basePath}/min`),
    pct10: createIndexes2(client, `${basePath}/pct10`),
    pct25: createIndexes2(client, `${basePath}/pct25`),
    pct75: createIndexes2(client, `${basePath}/pct75`),
    pct90: createIndexes2(client, `${basePath}/pct90`),
    sum: createIndexes3(client, `${basePath}/sum`)
  };
}

/**
 * @typedef {Object} RelativePattern
 * @property {Indexes21<StoredF32>} negUnrealizedLossRelToMarketCap
 * @property {Indexes21<StoredF32>} netUnrealizedPnlRelToMarketCap
 * @property {Indexes21<StoredF64>} supplyInLossRelToCirculatingSupply
 * @property {Indexes21<StoredF64>} supplyInLossRelToOwnSupply
 * @property {Indexes21<StoredF64>} supplyInProfitRelToCirculatingSupply
 * @property {Indexes21<StoredF64>} supplyInProfitRelToOwnSupply
 * @property {Indexes3<StoredF64>} supplyRelToCirculatingSupply
 * @property {Indexes21<StoredF32>} unrealizedLossRelToMarketCap
 * @property {Indexes21<StoredF32>} unrealizedProfitRelToMarketCap
 */

/**
 * Create a RelativePattern pattern node
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {RelativePattern}
 */
function createRelativePattern(client, basePath) {
  return {
    negUnrealizedLossRelToMarketCap: createIndexes21(client, `${basePath}/neg_unrealized_loss_rel_to_market_cap`),
    netUnrealizedPnlRelToMarketCap: createIndexes21(client, `${basePath}/net_unrealized_pnl_rel_to_market_cap`),
    supplyInLossRelToCirculatingSupply: createIndexes21(client, `${basePath}/supply_in_loss_rel_to_circulating_supply`),
    supplyInLossRelToOwnSupply: createIndexes21(client, `${basePath}/supply_in_loss_rel_to_own_supply`),
    supplyInProfitRelToCirculatingSupply: createIndexes21(client, `${basePath}/supply_in_profit_rel_to_circulating_supply`),
    supplyInProfitRelToOwnSupply: createIndexes21(client, `${basePath}/supply_in_profit_rel_to_own_supply`),
    supplyRelToCirculatingSupply: createIndexes3(client, `${basePath}/supply_rel_to_circulating_supply`),
    unrealizedLossRelToMarketCap: createIndexes21(client, `${basePath}/unrealized_loss_rel_to_market_cap`),
    unrealizedProfitRelToMarketCap: createIndexes21(client, `${basePath}/unrealized_profit_rel_to_market_cap`)
  };
}

/**
 * @typedef {Object} UnrealizedPattern
 * @property {Indexes21<Dollars>} negUnrealizedLoss
 * @property {Indexes21<Dollars>} netUnrealizedPnl
 * @property {SupplyPattern} supplyInLoss
 * @property {SupplyValuePattern} supplyInLossValue
 * @property {SupplyPattern} supplyInProfit
 * @property {SupplyValuePattern} supplyInProfitValue
 * @property {Indexes21<Dollars>} totalUnrealizedPnl
 * @property {Indexes21<Dollars>} unrealizedLoss
 * @property {Indexes21<Dollars>} unrealizedProfit
 */

/**
 * Create a UnrealizedPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {UnrealizedPattern}
 */
function createUnrealizedPattern(client, basePath) {
  return {
    negUnrealizedLoss: createIndexes21(client, `${basePath}/neg_unrealized_loss`),
    netUnrealizedPnl: createIndexes21(client, `${basePath}/net_unrealized_pnl`),
    supplyInLoss: createSupplyPattern(client, `${basePath}/supply_in_loss`),
    supplyInLossValue: createSupplyValuePattern(client, `${basePath}/supply_in_loss_value`),
    supplyInProfit: createSupplyPattern(client, `${basePath}/supply_in_profit`),
    supplyInProfitValue: createSupplyValuePattern(client, `${basePath}/supply_in_profit_value`),
    totalUnrealizedPnl: createIndexes21(client, `${basePath}/total_unrealized_pnl`),
    unrealizedLoss: createIndexes21(client, `${basePath}/unrealized_loss`),
    unrealizedProfit: createIndexes21(client, `${basePath}/unrealized_profit`)
  };
}

/**
 * @template T
 * @typedef {Object} BlockIntervalPattern
 * @property {Indexes4<T>} average
 * @property {Indexes4<T>} max
 * @property {Indexes5<T>} median
 * @property {Indexes4<T>} min
 * @property {Indexes5<T>} pct10
 * @property {Indexes5<T>} pct25
 * @property {Indexes5<T>} pct75
 * @property {Indexes5<T>} pct90
 */

/**
 * Create a BlockIntervalPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {BlockIntervalPattern}
 */
function createBlockIntervalPattern(client, acc) {
  return {
    average: createIndexes4(client, `/${acc}_avg`),
    max: createIndexes4(client, `/${acc}_max`),
    median: createIndexes5(client, `/${acc}_median`),
    min: createIndexes4(client, `/${acc}_min`),
    pct10: createIndexes5(client, `/${acc}_pct10`),
    pct25: createIndexes5(client, `/${acc}_pct25`),
    pct75: createIndexes5(client, `/${acc}_pct75`),
    pct90: createIndexes5(client, `/${acc}_pct90`)
  };
}

/**
 * @template T
 * @typedef {Object} AddresstypeToHeightToAddrCountPattern
 * @property {Indexes2<T>} p2a
 * @property {Indexes2<T>} p2pk33
 * @property {Indexes2<T>} p2pk65
 * @property {Indexes2<T>} p2pkh
 * @property {Indexes2<T>} p2sh
 * @property {Indexes2<T>} p2tr
 * @property {Indexes2<T>} p2wpkh
 * @property {Indexes2<T>} p2wsh
 */

/**
 * Create a AddresstypeToHeightToAddrCountPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {AddresstypeToHeightToAddrCountPattern}
 */
function createAddresstypeToHeightToAddrCountPattern(client, basePath) {
  return {
    p2a: createIndexes2(client, `${basePath}/p2a`),
    p2pk33: createIndexes2(client, `${basePath}/p2pk33`),
    p2pk65: createIndexes2(client, `${basePath}/p2pk65`),
    p2pkh: createIndexes2(client, `${basePath}/p2pkh`),
    p2sh: createIndexes2(client, `${basePath}/p2sh`),
    p2tr: createIndexes2(client, `${basePath}/p2tr`),
    p2wpkh: createIndexes2(client, `${basePath}/p2wpkh`),
    p2wsh: createIndexes2(client, `${basePath}/p2wsh`)
  };
}

/**
 * @typedef {Object} _0satsPattern
 * @property {ActivityPattern} activity
 * @property {Indexes3<StoredU64>} addrCount
 * @property {PricePaidPattern} pricePaid
 * @property {RealizedPattern} realized
 * @property {RelativePattern} relative
 * @property {SupplyPattern2} supply
 * @property {UnrealizedPattern} unrealized
 */

/**
 * Create a _0satsPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {_0satsPattern}
 */
function create_0satsPattern(client, basePath) {
  return {
    activity: createActivityPattern(client, `${basePath}/activity`),
    addrCount: createIndexes3(client, `${basePath}/addr_count`),
    pricePaid: createPricePaidPattern(client, `${basePath}/price_paid`),
    realized: createRealizedPattern(client, `${basePath}/realized`),
    relative: createRelativePattern(client, `${basePath}/relative`),
    supply: createSupplyPattern2(client, `${basePath}/supply`),
    unrealized: createUnrealizedPattern(client, `${basePath}/unrealized`)
  };
}

/**
 * @typedef {Object} _10yTo12yPattern
 * @property {ActivityPattern} activity
 * @property {PricePaidPattern2} pricePaid
 * @property {RealizedPattern2} realized
 * @property {RelativePattern2} relative
 * @property {SupplyPattern2} supply
 * @property {UnrealizedPattern} unrealized
 */

/**
 * Create a _10yTo12yPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {_10yTo12yPattern}
 */
function create_10yTo12yPattern(client, basePath) {
  return {
    activity: createActivityPattern(client, `${basePath}/activity`),
    pricePaid: createPricePaidPattern2(client, `${basePath}/price_paid`),
    realized: createRealizedPattern2(client, `${basePath}/realized`),
    relative: createRelativePattern2(client, `${basePath}/relative`),
    supply: createSupplyPattern2(client, `${basePath}/supply`),
    unrealized: createUnrealizedPattern(client, `${basePath}/unrealized`)
  };
}

/**
 * @typedef {Object} _0satsPattern2
 * @property {ActivityPattern} activity
 * @property {PricePaidPattern} pricePaid
 * @property {RealizedPattern} realized
 * @property {RelativePattern} relative
 * @property {SupplyPattern2} supply
 * @property {UnrealizedPattern} unrealized
 */

/**
 * Create a _0satsPattern2 pattern node
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {_0satsPattern2}
 */
function create_0satsPattern2(client, basePath) {
  return {
    activity: createActivityPattern(client, `${basePath}/activity`),
    pricePaid: createPricePaidPattern(client, `${basePath}/price_paid`),
    realized: createRealizedPattern(client, `${basePath}/realized`),
    relative: createRelativePattern(client, `${basePath}/relative`),
    supply: createSupplyPattern2(client, `${basePath}/supply`),
    unrealized: createUnrealizedPattern(client, `${basePath}/unrealized`)
  };
}

/**
 * @typedef {Object} UpTo1dPattern
 * @property {ActivityPattern} activity
 * @property {PricePaidPattern2} pricePaid
 * @property {RealizedPattern3} realized
 * @property {RelativePattern2} relative
 * @property {SupplyPattern2} supply
 * @property {UnrealizedPattern} unrealized
 */

/**
 * Create a UpTo1dPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {UpTo1dPattern}
 */
function createUpTo1dPattern(client, basePath) {
  return {
    activity: createActivityPattern(client, `${basePath}/activity`),
    pricePaid: createPricePaidPattern2(client, `${basePath}/price_paid`),
    realized: createRealizedPattern3(client, `${basePath}/realized`),
    relative: createRelativePattern2(client, `${basePath}/relative`),
    supply: createSupplyPattern2(client, `${basePath}/supply`),
    unrealized: createUnrealizedPattern(client, `${basePath}/unrealized`)
  };
}

/**
 * @typedef {Object} ActivityPattern
 * @property {BlockCountPattern} coinblocksDestroyed
 * @property {BlockCountPattern} coindaysDestroyed
 * @property {Indexes2<Sats>} satblocksDestroyed
 * @property {Indexes2<Sats>} satdaysDestroyed
 * @property {SentPattern} sent
 */

/**
 * Create a ActivityPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {ActivityPattern}
 */
function createActivityPattern(client, basePath) {
  return {
    coinblocksDestroyed: createBlockCountPattern(client, `${basePath}/coinblocks_destroyed`),
    coindaysDestroyed: createBlockCountPattern(client, `${basePath}/coindays_destroyed`),
    satblocksDestroyed: createIndexes2(client, `${basePath}/satblocks_destroyed`),
    satdaysDestroyed: createIndexes2(client, `${basePath}/satdays_destroyed`),
    sent: createSentPattern(client, `${basePath}/sent`)
  };
}

/**
 * @typedef {Object} SupplyPattern2
 * @property {SupplyPattern} supply
 * @property {SentSumPattern} supplyHalf
 * @property {SentSumPattern} supplyHalfValue
 * @property {SupplyValuePattern} supplyValue
 * @property {Indexes3<StoredU64>} utxoCount
 */

/**
 * Create a SupplyPattern2 pattern node
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {SupplyPattern2}
 */
function createSupplyPattern2(client, basePath) {
  return {
    supply: createSupplyPattern(client, `${basePath}/supply`),
    supplyHalf: createSentSumPattern(client, `${basePath}/supply_half`),
    supplyHalfValue: createSentSumPattern(client, `${basePath}/supply_half_value`),
    supplyValue: createSupplyValuePattern(client, `${basePath}/supply_value`),
    utxoCount: createIndexes3(client, `${basePath}/utxo_count`)
  };
}

/**
 * @typedef {Object} SupplyPattern
 * @property {Indexes2<Sats>} base
 * @property {Indexes<Bitcoin>} bitcoin
 * @property {Indexes<Dollars>} dollars
 * @property {Indexes<Sats>} sats
 */

/**
 * Create a SupplyPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {SupplyPattern}
 */
function createSupplyPattern(client, basePath) {
  return {
    base: createIndexes2(client, `${basePath}/base`),
    bitcoin: createIndexes(client, `${basePath}/bitcoin`),
    dollars: createIndexes(client, `${basePath}/dollars`),
    sats: createIndexes(client, `${basePath}/sats`)
  };
}

/**
 * @typedef {Object} SentPattern
 * @property {Indexes2<Sats>} base
 * @property {BlockCountPattern} bitcoin
 * @property {BlockCountPattern} dollars
 * @property {SatsPattern} sats
 */

/**
 * Create a SentPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {SentPattern}
 */
function createSentPattern(client, basePath) {
  return {
    base: createIndexes2(client, `${basePath}/base`),
    bitcoin: createBlockCountPattern(client, `${basePath}/bitcoin`),
    dollars: createBlockCountPattern(client, `${basePath}/dollars`),
    sats: createSatsPattern(client, `${basePath}/sats`)
  };
}

/**
 * @typedef {Object} PricePaidPattern2
 * @property {Indexes3<Dollars>} maxPricePaid
 * @property {Indexes3<Dollars>} minPricePaid
 * @property {PricePercentilesPattern} pricePercentiles
 */

/**
 * Create a PricePaidPattern2 pattern node
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {PricePaidPattern2}
 */
function createPricePaidPattern2(client, basePath) {
  return {
    maxPricePaid: createIndexes3(client, `${basePath}/max_price_paid`),
    minPricePaid: createIndexes3(client, `${basePath}/min_price_paid`),
    pricePercentiles: createPricePercentilesPattern(client, `${basePath}/price_percentiles`)
  };
}

/**
 * @typedef {Object} CoinbasePattern
 * @property {BitcoinPattern} bitcoin
 * @property {BitcoinPattern} dollars
 * @property {BitcoinPattern} sats
 */

/**
 * Create a CoinbasePattern pattern node
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {CoinbasePattern}
 */
function createCoinbasePattern(client, basePath) {
  return {
    bitcoin: createBitcoinPattern(client, `${basePath}/bitcoin`),
    dollars: createBitcoinPattern(client, `${basePath}/dollars`),
    sats: createBitcoinPattern(client, `${basePath}/sats`)
  };
}

/**
 * @typedef {Object} SentSumPattern
 * @property {Indexes3<Bitcoin>} bitcoin
 * @property {Indexes3<Dollars>} dollars
 * @property {Indexes3<Sats>} sats
 */

/**
 * Create a SentSumPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {SentSumPattern}
 */
function createSentSumPattern(client, basePath) {
  return {
    bitcoin: createIndexes3(client, `${basePath}/bitcoin`),
    dollars: createIndexes3(client, `${basePath}/dollars`),
    sats: createIndexes3(client, `${basePath}/sats`)
  };
}

/**
 * @typedef {Object} UnclaimedRewardsPattern
 * @property {BlockCountPattern} bitcoin
 * @property {BlockCountPattern} dollars
 * @property {BlockCountPattern} sats
 */

/**
 * Create a UnclaimedRewardsPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {UnclaimedRewardsPattern}
 */
function createUnclaimedRewardsPattern(client, basePath) {
  return {
    bitcoin: createBlockCountPattern(client, `${basePath}/bitcoin`),
    dollars: createBlockCountPattern(client, `${basePath}/dollars`),
    sats: createBlockCountPattern(client, `${basePath}/sats`)
  };
}

/**
 * @template T
 * @typedef {Object} BlockCountPattern
 * @property {Indexes2<T>} base
 * @property {Indexes3<T>} cumulative
 * @property {Indexes4<T>} sum
 */

/**
 * Create a BlockCountPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {BlockCountPattern}
 */
function createBlockCountPattern(client, basePath) {
  return {
    base: createIndexes2(client, `${basePath}/base`),
    cumulative: createIndexes3(client, `${basePath}/cumulative`),
    sum: createIndexes4(client, `${basePath}/sum`)
  };
}

/**
 * @typedef {Object} _1dReturns1mSdPattern
 * @property {Indexes<StoredF32>} sd
 * @property {Indexes<StoredF32>} sma
 */

/**
 * Create a _1dReturns1mSdPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {_1dReturns1mSdPattern}
 */
function create_1dReturns1mSdPattern(client, acc) {
  return {
    sd: createIndexes(client, `/${acc}_sd`),
    sma: createIndexes(client, `/${acc}_sma`)
  };
}

/**
 * @typedef {Object} SupplyValuePattern
 * @property {Indexes2<Bitcoin>} bitcoin
 * @property {Indexes2<Dollars>} dollars
 */

/**
 * Create a SupplyValuePattern pattern node
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {SupplyValuePattern}
 */
function createSupplyValuePattern(client, basePath) {
  return {
    bitcoin: createIndexes2(client, `${basePath}/bitcoin`),
    dollars: createIndexes2(client, `${basePath}/dollars`)
  };
}

/**
 * @typedef {Object} SatsPattern
 * @property {Indexes3<Sats>} cumulative
 * @property {Indexes4<Sats>} sum
 */

/**
 * Create a SatsPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {SatsPattern}
 */
function createSatsPattern(client, basePath) {
  return {
    cumulative: createIndexes3(client, `${basePath}/cumulative`),
    sum: createIndexes4(client, `${basePath}/sum`)
  };
}

/**
 * @typedef {Object} PricePaidPattern
 * @property {Indexes3<Dollars>} maxPricePaid
 * @property {Indexes3<Dollars>} minPricePaid
 */

/**
 * Create a PricePaidPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {PricePaidPattern}
 */
function createPricePaidPattern(client, basePath) {
  return {
    maxPricePaid: createIndexes3(client, `${basePath}/max_price_paid`),
    minPricePaid: createIndexes3(client, `${basePath}/min_price_paid`)
  };
}

/**
 * @typedef {Object} RealizedPriceExtraPattern
 * @property {Indexes<StoredF32>} ratio
 */

/**
 * Create a RealizedPriceExtraPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {RealizedPriceExtraPattern}
 */
function createRealizedPriceExtraPattern(client, basePath) {
  return {
    ratio: createIndexes(client, `${basePath}/ratio`)
  };
}

// Catalog tree typedefs

/**
 * @typedef {Object} CatalogTree
 * @property {MetricNode<CatalogTree_Computed>} computed
 * @property {MetricNode<CatalogTree_Indexed>} indexed
 */

/**
 * @typedef {Object} CatalogTree_Computed
 * @property {MetricNode<CatalogTree_Computed_Blks>} blks
 * @property {MetricNode<CatalogTree_Computed_Chain>} chain
 * @property {MetricNode<CatalogTree_Computed_Cointime>} cointime
 * @property {MetricNode<CatalogTree_Computed_Constants>} constants
 * @property {MetricNode<CatalogTree_Computed_Fetched>} fetched
 * @property {MetricNode<CatalogTree_Computed_Indexes>} indexes
 * @property {MetricNode<CatalogTree_Computed_Market>} market
 * @property {MetricNode<CatalogTree_Computed_Pools>} pools
 * @property {MetricNode<CatalogTree_Computed_Price>} price
 * @property {MetricNode<CatalogTree_Computed_Stateful>} stateful
 */

/**
 * @typedef {Object} CatalogTree_Computed_Blks
 * @property {MetricNode<BlkPosition>} position
 */

/**
 * @typedef {Object} CatalogTree_Computed_Chain
 * @property {Indexes<StoredU32>} _1mBlockCount
 * @property {Indexes<StoredU32>} _1wBlockCount
 * @property {Indexes<StoredU32>} _1yBlockCount
 * @property {Indexes2<StoredU32>} _24hBlockCount
 * @property {Indexes2<Sats>} _24hCoinbaseSum
 * @property {Indexes2<Dollars>} _24hCoinbaseUsdSum
 * @property {Indexes<Sats>} annualizedVolume
 * @property {Indexes<Bitcoin>} annualizedVolumeBtc
 * @property {Indexes<Dollars>} annualizedVolumeUsd
 * @property {BlockCountPattern<StoredU32>} blockCount
 * @property {Indexes<StoredU64>} blockCountTarget
 * @property {BlockIntervalPattern<Timestamp>} blockInterval
 * @property {BlockSizePattern<StoredU64>} blockSize
 * @property {BlockSizePattern<StoredU64>} blockVbytes
 * @property {BlockSizePattern<Weight>} blockWeight
 * @property {Indexes3<StoredU32>} blocksBeforeNextDifficultyAdjustment
 * @property {Indexes3<StoredU32>} blocksBeforeNextHalving
 * @property {CoinbasePattern} coinbase
 * @property {Indexes3<StoredF32>} daysBeforeNextDifficultyAdjustment
 * @property {Indexes3<StoredF32>} daysBeforeNextHalving
 * @property {Indexes4<StoredF64>} difficulty
 * @property {Indexes3<StoredF32>} difficultyAdjustment
 * @property {Indexes3<StoredF32>} difficultyAsHash
 * @property {Indexes<DifficultyEpoch>} difficultyepoch
 * @property {BitcoinPattern<StoredU64>} emptyoutputCount
 * @property {Indexes3<StoredU64>} exactUtxoCount
 * @property {MetricNode<CatalogTree_Computed_Chain_Fee>} fee
 * @property {Indexes5<StoredF32>} feeDominance
 * @property {MetricNode<CatalogTree_Computed_Chain_FeeRate>} feeRate
 * @property {Indexes<HalvingEpoch>} halvingepoch
 * @property {Indexes3<StoredF32>} hashPricePhs
 * @property {Indexes3<StoredF32>} hashPricePhsMin
 * @property {Indexes3<StoredF32>} hashPriceRebound
 * @property {Indexes3<StoredF32>} hashPriceThs
 * @property {Indexes3<StoredF32>} hashPriceThsMin
 * @property {Indexes3<StoredF64>} hashRate
 * @property {Indexes<StoredF32>} hashRate1mSma
 * @property {Indexes<StoredF64>} hashRate1wSma
 * @property {Indexes<StoredF32>} hashRate1ySma
 * @property {Indexes<StoredF32>} hashRate2mSma
 * @property {Indexes3<StoredF32>} hashValuePhs
 * @property {Indexes3<StoredF32>} hashValuePhsMin
 * @property {Indexes3<StoredF32>} hashValueRebound
 * @property {Indexes3<StoredF32>} hashValueThs
 * @property {Indexes3<StoredF32>} hashValueThsMin
 * @property {Indexes<StoredF32>} inflationRate
 * @property {BlockSizePattern<StoredU64>} inputCount
 * @property {Indexes6<Sats>} inputValue
 * @property {Indexes<StoredF32>} inputsPerSec
 * @property {Indexes2<Timestamp>} interval
 * @property {Indexes6<StoredBool>} isCoinbase
 * @property {BitcoinPattern<StoredU64>} opreturnCount
 * @property {BlockSizePattern<StoredU64>} outputCount
 * @property {Indexes6<Sats>} outputValue
 * @property {Indexes<StoredF32>} outputsPerSec
 * @property {BitcoinPattern<StoredU64>} p2aCount
 * @property {BitcoinPattern<StoredU64>} p2msCount
 * @property {BitcoinPattern<StoredU64>} p2pk33Count
 * @property {BitcoinPattern<StoredU64>} p2pk65Count
 * @property {BitcoinPattern<StoredU64>} p2pkhCount
 * @property {BitcoinPattern<StoredU64>} p2shCount
 * @property {BitcoinPattern<StoredU64>} p2trCount
 * @property {BitcoinPattern<StoredU64>} p2wpkhCount
 * @property {BitcoinPattern<StoredU64>} p2wshCount
 * @property {Indexes<StoredF32>} puellMultiple
 * @property {SentSumPattern} sentSum
 * @property {CoinbasePattern} subsidy
 * @property {Indexes5<StoredF32>} subsidyDominance
 * @property {Indexes<Dollars>} subsidyUsd1ySma
 * @property {MetricNode<Timestamp>} timestamp
 * @property {Indexes<StoredF64>} txBtcVelocity
 * @property {BitcoinPattern<StoredU64>} txCount
 * @property {Indexes<StoredF32>} txPerSec
 * @property {Indexes<StoredF64>} txUsdVelocity
 * @property {BlockCountPattern<StoredU64>} txV1
 * @property {BlockCountPattern<StoredU64>} txV2
 * @property {BlockCountPattern<StoredU64>} txV3
 * @property {BlockIntervalPattern<VSize>} txVsize
 * @property {BlockIntervalPattern<Weight>} txWeight
 * @property {UnclaimedRewardsPattern} unclaimedRewards
 * @property {BitcoinPattern<StoredU64>} unknownoutputCount
 * @property {Indexes7<Sats>} value
 * @property {Indexes2<StoredU64>} vbytes
 * @property {Indexes6<VSize>} vsize
 * @property {Indexes6<Weight>} weight
 */

/**
 * @typedef {Object} CatalogTree_Computed_Chain_Fee
 * @property {Indexes6<Sats>} base
 * @property {BlockSizePattern<Bitcoin>} bitcoin
 * @property {Indexes6<Bitcoin>} bitcoinTxindex
 * @property {BlockSizePattern<Dollars>} dollars
 * @property {Indexes6<Dollars>} dollarsTxindex
 * @property {BlockSizePattern<Sats>} sats
 */

/**
 * @typedef {Object} CatalogTree_Computed_Chain_FeeRate
 * @property {Indexes3<FeeRate>} average
 * @property {Indexes6<FeeRate>} base
 * @property {Indexes3<FeeRate>} max
 * @property {Indexes2<FeeRate>} median
 * @property {Indexes3<FeeRate>} min
 * @property {Indexes2<FeeRate>} pct10
 * @property {Indexes2<FeeRate>} pct25
 * @property {Indexes2<FeeRate>} pct75
 * @property {Indexes2<FeeRate>} pct90
 */

/**
 * @typedef {Object} CatalogTree_Computed_Cointime
 * @property {Indexes3<Dollars>} activeCap
 * @property {Indexes3<Dollars>} activePrice
 * @property {ActivePriceRatioPattern} activePriceRatio
 * @property {SentSumPattern} activeSupply
 * @property {Indexes3<StoredF64>} activityToVaultednessRatio
 * @property {BlockCountPattern<StoredF64>} coinblocksCreated
 * @property {BlockCountPattern<StoredF64>} coinblocksStored
 * @property {Indexes<StoredF32>} cointimeAdjInflationRate
 * @property {Indexes<StoredF64>} cointimeAdjTxBtcVelocity
 * @property {Indexes<StoredF64>} cointimeAdjTxUsdVelocity
 * @property {Indexes3<Dollars>} cointimeCap
 * @property {Indexes3<Dollars>} cointimePrice
 * @property {ActivePriceRatioPattern} cointimePriceRatio
 * @property {BlockCountPattern<StoredF64>} cointimeValueCreated
 * @property {BlockCountPattern<StoredF64>} cointimeValueDestroyed
 * @property {BlockCountPattern<StoredF64>} cointimeValueStored
 * @property {Indexes3<Dollars>} investorCap
 * @property {Indexes3<StoredF64>} liveliness
 * @property {Indexes3<Dollars>} thermoCap
 * @property {Indexes3<Dollars>} trueMarketMean
 * @property {ActivePriceRatioPattern} trueMarketMeanRatio
 * @property {Indexes3<Dollars>} vaultedCap
 * @property {Indexes3<Dollars>} vaultedPrice
 * @property {ActivePriceRatioPattern} vaultedPriceRatio
 * @property {SentSumPattern} vaultedSupply
 * @property {Indexes3<StoredF64>} vaultedness
 */

/**
 * @typedef {Object} CatalogTree_Computed_Constants
 * @property {Indexes3<StoredU16>} constant0
 * @property {Indexes3<StoredU16>} constant1
 * @property {Indexes3<StoredU16>} constant100
 * @property {Indexes3<StoredU16>} constant2
 * @property {Indexes3<StoredU16>} constant3
 * @property {Indexes3<StoredF32>} constant382
 * @property {Indexes3<StoredU16>} constant4
 * @property {Indexes3<StoredU16>} constant50
 * @property {Indexes3<StoredU16>} constant600
 * @property {Indexes3<StoredF32>} constant618
 * @property {Indexes3<StoredI16>} constantMinus1
 * @property {Indexes3<StoredI16>} constantMinus2
 * @property {Indexes3<StoredI16>} constantMinus3
 * @property {Indexes3<StoredI16>} constantMinus4
 */

/**
 * @typedef {Object} CatalogTree_Computed_Fetched
 * @property {Indexes8<OHLCCents>} priceOhlcInCents
 */

/**
 * @typedef {Object} CatalogTree_Computed_Indexes
 * @property {Indexes8<Date>} date
 * @property {Indexes2<Date>} dateFixed
 * @property {Indexes8<DateIndex>} dateindex
 * @property {Indexes9<StoredU64>} dateindexCount
 * @property {MetricNode<DecadeIndex>} decadeindex
 * @property {MetricNode<DifficultyEpoch>} difficultyepoch
 * @property {MetricNode<EmptyOutputIndex>} emptyoutputindex
 * @property {Indexes9<DateIndex>} firstDateindex
 * @property {MetricNode<Height>} firstHeight
 * @property {Indexes10<MonthIndex>} firstMonthindex
 * @property {Indexes11<YearIndex>} firstYearindex
 * @property {MetricNode<HalvingEpoch>} halvingepoch
 * @property {Indexes2<Height>} height
 * @property {MetricNode<StoredU64>} heightCount
 * @property {Indexes6<StoredU64>} inputCount
 * @property {MetricNode<MonthIndex>} monthindex
 * @property {Indexes10<StoredU64>} monthindexCount
 * @property {MetricNode<OpReturnIndex>} opreturnindex
 * @property {Indexes6<StoredU64>} outputCount
 * @property {Indexes12<P2AAddressIndex>} p2aaddressindex
 * @property {MetricNode<P2MSOutputIndex>} p2msoutputindex
 * @property {Indexes13<P2PK33AddressIndex>} p2pk33addressindex
 * @property {Indexes14<P2PK65AddressIndex>} p2pk65addressindex
 * @property {Indexes15<P2PKHAddressIndex>} p2pkhaddressindex
 * @property {Indexes16<P2SHAddressIndex>} p2shaddressindex
 * @property {Indexes17<P2TRAddressIndex>} p2traddressindex
 * @property {Indexes18<P2WPKHAddressIndex>} p2wpkhaddressindex
 * @property {Indexes19<P2WSHAddressIndex>} p2wshaddressindex
 * @property {MetricNode<QuarterIndex>} quarterindex
 * @property {MetricNode<SemesterIndex>} semesterindex
 * @property {Indexes2<Timestamp>} timestampFixed
 * @property {Indexes6<TxIndex>} txindex
 * @property {Indexes2<StoredU64>} txindexCount
 * @property {Indexes7<TxInIndex>} txinindex
 * @property {Indexes20<TxOutIndex>} txoutindex
 * @property {MetricNode<UnknownOutputIndex>} unknownoutputindex
 * @property {MetricNode<WeekIndex>} weekindex
 * @property {MetricNode<YearIndex>} yearindex
 * @property {Indexes11<StoredU64>} yearindexCount
 */

/**
 * @typedef {Object} CatalogTree_Computed_Market
 * @property {_1dReturns1mSdPattern} _1dReturns1mSd
 * @property {_1dReturns1mSdPattern} _1dReturns1wSd
 * @property {_1dReturns1mSdPattern} _1dReturns1ySd
 * @property {Indexes<StoredF32>} _10yCagr
 * @property {Indexes<Dollars>} _10yDcaAvgPrice
 * @property {Indexes<StoredF32>} _10yDcaCagr
 * @property {Indexes<StoredF32>} _10yDcaReturns
 * @property {Indexes<Sats>} _10yDcaStack
 * @property {Indexes<StoredF32>} _10yPriceReturns
 * @property {Indexes<StoredF32>} _1dPriceReturns
 * @property {Indexes<Dollars>} _1mDcaAvgPrice
 * @property {Indexes<StoredF32>} _1mDcaReturns
 * @property {Indexes<Sats>} _1mDcaStack
 * @property {Indexes<StoredF32>} _1mPriceReturns
 * @property {Indexes<Dollars>} _1wDcaAvgPrice
 * @property {Indexes<StoredF32>} _1wDcaReturns
 * @property {Indexes<Sats>} _1wDcaStack
 * @property {Indexes<StoredF32>} _1wPriceReturns
 * @property {Indexes<Dollars>} _1yDcaAvgPrice
 * @property {Indexes<StoredF32>} _1yDcaReturns
 * @property {Indexes<Sats>} _1yDcaStack
 * @property {Indexes<StoredF32>} _1yPriceReturns
 * @property {Indexes<StoredF32>} _2yCagr
 * @property {Indexes<Dollars>} _2yDcaAvgPrice
 * @property {Indexes<StoredF32>} _2yDcaCagr
 * @property {Indexes<StoredF32>} _2yDcaReturns
 * @property {Indexes<Sats>} _2yDcaStack
 * @property {Indexes<StoredF32>} _2yPriceReturns
 * @property {Indexes<Dollars>} _3mDcaAvgPrice
 * @property {Indexes<StoredF32>} _3mDcaReturns
 * @property {Indexes<Sats>} _3mDcaStack
 * @property {Indexes<StoredF32>} _3mPriceReturns
 * @property {Indexes<StoredF32>} _3yCagr
 * @property {Indexes<Dollars>} _3yDcaAvgPrice
 * @property {Indexes<StoredF32>} _3yDcaCagr
 * @property {Indexes<StoredF32>} _3yDcaReturns
 * @property {Indexes<Sats>} _3yDcaStack
 * @property {Indexes<StoredF32>} _3yPriceReturns
 * @property {Indexes<StoredF32>} _4yCagr
 * @property {Indexes<Dollars>} _4yDcaAvgPrice
 * @property {Indexes<StoredF32>} _4yDcaCagr
 * @property {Indexes<StoredF32>} _4yDcaReturns
 * @property {Indexes<Sats>} _4yDcaStack
 * @property {Indexes<StoredF32>} _4yPriceReturns
 * @property {Indexes<StoredF32>} _5yCagr
 * @property {Indexes<Dollars>} _5yDcaAvgPrice
 * @property {Indexes<StoredF32>} _5yDcaCagr
 * @property {Indexes<StoredF32>} _5yDcaReturns
 * @property {Indexes<Sats>} _5yDcaStack
 * @property {Indexes<StoredF32>} _5yPriceReturns
 * @property {Indexes<Dollars>} _6mDcaAvgPrice
 * @property {Indexes<StoredF32>} _6mDcaReturns
 * @property {Indexes<Sats>} _6mDcaStack
 * @property {Indexes<StoredF32>} _6mPriceReturns
 * @property {Indexes<StoredF32>} _6yCagr
 * @property {Indexes<Dollars>} _6yDcaAvgPrice
 * @property {Indexes<StoredF32>} _6yDcaCagr
 * @property {Indexes<StoredF32>} _6yDcaReturns
 * @property {Indexes<Sats>} _6yDcaStack
 * @property {Indexes<StoredF32>} _6yPriceReturns
 * @property {Indexes<StoredF32>} _8yCagr
 * @property {Indexes<Dollars>} _8yDcaAvgPrice
 * @property {Indexes<StoredF32>} _8yDcaCagr
 * @property {Indexes<StoredF32>} _8yDcaReturns
 * @property {Indexes<Sats>} _8yDcaStack
 * @property {Indexes<StoredF32>} _8yPriceReturns
 * @property {Indexes<StoredU16>} daysSincePriceAth
 * @property {Indexes<Dollars>} dcaClass2015AvgPrice
 * @property {Indexes<StoredF32>} dcaClass2015Returns
 * @property {Indexes<Sats>} dcaClass2015Stack
 * @property {Indexes<Dollars>} dcaClass2016AvgPrice
 * @property {Indexes<StoredF32>} dcaClass2016Returns
 * @property {Indexes<Sats>} dcaClass2016Stack
 * @property {Indexes<Dollars>} dcaClass2017AvgPrice
 * @property {Indexes<StoredF32>} dcaClass2017Returns
 * @property {Indexes<Sats>} dcaClass2017Stack
 * @property {Indexes<Dollars>} dcaClass2018AvgPrice
 * @property {Indexes<StoredF32>} dcaClass2018Returns
 * @property {Indexes<Sats>} dcaClass2018Stack
 * @property {Indexes<Dollars>} dcaClass2019AvgPrice
 * @property {Indexes<StoredF32>} dcaClass2019Returns
 * @property {Indexes<Sats>} dcaClass2019Stack
 * @property {Indexes<Dollars>} dcaClass2020AvgPrice
 * @property {Indexes<StoredF32>} dcaClass2020Returns
 * @property {Indexes<Sats>} dcaClass2020Stack
 * @property {Indexes<Dollars>} dcaClass2021AvgPrice
 * @property {Indexes<StoredF32>} dcaClass2021Returns
 * @property {Indexes<Sats>} dcaClass2021Stack
 * @property {Indexes<Dollars>} dcaClass2022AvgPrice
 * @property {Indexes<StoredF32>} dcaClass2022Returns
 * @property {Indexes<Sats>} dcaClass2022Stack
 * @property {Indexes<Dollars>} dcaClass2023AvgPrice
 * @property {Indexes<StoredF32>} dcaClass2023Returns
 * @property {Indexes<Sats>} dcaClass2023Stack
 * @property {Indexes<Dollars>} dcaClass2024AvgPrice
 * @property {Indexes<StoredF32>} dcaClass2024Returns
 * @property {Indexes<Sats>} dcaClass2024Stack
 * @property {Indexes<Dollars>} dcaClass2025AvgPrice
 * @property {Indexes<StoredF32>} dcaClass2025Returns
 * @property {Indexes<Sats>} dcaClass2025Stack
 * @property {Indexes<StoredU16>} maxDaysBetweenPriceAths
 * @property {Indexes<StoredF32>} maxYearsBetweenPriceAths
 * @property {Indexes<Dollars>} price10yAgo
 * @property {Price13dEmaPattern} price13dEma
 * @property {Price13dEmaPattern} price13dSma
 * @property {Price13dEmaPattern} price144dEma
 * @property {Price13dEmaPattern} price144dSma
 * @property {Indexes<Dollars>} price1dAgo
 * @property {Indexes<Dollars>} price1mAgo
 * @property {Price13dEmaPattern} price1mEma
 * @property {Indexes<Dollars>} price1mMax
 * @property {Indexes<Dollars>} price1mMin
 * @property {Price13dEmaPattern} price1mSma
 * @property {Indexes<StoredF32>} price1mVolatility
 * @property {Indexes<Dollars>} price1wAgo
 * @property {Price13dEmaPattern} price1wEma
 * @property {Indexes<Dollars>} price1wMax
 * @property {Indexes<Dollars>} price1wMin
 * @property {Price13dEmaPattern} price1wSma
 * @property {Indexes<StoredF32>} price1wVolatility
 * @property {Indexes<Dollars>} price1yAgo
 * @property {Price13dEmaPattern} price1yEma
 * @property {Indexes<Dollars>} price1yMax
 * @property {Indexes<Dollars>} price1yMin
 * @property {Price13dEmaPattern} price1ySma
 * @property {Indexes<StoredF32>} price1yVolatility
 * @property {Price13dEmaPattern} price200dEma
 * @property {Price13dEmaPattern} price200dSma
 * @property {Indexes<Dollars>} price200dSmaX08
 * @property {Indexes<Dollars>} price200dSmaX24
 * @property {Price13dEmaPattern} price200wEma
 * @property {Price13dEmaPattern} price200wSma
 * @property {Price13dEmaPattern} price21dEma
 * @property {Price13dEmaPattern} price21dSma
 * @property {Indexes<StoredF32>} price2wChoppinessIndex
 * @property {Indexes<Dollars>} price2wMax
 * @property {Indexes<Dollars>} price2wMin
 * @property {Indexes<Dollars>} price2yAgo
 * @property {Price13dEmaPattern} price2yEma
 * @property {Price13dEmaPattern} price2ySma
 * @property {Price13dEmaPattern} price34dEma
 * @property {Price13dEmaPattern} price34dSma
 * @property {Indexes<Dollars>} price3mAgo
 * @property {Indexes<Dollars>} price3yAgo
 * @property {Indexes<Dollars>} price4yAgo
 * @property {Price13dEmaPattern} price4yEma
 * @property {Price13dEmaPattern} price4ySma
 * @property {Price13dEmaPattern} price55dEma
 * @property {Price13dEmaPattern} price55dSma
 * @property {Indexes<Dollars>} price5yAgo
 * @property {Indexes<Dollars>} price6mAgo
 * @property {Indexes<Dollars>} price6yAgo
 * @property {Price13dEmaPattern} price89dEma
 * @property {Price13dEmaPattern} price89dSma
 * @property {Price13dEmaPattern} price8dEma
 * @property {Price13dEmaPattern} price8dSma
 * @property {Indexes<Dollars>} price8yAgo
 * @property {Indexes21<Dollars>} priceAth
 * @property {Indexes21<StoredF32>} priceDrawdown
 * @property {Indexes5<StoredF32>} priceTrueRange
 * @property {Indexes5<StoredF32>} priceTrueRange2wSum
 */

/**
 * @typedef {Object} CatalogTree_Computed_Pools
 * @property {Indexes2<PoolSlug>} pool
 * @property {MetricNode<CatalogTree_Computed_Pools_Vecs>} vecs
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
 * @typedef {Object} CatalogTree_Computed_Price
 * @property {Indexes3<Dollars>} priceClose
 * @property {Indexes8<Cents>} priceCloseInCents
 * @property {Indexes3<Sats>} priceCloseInSats
 * @property {Indexes3<Dollars>} priceHigh
 * @property {Indexes8<Cents>} priceHighInCents
 * @property {Indexes3<Sats>} priceHighInSats
 * @property {Indexes3<Dollars>} priceLow
 * @property {Indexes8<Cents>} priceLowInCents
 * @property {Indexes3<Sats>} priceLowInSats
 * @property {Indexes3<OHLCDollars>} priceOhlc
 * @property {Indexes3<OHLCSats>} priceOhlcInSats
 * @property {Indexes3<Dollars>} priceOpen
 * @property {Indexes8<Cents>} priceOpenInCents
 * @property {Indexes3<Sats>} priceOpenInSats
 */

/**
 * @typedef {Object} CatalogTree_Computed_Stateful
 * @property {Indexes3<StoredU64>} addrCount
 * @property {MetricNode<CatalogTree_Computed_Stateful_AddressCohorts>} addressCohorts
 * @property {MetricNode<CatalogTree_Computed_Stateful_AddressesData>} addressesData
 * @property {AddresstypeToHeightToAddrCountPattern<StoredU64>} addresstypeToHeightToAddrCount
 * @property {AddresstypeToHeightToAddrCountPattern<StoredU64>} addresstypeToHeightToEmptyAddrCount
 * @property {AddresstypeToHeightToAddrCountPattern<StoredU64>} addresstypeToIndexesToAddrCount
 * @property {AddresstypeToHeightToAddrCountPattern<StoredU64>} addresstypeToIndexesToEmptyAddrCount
 * @property {AddresstypeToHeightToAddrCountPattern<AnyAddressIndex>} anyAddressIndexes
 * @property {Indexes2<SupplyState>} chainState
 * @property {Indexes3<StoredU64>} emptyAddrCount
 * @property {Indexes23<EmptyAddressIndex>} emptyaddressindex
 * @property {Indexes24<LoadedAddressIndex>} loadedaddressindex
 * @property {Indexes21<Dollars>} marketCap
 * @property {SupplyPattern} opreturnSupply
 * @property {Indexes20<TxInIndex>} txinindex
 * @property {SupplyPattern} unspendableSupply
 * @property {MetricNode<CatalogTree_Computed_Stateful_UtxoCohorts>} utxoCohorts
 */

/**
 * @typedef {Object} CatalogTree_Computed_Stateful_AddressCohorts
 * @property {MetricNode<CatalogTree_Computed_Stateful_AddressCohorts_AmountRange>} amountRange
 * @property {MetricNode<CatalogTree_Computed_Stateful_AddressCohorts_GeAmount>} geAmount
 * @property {MetricNode<CatalogTree_Computed_Stateful_AddressCohorts_LtAmount>} ltAmount
 */

/**
 * @typedef {Object} CatalogTree_Computed_Stateful_AddressCohorts_AmountRange
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
 * @typedef {Object} CatalogTree_Computed_Stateful_AddressCohorts_GeAmount
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
 * @typedef {Object} CatalogTree_Computed_Stateful_AddressCohorts_LtAmount
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
 * @typedef {Object} CatalogTree_Computed_Stateful_AddressesData
 * @property {Indexes23<EmptyAddressData>} empty
 * @property {Indexes24<LoadedAddressData>} loaded
 */

/**
 * @typedef {Object} CatalogTree_Computed_Stateful_UtxoCohorts
 * @property {MetricNode<CatalogTree_Computed_Stateful_UtxoCohorts_AgeRange>} ageRange
 * @property {MetricNode<CatalogTree_Computed_Stateful_UtxoCohorts_All>} all
 * @property {MetricNode<CatalogTree_Computed_Stateful_UtxoCohorts_AmountRange>} amountRange
 * @property {MetricNode<CatalogTree_Computed_Stateful_UtxoCohorts_Epoch>} epoch
 * @property {MetricNode<CatalogTree_Computed_Stateful_UtxoCohorts_GeAmount>} geAmount
 * @property {MetricNode<CatalogTree_Computed_Stateful_UtxoCohorts_LtAmount>} ltAmount
 * @property {MetricNode<CatalogTree_Computed_Stateful_UtxoCohorts_MaxAge>} maxAge
 * @property {MetricNode<CatalogTree_Computed_Stateful_UtxoCohorts_MinAge>} minAge
 * @property {MetricNode<CatalogTree_Computed_Stateful_UtxoCohorts_Term>} term
 * @property {MetricNode<CatalogTree_Computed_Stateful_UtxoCohorts_Type>} type
 * @property {MetricNode<CatalogTree_Computed_Stateful_UtxoCohorts_Year>} year
 */

/**
 * @typedef {Object} CatalogTree_Computed_Stateful_UtxoCohorts_AgeRange
 * @property {_10yTo12yPattern} _10yTo12y
 * @property {_10yTo12yPattern} _12yTo15y
 * @property {_10yTo12yPattern} _1dTo1w
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
 * @property {UpTo1dPattern} upTo1d
 */

/**
 * @typedef {Object} CatalogTree_Computed_Stateful_UtxoCohorts_All
 * @property {ActivityPattern} activity
 * @property {PricePaidPattern2} pricePaid
 * @property {RealizedPattern3} realized
 * @property {MetricNode<CatalogTree_Computed_Stateful_UtxoCohorts_All_Relative>} relative
 * @property {SupplyPattern2} supply
 * @property {UnrealizedPattern} unrealized
 */

/**
 * @typedef {Object} CatalogTree_Computed_Stateful_UtxoCohorts_All_Relative
 * @property {Indexes21<StoredF32>} negUnrealizedLossRelToMarketCap
 * @property {Indexes21<StoredF32>} negUnrealizedLossRelToOwnTotalUnrealizedPnl
 * @property {Indexes21<StoredF32>} netUnrealizedPnlRelToMarketCap
 * @property {Indexes21<StoredF32>} netUnrealizedPnlRelToOwnTotalUnrealizedPnl
 * @property {Indexes21<StoredF64>} supplyInLossRelToOwnSupply
 * @property {Indexes21<StoredF64>} supplyInProfitRelToOwnSupply
 * @property {Indexes21<StoredF32>} unrealizedLossRelToMarketCap
 * @property {Indexes21<StoredF32>} unrealizedLossRelToOwnTotalUnrealizedPnl
 * @property {Indexes21<StoredF32>} unrealizedProfitRelToMarketCap
 * @property {Indexes21<StoredF32>} unrealizedProfitRelToOwnTotalUnrealizedPnl
 */

/**
 * @typedef {Object} CatalogTree_Computed_Stateful_UtxoCohorts_AmountRange
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
 * @typedef {Object} CatalogTree_Computed_Stateful_UtxoCohorts_Epoch
 * @property {_10yTo12yPattern} _0
 * @property {_10yTo12yPattern} _1
 * @property {_10yTo12yPattern} _2
 * @property {_10yTo12yPattern} _3
 * @property {_10yTo12yPattern} _4
 */

/**
 * @typedef {Object} CatalogTree_Computed_Stateful_UtxoCohorts_GeAmount
 * @property {_0satsPattern2} _100btc
 * @property {_0satsPattern2} _100kSats
 * @property {_0satsPattern2} _100sats
 * @property {_0satsPattern2} _10btc
 * @property {_0satsPattern2} _10kBtc
 * @property {_0satsPattern2} _10kSats
 * @property {_0satsPattern2} _10mSats
 * @property {_0satsPattern2} _10sats
 * @property {_0satsPattern2} _1btc
 * @property {_0satsPattern2} _1kBtc
 * @property {_0satsPattern2} _1kSats
 * @property {_0satsPattern2} _1mSats
 * @property {_0satsPattern2} _1sat
 */

/**
 * @typedef {Object} CatalogTree_Computed_Stateful_UtxoCohorts_LtAmount
 * @property {_0satsPattern2} _100btc
 * @property {_0satsPattern2} _100kBtc
 * @property {_0satsPattern2} _100kSats
 * @property {_0satsPattern2} _100sats
 * @property {_0satsPattern2} _10btc
 * @property {_0satsPattern2} _10kBtc
 * @property {_0satsPattern2} _10kSats
 * @property {_0satsPattern2} _10mSats
 * @property {_0satsPattern2} _10sats
 * @property {_0satsPattern2} _1btc
 * @property {_0satsPattern2} _1kBtc
 * @property {_0satsPattern2} _1kSats
 * @property {_0satsPattern2} _1mSats
 */

/**
 * @typedef {Object} CatalogTree_Computed_Stateful_UtxoCohorts_MaxAge
 * @property {UpTo1dPattern} _10y
 * @property {UpTo1dPattern} _12y
 * @property {UpTo1dPattern} _15y
 * @property {UpTo1dPattern} _1m
 * @property {UpTo1dPattern} _1w
 * @property {UpTo1dPattern} _1y
 * @property {UpTo1dPattern} _2m
 * @property {UpTo1dPattern} _2y
 * @property {UpTo1dPattern} _3m
 * @property {UpTo1dPattern} _3y
 * @property {UpTo1dPattern} _4m
 * @property {UpTo1dPattern} _4y
 * @property {UpTo1dPattern} _5m
 * @property {UpTo1dPattern} _5y
 * @property {UpTo1dPattern} _6m
 * @property {UpTo1dPattern} _6y
 * @property {UpTo1dPattern} _7y
 * @property {UpTo1dPattern} _8y
 */

/**
 * @typedef {Object} CatalogTree_Computed_Stateful_UtxoCohorts_MinAge
 * @property {_10yTo12yPattern} _10y
 * @property {_10yTo12yPattern} _12y
 * @property {_10yTo12yPattern} _1d
 * @property {_10yTo12yPattern} _1m
 * @property {_10yTo12yPattern} _1w
 * @property {_10yTo12yPattern} _1y
 * @property {_10yTo12yPattern} _2m
 * @property {_10yTo12yPattern} _2y
 * @property {_10yTo12yPattern} _3m
 * @property {_10yTo12yPattern} _3y
 * @property {_10yTo12yPattern} _4m
 * @property {_10yTo12yPattern} _4y
 * @property {_10yTo12yPattern} _5m
 * @property {_10yTo12yPattern} _5y
 * @property {_10yTo12yPattern} _6m
 * @property {_10yTo12yPattern} _6y
 * @property {_10yTo12yPattern} _7y
 * @property {_10yTo12yPattern} _8y
 */

/**
 * @typedef {Object} CatalogTree_Computed_Stateful_UtxoCohorts_Term
 * @property {UpTo1dPattern} long
 * @property {UpTo1dPattern} short
 */

/**
 * @typedef {Object} CatalogTree_Computed_Stateful_UtxoCohorts_Type
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
 * @typedef {Object} CatalogTree_Computed_Stateful_UtxoCohorts_Year
 * @property {_10yTo12yPattern} _2009
 * @property {_10yTo12yPattern} _2010
 * @property {_10yTo12yPattern} _2011
 * @property {_10yTo12yPattern} _2012
 * @property {_10yTo12yPattern} _2013
 * @property {_10yTo12yPattern} _2014
 * @property {_10yTo12yPattern} _2015
 * @property {_10yTo12yPattern} _2016
 * @property {_10yTo12yPattern} _2017
 * @property {_10yTo12yPattern} _2018
 * @property {_10yTo12yPattern} _2019
 * @property {_10yTo12yPattern} _2020
 * @property {_10yTo12yPattern} _2021
 * @property {_10yTo12yPattern} _2022
 * @property {_10yTo12yPattern} _2023
 * @property {_10yTo12yPattern} _2024
 * @property {_10yTo12yPattern} _2025
 * @property {_10yTo12yPattern} _2026
 */

/**
 * @typedef {Object} CatalogTree_Indexed
 * @property {MetricNode<CatalogTree_Indexed_Address>} address
 * @property {MetricNode<CatalogTree_Indexed_Block>} block
 * @property {MetricNode<CatalogTree_Indexed_Output>} output
 * @property {MetricNode<CatalogTree_Indexed_Tx>} tx
 * @property {MetricNode<CatalogTree_Indexed_Txin>} txin
 * @property {MetricNode<CatalogTree_Indexed_Txout>} txout
 */

/**
 * @typedef {Object} CatalogTree_Indexed_Address
 * @property {Indexes2<P2AAddressIndex>} firstP2aaddressindex
 * @property {Indexes2<P2PK33AddressIndex>} firstP2pk33addressindex
 * @property {Indexes2<P2PK65AddressIndex>} firstP2pk65addressindex
 * @property {Indexes2<P2PKHAddressIndex>} firstP2pkhaddressindex
 * @property {Indexes2<P2SHAddressIndex>} firstP2shaddressindex
 * @property {Indexes2<P2TRAddressIndex>} firstP2traddressindex
 * @property {Indexes2<P2WPKHAddressIndex>} firstP2wpkhaddressindex
 * @property {Indexes2<P2WSHAddressIndex>} firstP2wshaddressindex
 * @property {Indexes12<P2ABytes>} p2abytes
 * @property {Indexes13<P2PK33Bytes>} p2pk33bytes
 * @property {Indexes14<P2PK65Bytes>} p2pk65bytes
 * @property {Indexes15<P2PKHBytes>} p2pkhbytes
 * @property {Indexes16<P2SHBytes>} p2shbytes
 * @property {Indexes17<P2TRBytes>} p2trbytes
 * @property {Indexes18<P2WPKHBytes>} p2wpkhbytes
 * @property {Indexes19<P2WSHBytes>} p2wshbytes
 */

/**
 * @typedef {Object} CatalogTree_Indexed_Block
 * @property {Indexes2<BlockHash>} blockhash
 * @property {Indexes2<StoredF64>} difficulty
 * @property {Indexes2<Timestamp>} timestamp
 * @property {Indexes2<StoredU64>} totalSize
 * @property {Indexes2<Weight>} weight
 */

/**
 * @typedef {Object} CatalogTree_Indexed_Output
 * @property {Indexes2<EmptyOutputIndex>} firstEmptyoutputindex
 * @property {Indexes2<OpReturnIndex>} firstOpreturnindex
 * @property {Indexes2<P2MSOutputIndex>} firstP2msoutputindex
 * @property {Indexes2<UnknownOutputIndex>} firstUnknownoutputindex
 * @property {MetricNode<TxIndex>} txindex
 */

/**
 * @typedef {Object} CatalogTree_Indexed_Tx
 * @property {Indexes6<StoredU32>} baseSize
 * @property {Indexes2<TxIndex>} firstTxindex
 * @property {Indexes6<TxInIndex>} firstTxinindex
 * @property {Indexes6<TxOutIndex>} firstTxoutindex
 * @property {Indexes6<Height>} height
 * @property {Indexes6<StoredBool>} isExplicitlyRbf
 * @property {Indexes6<RawLockTime>} rawlocktime
 * @property {Indexes6<StoredU32>} totalSize
 * @property {Indexes6<Txid>} txid
 * @property {Indexes6<TxVersion>} txversion
 */

/**
 * @typedef {Object} CatalogTree_Indexed_Txin
 * @property {Indexes2<TxInIndex>} firstTxinindex
 * @property {Indexes7<OutPoint>} outpoint
 * @property {Indexes7<TxIndex>} txindex
 */

/**
 * @typedef {Object} CatalogTree_Indexed_Txout
 * @property {Indexes2<TxOutIndex>} firstTxoutindex
 * @property {Indexes20<OutputType>} outputtype
 * @property {Indexes20<TxIndex>} txindex
 * @property {Indexes20<TypeIndex>} typeindex
 * @property {Indexes20<Sats>} value
 */

/**
 * Main BRK client with catalog tree and API methods
 * @extends BrkClientBase
 */
class BrkClient extends BrkClientBase {
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
        blks: {
          position: new MetricNode(this, '/position')
        },
        chain: {
          _1mBlockCount: createIndexes(this, '/1m_block_count'),
          _1wBlockCount: createIndexes(this, '/1w_block_count'),
          _1yBlockCount: createIndexes(this, '/1y_block_count'),
          _24hBlockCount: createIndexes2(this, '/24h_block_count'),
          _24hCoinbaseSum: createIndexes2(this, '/24h_coinbase_sum'),
          _24hCoinbaseUsdSum: createIndexes2(this, '/24h_coinbase_usd_sum'),
          annualizedVolume: createIndexes(this, '/annualized_volume'),
          annualizedVolumeBtc: createIndexes(this, '/annualized_volume_btc'),
          annualizedVolumeUsd: createIndexes(this, '/annualized_volume_usd'),
          blockCount: createBlockCountPattern(this, 'computed_chain/block_count'),
          blockCountTarget: createIndexes(this, '/block_count_target'),
          blockInterval: createBlockIntervalPattern(this, 'block_interval'),
          blockSize: createBlockSizePattern(this, 'computed_chain/block_size'),
          blockVbytes: createBlockSizePattern(this, 'computed_chain/block_vbytes'),
          blockWeight: createBlockSizePattern(this, 'computed_chain/block_weight'),
          blocksBeforeNextDifficultyAdjustment: createIndexes3(this, '/blocks_before_next_difficulty_adjustment'),
          blocksBeforeNextHalving: createIndexes3(this, '/blocks_before_next_halving'),
          coinbase: createCoinbasePattern(this, 'computed_chain/coinbase'),
          daysBeforeNextDifficultyAdjustment: createIndexes3(this, '/days_before_next_difficulty_adjustment'),
          daysBeforeNextHalving: createIndexes3(this, '/days_before_next_halving'),
          difficulty: createIndexes4(this, '/difficulty'),
          difficultyAdjustment: createIndexes3(this, '/difficulty_adjustment'),
          difficultyAsHash: createIndexes3(this, '/difficulty_as_hash'),
          difficultyepoch: createIndexes(this, '/difficultyepoch'),
          emptyoutputCount: createBitcoinPattern(this, 'computed_chain/emptyoutput_count'),
          exactUtxoCount: createIndexes3(this, '/exact_utxo_count'),
          fee: {
            base: createIndexes6(this, '/fee'),
            bitcoin: createBlockSizePattern(this, 'fee/bitcoin'),
            bitcoinTxindex: createIndexes6(this, '/fee_btc'),
            dollars: createBlockSizePattern(this, 'fee/dollars'),
            dollarsTxindex: createIndexes6(this, '/fee_usd'),
            sats: createBlockSizePattern(this, 'fee/sats')
          },
          feeDominance: createIndexes5(this, '/fee_dominance'),
          feeRate: {
            average: createIndexes3(this, '/fee_rate_avg'),
            base: createIndexes6(this, '/fee_rate'),
            max: createIndexes3(this, '/fee_rate_max'),
            median: createIndexes2(this, '/fee_rate_median'),
            min: createIndexes3(this, '/fee_rate_min'),
            pct10: createIndexes2(this, '/fee_rate_pct10'),
            pct25: createIndexes2(this, '/fee_rate_pct25'),
            pct75: createIndexes2(this, '/fee_rate_pct75'),
            pct90: createIndexes2(this, '/fee_rate_pct90')
          },
          halvingepoch: createIndexes(this, '/halvingepoch'),
          hashPricePhs: createIndexes3(this, '/hash_price_phs'),
          hashPricePhsMin: createIndexes3(this, '/hash_price_phs_min'),
          hashPriceRebound: createIndexes3(this, '/hash_price_rebound'),
          hashPriceThs: createIndexes3(this, '/hash_price_ths'),
          hashPriceThsMin: createIndexes3(this, '/hash_price_ths_min'),
          hashRate: createIndexes3(this, '/hash_rate'),
          hashRate1mSma: createIndexes(this, '/hash_rate_1m_sma'),
          hashRate1wSma: createIndexes(this, '/hash_rate_1w_sma'),
          hashRate1ySma: createIndexes(this, '/hash_rate_1y_sma'),
          hashRate2mSma: createIndexes(this, '/hash_rate_2m_sma'),
          hashValuePhs: createIndexes3(this, '/hash_value_phs'),
          hashValuePhsMin: createIndexes3(this, '/hash_value_phs_min'),
          hashValueRebound: createIndexes3(this, '/hash_value_rebound'),
          hashValueThs: createIndexes3(this, '/hash_value_ths'),
          hashValueThsMin: createIndexes3(this, '/hash_value_ths_min'),
          inflationRate: createIndexes(this, '/inflation_rate'),
          inputCount: createBlockSizePattern(this, 'computed_chain/input_count'),
          inputValue: createIndexes6(this, '/input_value'),
          inputsPerSec: createIndexes(this, '/inputs_per_sec'),
          interval: createIndexes2(this, '/interval'),
          isCoinbase: createIndexes6(this, '/is_coinbase'),
          opreturnCount: createBitcoinPattern(this, 'computed_chain/opreturn_count'),
          outputCount: createBlockSizePattern(this, 'computed_chain/output_count'),
          outputValue: createIndexes6(this, '/output_value'),
          outputsPerSec: createIndexes(this, '/outputs_per_sec'),
          p2aCount: createBitcoinPattern(this, 'computed_chain/p2a_count'),
          p2msCount: createBitcoinPattern(this, 'computed_chain/p2ms_count'),
          p2pk33Count: createBitcoinPattern(this, 'computed_chain/p2pk33_count'),
          p2pk65Count: createBitcoinPattern(this, 'computed_chain/p2pk65_count'),
          p2pkhCount: createBitcoinPattern(this, 'computed_chain/p2pkh_count'),
          p2shCount: createBitcoinPattern(this, 'computed_chain/p2sh_count'),
          p2trCount: createBitcoinPattern(this, 'computed_chain/p2tr_count'),
          p2wpkhCount: createBitcoinPattern(this, 'computed_chain/p2wpkh_count'),
          p2wshCount: createBitcoinPattern(this, 'computed_chain/p2wsh_count'),
          puellMultiple: createIndexes(this, '/puell_multiple'),
          sentSum: createSentSumPattern(this, 'computed_chain/sent_sum'),
          subsidy: createCoinbasePattern(this, 'computed_chain/subsidy'),
          subsidyDominance: createIndexes5(this, '/subsidy_dominance'),
          subsidyUsd1ySma: createIndexes(this, '/subsidy_usd_1y_sma'),
          timestamp: new MetricNode(this, '/timestamp'),
          txBtcVelocity: createIndexes(this, '/tx_btc_velocity'),
          txCount: createBitcoinPattern(this, 'computed_chain/tx_count'),
          txPerSec: createIndexes(this, '/tx_per_sec'),
          txUsdVelocity: createIndexes(this, '/tx_usd_velocity'),
          txV1: createBlockCountPattern(this, 'computed_chain/tx_v1'),
          txV2: createBlockCountPattern(this, 'computed_chain/tx_v2'),
          txV3: createBlockCountPattern(this, 'computed_chain/tx_v3'),
          txVsize: createBlockIntervalPattern(this, 'tx_vsize'),
          txWeight: createBlockIntervalPattern(this, 'tx_weight'),
          unclaimedRewards: createUnclaimedRewardsPattern(this, 'computed_chain/unclaimed_rewards'),
          unknownoutputCount: createBitcoinPattern(this, 'computed_chain/unknownoutput_count'),
          value: createIndexes7(this, '/value'),
          vbytes: createIndexes2(this, '/vbytes'),
          vsize: createIndexes6(this, '/vsize'),
          weight: createIndexes6(this, '/weight')
        },
        cointime: {
          activeCap: createIndexes3(this, '/active_cap'),
          activePrice: createIndexes3(this, '/active_price'),
          activePriceRatio: createActivePriceRatioPattern(this, 'computed_cointime/active_price_ratio'),
          activeSupply: createSentSumPattern(this, 'computed_cointime/active_supply'),
          activityToVaultednessRatio: createIndexes3(this, '/activity_to_vaultedness_ratio'),
          coinblocksCreated: createBlockCountPattern(this, 'computed_cointime/coinblocks_created'),
          coinblocksStored: createBlockCountPattern(this, 'computed_cointime/coinblocks_stored'),
          cointimeAdjInflationRate: createIndexes(this, '/cointime_adj_inflation_rate'),
          cointimeAdjTxBtcVelocity: createIndexes(this, '/cointime_adj_tx_btc_velocity'),
          cointimeAdjTxUsdVelocity: createIndexes(this, '/cointime_adj_tx_usd_velocity'),
          cointimeCap: createIndexes3(this, '/cointime_cap'),
          cointimePrice: createIndexes3(this, '/cointime_price'),
          cointimePriceRatio: createActivePriceRatioPattern(this, 'computed_cointime/cointime_price_ratio'),
          cointimeValueCreated: createBlockCountPattern(this, 'computed_cointime/cointime_value_created'),
          cointimeValueDestroyed: createBlockCountPattern(this, 'computed_cointime/cointime_value_destroyed'),
          cointimeValueStored: createBlockCountPattern(this, 'computed_cointime/cointime_value_stored'),
          investorCap: createIndexes3(this, '/investor_cap'),
          liveliness: createIndexes3(this, '/liveliness'),
          thermoCap: createIndexes3(this, '/thermo_cap'),
          trueMarketMean: createIndexes3(this, '/true_market_mean'),
          trueMarketMeanRatio: createActivePriceRatioPattern(this, 'computed_cointime/true_market_mean_ratio'),
          vaultedCap: createIndexes3(this, '/vaulted_cap'),
          vaultedPrice: createIndexes3(this, '/vaulted_price'),
          vaultedPriceRatio: createActivePriceRatioPattern(this, 'computed_cointime/vaulted_price_ratio'),
          vaultedSupply: createSentSumPattern(this, 'computed_cointime/vaulted_supply'),
          vaultedness: createIndexes3(this, '/vaultedness')
        },
        constants: {
          constant0: createIndexes3(this, '/constant_0'),
          constant1: createIndexes3(this, '/constant_1'),
          constant100: createIndexes3(this, '/constant_100'),
          constant2: createIndexes3(this, '/constant_2'),
          constant3: createIndexes3(this, '/constant_3'),
          constant382: createIndexes3(this, '/constant_38_2'),
          constant4: createIndexes3(this, '/constant_4'),
          constant50: createIndexes3(this, '/constant_50'),
          constant600: createIndexes3(this, '/constant_600'),
          constant618: createIndexes3(this, '/constant_61_8'),
          constantMinus1: createIndexes3(this, '/constant_minus_1'),
          constantMinus2: createIndexes3(this, '/constant_minus_2'),
          constantMinus3: createIndexes3(this, '/constant_minus_3'),
          constantMinus4: createIndexes3(this, '/constant_minus_4')
        },
        fetched: {
          priceOhlcInCents: createIndexes8(this, '/price_ohlc_in_cents')
        },
        indexes: {
          date: createIndexes8(this, '/date'),
          dateFixed: createIndexes2(this, '/date_fixed'),
          dateindex: createIndexes8(this, '/dateindex'),
          dateindexCount: createIndexes9(this, '/dateindex_count'),
          decadeindex: new MetricNode(this, '/decadeindex'),
          difficultyepoch: new MetricNode(this, '/difficultyepoch'),
          emptyoutputindex: new MetricNode(this, '/emptyoutputindex'),
          firstDateindex: createIndexes9(this, '/first_dateindex'),
          firstHeight: new MetricNode(this, '/first_height'),
          firstMonthindex: createIndexes10(this, '/first_monthindex'),
          firstYearindex: createIndexes11(this, '/first_yearindex'),
          halvingepoch: new MetricNode(this, '/halvingepoch'),
          height: createIndexes2(this, '/height'),
          heightCount: new MetricNode(this, '/height_count'),
          inputCount: createIndexes6(this, '/input_count'),
          monthindex: new MetricNode(this, '/monthindex'),
          monthindexCount: createIndexes10(this, '/monthindex_count'),
          opreturnindex: new MetricNode(this, '/opreturnindex'),
          outputCount: createIndexes6(this, '/output_count'),
          p2aaddressindex: createIndexes12(this, '/p2aaddressindex'),
          p2msoutputindex: new MetricNode(this, '/p2msoutputindex'),
          p2pk33addressindex: createIndexes13(this, '/p2pk33addressindex'),
          p2pk65addressindex: createIndexes14(this, '/p2pk65addressindex'),
          p2pkhaddressindex: createIndexes15(this, '/p2pkhaddressindex'),
          p2shaddressindex: createIndexes16(this, '/p2shaddressindex'),
          p2traddressindex: createIndexes17(this, '/p2traddressindex'),
          p2wpkhaddressindex: createIndexes18(this, '/p2wpkhaddressindex'),
          p2wshaddressindex: createIndexes19(this, '/p2wshaddressindex'),
          quarterindex: new MetricNode(this, '/quarterindex'),
          semesterindex: new MetricNode(this, '/semesterindex'),
          timestampFixed: createIndexes2(this, '/timestamp_fixed'),
          txindex: createIndexes6(this, '/txindex'),
          txindexCount: createIndexes2(this, '/txindex_count'),
          txinindex: createIndexes7(this, '/txinindex'),
          txoutindex: createIndexes20(this, '/txoutindex'),
          unknownoutputindex: new MetricNode(this, '/unknownoutputindex'),
          weekindex: new MetricNode(this, '/weekindex'),
          yearindex: new MetricNode(this, '/yearindex'),
          yearindexCount: createIndexes11(this, '/yearindex_count')
        },
        market: {
          _1dReturns1mSd: create_1dReturns1mSdPattern(this, '1d_returns_1m_sd'),
          _1dReturns1wSd: create_1dReturns1mSdPattern(this, '1d_returns_1w_sd'),
          _1dReturns1ySd: create_1dReturns1mSdPattern(this, '1d_returns_1y_sd'),
          _10yCagr: createIndexes(this, '/10y_cagr'),
          _10yDcaAvgPrice: createIndexes(this, '/10y_dca_avg_price'),
          _10yDcaCagr: createIndexes(this, '/10y_dca_cagr'),
          _10yDcaReturns: createIndexes(this, '/10y_dca_returns'),
          _10yDcaStack: createIndexes(this, '/10y_dca_stack'),
          _10yPriceReturns: createIndexes(this, '/10y_price_returns'),
          _1dPriceReturns: createIndexes(this, '/1d_price_returns'),
          _1mDcaAvgPrice: createIndexes(this, '/1m_dca_avg_price'),
          _1mDcaReturns: createIndexes(this, '/1m_dca_returns'),
          _1mDcaStack: createIndexes(this, '/1m_dca_stack'),
          _1mPriceReturns: createIndexes(this, '/1m_price_returns'),
          _1wDcaAvgPrice: createIndexes(this, '/1w_dca_avg_price'),
          _1wDcaReturns: createIndexes(this, '/1w_dca_returns'),
          _1wDcaStack: createIndexes(this, '/1w_dca_stack'),
          _1wPriceReturns: createIndexes(this, '/1w_price_returns'),
          _1yDcaAvgPrice: createIndexes(this, '/1y_dca_avg_price'),
          _1yDcaReturns: createIndexes(this, '/1y_dca_returns'),
          _1yDcaStack: createIndexes(this, '/1y_dca_stack'),
          _1yPriceReturns: createIndexes(this, '/1y_price_returns'),
          _2yCagr: createIndexes(this, '/2y_cagr'),
          _2yDcaAvgPrice: createIndexes(this, '/2y_dca_avg_price'),
          _2yDcaCagr: createIndexes(this, '/2y_dca_cagr'),
          _2yDcaReturns: createIndexes(this, '/2y_dca_returns'),
          _2yDcaStack: createIndexes(this, '/2y_dca_stack'),
          _2yPriceReturns: createIndexes(this, '/2y_price_returns'),
          _3mDcaAvgPrice: createIndexes(this, '/3m_dca_avg_price'),
          _3mDcaReturns: createIndexes(this, '/3m_dca_returns'),
          _3mDcaStack: createIndexes(this, '/3m_dca_stack'),
          _3mPriceReturns: createIndexes(this, '/3m_price_returns'),
          _3yCagr: createIndexes(this, '/3y_cagr'),
          _3yDcaAvgPrice: createIndexes(this, '/3y_dca_avg_price'),
          _3yDcaCagr: createIndexes(this, '/3y_dca_cagr'),
          _3yDcaReturns: createIndexes(this, '/3y_dca_returns'),
          _3yDcaStack: createIndexes(this, '/3y_dca_stack'),
          _3yPriceReturns: createIndexes(this, '/3y_price_returns'),
          _4yCagr: createIndexes(this, '/4y_cagr'),
          _4yDcaAvgPrice: createIndexes(this, '/4y_dca_avg_price'),
          _4yDcaCagr: createIndexes(this, '/4y_dca_cagr'),
          _4yDcaReturns: createIndexes(this, '/4y_dca_returns'),
          _4yDcaStack: createIndexes(this, '/4y_dca_stack'),
          _4yPriceReturns: createIndexes(this, '/4y_price_returns'),
          _5yCagr: createIndexes(this, '/5y_cagr'),
          _5yDcaAvgPrice: createIndexes(this, '/5y_dca_avg_price'),
          _5yDcaCagr: createIndexes(this, '/5y_dca_cagr'),
          _5yDcaReturns: createIndexes(this, '/5y_dca_returns'),
          _5yDcaStack: createIndexes(this, '/5y_dca_stack'),
          _5yPriceReturns: createIndexes(this, '/5y_price_returns'),
          _6mDcaAvgPrice: createIndexes(this, '/6m_dca_avg_price'),
          _6mDcaReturns: createIndexes(this, '/6m_dca_returns'),
          _6mDcaStack: createIndexes(this, '/6m_dca_stack'),
          _6mPriceReturns: createIndexes(this, '/6m_price_returns'),
          _6yCagr: createIndexes(this, '/6y_cagr'),
          _6yDcaAvgPrice: createIndexes(this, '/6y_dca_avg_price'),
          _6yDcaCagr: createIndexes(this, '/6y_dca_cagr'),
          _6yDcaReturns: createIndexes(this, '/6y_dca_returns'),
          _6yDcaStack: createIndexes(this, '/6y_dca_stack'),
          _6yPriceReturns: createIndexes(this, '/6y_price_returns'),
          _8yCagr: createIndexes(this, '/8y_cagr'),
          _8yDcaAvgPrice: createIndexes(this, '/8y_dca_avg_price'),
          _8yDcaCagr: createIndexes(this, '/8y_dca_cagr'),
          _8yDcaReturns: createIndexes(this, '/8y_dca_returns'),
          _8yDcaStack: createIndexes(this, '/8y_dca_stack'),
          _8yPriceReturns: createIndexes(this, '/8y_price_returns'),
          daysSincePriceAth: createIndexes(this, '/days_since_price_ath'),
          dcaClass2015AvgPrice: createIndexes(this, '/dca_class_2015_avg_price'),
          dcaClass2015Returns: createIndexes(this, '/dca_class_2015_returns'),
          dcaClass2015Stack: createIndexes(this, '/dca_class_2015_stack'),
          dcaClass2016AvgPrice: createIndexes(this, '/dca_class_2016_avg_price'),
          dcaClass2016Returns: createIndexes(this, '/dca_class_2016_returns'),
          dcaClass2016Stack: createIndexes(this, '/dca_class_2016_stack'),
          dcaClass2017AvgPrice: createIndexes(this, '/dca_class_2017_avg_price'),
          dcaClass2017Returns: createIndexes(this, '/dca_class_2017_returns'),
          dcaClass2017Stack: createIndexes(this, '/dca_class_2017_stack'),
          dcaClass2018AvgPrice: createIndexes(this, '/dca_class_2018_avg_price'),
          dcaClass2018Returns: createIndexes(this, '/dca_class_2018_returns'),
          dcaClass2018Stack: createIndexes(this, '/dca_class_2018_stack'),
          dcaClass2019AvgPrice: createIndexes(this, '/dca_class_2019_avg_price'),
          dcaClass2019Returns: createIndexes(this, '/dca_class_2019_returns'),
          dcaClass2019Stack: createIndexes(this, '/dca_class_2019_stack'),
          dcaClass2020AvgPrice: createIndexes(this, '/dca_class_2020_avg_price'),
          dcaClass2020Returns: createIndexes(this, '/dca_class_2020_returns'),
          dcaClass2020Stack: createIndexes(this, '/dca_class_2020_stack'),
          dcaClass2021AvgPrice: createIndexes(this, '/dca_class_2021_avg_price'),
          dcaClass2021Returns: createIndexes(this, '/dca_class_2021_returns'),
          dcaClass2021Stack: createIndexes(this, '/dca_class_2021_stack'),
          dcaClass2022AvgPrice: createIndexes(this, '/dca_class_2022_avg_price'),
          dcaClass2022Returns: createIndexes(this, '/dca_class_2022_returns'),
          dcaClass2022Stack: createIndexes(this, '/dca_class_2022_stack'),
          dcaClass2023AvgPrice: createIndexes(this, '/dca_class_2023_avg_price'),
          dcaClass2023Returns: createIndexes(this, '/dca_class_2023_returns'),
          dcaClass2023Stack: createIndexes(this, '/dca_class_2023_stack'),
          dcaClass2024AvgPrice: createIndexes(this, '/dca_class_2024_avg_price'),
          dcaClass2024Returns: createIndexes(this, '/dca_class_2024_returns'),
          dcaClass2024Stack: createIndexes(this, '/dca_class_2024_stack'),
          dcaClass2025AvgPrice: createIndexes(this, '/dca_class_2025_avg_price'),
          dcaClass2025Returns: createIndexes(this, '/dca_class_2025_returns'),
          dcaClass2025Stack: createIndexes(this, '/dca_class_2025_stack'),
          maxDaysBetweenPriceAths: createIndexes(this, '/max_days_between_price_aths'),
          maxYearsBetweenPriceAths: createIndexes(this, '/max_years_between_price_aths'),
          price10yAgo: createIndexes(this, '/price_10y_ago'),
          price13dEma: createPrice13dEmaPattern(this, 'price_13d_ema'),
          price13dSma: createPrice13dEmaPattern(this, 'price_13d_sma'),
          price144dEma: createPrice13dEmaPattern(this, 'price_144d_ema'),
          price144dSma: createPrice13dEmaPattern(this, 'price_144d_sma'),
          price1dAgo: createIndexes(this, '/price_1d_ago'),
          price1mAgo: createIndexes(this, '/price_1m_ago'),
          price1mEma: createPrice13dEmaPattern(this, 'price_1m_ema'),
          price1mMax: createIndexes(this, '/price_1m_max'),
          price1mMin: createIndexes(this, '/price_1m_min'),
          price1mSma: createPrice13dEmaPattern(this, 'price_1m_sma'),
          price1mVolatility: createIndexes(this, '/price_1m_volatility'),
          price1wAgo: createIndexes(this, '/price_1w_ago'),
          price1wEma: createPrice13dEmaPattern(this, 'price_1w_ema'),
          price1wMax: createIndexes(this, '/price_1w_max'),
          price1wMin: createIndexes(this, '/price_1w_min'),
          price1wSma: createPrice13dEmaPattern(this, 'price_1w_sma'),
          price1wVolatility: createIndexes(this, '/price_1w_volatility'),
          price1yAgo: createIndexes(this, '/price_1y_ago'),
          price1yEma: createPrice13dEmaPattern(this, 'price_1y_ema'),
          price1yMax: createIndexes(this, '/price_1y_max'),
          price1yMin: createIndexes(this, '/price_1y_min'),
          price1ySma: createPrice13dEmaPattern(this, 'price_1y_sma'),
          price1yVolatility: createIndexes(this, '/price_1y_volatility'),
          price200dEma: createPrice13dEmaPattern(this, 'price_200d_ema'),
          price200dSma: createPrice13dEmaPattern(this, 'price_200d_sma'),
          price200dSmaX08: createIndexes(this, '/price_200d_sma_x0_8'),
          price200dSmaX24: createIndexes(this, '/price_200d_sma_x2_4'),
          price200wEma: createPrice13dEmaPattern(this, 'price_200w_ema'),
          price200wSma: createPrice13dEmaPattern(this, 'price_200w_sma'),
          price21dEma: createPrice13dEmaPattern(this, 'price_21d_ema'),
          price21dSma: createPrice13dEmaPattern(this, 'price_21d_sma'),
          price2wChoppinessIndex: createIndexes(this, '/price_2w_choppiness_index'),
          price2wMax: createIndexes(this, '/price_2w_max'),
          price2wMin: createIndexes(this, '/price_2w_min'),
          price2yAgo: createIndexes(this, '/price_2y_ago'),
          price2yEma: createPrice13dEmaPattern(this, 'price_2y_ema'),
          price2ySma: createPrice13dEmaPattern(this, 'price_2y_sma'),
          price34dEma: createPrice13dEmaPattern(this, 'price_34d_ema'),
          price34dSma: createPrice13dEmaPattern(this, 'price_34d_sma'),
          price3mAgo: createIndexes(this, '/price_3m_ago'),
          price3yAgo: createIndexes(this, '/price_3y_ago'),
          price4yAgo: createIndexes(this, '/price_4y_ago'),
          price4yEma: createPrice13dEmaPattern(this, 'price_4y_ema'),
          price4ySma: createPrice13dEmaPattern(this, 'price_4y_sma'),
          price55dEma: createPrice13dEmaPattern(this, 'price_55d_ema'),
          price55dSma: createPrice13dEmaPattern(this, 'price_55d_sma'),
          price5yAgo: createIndexes(this, '/price_5y_ago'),
          price6mAgo: createIndexes(this, '/price_6m_ago'),
          price6yAgo: createIndexes(this, '/price_6y_ago'),
          price89dEma: createPrice13dEmaPattern(this, 'price_89d_ema'),
          price89dSma: createPrice13dEmaPattern(this, 'price_89d_sma'),
          price8dEma: createPrice13dEmaPattern(this, 'price_8d_ema'),
          price8dSma: createPrice13dEmaPattern(this, 'price_8d_sma'),
          price8yAgo: createIndexes(this, '/price_8y_ago'),
          priceAth: createIndexes21(this, '/price_ath'),
          priceDrawdown: createIndexes21(this, '/price_drawdown'),
          priceTrueRange: createIndexes5(this, '/price_true_range'),
          priceTrueRange2wSum: createIndexes5(this, '/price_true_range_2w_sum')
        },
        pools: {
          pool: createIndexes2(this, '/pool'),
          vecs: {
            aXbt: createAXbtPattern(this, 'computed_pools_vecs/AXbt'),
            aaoPool: createAXbtPattern(this, 'computed_pools_vecs/AaoPool'),
            antPool: createAXbtPattern(this, 'computed_pools_vecs/AntPool'),
            arkPool: createAXbtPattern(this, 'computed_pools_vecs/ArkPool'),
            asicMiner: createAXbtPattern(this, 'computed_pools_vecs/AsicMiner'),
            batPool: createAXbtPattern(this, 'computed_pools_vecs/BatPool'),
            bcMonster: createAXbtPattern(this, 'computed_pools_vecs/BcMonster'),
            bcpoolIo: createAXbtPattern(this, 'computed_pools_vecs/BcpoolIo'),
            binancePool: createAXbtPattern(this, 'computed_pools_vecs/BinancePool'),
            bitClub: createAXbtPattern(this, 'computed_pools_vecs/BitClub'),
            bitFuFuPool: createAXbtPattern(this, 'computed_pools_vecs/BitFuFuPool'),
            bitFury: createAXbtPattern(this, 'computed_pools_vecs/BitFury'),
            bitMinter: createAXbtPattern(this, 'computed_pools_vecs/BitMinter'),
            bitalo: createAXbtPattern(this, 'computed_pools_vecs/Bitalo'),
            bitcoinAffiliateNetwork: createAXbtPattern(this, 'computed_pools_vecs/BitcoinAffiliateNetwork'),
            bitcoinCom: createAXbtPattern(this, 'computed_pools_vecs/BitcoinCom'),
            bitcoinIndia: createAXbtPattern(this, 'computed_pools_vecs/BitcoinIndia'),
            bitcoinRussia: createAXbtPattern(this, 'computed_pools_vecs/BitcoinRussia'),
            bitcoinUkraine: createAXbtPattern(this, 'computed_pools_vecs/BitcoinUkraine'),
            bitfarms: createAXbtPattern(this, 'computed_pools_vecs/Bitfarms'),
            bitparking: createAXbtPattern(this, 'computed_pools_vecs/Bitparking'),
            bitsolo: createAXbtPattern(this, 'computed_pools_vecs/Bitsolo'),
            bixin: createAXbtPattern(this, 'computed_pools_vecs/Bixin'),
            blockFills: createAXbtPattern(this, 'computed_pools_vecs/BlockFills'),
            braiinsPool: createAXbtPattern(this, 'computed_pools_vecs/BraiinsPool'),
            bravoMining: createAXbtPattern(this, 'computed_pools_vecs/BravoMining'),
            btPool: createAXbtPattern(this, 'computed_pools_vecs/BtPool'),
            btcCom: createAXbtPattern(this, 'computed_pools_vecs/BtcCom'),
            btcDig: createAXbtPattern(this, 'computed_pools_vecs/BtcDig'),
            btcGuild: createAXbtPattern(this, 'computed_pools_vecs/BtcGuild'),
            btcLab: createAXbtPattern(this, 'computed_pools_vecs/BtcLab'),
            btcMp: createAXbtPattern(this, 'computed_pools_vecs/BtcMp'),
            btcNuggets: createAXbtPattern(this, 'computed_pools_vecs/BtcNuggets'),
            btcPoolParty: createAXbtPattern(this, 'computed_pools_vecs/BtcPoolParty'),
            btcServ: createAXbtPattern(this, 'computed_pools_vecs/BtcServ'),
            btcTop: createAXbtPattern(this, 'computed_pools_vecs/BtcTop'),
            btcc: createAXbtPattern(this, 'computed_pools_vecs/Btcc'),
            bwPool: createAXbtPattern(this, 'computed_pools_vecs/BwPool'),
            bytePool: createAXbtPattern(this, 'computed_pools_vecs/BytePool'),
            canoe: createAXbtPattern(this, 'computed_pools_vecs/Canoe'),
            canoePool: createAXbtPattern(this, 'computed_pools_vecs/CanoePool'),
            carbonNegative: createAXbtPattern(this, 'computed_pools_vecs/CarbonNegative'),
            ckPool: createAXbtPattern(this, 'computed_pools_vecs/CkPool'),
            cloudHashing: createAXbtPattern(this, 'computed_pools_vecs/CloudHashing'),
            coinLab: createAXbtPattern(this, 'computed_pools_vecs/CoinLab'),
            cointerra: createAXbtPattern(this, 'computed_pools_vecs/Cointerra'),
            connectBtc: createAXbtPattern(this, 'computed_pools_vecs/ConnectBtc'),
            dPool: createAXbtPattern(this, 'computed_pools_vecs/DPool'),
            dcExploration: createAXbtPattern(this, 'computed_pools_vecs/DcExploration'),
            dcex: createAXbtPattern(this, 'computed_pools_vecs/Dcex'),
            digitalBtc: createAXbtPattern(this, 'computed_pools_vecs/DigitalBtc'),
            digitalXMintsy: createAXbtPattern(this, 'computed_pools_vecs/DigitalXMintsy'),
            eclipseMc: createAXbtPattern(this, 'computed_pools_vecs/EclipseMc'),
            eightBaochi: createAXbtPattern(this, 'computed_pools_vecs/EightBaochi'),
            ekanemBtc: createAXbtPattern(this, 'computed_pools_vecs/EkanemBtc'),
            eligius: createAXbtPattern(this, 'computed_pools_vecs/Eligius'),
            emcdPool: createAXbtPattern(this, 'computed_pools_vecs/EmcdPool'),
            entrustCharityPool: createAXbtPattern(this, 'computed_pools_vecs/EntrustCharityPool'),
            eobot: createAXbtPattern(this, 'computed_pools_vecs/Eobot'),
            exxBw: createAXbtPattern(this, 'computed_pools_vecs/ExxBw'),
            f2Pool: createAXbtPattern(this, 'computed_pools_vecs/F2Pool'),
            fiftyEightCoin: createAXbtPattern(this, 'computed_pools_vecs/FiftyEightCoin'),
            foundryUsa: createAXbtPattern(this, 'computed_pools_vecs/FoundryUsa'),
            futureBitApolloSolo: createAXbtPattern(this, 'computed_pools_vecs/FutureBitApolloSolo'),
            gbMiners: createAXbtPattern(this, 'computed_pools_vecs/GbMiners'),
            ghashIo: createAXbtPattern(this, 'computed_pools_vecs/GhashIo'),
            giveMeCoins: createAXbtPattern(this, 'computed_pools_vecs/GiveMeCoins'),
            goGreenLight: createAXbtPattern(this, 'computed_pools_vecs/GoGreenLight'),
            haoZhuZhu: createAXbtPattern(this, 'computed_pools_vecs/HaoZhuZhu'),
            haominer: createAXbtPattern(this, 'computed_pools_vecs/Haominer'),
            hashBx: createAXbtPattern(this, 'computed_pools_vecs/HashBx'),
            hashPool: createAXbtPattern(this, 'computed_pools_vecs/HashPool'),
            helix: createAXbtPattern(this, 'computed_pools_vecs/Helix'),
            hhtt: createAXbtPattern(this, 'computed_pools_vecs/Hhtt'),
            hotPool: createAXbtPattern(this, 'computed_pools_vecs/HotPool'),
            hummerpool: createAXbtPattern(this, 'computed_pools_vecs/Hummerpool'),
            huobiPool: createAXbtPattern(this, 'computed_pools_vecs/HuobiPool'),
            innopolisTech: createAXbtPattern(this, 'computed_pools_vecs/InnopolisTech'),
            kanoPool: createAXbtPattern(this, 'computed_pools_vecs/KanoPool'),
            kncMiner: createAXbtPattern(this, 'computed_pools_vecs/KncMiner'),
            kuCoinPool: createAXbtPattern(this, 'computed_pools_vecs/KuCoinPool'),
            lubianCom: createAXbtPattern(this, 'computed_pools_vecs/LubianCom'),
            luckyPool: createAXbtPattern(this, 'computed_pools_vecs/LuckyPool'),
            luxor: createAXbtPattern(this, 'computed_pools_vecs/Luxor'),
            maraPool: createAXbtPattern(this, 'computed_pools_vecs/MaraPool'),
            maxBtc: createAXbtPattern(this, 'computed_pools_vecs/MaxBtc'),
            maxiPool: createAXbtPattern(this, 'computed_pools_vecs/MaxiPool'),
            megaBigPower: createAXbtPattern(this, 'computed_pools_vecs/MegaBigPower'),
            minerium: createAXbtPattern(this, 'computed_pools_vecs/Minerium'),
            miningCity: createAXbtPattern(this, 'computed_pools_vecs/MiningCity'),
            miningDutch: createAXbtPattern(this, 'computed_pools_vecs/MiningDutch'),
            miningKings: createAXbtPattern(this, 'computed_pools_vecs/MiningKings'),
            miningSquared: createAXbtPattern(this, 'computed_pools_vecs/MiningSquared'),
            mmpool: createAXbtPattern(this, 'computed_pools_vecs/Mmpool'),
            mtRed: createAXbtPattern(this, 'computed_pools_vecs/MtRed'),
            multiCoinCo: createAXbtPattern(this, 'computed_pools_vecs/MultiCoinCo'),
            multipool: createAXbtPattern(this, 'computed_pools_vecs/Multipool'),
            myBtcCoinPool: createAXbtPattern(this, 'computed_pools_vecs/MyBtcCoinPool'),
            neopool: createAXbtPattern(this, 'computed_pools_vecs/Neopool'),
            nexious: createAXbtPattern(this, 'computed_pools_vecs/Nexious'),
            niceHash: createAXbtPattern(this, 'computed_pools_vecs/NiceHash'),
            nmcBit: createAXbtPattern(this, 'computed_pools_vecs/NmcBit'),
            novaBlock: createAXbtPattern(this, 'computed_pools_vecs/NovaBlock'),
            ocean: createAXbtPattern(this, 'computed_pools_vecs/Ocean'),
            okExPool: createAXbtPattern(this, 'computed_pools_vecs/OkExPool'),
            okMiner: createAXbtPattern(this, 'computed_pools_vecs/OkMiner'),
            okkong: createAXbtPattern(this, 'computed_pools_vecs/Okkong'),
            okpoolTop: createAXbtPattern(this, 'computed_pools_vecs/OkpoolTop'),
            oneHash: createAXbtPattern(this, 'computed_pools_vecs/OneHash'),
            oneM1x: createAXbtPattern(this, 'computed_pools_vecs/OneM1x'),
            oneThash: createAXbtPattern(this, 'computed_pools_vecs/OneThash'),
            ozCoin: createAXbtPattern(this, 'computed_pools_vecs/OzCoin'),
            pHashIo: createAXbtPattern(this, 'computed_pools_vecs/PHashIo'),
            parasite: createAXbtPattern(this, 'computed_pools_vecs/Parasite'),
            patels: createAXbtPattern(this, 'computed_pools_vecs/Patels'),
            pegaPool: createAXbtPattern(this, 'computed_pools_vecs/PegaPool'),
            phoenix: createAXbtPattern(this, 'computed_pools_vecs/Phoenix'),
            polmine: createAXbtPattern(this, 'computed_pools_vecs/Polmine'),
            pool175btc: createAXbtPattern(this, 'computed_pools_vecs/Pool175btc'),
            pool50btc: createAXbtPattern(this, 'computed_pools_vecs/Pool50btc'),
            poolin: createAXbtPattern(this, 'computed_pools_vecs/Poolin'),
            portlandHodl: createAXbtPattern(this, 'computed_pools_vecs/PortlandHodl'),
            publicPool: createAXbtPattern(this, 'computed_pools_vecs/PublicPool'),
            pureBtcCom: createAXbtPattern(this, 'computed_pools_vecs/PureBtcCom'),
            rawpool: createAXbtPattern(this, 'computed_pools_vecs/Rawpool'),
            rigPool: createAXbtPattern(this, 'computed_pools_vecs/RigPool'),
            sbiCrypto: createAXbtPattern(this, 'computed_pools_vecs/SbiCrypto'),
            secPool: createAXbtPattern(this, 'computed_pools_vecs/SecPool'),
            secretSuperstar: createAXbtPattern(this, 'computed_pools_vecs/SecretSuperstar'),
            sevenPool: createAXbtPattern(this, 'computed_pools_vecs/SevenPool'),
            shawnP0wers: createAXbtPattern(this, 'computed_pools_vecs/ShawnP0wers'),
            sigmapoolCom: createAXbtPattern(this, 'computed_pools_vecs/SigmapoolCom'),
            simplecoinUs: createAXbtPattern(this, 'computed_pools_vecs/SimplecoinUs'),
            soloCk: createAXbtPattern(this, 'computed_pools_vecs/SoloCk'),
            spiderPool: createAXbtPattern(this, 'computed_pools_vecs/SpiderPool'),
            stMiningCorp: createAXbtPattern(this, 'computed_pools_vecs/StMiningCorp'),
            tangpool: createAXbtPattern(this, 'computed_pools_vecs/Tangpool'),
            tatmasPool: createAXbtPattern(this, 'computed_pools_vecs/TatmasPool'),
            tbDice: createAXbtPattern(this, 'computed_pools_vecs/TbDice'),
            telco214: createAXbtPattern(this, 'computed_pools_vecs/Telco214'),
            terraPool: createAXbtPattern(this, 'computed_pools_vecs/TerraPool'),
            tiger: createAXbtPattern(this, 'computed_pools_vecs/Tiger'),
            tigerpoolNet: createAXbtPattern(this, 'computed_pools_vecs/TigerpoolNet'),
            titan: createAXbtPattern(this, 'computed_pools_vecs/Titan'),
            transactionCoinMining: createAXbtPattern(this, 'computed_pools_vecs/TransactionCoinMining'),
            trickysBtcPool: createAXbtPattern(this, 'computed_pools_vecs/TrickysBtcPool'),
            tripleMining: createAXbtPattern(this, 'computed_pools_vecs/TripleMining'),
            twentyOneInc: createAXbtPattern(this, 'computed_pools_vecs/TwentyOneInc'),
            ultimusPool: createAXbtPattern(this, 'computed_pools_vecs/UltimusPool'),
            unknown: createAXbtPattern(this, 'computed_pools_vecs/Unknown'),
            unomp: createAXbtPattern(this, 'computed_pools_vecs/Unomp'),
            viaBtc: createAXbtPattern(this, 'computed_pools_vecs/ViaBtc'),
            waterhole: createAXbtPattern(this, 'computed_pools_vecs/Waterhole'),
            wayiCn: createAXbtPattern(this, 'computed_pools_vecs/WayiCn'),
            whitePool: createAXbtPattern(this, 'computed_pools_vecs/WhitePool'),
            wk057: createAXbtPattern(this, 'computed_pools_vecs/Wk057'),
            yourbtcNet: createAXbtPattern(this, 'computed_pools_vecs/YourbtcNet'),
            zulupool: createAXbtPattern(this, 'computed_pools_vecs/Zulupool')
          }
        },
        price: {
          priceClose: createIndexes3(this, '/price_close'),
          priceCloseInCents: createIndexes8(this, '/price_close_in_cents'),
          priceCloseInSats: createIndexes3(this, '/price_close_in_sats'),
          priceHigh: createIndexes3(this, '/price_high'),
          priceHighInCents: createIndexes8(this, '/price_high_in_cents'),
          priceHighInSats: createIndexes3(this, '/price_high_in_sats'),
          priceLow: createIndexes3(this, '/price_low'),
          priceLowInCents: createIndexes8(this, '/price_low_in_cents'),
          priceLowInSats: createIndexes3(this, '/price_low_in_sats'),
          priceOhlc: createIndexes3(this, '/price_ohlc'),
          priceOhlcInSats: createIndexes3(this, '/price_ohlc_in_sats'),
          priceOpen: createIndexes3(this, '/price_open'),
          priceOpenInCents: createIndexes8(this, '/price_open_in_cents'),
          priceOpenInSats: createIndexes3(this, '/price_open_in_sats')
        },
        stateful: {
          addrCount: createIndexes3(this, '/addr_count'),
          addressCohorts: {
            amountRange: {
              _0sats: create_0satsPattern(this, 'computed_stateful_address_cohorts_amount_range/_0sats'),
              _100btcTo1kBtc: create_0satsPattern(this, 'computed_stateful_address_cohorts_amount_range/_100btc_to_1k_btc'),
              _100kBtcOrMore: create_0satsPattern(this, 'computed_stateful_address_cohorts_amount_range/_100k_btc_or_more'),
              _100kSatsTo1mSats: create_0satsPattern(this, 'computed_stateful_address_cohorts_amount_range/_100k_sats_to_1m_sats'),
              _100satsTo1kSats: create_0satsPattern(this, 'computed_stateful_address_cohorts_amount_range/_100sats_to_1k_sats'),
              _10btcTo100btc: create_0satsPattern(this, 'computed_stateful_address_cohorts_amount_range/_10btc_to_100btc'),
              _10kBtcTo100kBtc: create_0satsPattern(this, 'computed_stateful_address_cohorts_amount_range/_10k_btc_to_100k_btc'),
              _10kSatsTo100kSats: create_0satsPattern(this, 'computed_stateful_address_cohorts_amount_range/_10k_sats_to_100k_sats'),
              _10mSatsTo1btc: create_0satsPattern(this, 'computed_stateful_address_cohorts_amount_range/_10m_sats_to_1btc'),
              _10satsTo100sats: create_0satsPattern(this, 'computed_stateful_address_cohorts_amount_range/_10sats_to_100sats'),
              _1btcTo10btc: create_0satsPattern(this, 'computed_stateful_address_cohorts_amount_range/_1btc_to_10btc'),
              _1kBtcTo10kBtc: create_0satsPattern(this, 'computed_stateful_address_cohorts_amount_range/_1k_btc_to_10k_btc'),
              _1kSatsTo10kSats: create_0satsPattern(this, 'computed_stateful_address_cohorts_amount_range/_1k_sats_to_10k_sats'),
              _1mSatsTo10mSats: create_0satsPattern(this, 'computed_stateful_address_cohorts_amount_range/_1m_sats_to_10m_sats'),
              _1satTo10sats: create_0satsPattern(this, 'computed_stateful_address_cohorts_amount_range/_1sat_to_10sats')
            },
            geAmount: {
              _100btc: create_0satsPattern(this, 'computed_stateful_address_cohorts_ge_amount/_100btc'),
              _100kSats: create_0satsPattern(this, 'computed_stateful_address_cohorts_ge_amount/_100k_sats'),
              _100sats: create_0satsPattern(this, 'computed_stateful_address_cohorts_ge_amount/_100sats'),
              _10btc: create_0satsPattern(this, 'computed_stateful_address_cohorts_ge_amount/_10btc'),
              _10kBtc: create_0satsPattern(this, 'computed_stateful_address_cohorts_ge_amount/_10k_btc'),
              _10kSats: create_0satsPattern(this, 'computed_stateful_address_cohorts_ge_amount/_10k_sats'),
              _10mSats: create_0satsPattern(this, 'computed_stateful_address_cohorts_ge_amount/_10m_sats'),
              _10sats: create_0satsPattern(this, 'computed_stateful_address_cohorts_ge_amount/_10sats'),
              _1btc: create_0satsPattern(this, 'computed_stateful_address_cohorts_ge_amount/_1btc'),
              _1kBtc: create_0satsPattern(this, 'computed_stateful_address_cohorts_ge_amount/_1k_btc'),
              _1kSats: create_0satsPattern(this, 'computed_stateful_address_cohorts_ge_amount/_1k_sats'),
              _1mSats: create_0satsPattern(this, 'computed_stateful_address_cohorts_ge_amount/_1m_sats'),
              _1sat: create_0satsPattern(this, 'computed_stateful_address_cohorts_ge_amount/_1sat')
            },
            ltAmount: {
              _100btc: create_0satsPattern(this, 'computed_stateful_address_cohorts_lt_amount/_100btc'),
              _100kBtc: create_0satsPattern(this, 'computed_stateful_address_cohorts_lt_amount/_100k_btc'),
              _100kSats: create_0satsPattern(this, 'computed_stateful_address_cohorts_lt_amount/_100k_sats'),
              _100sats: create_0satsPattern(this, 'computed_stateful_address_cohorts_lt_amount/_100sats'),
              _10btc: create_0satsPattern(this, 'computed_stateful_address_cohorts_lt_amount/_10btc'),
              _10kBtc: create_0satsPattern(this, 'computed_stateful_address_cohorts_lt_amount/_10k_btc'),
              _10kSats: create_0satsPattern(this, 'computed_stateful_address_cohorts_lt_amount/_10k_sats'),
              _10mSats: create_0satsPattern(this, 'computed_stateful_address_cohorts_lt_amount/_10m_sats'),
              _10sats: create_0satsPattern(this, 'computed_stateful_address_cohorts_lt_amount/_10sats'),
              _1btc: create_0satsPattern(this, 'computed_stateful_address_cohorts_lt_amount/_1btc'),
              _1kBtc: create_0satsPattern(this, 'computed_stateful_address_cohorts_lt_amount/_1k_btc'),
              _1kSats: create_0satsPattern(this, 'computed_stateful_address_cohorts_lt_amount/_1k_sats'),
              _1mSats: create_0satsPattern(this, 'computed_stateful_address_cohorts_lt_amount/_1m_sats')
            }
          },
          addressesData: {
            empty: createIndexes23(this, '/emptyaddressdata'),
            loaded: createIndexes24(this, '/loadedaddressdata')
          },
          addresstypeToHeightToAddrCount: createAddresstypeToHeightToAddrCountPattern(this, 'computed_stateful/addresstype_to_height_to_addr_count'),
          addresstypeToHeightToEmptyAddrCount: createAddresstypeToHeightToAddrCountPattern(this, 'computed_stateful/addresstype_to_height_to_empty_addr_count'),
          addresstypeToIndexesToAddrCount: createAddresstypeToHeightToAddrCountPattern(this, 'computed_stateful/addresstype_to_indexes_to_addr_count'),
          addresstypeToIndexesToEmptyAddrCount: createAddresstypeToHeightToAddrCountPattern(this, 'computed_stateful/addresstype_to_indexes_to_empty_addr_count'),
          anyAddressIndexes: createAddresstypeToHeightToAddrCountPattern(this, 'computed_stateful/any_address_indexes'),
          chainState: createIndexes2(this, '/chain'),
          emptyAddrCount: createIndexes3(this, '/empty_addr_count'),
          emptyaddressindex: createIndexes23(this, '/emptyaddressindex'),
          loadedaddressindex: createIndexes24(this, '/loadedaddressindex'),
          marketCap: createIndexes21(this, '/market_cap'),
          opreturnSupply: createSupplyPattern(this, 'computed_stateful/opreturn_supply'),
          txinindex: createIndexes20(this, '/txinindex'),
          unspendableSupply: createSupplyPattern(this, 'computed_stateful/unspendable_supply'),
          utxoCohorts: {
            ageRange: {
              _10yTo12y: create_10yTo12yPattern(this, 'computed_stateful_utxo_cohorts_age_range/_10y_to_12y'),
              _12yTo15y: create_10yTo12yPattern(this, 'computed_stateful_utxo_cohorts_age_range/_12y_to_15y'),
              _1dTo1w: create_10yTo12yPattern(this, 'computed_stateful_utxo_cohorts_age_range/_1d_to_1w'),
              _1mTo2m: create_10yTo12yPattern(this, 'computed_stateful_utxo_cohorts_age_range/_1m_to_2m'),
              _1wTo1m: create_10yTo12yPattern(this, 'computed_stateful_utxo_cohorts_age_range/_1w_to_1m'),
              _1yTo2y: create_10yTo12yPattern(this, 'computed_stateful_utxo_cohorts_age_range/_1y_to_2y'),
              _2mTo3m: create_10yTo12yPattern(this, 'computed_stateful_utxo_cohorts_age_range/_2m_to_3m'),
              _2yTo3y: create_10yTo12yPattern(this, 'computed_stateful_utxo_cohorts_age_range/_2y_to_3y'),
              _3mTo4m: create_10yTo12yPattern(this, 'computed_stateful_utxo_cohorts_age_range/_3m_to_4m'),
              _3yTo4y: create_10yTo12yPattern(this, 'computed_stateful_utxo_cohorts_age_range/_3y_to_4y'),
              _4mTo5m: create_10yTo12yPattern(this, 'computed_stateful_utxo_cohorts_age_range/_4m_to_5m'),
              _4yTo5y: create_10yTo12yPattern(this, 'computed_stateful_utxo_cohorts_age_range/_4y_to_5y'),
              _5mTo6m: create_10yTo12yPattern(this, 'computed_stateful_utxo_cohorts_age_range/_5m_to_6m'),
              _5yTo6y: create_10yTo12yPattern(this, 'computed_stateful_utxo_cohorts_age_range/_5y_to_6y'),
              _6mTo1y: create_10yTo12yPattern(this, 'computed_stateful_utxo_cohorts_age_range/_6m_to_1y'),
              _6yTo7y: create_10yTo12yPattern(this, 'computed_stateful_utxo_cohorts_age_range/_6y_to_7y'),
              _7yTo8y: create_10yTo12yPattern(this, 'computed_stateful_utxo_cohorts_age_range/_7y_to_8y'),
              _8yTo10y: create_10yTo12yPattern(this, 'computed_stateful_utxo_cohorts_age_range/_8y_to_10y'),
              from15y: create_10yTo12yPattern(this, 'computed_stateful_utxo_cohorts_age_range/from_15y'),
              upTo1d: createUpTo1dPattern(this, 'computed_stateful_utxo_cohorts_age_range/up_to_1d')
            },
            all: {
              activity: createActivityPattern(this, 'computed_stateful_utxo_cohorts_all/activity'),
              pricePaid: createPricePaidPattern2(this, 'computed_stateful_utxo_cohorts_all/price_paid'),
              realized: createRealizedPattern3(this, 'computed_stateful_utxo_cohorts_all/realized'),
              relative: {
                negUnrealizedLossRelToMarketCap: createIndexes21(this, '/neg_unrealized_loss_rel_to_market_cap'),
                negUnrealizedLossRelToOwnTotalUnrealizedPnl: createIndexes21(this, '/neg_unrealized_loss_rel_to_own_total_unrealized_pnl'),
                netUnrealizedPnlRelToMarketCap: createIndexes21(this, '/net_unrealized_pnl_rel_to_market_cap'),
                netUnrealizedPnlRelToOwnTotalUnrealizedPnl: createIndexes21(this, '/net_unrealized_pnl_rel_to_own_total_unrealized_pnl'),
                supplyInLossRelToOwnSupply: createIndexes21(this, '/supply_in_loss_rel_to_own_supply'),
                supplyInProfitRelToOwnSupply: createIndexes21(this, '/supply_in_profit_rel_to_own_supply'),
                unrealizedLossRelToMarketCap: createIndexes21(this, '/unrealized_loss_rel_to_market_cap'),
                unrealizedLossRelToOwnTotalUnrealizedPnl: createIndexes21(this, '/unrealized_loss_rel_to_own_total_unrealized_pnl'),
                unrealizedProfitRelToMarketCap: createIndexes21(this, '/unrealized_profit_rel_to_market_cap'),
                unrealizedProfitRelToOwnTotalUnrealizedPnl: createIndexes21(this, '/unrealized_profit_rel_to_own_total_unrealized_pnl')
              },
              supply: createSupplyPattern2(this, 'computed_stateful_utxo_cohorts_all/supply'),
              unrealized: createUnrealizedPattern(this, 'computed_stateful_utxo_cohorts_all/unrealized')
            },
            amountRange: {
              _0sats: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_amount_range/_0sats'),
              _100btcTo1kBtc: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_amount_range/_100btc_to_1k_btc'),
              _100kBtcOrMore: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_amount_range/_100k_btc_or_more'),
              _100kSatsTo1mSats: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_amount_range/_100k_sats_to_1m_sats'),
              _100satsTo1kSats: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_amount_range/_100sats_to_1k_sats'),
              _10btcTo100btc: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_amount_range/_10btc_to_100btc'),
              _10kBtcTo100kBtc: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_amount_range/_10k_btc_to_100k_btc'),
              _10kSatsTo100kSats: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_amount_range/_10k_sats_to_100k_sats'),
              _10mSatsTo1btc: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_amount_range/_10m_sats_to_1btc'),
              _10satsTo100sats: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_amount_range/_10sats_to_100sats'),
              _1btcTo10btc: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_amount_range/_1btc_to_10btc'),
              _1kBtcTo10kBtc: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_amount_range/_1k_btc_to_10k_btc'),
              _1kSatsTo10kSats: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_amount_range/_1k_sats_to_10k_sats'),
              _1mSatsTo10mSats: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_amount_range/_1m_sats_to_10m_sats'),
              _1satTo10sats: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_amount_range/_1sat_to_10sats')
            },
            epoch: {
              _0: create_10yTo12yPattern(this, 'epoch/_0'),
              _1: create_10yTo12yPattern(this, 'epoch/_1'),
              _2: create_10yTo12yPattern(this, 'epoch/_2'),
              _3: create_10yTo12yPattern(this, 'epoch/_3'),
              _4: create_10yTo12yPattern(this, 'epoch/_4')
            },
            geAmount: {
              _100btc: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_ge_amount/_100btc'),
              _100kSats: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_ge_amount/_100k_sats'),
              _100sats: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_ge_amount/_100sats'),
              _10btc: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_ge_amount/_10btc'),
              _10kBtc: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_ge_amount/_10k_btc'),
              _10kSats: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_ge_amount/_10k_sats'),
              _10mSats: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_ge_amount/_10m_sats'),
              _10sats: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_ge_amount/_10sats'),
              _1btc: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_ge_amount/_1btc'),
              _1kBtc: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_ge_amount/_1k_btc'),
              _1kSats: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_ge_amount/_1k_sats'),
              _1mSats: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_ge_amount/_1m_sats'),
              _1sat: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_ge_amount/_1sat')
            },
            ltAmount: {
              _100btc: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_lt_amount/_100btc'),
              _100kBtc: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_lt_amount/_100k_btc'),
              _100kSats: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_lt_amount/_100k_sats'),
              _100sats: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_lt_amount/_100sats'),
              _10btc: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_lt_amount/_10btc'),
              _10kBtc: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_lt_amount/_10k_btc'),
              _10kSats: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_lt_amount/_10k_sats'),
              _10mSats: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_lt_amount/_10m_sats'),
              _10sats: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_lt_amount/_10sats'),
              _1btc: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_lt_amount/_1btc'),
              _1kBtc: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_lt_amount/_1k_btc'),
              _1kSats: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_lt_amount/_1k_sats'),
              _1mSats: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_lt_amount/_1m_sats')
            },
            maxAge: {
              _10y: createUpTo1dPattern(this, 'computed_stateful_utxo_cohorts_max_age/_10y'),
              _12y: createUpTo1dPattern(this, 'computed_stateful_utxo_cohorts_max_age/_12y'),
              _15y: createUpTo1dPattern(this, 'computed_stateful_utxo_cohorts_max_age/_15y'),
              _1m: createUpTo1dPattern(this, 'computed_stateful_utxo_cohorts_max_age/_1m'),
              _1w: createUpTo1dPattern(this, 'computed_stateful_utxo_cohorts_max_age/_1w'),
              _1y: createUpTo1dPattern(this, 'computed_stateful_utxo_cohorts_max_age/_1y'),
              _2m: createUpTo1dPattern(this, 'computed_stateful_utxo_cohorts_max_age/_2m'),
              _2y: createUpTo1dPattern(this, 'computed_stateful_utxo_cohorts_max_age/_2y'),
              _3m: createUpTo1dPattern(this, 'computed_stateful_utxo_cohorts_max_age/_3m'),
              _3y: createUpTo1dPattern(this, 'computed_stateful_utxo_cohorts_max_age/_3y'),
              _4m: createUpTo1dPattern(this, 'computed_stateful_utxo_cohorts_max_age/_4m'),
              _4y: createUpTo1dPattern(this, 'computed_stateful_utxo_cohorts_max_age/_4y'),
              _5m: createUpTo1dPattern(this, 'computed_stateful_utxo_cohorts_max_age/_5m'),
              _5y: createUpTo1dPattern(this, 'computed_stateful_utxo_cohorts_max_age/_5y'),
              _6m: createUpTo1dPattern(this, 'computed_stateful_utxo_cohorts_max_age/_6m'),
              _6y: createUpTo1dPattern(this, 'computed_stateful_utxo_cohorts_max_age/_6y'),
              _7y: createUpTo1dPattern(this, 'computed_stateful_utxo_cohorts_max_age/_7y'),
              _8y: createUpTo1dPattern(this, 'computed_stateful_utxo_cohorts_max_age/_8y')
            },
            minAge: {
              _10y: create_10yTo12yPattern(this, 'computed_stateful_utxo_cohorts_min_age/_10y'),
              _12y: create_10yTo12yPattern(this, 'computed_stateful_utxo_cohorts_min_age/_12y'),
              _1d: create_10yTo12yPattern(this, 'computed_stateful_utxo_cohorts_min_age/_1d'),
              _1m: create_10yTo12yPattern(this, 'computed_stateful_utxo_cohorts_min_age/_1m'),
              _1w: create_10yTo12yPattern(this, 'computed_stateful_utxo_cohorts_min_age/_1w'),
              _1y: create_10yTo12yPattern(this, 'computed_stateful_utxo_cohorts_min_age/_1y'),
              _2m: create_10yTo12yPattern(this, 'computed_stateful_utxo_cohorts_min_age/_2m'),
              _2y: create_10yTo12yPattern(this, 'computed_stateful_utxo_cohorts_min_age/_2y'),
              _3m: create_10yTo12yPattern(this, 'computed_stateful_utxo_cohorts_min_age/_3m'),
              _3y: create_10yTo12yPattern(this, 'computed_stateful_utxo_cohorts_min_age/_3y'),
              _4m: create_10yTo12yPattern(this, 'computed_stateful_utxo_cohorts_min_age/_4m'),
              _4y: create_10yTo12yPattern(this, 'computed_stateful_utxo_cohorts_min_age/_4y'),
              _5m: create_10yTo12yPattern(this, 'computed_stateful_utxo_cohorts_min_age/_5m'),
              _5y: create_10yTo12yPattern(this, 'computed_stateful_utxo_cohorts_min_age/_5y'),
              _6m: create_10yTo12yPattern(this, 'computed_stateful_utxo_cohorts_min_age/_6m'),
              _6y: create_10yTo12yPattern(this, 'computed_stateful_utxo_cohorts_min_age/_6y'),
              _7y: create_10yTo12yPattern(this, 'computed_stateful_utxo_cohorts_min_age/_7y'),
              _8y: create_10yTo12yPattern(this, 'computed_stateful_utxo_cohorts_min_age/_8y')
            },
            term: {
              long: createUpTo1dPattern(this, 'computed_stateful_utxo_cohorts_term/long'),
              short: createUpTo1dPattern(this, 'computed_stateful_utxo_cohorts_term/short')
            },
            type: {
              empty: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_type_/empty'),
              p2a: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_type_/p2a'),
              p2ms: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_type_/p2ms'),
              p2pk33: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_type_/p2pk33'),
              p2pk65: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_type_/p2pk65'),
              p2pkh: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_type_/p2pkh'),
              p2sh: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_type_/p2sh'),
              p2tr: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_type_/p2tr'),
              p2wpkh: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_type_/p2wpkh'),
              p2wsh: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_type_/p2wsh'),
              unknown: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_type_/unknown')
            },
            year: {
              _2009: create_10yTo12yPattern(this, 'year/_2009'),
              _2010: create_10yTo12yPattern(this, 'year/_2010'),
              _2011: create_10yTo12yPattern(this, 'year/_2011'),
              _2012: create_10yTo12yPattern(this, 'year/_2012'),
              _2013: create_10yTo12yPattern(this, 'year/_2013'),
              _2014: create_10yTo12yPattern(this, 'year/_2014'),
              _2015: create_10yTo12yPattern(this, 'year/_2015'),
              _2016: create_10yTo12yPattern(this, 'year/_2016'),
              _2017: create_10yTo12yPattern(this, 'year/_2017'),
              _2018: create_10yTo12yPattern(this, 'year/_2018'),
              _2019: create_10yTo12yPattern(this, 'year/_2019'),
              _2020: create_10yTo12yPattern(this, 'year/_2020'),
              _2021: create_10yTo12yPattern(this, 'year/_2021'),
              _2022: create_10yTo12yPattern(this, 'year/_2022'),
              _2023: create_10yTo12yPattern(this, 'year/_2023'),
              _2024: create_10yTo12yPattern(this, 'year/_2024'),
              _2025: create_10yTo12yPattern(this, 'year/_2025'),
              _2026: create_10yTo12yPattern(this, 'year/_2026')
            }
          }
        }
      },
      indexed: {
        address: {
          firstP2aaddressindex: createIndexes2(this, '/first_p2aaddressindex'),
          firstP2pk33addressindex: createIndexes2(this, '/first_p2pk33addressindex'),
          firstP2pk65addressindex: createIndexes2(this, '/first_p2pk65addressindex'),
          firstP2pkhaddressindex: createIndexes2(this, '/first_p2pkhaddressindex'),
          firstP2shaddressindex: createIndexes2(this, '/first_p2shaddressindex'),
          firstP2traddressindex: createIndexes2(this, '/first_p2traddressindex'),
          firstP2wpkhaddressindex: createIndexes2(this, '/first_p2wpkhaddressindex'),
          firstP2wshaddressindex: createIndexes2(this, '/first_p2wshaddressindex'),
          p2abytes: createIndexes12(this, '/p2abytes'),
          p2pk33bytes: createIndexes13(this, '/p2pk33bytes'),
          p2pk65bytes: createIndexes14(this, '/p2pk65bytes'),
          p2pkhbytes: createIndexes15(this, '/p2pkhbytes'),
          p2shbytes: createIndexes16(this, '/p2shbytes'),
          p2trbytes: createIndexes17(this, '/p2trbytes'),
          p2wpkhbytes: createIndexes18(this, '/p2wpkhbytes'),
          p2wshbytes: createIndexes19(this, '/p2wshbytes')
        },
        block: {
          blockhash: createIndexes2(this, '/blockhash'),
          difficulty: createIndexes2(this, '/difficulty'),
          timestamp: createIndexes2(this, '/timestamp'),
          totalSize: createIndexes2(this, '/total_size'),
          weight: createIndexes2(this, '/weight')
        },
        output: {
          firstEmptyoutputindex: createIndexes2(this, '/first_emptyoutputindex'),
          firstOpreturnindex: createIndexes2(this, '/first_opreturnindex'),
          firstP2msoutputindex: createIndexes2(this, '/first_p2msoutputindex'),
          firstUnknownoutputindex: createIndexes2(this, '/first_unknownoutputindex'),
          txindex: new MetricNode(this, '/txindex')
        },
        tx: {
          baseSize: createIndexes6(this, '/base_size'),
          firstTxindex: createIndexes2(this, '/first_txindex'),
          firstTxinindex: createIndexes6(this, '/first_txinindex'),
          firstTxoutindex: createIndexes6(this, '/first_txoutindex'),
          height: createIndexes6(this, '/height'),
          isExplicitlyRbf: createIndexes6(this, '/is_explicitly_rbf'),
          rawlocktime: createIndexes6(this, '/rawlocktime'),
          totalSize: createIndexes6(this, '/total_size'),
          txid: createIndexes6(this, '/txid'),
          txversion: createIndexes6(this, '/txversion')
        },
        txin: {
          firstTxinindex: createIndexes2(this, '/first_txinindex'),
          outpoint: createIndexes7(this, '/outpoint'),
          txindex: createIndexes7(this, '/txindex')
        },
        txout: {
          firstTxoutindex: createIndexes2(this, '/first_txoutindex'),
          outputtype: createIndexes20(this, '/outputtype'),
          txindex: createIndexes20(this, '/txindex'),
          typeindex: createIndexes20(this, '/typeindex'),
          value: createIndexes20(this, '/value')
        }
      }
    };
  }

  /**
   * Address information
   * @param {string} address 
   * @returns {Promise<AddressStats>}
   */
  async getApiAddressByAddress(address) {
    return this.get(`/api/address/${address}`);
  }

  /**
   * Address transaction IDs
   * @param {string} address 
   * @param {string=} [after_txid] Txid to paginate from (return transactions before this one)
   * @param {string=} [limit] Maximum number of results to return. Defaults to 25 if not specified.
   * @returns {Promise<Txid[]>}
   */
  async getApiAddressByAddressTxs(address, after_txid, limit) {
    const params = new URLSearchParams();
    if (after_txid !== undefined) params.set('after_txid', String(after_txid));
    if (limit !== undefined) params.set('limit', String(limit));
    const query = params.toString();
    return this.get(`/api/address/${address}/txs${query ? '?' + query : ''}`);
  }

  /**
   * Address confirmed transactions
   * @param {string} address 
   * @param {string=} [after_txid] Txid to paginate from (return transactions before this one)
   * @param {string=} [limit] Maximum number of results to return. Defaults to 25 if not specified.
   * @returns {Promise<Txid[]>}
   */
  async getApiAddressByAddressTxsChain(address, after_txid, limit) {
    const params = new URLSearchParams();
    if (after_txid !== undefined) params.set('after_txid', String(after_txid));
    if (limit !== undefined) params.set('limit', String(limit));
    const query = params.toString();
    return this.get(`/api/address/${address}/txs/chain${query ? '?' + query : ''}`);
  }

  /**
   * Address mempool transactions
   * @param {string} address 
   * @returns {Promise<Txid[]>}
   */
  async getApiAddressByAddressTxsMempool(address) {
    return this.get(`/api/address/${address}/txs/mempool`);
  }

  /**
   * Address UTXOs
   * @param {string} address 
   * @returns {Promise<Utxo[]>}
   */
  async getApiAddressByAddressUtxo(address) {
    return this.get(`/api/address/${address}/utxo`);
  }

  /**
   * Block by height
   * @param {string} height 
   * @returns {Promise<BlockInfo>}
   */
  async getApiBlockHeightByHeight(height) {
    return this.get(`/api/block-height/${height}`);
  }

  /**
   * Block information
   * @param {string} hash 
   * @returns {Promise<BlockInfo>}
   */
  async getApiBlockByHash(hash) {
    return this.get(`/api/block/${hash}`);
  }

  /**
   * Raw block
   * @param {string} hash 
   * @returns {Promise<number[]>}
   */
  async getApiBlockByHashRaw(hash) {
    return this.get(`/api/block/${hash}/raw`);
  }

  /**
   * Block status
   * @param {string} hash 
   * @returns {Promise<BlockStatus>}
   */
  async getApiBlockByHashStatus(hash) {
    return this.get(`/api/block/${hash}/status`);
  }

  /**
   * Transaction ID at index
   * @param {string} hash Bitcoin block hash
   * @param {string} index Transaction index within the block (0-based)
   * @returns {Promise<Txid>}
   */
  async getApiBlockByHashTxidByIndex(hash, index) {
    return this.get(`/api/block/${hash}/txid/${index}`);
  }

  /**
   * Block transaction IDs
   * @param {string} hash 
   * @returns {Promise<Txid[]>}
   */
  async getApiBlockByHashTxids(hash) {
    return this.get(`/api/block/${hash}/txids`);
  }

  /**
   * Block transactions (paginated)
   * @param {string} hash Bitcoin block hash
   * @param {string} start_index Starting transaction index within the block (0-based)
   * @returns {Promise<Transaction[]>}
   */
  async getApiBlockByHashTxsByStartIndex(hash, start_index) {
    return this.get(`/api/block/${hash}/txs/${start_index}`);
  }

  /**
   * Recent blocks
   * @returns {Promise<BlockInfo[]>}
   */
  async getApiBlocks() {
    return this.get(`/api/blocks`);
  }

  /**
   * Blocks from height
   * @param {string} height 
   * @returns {Promise<BlockInfo[]>}
   */
  async getApiBlocksByHeight(height) {
    return this.get(`/api/blocks/${height}`);
  }

  /**
   * Mempool statistics
   * @returns {Promise<MempoolInfo>}
   */
  async getApiMempoolInfo() {
    return this.get(`/api/mempool/info`);
  }

  /**
   * Mempool transaction IDs
   * @returns {Promise<Txid[]>}
   */
  async getApiMempoolTxids() {
    return this.get(`/api/mempool/txids`);
  }

  /**
   * Get supported indexes for a metric
   * @param {string} metric 
   * @returns {Promise<Index[]>}
   */
  async getApiMetricByMetric(metric) {
    return this.get(`/api/metric/${metric}`);
  }

  /**
   * Get metric data
   * @param {string} metric Metric name
   * @param {string} index Aggregation index
   * @param {string=} [from] Inclusive starting index, if negative counts from end
   * @param {string=} [to] Exclusive ending index, if negative counts from end
   * @param {string=} [count] Number of values to return (ignored if `to` is set)
   * @param {string=} [format] Format of the output
   * @returns {Promise<MetricData>}
   */
  async getApiMetricByMetricByIndex(metric, index, from, to, count, format) {
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
   * @param {string} [metrics] Requested metrics
   * @param {string} [index] Index to query
   * @param {string=} [from] Inclusive starting index, if negative counts from end
   * @param {string=} [to] Exclusive ending index, if negative counts from end
   * @param {string=} [count] Number of values to return (ignored if `to` is set)
   * @param {string=} [format] Format of the output
   * @returns {Promise<MetricData[]>}
   */
  async getApiMetricsBulk(metrics, index, from, to, count, format) {
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
   * @returns {Promise<TreeNode>}
   */
  async getApiMetricsCatalog() {
    return this.get(`/api/metrics/catalog`);
  }

  /**
   * Metric count
   * @returns {Promise<MetricCount[]>}
   */
  async getApiMetricsCount() {
    return this.get(`/api/metrics/count`);
  }

  /**
   * List available indexes
   * @returns {Promise<IndexInfo[]>}
   */
  async getApiMetricsIndexes() {
    return this.get(`/api/metrics/indexes`);
  }

  /**
   * Metrics list
   * @param {string=} [page] Pagination index
   * @returns {Promise<PaginatedMetrics>}
   */
  async getApiMetricsList(page) {
    const params = new URLSearchParams();
    if (page !== undefined) params.set('page', String(page));
    const query = params.toString();
    return this.get(`/api/metrics/list${query ? '?' + query : ''}`);
  }

  /**
   * Search metrics
   * @param {string} metric 
   * @param {string=} [limit] 
   * @returns {Promise<string[]>}
   */
  async getApiMetricsSearchByMetric(metric, limit) {
    const params = new URLSearchParams();
    if (limit !== undefined) params.set('limit', String(limit));
    const query = params.toString();
    return this.get(`/api/metrics/search/${metric}${query ? '?' + query : ''}`);
  }

  /**
   * Transaction information
   * @param {string} txid 
   * @returns {Promise<Transaction>}
   */
  async getApiTxByTxid(txid) {
    return this.get(`/api/tx/${txid}`);
  }

  /**
   * Transaction hex
   * @param {string} txid 
   * @returns {Promise<string>}
   */
  async getApiTxByTxidHex(txid) {
    return this.get(`/api/tx/${txid}/hex`);
  }

  /**
   * Output spend status
   * @param {string} txid Transaction ID
   * @param {string} vout Output index
   * @returns {Promise<TxOutspend>}
   */
  async getApiTxByTxidOutspendByVout(txid, vout) {
    return this.get(`/api/tx/${txid}/outspend/${vout}`);
  }

  /**
   * All output spend statuses
   * @param {string} txid 
   * @returns {Promise<TxOutspend[]>}
   */
  async getApiTxByTxidOutspends(txid) {
    return this.get(`/api/tx/${txid}/outspends`);
  }

  /**
   * Transaction status
   * @param {string} txid 
   * @returns {Promise<TxStatus>}
   */
  async getApiTxByTxidStatus(txid) {
    return this.get(`/api/tx/${txid}/status`);
  }

  /**
   * Difficulty adjustment
   * @returns {Promise<DifficultyAdjustment>}
   */
  async getApiV1DifficultyAdjustment() {
    return this.get(`/api/v1/difficulty-adjustment`);
  }

  /**
   * Projected mempool blocks
   * @returns {Promise<MempoolBlock[]>}
   */
  async getApiV1FeesMempoolBlocks() {
    return this.get(`/api/v1/fees/mempool-blocks`);
  }

  /**
   * Recommended fees
   * @returns {Promise<RecommendedFees>}
   */
  async getApiV1FeesRecommended() {
    return this.get(`/api/v1/fees/recommended`);
  }

  /**
   * Block fees
   * @param {string} time_period 
   * @returns {Promise<BlockFeesEntry[]>}
   */
  async getApiV1MiningBlocksFeesByTimePeriod(time_period) {
    return this.get(`/api/v1/mining/blocks/fees/${time_period}`);
  }

  /**
   * Block rewards
   * @param {string} time_period 
   * @returns {Promise<BlockRewardsEntry[]>}
   */
  async getApiV1MiningBlocksRewardsByTimePeriod(time_period) {
    return this.get(`/api/v1/mining/blocks/rewards/${time_period}`);
  }

  /**
   * Block sizes and weights
   * @param {string} time_period 
   * @returns {Promise<BlockSizesWeights>}
   */
  async getApiV1MiningBlocksSizesWeightsByTimePeriod(time_period) {
    return this.get(`/api/v1/mining/blocks/sizes-weights/${time_period}`);
  }

  /**
   * Block by timestamp
   * @param {string} timestamp 
   * @returns {Promise<BlockTimestamp>}
   */
  async getApiV1MiningBlocksTimestampByTimestamp(timestamp) {
    return this.get(`/api/v1/mining/blocks/timestamp/${timestamp}`);
  }

  /**
   * Difficulty adjustments (all time)
   * @returns {Promise<DifficultyAdjustmentEntry[]>}
   */
  async getApiV1MiningDifficultyAdjustments() {
    return this.get(`/api/v1/mining/difficulty-adjustments`);
  }

  /**
   * Difficulty adjustments
   * @param {string} time_period 
   * @returns {Promise<DifficultyAdjustmentEntry[]>}
   */
  async getApiV1MiningDifficultyAdjustmentsByTimePeriod(time_period) {
    return this.get(`/api/v1/mining/difficulty-adjustments/${time_period}`);
  }

  /**
   * Network hashrate (all time)
   * @returns {Promise<HashrateSummary>}
   */
  async getApiV1MiningHashrate() {
    return this.get(`/api/v1/mining/hashrate`);
  }

  /**
   * Network hashrate
   * @param {string} time_period 
   * @returns {Promise<HashrateSummary>}
   */
  async getApiV1MiningHashrateByTimePeriod(time_period) {
    return this.get(`/api/v1/mining/hashrate/${time_period}`);
  }

  /**
   * Mining pool details
   * @param {string} slug 
   * @returns {Promise<PoolDetail>}
   */
  async getApiV1MiningPoolBySlug(slug) {
    return this.get(`/api/v1/mining/pool/${slug}`);
  }

  /**
   * List all mining pools
   * @returns {Promise<PoolInfo[]>}
   */
  async getApiV1MiningPools() {
    return this.get(`/api/v1/mining/pools`);
  }

  /**
   * Mining pool statistics
   * @param {string} time_period 
   * @returns {Promise<PoolsSummary>}
   */
  async getApiV1MiningPoolsByTimePeriod(time_period) {
    return this.get(`/api/v1/mining/pools/${time_period}`);
  }

  /**
   * Mining reward statistics
   * @param {string} block_count Number of recent blocks to include
   * @returns {Promise<RewardStats>}
   */
  async getApiV1MiningRewardStatsByBlockCount(block_count) {
    return this.get(`/api/v1/mining/reward-stats/${block_count}`);
  }

  /**
   * Validate address
   * @param {string} address Bitcoin address to validate (can be any string)
   * @returns {Promise<AddressValidation>}
   */
  async getApiV1ValidateAddressByAddress(address) {
    return this.get(`/api/v1/validate-address/${address}`);
  }

  /**
   * Health check
   * @returns {Promise<Health>}
   */
  async getHealth() {
    return this.get(`/health`);
  }

  /**
   * API version
   * @returns {Promise<string>}
   */
  async getVersion() {
    return this.get(`/version`);
  }

}

export { BrkClient, BrkClientBase, BrkError, MetricNode };
