/**
 * @import { IChartApi, ISeriesApi as _ISeriesApi, SeriesDefinition, SingleValueData as _SingleValueData, CandlestickData as _CandlestickData, BaselineData as _BaselineData, HistogramData as _HistogramData, SeriesType as LCSeriesType, IPaneApi, LineSeriesPartialOptions as _LineSeriesPartialOptions, HistogramSeriesPartialOptions as _HistogramSeriesPartialOptions, BaselineSeriesPartialOptions as _BaselineSeriesPartialOptions, CandlestickSeriesPartialOptions as _CandlestickSeriesPartialOptions, WhitespaceData, DeepPartial, ChartOptions, Time, LineData as _LineData, createChart as CreateLCChart, LineStyle, createSeriesMarkers as CreateSeriesMarkers, SeriesMarker, ISeriesMarkersPluginApi } from './modules/lightweight-charts/5.1.0/dist/typings.js'
 *
 * @import * as Brk from "./modules/brk-client/index.js"
 * @import { BrkClient, Index, SeriesData } from "./modules/brk-client/index.js"
 *
 * @import { Options } from './options/full.js'
 *
 * @import { PersistedValue } from './utils/persisted.js'
 *
 * @import { SingleValueData, CandlestickData, Series, AnySeries, ISeries, HistogramData, LineData, BaselineData, LineSeriesPartialOptions, BaselineSeriesPartialOptions, HistogramSeriesPartialOptions, CandlestickSeriesPartialOptions, Chart, Legend } from "./chart/index.js"
 *
 * @import { Color } from "./utils/colors.js"
 *
 * @import { Option, PartialChartOption, ChartOption, AnyPartialOption, ProcessedOptionAddons, OptionsTree, AnySeriesBlueprint, SeriesType, AnyFetchedSeriesBlueprint, ExplorerOption, UrlOption, PartialOptionsGroup, OptionsGroup, PartialOptionsTree, UtxoCohortObject, AddrCohortObject, CohortObject, CohortGroupObject, FetchedLineSeriesBlueprint, FetchedBaselineSeriesBlueprint, FetchedHistogramSeriesBlueprint, FetchedDotsBaselineSeriesBlueprint, PatternAll, PatternFull, PatternWithAdjusted, PatternWithPercentiles, PatternBasic, PatternBasicWithMarketCap, PatternBasicWithoutMarketCap, PatternWithoutRelative, CohortAll, CohortFull, CohortWithAdjusted, CohortWithPercentiles, CohortBasic, CohortBasicWithMarketCap, CohortBasicWithoutMarketCap, CohortWithoutRelative, CohortAddr, CohortLongTerm, CohortAgeRange, CohortAgeRangeWithMatured, CohortGroupFull, CohortGroupWithAdjusted, CohortGroupWithPercentiles, CohortGroupLongTerm, CohortGroupAgeRange, CohortGroupBasic, CohortGroupBasicWithMarketCap, CohortGroupBasicWithoutMarketCap, CohortGroupWithoutRelative, CohortGroupAddr, UtxoCohortGroupObject, AddrCohortGroupObject, FetchedDotsSeriesBlueprint, FetchedCandlestickSeriesBlueprint, FetchedPriceSeriesBlueprint, AnyPricePattern, AnyValuePattern } from "./options/partial.js"
 *
 *
 * @import { UnitObject as Unit } from "./utils/units.js"
 *
 * @import { ChartableIndex, IndexLabel } from "./utils/serde.js";
 */

// import uFuzzy = require("./modules/leeoniya-ufuzzy/1.0.19/dist/uFuzzy.d.ts");

