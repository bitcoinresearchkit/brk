/**
 * @import { IChartApi, ISeriesApi as _ISeriesApi, SeriesDefinition, SingleValueData as _SingleValueData, CandlestickData as _CandlestickData, BaselineData as _BaselineData, HistogramData as _HistogramData, SeriesType as LCSeriesType, IPaneApi, LineSeriesPartialOptions as _LineSeriesPartialOptions, HistogramSeriesPartialOptions as _HistogramSeriesPartialOptions, BaselineSeriesPartialOptions as _BaselineSeriesPartialOptions, CandlestickSeriesPartialOptions as _CandlestickSeriesPartialOptions, WhitespaceData, DeepPartial, ChartOptions, Time, LineData as _LineData, createChart as CreateLCChart, LineStyle, createSeriesMarkers as CreateSeriesMarkers, SeriesMarker, ISeriesMarkersPluginApi } from './modules/lightweight-charts/5.1.0/dist/typings.js'
 *
 * @import * as Brk from "./modules/brk-client/index.js"
 * @import { BrkClient, Index, Series as BrkSeries, SeriesData } from "./modules/brk-client/index.js"
 *
 * @import { Options } from './options/full.js'
 *
 * @import { PersistedValue } from './utils/persisted.js'
 *
 * @import { SingleValueData, CandlestickData, Series, AnySeries, ISeries, HistogramData, LineData, BaselineData, LineSeriesPartialOptions, BaselineSeriesPartialOptions, HistogramSeriesPartialOptions, CandlestickSeriesPartialOptions, Chart, Legend } from "./chart/index.js"
 *
 * @import { Color } from "./utils/colors.js"
 *
 * @import { Option, PartialChartOption, ChartOption, AnyPartialOption, ProcessedOptionAddons, OptionsTree, SimulationOption, AnySeriesBlueprint, SeriesType, AnyFetchedSeriesBlueprint, TableOption, ExplorerOption, UrlOption, PartialOptionsGroup, OptionsGroup, PartialOptionsTree, UtxoCohortObject, AddressCohortObject, CohortObject, CohortGroupObject, FetchedLineSeriesBlueprint, FetchedBaselineSeriesBlueprint, FetchedHistogramSeriesBlueprint, FetchedDotsBaselineSeriesBlueprint, PatternAll, PatternFull, PatternWithAdjusted, PatternWithPercentiles, PatternBasic, PatternBasicWithMarketCap, PatternBasicWithoutMarketCap, PatternWithoutRelative, CohortAll, CohortFull, CohortWithAdjusted, CohortWithPercentiles, CohortBasic, CohortBasicWithMarketCap, CohortBasicWithoutMarketCap, CohortWithoutRelative, CohortAddress, CohortLongTerm, CohortAgeRange, CohortAgeRangeWithMatured, CohortGroupFull, CohortGroupWithAdjusted, CohortGroupWithPercentiles, CohortGroupLongTerm, CohortGroupAgeRange, CohortGroupBasic, CohortGroupBasicWithMarketCap, CohortGroupBasicWithoutMarketCap, CohortGroupWithoutRelative, CohortGroupAddress, UtxoCohortGroupObject, AddressCohortGroupObject, FetchedDotsSeriesBlueprint, FetchedCandlestickSeriesBlueprint, FetchedPriceSeriesBlueprint, AnyPricePattern, AnyValuePattern } from "./options/partial.js"
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
 * @typedef {Brk.SeriesTree_Cohorts_Address} AddressCohortTree
 * @typedef {Brk.SeriesTree_Cohorts_Utxo_All} AllUtxoPattern
 * @typedef {Brk.SeriesTree_Cohorts_Utxo_Sth} ShortTermPattern
 * @typedef {Brk.SeriesTree_Cohorts_Utxo_Lth} LongTermPattern
 * @typedef {Brk.SeriesTree_Cohorts_Utxo_All_Unrealized} AllRelativePattern
 * @typedef {keyof Brk.BtcCentsSatsUsdPattern} BtcSatsUsdKey
 * @typedef {Brk.BtcCentsSatsUsdPattern} SupplyPattern
 * @typedef {Brk.AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern} BlockSizePattern
 * @typedef {keyof Brk.SeriesTree_Cohorts_Utxo_Type} SpendableType
 * @typedef {keyof Brk.SeriesTree_Addresses_Raw} AddressableType
 *
 * Brk pattern types (using new pattern names)
 * @typedef {Brk.ActivityOutputsRealizedSupplyUnrealizedPattern} MaxAgePattern
 * @typedef {Brk.ActivityOutputsRealizedSupplyUnrealizedPattern} AgeRangePattern
 * @typedef {Brk.OutputsRealizedSupplyUnrealizedPattern} UtxoAmountPattern
 * @typedef {Brk.AddressOutputsRealizedSupplyUnrealizedPattern} AddressAmountPattern
 * @typedef {Brk.ActivityOutputsRealizedSupplyUnrealizedPattern} BasicUtxoPattern
 * @typedef {Brk.ActivityOutputsRealizedSupplyUnrealizedPattern} EpochPattern
 * @typedef {Brk.OutputsRealizedSupplyUnrealizedPattern2} EmptyPattern
 * @typedef {Brk._0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdZscorePattern} Ratio1ySdPattern
 * @typedef {Brk.Dollars} Dollars
 * CoinbasePattern: base + cumulative + rolling windows (flattened)
 * @typedef {Brk.BaseCumulativeSumPattern4} CoinbasePattern
 * ActivePriceRatioPattern: ratio pattern with price (extended)
 * @typedef {Brk.BpsPriceRatioPattern} ActivePriceRatioPattern
 * PriceRatioPercentilesPattern: price pattern with ratio + percentiles (no SMAs/stdDev)
 * @typedef {Brk.BpsCentsPercentilesRatioSatsUsdPattern} PriceRatioPercentilesPattern
 * AnyRatioPattern: full ratio pattern with percentiles, SMAs, and std dev bands
 * @typedef {Brk.BpsCentsPercentilesRatioSatsSmaStdUsdPattern} AnyRatioPattern
 * ValuePattern: patterns with base + cumulative (no rolling)
 * @typedef {Brk.BaseCumulativeSumPattern<number> | Brk.BaseCumulativeRelPattern} ValuePattern
 * FullValuePattern: base + cumulative + rolling windows (flattened)
 * @typedef {Brk.BaseCumulativeSumPattern4} FullValuePattern
 * RollingWindowSlot: a single rolling window with stats (average, pct10, pct25, median, pct75, pct90, max, min) per unit
 * @typedef {Brk.AverageMaxMedianMinPct10Pct25Pct75Pct90Pattern} RollingWindowSlot
 * AnyValuePatternType: union of all value pattern types
 * @typedef {Brk.BaseCumulativeSumPattern4 | Brk.BaseCumulativeSumPattern<number> | Brk.BaseCumulativeRelPattern} AnyValuePatternType
 * @typedef {Brk.AnySeriesPattern} AnySeriesPattern
 * @typedef {Brk.CentsSatsUsdPattern} ActivePricePattern
 * @typedef {Brk.AnySeriesEndpointBuilder} AnySeriesEndpoint
 * @typedef {Brk.AnySeriesData} AnySeriesData
 * @typedef {Brk.AllP2aP2pk33P2pk65P2pkhP2shP2trP2wpkhP2wshPattern3} AddrCountPattern
 * Relative patterns by capability:
 * - BasicRelativePattern: minimal relative (investedCapitalIn*Pct, supplyIn*RelToOwnSupply only)
 * - GlobalRelativePattern: has RelToMarketCap series (netUnrealizedPnlRelToMarketCap, etc)
 * - OwnRelativePattern: has RelToOwnMarketCap series (netUnrealizedPnlRelToOwnMarketCap, etc)
 * - FullRelativePattern: has BOTH RelToMarketCap AND RelToOwnMarketCap
 * @typedef {Brk.LossNetNuplProfitPattern} BasicRelativePattern
 * @typedef {Brk.LossNetNuplProfitPattern} GlobalRelativePattern
 * @typedef {Brk.GrossInvestedLossNetNuplProfitSentimentPattern2} OwnRelativePattern
 * @typedef {Brk.GrossInvestedLossNetNuplProfitSentimentPattern2} FullRelativePattern
 * @typedef {Brk.GrossInvestedLossNetNuplProfitSentimentPattern2} UnrealizedPattern
 *
 * Profitability bucket pattern (supply + realized_cap + nupl)
 * @typedef {Brk.NuplRealizedSupplyPattern} RealizedSupplyPattern
 *
 * Realized patterns
 * @typedef {Brk.CapGrossInvestorLossMvrvNetPeakPriceProfitSellSoprPattern} RealizedPattern
 * @typedef {Brk.CapGrossInvestorLossMvrvNetPeakPriceProfitSellSoprPattern} RealizedPattern2
 * @typedef {Brk.CapGrossInvestorLossMvrvNetPeakPriceProfitSellSoprPattern} RealizedPattern3
 * @typedef {Brk.CapGrossInvestorLossMvrvNetPeakPriceProfitSellSoprPattern} RealizedPattern4
 */