/**
 * @typedef {[number, number, number, number]} OHLCTuple
 *
 * Lightweight Charts markers
 * @typedef {ISeriesMarkersPluginApi<Time>} SeriesMarkersPlugin
 * @typedef {SeriesMarker<Time>} TimeSeriesMarker
 *
 * Brk tree types (stable across regenerations)
 * @typedef {Brk.SeriesTree_Cohorts_Utxo} UtxoCohortTree
 * @typedef {Brk.SeriesTree_Cohorts_Addr} AddrCohortTree
 * @typedef {Brk.SeriesTree_Cohorts_Utxo_All} AllUtxoPattern
 * @typedef {Brk.SeriesTree_Cohorts_Utxo_Sth} ShortTermPattern
 * @typedef {Brk.SeriesTree_Cohorts_Utxo_Lth} LongTermPattern
 * @typedef {Brk.SeriesTree_Cohorts_Utxo_All_Unrealized} AllRelativePattern
 * @typedef {keyof Brk.BtcCentsSatsUsdPattern} BtcSatsUsdKey
 * @typedef {Brk.BtcCentsSatsUsdPattern} SupplyPattern
 * @typedef {Brk.AverageBlockCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern} BlockSizePattern
 * @typedef {keyof Brk.SeriesTree_Cohorts_Utxo_Type} SpendableType
 * @typedef {Brk.SpendingSpentUnspentPattern} OutputsPattern
 * @typedef {keyof Brk.SeriesTree_Addrs_Raw} AddressableType
 *
 * Brk pattern types (using new pattern names)
 * @typedef {Brk.ActivityOutputsRealizedSupplyUnrealizedPattern} MaxAgePattern
 * @typedef {Brk.ActivityOutputsRealizedSupplyUnrealizedPattern} AgeRangePattern
 * @typedef {Brk.ActivityOutputsRealizedSupplyUnrealizedPattern2} UtxoAmountPattern
 * @typedef {Brk.ActivityAddrOutputsRealizedSupplyUnrealizedPattern} AddrAmountPattern
 * @typedef {Brk.ActivityOutputsRealizedSupplyUnrealizedPattern} BasicUtxoPattern
 * @typedef {Brk.ActivityOutputsRealizedSupplyUnrealizedPattern} EpochPattern
 * @typedef {Brk.ActivityOutputsRealizedSupplyUnrealizedPattern3} EmptyPattern
 * @typedef {Brk._0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdZscorePattern} Ratio1ySdPattern
 * @typedef {Brk.Dollars} Dollars
 * @typedef {Brk.BlockInfo} BlockInfo
 * @typedef {Brk.BlockHash} BlockHash
 * @typedef {Brk.BlockInfoV1} BlockInfoV1
 * @typedef {Brk.Transaction} Transaction
 * ActivePriceRatioPattern: ratio pattern with price (extended)
 * @typedef {Brk.BpsPriceRatioPattern} ActivePriceRatioPattern
 * PriceRatioPercentilesPattern: price pattern with ratio + percentiles (no SMAs/stdDev)
 * @typedef {Brk.BpsCentsPercentilesRatioSatsUsdPattern} PriceRatioPercentilesPattern
 * AnyRatioPattern: full ratio pattern with percentiles, SMAs, and std dev bands
 * @typedef {Brk.BpsCentsPercentilesRatioSatsSmaStdUsdPattern} AnyRatioPattern
 * FullValuePattern: block + cumulative + sum + average rolling windows (sats/btc/cents/usd)
 * @typedef {Brk.AverageBlockCumulativeSumPattern3} FullValuePattern
 * RollingWindowSlot: a single rolling window with stats (pct10, pct25, median, pct75, pct90, max, min) per unit
 * @typedef {Brk.MaxMedianMinPct10Pct25Pct75Pct90Pattern<number>} RollingWindowSlot
 * @typedef {Brk.AnySeriesPattern} AnySeriesPattern
 * @typedef {Brk.CentsSatsUsdPattern} ActivePricePattern
 * @typedef {Brk.AnySeriesEndpoint} AnySeriesEndpoint
 * @typedef {Brk.AnySeriesData} AnySeriesData
 * Relative patterns by capability:
 * Unrealized patterns by capability level
 * @typedef {Brk.LossNetNuplProfitPattern} BasicRelativePattern
 * @typedef {Brk.GrossInvestedInvestorLossNetNuplProfitSentimentPattern2} FullRelativePattern
 *
 * Profitability bucket pattern (supply + realized_cap + unrealized_pnl + nupl)
 * @typedef {Brk.NuplRealizedSupplyUnrealizedPattern} RealizedSupplyPattern
 *
 * Realized pattern (full: cap + gross + investor + loss + mvrv + net + peak + price + profit + sell + sopr)
 * @typedef {Brk.CapGrossInvestorLossMvrvNetPeakPriceProfitSellSoprPattern} RealizedPattern
 *
 * Transfer volume pattern (block + cumulative + inProfit/inLoss + sum windows)
 * @typedef {Brk.AverageBlockCumulativeInSumPattern} TransferVolumePattern
 *
 * Realized profit/loss pattern (block + cumulative + sum windows, cents/usd)
 * @typedef {Brk.BlockCumulativeSumPattern} RealizedProfitLossPattern
 *
 * Full activity pattern (coindays, coinyears, dormancy, transfer volume)
 * @typedef {Brk.CoindaysCoinyearsDormancyTransferPattern} FullActivityPattern
 *
 *
 * BPS + percent + ratio pattern
 * @typedef {Brk.BpsPercentRatioPattern3} PercentRatioPattern
 *
 * BPS + ratio pattern (for NUPL and similar)
 * @typedef {Brk.BpsRatioPattern} NuplPattern
 *
 * LTH realized tree
 * @typedef {Brk.SeriesTree_Cohorts_Utxo_Lth_Realized} LthRealizedPattern
 *
 * Net PnL pattern with change (base + change + cumulative + delta + rel + sum)
 * @typedef {Brk.BlockChangeCumulativeDeltaSumPattern} NetPnlFullPattern
 *
 * Net PnL basic pattern (base + cumulative + delta + sum)
 * @typedef {Brk.BlockCumulativeDeltaSumPattern} NetPnlBasicPattern
 *
 * Mid realized pattern (cap + loss + MVRV + net + price + profit + SOPR)
 * @typedef {Brk.CapLossMvrvNetPriceProfitSoprPattern} MidRealizedPattern
 *
 * Basic realized pattern (cap + loss + MVRV + price + profit, no net/sopr)
 * @typedef {Brk.CapLossMvrvPriceProfitPattern} BasicRealizedPattern
 *
 * Moving average price ratio pattern (bps + cents + ratio + sats + usd)
 * @typedef {Brk.BpsCentsRatioSatsUsdPattern} MaPriceRatioPattern
 *
 * Address count pattern (base + delta with absolute + rate)
 * @typedef {Brk.BaseDeltaPattern} AddrCountPattern
 */