/**
 * @template T
 * @typedef {Brk.SeriesEndpointBuilder<T>} SeriesEndpoint
 */
/**
 * Stats pattern: average, min, max, percentiles (height-only indexes, NO base)
 * @typedef {Brk.AverageMaxMedianMinPct10Pct25Pct75Pct90Pattern} StatsPattern
 */
/**
 * Base stats pattern: average, min, max, percentiles
 * @typedef {Brk.AverageMaxMedianMinPct10Pct25Pct75Pct90Pattern} BaseStatsPattern
 */
/**
 * Full stats pattern: cumulative, sum, average, min, max, percentiles + rolling
 * @typedef {Brk.AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90RollingSumPattern} FullStatsPattern
 */
/**
 * Sum stats pattern: cumulative, sum, average, min, max, percentiles + rolling (same as FullStatsPattern)
 * @typedef {Brk.AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90RollingSumPattern} SumStatsPattern
 */
/**
 * Full stats pattern for Bitcoin (non-generic variant) - same as FullStatsPattern
 * @typedef {Brk.AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90RollingSumPattern} BtcFullStatsPattern
 */
/**
 * Count pattern: height, cumulative, and rolling sum windows
 * @template T
 * @typedef {Brk.BaseCumulativeSumPattern<T>} CountPattern
 */
/**
 * Full per-block pattern: height, cumulative, sum, and distribution stats (all flat)
 * @typedef {Brk.AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern} FullPerBlockPattern
 */
/**
 * Any stats pattern union - patterns with sum/cumulative + percentiles
 * @typedef {FullStatsPattern | BtcFullStatsPattern} AnyStatsPattern
 */
/**
 * Distribution stats: 8 series fields (average, min, max, median, pct10/25/75/90)
 * @typedef {{ average: AnySeriesPattern, min: AnySeriesPattern, max: AnySeriesPattern, median: AnySeriesPattern, pct10: AnySeriesPattern, pct25: AnySeriesPattern, pct75: AnySeriesPattern, pct90: AnySeriesPattern }} DistributionStats
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
 * @typedef {Brk.SeriesTree_Market_Dca} MarketDca
 * @typedef {Brk._10y2y3y4y5y6y8yPattern} PeriodCagrPattern
 * Full stats pattern union (both generic and non-generic variants)
 * @typedef {FullStatsPattern | BtcFullStatsPattern} AnyFullStatsPattern
 *
 * DCA period keys - derived from pattern types
 * @typedef {keyof Brk._10y2y3y4y5y6y8yPattern} LongPeriodKey
 * @typedef {"_1w" | "_1m" | "_3m" | "_6m" | "_1y"} ShortPeriodKey
 * @typedef {ShortPeriodKey | LongPeriodKey} AllPeriodKey
 *
 * Pattern unions by cohort type
 * @typedef {AllUtxoPattern | AgeRangePattern | UtxoAmountPattern} UtxoCohortPattern
 * @typedef {AddressAmountPattern} AddressCohortPattern
 * @typedef {UtxoCohortPattern | AddressCohortPattern} CohortPattern
 *
 * Relative pattern capability types
 * @typedef {GlobalRelativePattern | FullRelativePattern | AllRelativePattern} RelativeWithMarketCap
 * @typedef {OwnRelativePattern | FullRelativePattern} RelativeWithOwnMarketCap
 * @typedef {OwnRelativePattern | FullRelativePattern | AllRelativePattern} RelativeWithOwnPnl
 * @typedef {GlobalRelativePattern | FullRelativePattern | AllRelativePattern} RelativeWithNupl
 * @typedef {BasicRelativePattern | GlobalRelativePattern | OwnRelativePattern | FullRelativePattern | AllRelativePattern} RelativeWithInvestedCapitalPct
 *
 * Realized pattern capability types
 * RealizedWithExtras: patterns with realizedCapRelToOwnMarketCap + realizedProfitToLossRatio
 * @typedef {RealizedPattern2 | RealizedPattern3} RealizedWithExtras
 *
 * Any realized pattern (all have sellSideRiskRatio, valueCreated, valueDestroyed, etc.)
 * @typedef {RealizedPattern | RealizedPattern2 | RealizedPattern3 | RealizedPattern4} AnyRealizedPattern
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
 * @typedef {AddressCohortObject} AddressCohortWithCirculatingSupplyRelative
 *
 * All cohorts with circulating supply relative series
 * @typedef {UtxoCohortWithCirculatingSupplyRelative | AddressCohortWithCirculatingSupplyRelative} CohortWithCirculatingSupplyRelative
 *
 * Delta patterns with absolute + rate rolling windows
 * @typedef {Brk.AbsoluteRatePattern} DeltaPattern
 * @typedef {Brk.AbsoluteRatePattern2} FiatDeltaPattern
 *
 * Investor price percentiles (pct1/2/5/95/98/99)
 * @typedef {Brk.Pct1Pct2Pct5Pct95Pct98Pct99Pattern} InvestorPercentilesPattern
 * @typedef {Brk.BpsPriceRatioPattern} InvestorPercentileEntry
 *
 * Generic tree node type for walking
 * @typedef {AnySeriesPattern | Record<string, unknown>} TreeNode
 *
 */