/**
 * @template T
 * @typedef {Brk.SeriesEndpoint<T>} SeriesEndpoint
 */
/**
 * Rolling windows pattern (24h, 1w, 1m, 1y)
 * @template T
 * @typedef {Brk._1m1w1y24hPattern<T>} RollingWindowPattern
 */
/**
 * Sell side risk rolling windows pattern
 * @typedef {Brk._1m1w1y24hPattern7} SellSideRiskPattern
 */
/**
 * Stats pattern: min, max, median, percentiles
 * @typedef {Brk.MaxMedianMinPct10Pct25Pct75Pct90Pattern<number>} StatsPattern
 */
/**
 * Full stats pattern: cumulative, sum, average, min, max, percentiles + rolling
 * @typedef {Brk.AverageBlockCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern} FullStatsPattern
 */
/**
 * Aggregated pattern: cumulative + rolling (with distribution stats) + sum (no base)
 * @typedef {Brk.CumulativeRollingSumPattern} AggregatedPattern
 */
/**
 * Count pattern: height, cumulative, and rolling sum windows
 * @template T
 * @typedef {Brk.AverageBlockCumulativeSumPattern<T>} CountPattern
 */
/**
 * Full per-block pattern: height, cumulative, sum, and distribution stats (all flat)
 * FullPerBlockPattern: cumulative + sum + average + distribution stats (used by chartsFromFull)
 * Note: some callers also have .block but the function doesn't use it
 * @typedef {Omit<Brk.AverageBlockCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern, 'block'>} FullPerBlockPattern
 */
/**
 * Any stats pattern union
 * @typedef {FullStatsPattern} AnyStatsPattern
 */
/**
 * Distribution stats: min, max, median, pct10/25/75/90
 * @typedef {{ min: AnySeriesPattern, max: AnySeriesPattern, median: AnySeriesPattern, pct10: AnySeriesPattern, pct25: AnySeriesPattern, pct75: AnySeriesPattern, pct90: AnySeriesPattern }} DistributionStats
 */
/**
 * Windowed distribution stats: each stat property is a rolling window record
 * @template T
 * @typedef {{ median: Record<string, T>, max: Record<string, T>, min: Record<string, T>, pct75: Record<string, T>, pct25: Record<string, T>, pct90: Record<string, T>, pct10: Record<string, T> }} WindowedStats
 */
/**
 * Dominance pattern: percent/ratio at top level + per rolling window
 * @typedef {Brk._1m1w1y24hBpsPercentRatioPattern} DominancePattern
 */

/**
 *
 * @typedef {InstanceType<typeof BrkClient>["INDEXES"]} Indexes
 * @typedef {Indexes[number]} IndexName
 * @typedef {InstanceType<typeof BrkClient>["POOL_ID_TO_POOL_NAME"]} PoolIdToPoolName
 * @typedef {keyof PoolIdToPoolName} PoolId
 *
 * Tree branch types
 * @typedef {Brk.SeriesTree_Market} Market
 * @typedef {Brk.SeriesTree_Market_MovingAverage} MarketMovingAverage
 * @typedef {Brk.SeriesTree_Investing} Investing
 * @typedef {Brk._10y2y3y4y5y6y8yPattern} PeriodCagrPattern
 * @typedef {FullStatsPattern} AnyFullStatsPattern
 *
 * DCA period keys - derived from pattern types
 * @typedef {keyof Brk._10y2y3y4y5y6y8yPattern} LongPeriodKey
 * @typedef {"_1w" | "_1m" | "_3m" | "_6m" | "_1y"} ShortPeriodKey
 * @typedef {ShortPeriodKey | LongPeriodKey} AllPeriodKey
 *
 * Pattern unions by cohort type
 * @typedef {AllUtxoPattern | AgeRangePattern | UtxoAmountPattern} UtxoCohortPattern
 * @typedef {AddrAmountPattern} AddrCohortPattern
 * @typedef {UtxoCohortPattern | AddrCohortPattern} CohortPattern
 *
 * Relative pattern capability types
 * @typedef {BasicRelativePattern | FullRelativePattern | AllRelativePattern} RelativeWithMarketCap
 * @typedef {FullRelativePattern | AllRelativePattern} RelativeWithOwnMarketCap
 * @typedef {FullRelativePattern | AllRelativePattern} RelativeWithOwnPnl
 * @typedef {BasicRelativePattern | FullRelativePattern | AllRelativePattern} RelativeWithNupl
 * @typedef {BasicRelativePattern | FullRelativePattern | AllRelativePattern} RelativeWithInvestedCapitalPct
 *
 * Realized pattern capability types
 * @typedef {RealizedPattern} AnyRealizedPattern
 *
 * Capability-based pattern groupings (patterns that have specific properties)
 * @typedef {AllUtxoPattern | AgeRangePattern | UtxoAmountPattern} PatternWithRealizedPrice
 * @typedef {AllUtxoPattern} PatternWithFullRealized
 * @typedef {ShortTermPattern | LongTermPattern | MaxAgePattern | BasicUtxoPattern} PatternWithNupl
 * @typedef {AllUtxoPattern | AgeRangePattern | UtxoAmountPattern} PatternWithCostBasis
 * @typedef {AllUtxoPattern | AgeRangePattern | UtxoAmountPattern} PatternWithActivity
 * @typedef {AllUtxoPattern | AgeRangePattern} PatternWithCostBasisPercentiles
 * @typedef {Brk.Pct05Pct10Pct15Pct20Pct25Pct30Pct35Pct40Pct45Pct50Pct55Pct60Pct65Pct70Pct75Pct80Pct85Pct90Pct95Pattern} PercentilesPattern
 *
 * Cohort objects with specific pattern capabilities
 * @typedef {{ name: string, title: string, color: Color, tree: PatternWithRealizedPrice }} CohortWithRealizedPrice
 * @typedef {{ name: string, title: string, color: Color, tree: PatternWithFullRealized }} CohortWithFullRealized
 * @typedef {{ name: string, title: string, color: Color, tree: PatternWithNupl }} CohortWithNupl
 * @typedef {{ name: string, title: string, color: Color, tree: PatternWithCostBasis }} CohortWithCostBasis
 * @typedef {{ name: string, title: string, color: Color, tree: PatternWithActivity }} CohortWithActivity
 * @typedef {{ name: string, title: string, color: Color, tree: PatternWithCostBasisPercentiles }} CohortWithCostBasisPercentiles
 *
 * Cohorts with nupl + percentiles (CohortFull and CohortLongTerm both have nupl and percentiles)
 * @typedef {CohortFull | CohortLongTerm} CohortWithNuplPercentiles
 * @typedef {{ name: string, title: string, list: readonly CohortWithNuplPercentiles[], all: CohortAll }} CohortGroupWithNuplPercentiles
 *
 * Cohorts with RealizedWithExtras (realizedCapRelToOwnMarketCap + realizedProfitToLossRatio)
 * @typedef {CohortAll | CohortFull | CohortWithPercentiles} CohortWithRealizedExtras
 *
 * Cohorts with circulating supply relative series (supplyRelToCirculatingSupply etc.)
 * These have GlobalRelativePattern or FullRelativePattern (same as RelativeWithMarketCap/RelativeWithNupl)
 * @typedef {CohortFull | CohortLongTerm | CohortWithAdjusted | CohortBasicWithMarketCap} UtxoCohortWithCirculatingSupplyRelative
 *
 * Address cohorts with circulating supply relative series (all address amount cohorts have these)
 * @typedef {AddrCohortObject} AddrCohortWithCirculatingSupplyRelative
 *
 * All cohorts with circulating supply relative series
 * @typedef {UtxoCohortWithCirculatingSupplyRelative | AddrCohortWithCirculatingSupplyRelative} CohortWithCirculatingSupplyRelative
 *
 * Delta patterns with absolute + rate rolling windows
 * @typedef {Brk.AbsoluteRatePattern} DeltaPattern
 * @typedef {Brk.AbsoluteRatePattern2} FiatDeltaPattern
 *
 * Investor price percentiles (pct1/2/5/95/98/99)
 * @typedef {Brk.Pct0Pct1Pct2Pct5Pct95Pct98Pct99Pattern} InvestorPercentilesPattern
 * @typedef {Brk.BpsPriceRatioPattern} InvestorPercentileEntry
 *
 * Generic tree node type for walking
 * @typedef {AnySeriesPattern | Record<string, unknown>} TreeNode
 */
