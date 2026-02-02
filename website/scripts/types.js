/**
 * @import { IChartApi, ISeriesApi as _ISeriesApi, SeriesDefinition, SingleValueData as _SingleValueData, CandlestickData as _CandlestickData, BaselineData as _BaselineData, HistogramData as _HistogramData, SeriesType as LCSeriesType, IPaneApi, LineSeriesPartialOptions as _LineSeriesPartialOptions, HistogramSeriesPartialOptions as _HistogramSeriesPartialOptions, BaselineSeriesPartialOptions as _BaselineSeriesPartialOptions, CandlestickSeriesPartialOptions as _CandlestickSeriesPartialOptions, WhitespaceData, DeepPartial, ChartOptions, Time, LineData as _LineData, createChart as CreateLCChart, LineStyle, createSeriesMarkers as CreateSeriesMarkers, SeriesMarker, ISeriesMarkersPluginApi } from './modules/lightweight-charts/5.1.0/dist/typings.js'
 *
 * @import * as Brk from "./modules/brk-client/index.js"
 * @import { BrkClient, Index, Metric, MetricData } from "./modules/brk-client/index.js"
 *
 * @import { Options } from './options/full.js'
 *
 * @import { PersistedValue } from './utils/persisted.js'
 *
 * @import { SingleValueData, CandlestickData, Series, AnySeries, ISeries, HistogramData, LineData, BaselineData, LineSeriesPartialOptions, BaselineSeriesPartialOptions, HistogramSeriesPartialOptions, CandlestickSeriesPartialOptions, Chart, Legend } from "./chart/index.js"
 *
 * @import { Color, ColorName, Colors } from "./chart/colors.js"
 *
 * @import { WebSockets } from "./utils/ws.js"
 *
 * @import { Option, PartialChartOption, ChartOption, AnyPartialOption, ProcessedOptionAddons, OptionsTree, SimulationOption, AnySeriesBlueprint, SeriesType, AnyFetchedSeriesBlueprint, TableOption, ExplorerOption, UrlOption, PartialOptionsGroup, OptionsGroup, PartialOptionsTree, UtxoCohortObject, AddressCohortObject, CohortObject, CohortGroupObject, FetchedLineSeriesBlueprint, FetchedBaselineSeriesBlueprint, FetchedHistogramSeriesBlueprint, PartialContext, PatternAll, PatternFull, PatternWithAdjusted, PatternWithPercentiles, PatternBasic, PatternBasicWithMarketCap, PatternBasicWithoutMarketCap, PatternWithoutRelative, CohortAll, CohortFull, CohortWithAdjusted, CohortWithPercentiles, CohortBasic, CohortBasicWithMarketCap, CohortBasicWithoutMarketCap, CohortWithoutRelative, CohortAddress, CohortLongTerm, CohortAgeRange, CohortMinAge, CohortGroupFull, CohortGroupWithAdjusted, CohortGroupWithPercentiles, CohortGroupLongTerm, CohortGroupAgeRange, CohortGroupBasic, CohortGroupBasicWithMarketCap, CohortGroupBasicWithoutMarketCap, CohortGroupWithoutRelative, CohortGroupMinAge, CohortGroupAddress, UtxoCohortGroupObject, AddressCohortGroupObject, FetchedDotsSeriesBlueprint, FetchedCandlestickSeriesBlueprint, FetchedPriceSeriesBlueprint, AnyPricePattern, AnyValuePattern } from "./options/partial.js"
 *
 *
 * @import { UnitObject as Unit } from "./utils/units.js"
 *
 * @import { ChartableIndexName } from "./utils/serde.js";
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
 * @typedef {Brk.MetricsTree_Distribution_UtxoCohorts} UtxoCohortTree
 * @typedef {Brk.MetricsTree_Distribution_AddressCohorts} AddressCohortTree
 * @typedef {Brk.MetricsTree_Distribution_UtxoCohorts_All} AllUtxoPattern
 * @typedef {Brk.MetricsTree_Distribution_UtxoCohorts_Term_Short} ShortTermPattern
 * @typedef {Brk.MetricsTree_Distribution_UtxoCohorts_Term_Long} LongTermPattern
 * @typedef {Brk.MetricsTree_Distribution_UtxoCohorts_All_Relative} AllRelativePattern
 * @typedef {Brk.MetricsTree_Supply_Circulating} SupplyPattern
 * @typedef {Brk.MetricsTree_Blocks_Size} BlockSizePattern
 * @typedef {keyof Brk.MetricsTree_Distribution_UtxoCohorts_Type} SpendableType
 * @typedef {keyof Brk.MetricsTree_Distribution_AnyAddressIndexes} AddressableType
 *
 * Brk pattern types (using new pattern names)
 * @typedef {Brk.ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5} MaxAgePattern
 * @typedef {Brk.ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern} AgeRangePattern
 * @typedef {Brk.ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} UtxoAmountPattern
 * @typedef {Brk.ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern} AddressAmountPattern
 * @typedef {Brk.ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4} BasicUtxoPattern
 * MinAgePattern: minAge cohorts have peakRegret in unrealized (Pattern6)
 * @typedef {Brk.ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6} MinAgePattern
 * @typedef {Brk.ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} EpochPattern
 * @typedef {Brk.ActivityCostOutputsRealizedSupplyUnrealizedPattern} EmptyPattern
 * @typedef {Brk._0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern} Ratio1ySdPattern
 * @typedef {Brk.Dollars} Dollars
 * CoinbasePattern: patterns with bitcoin/sats/dollars each having fullness + sum + cumulative
 * @typedef {Brk.BitcoinDollarsSatsPattern2} CoinbasePattern
 * ActivePriceRatioPattern: ratio pattern with price (extended)
 * @typedef {Brk.PriceRatioPattern} ActivePriceRatioPattern
 * AnyRatioPattern: full ratio patterns (with or without price) - has ratio, percentiles, z-scores
 * @typedef {Brk.RatioPattern | Brk.PriceRatioPattern} AnyRatioPattern
 * ValuePattern: patterns with minimal stats (sum, cumulative only) for bitcoin/sats/dollars
 * @typedef {Brk.BitcoinDollarsSatsPattern6 | Brk.BitcoinDollarsSatsPattern3} ValuePattern
 * FullValuePattern: patterns with full stats (base, sum, cumulative, average, percentiles) for bitcoin/sats/dollars
 * @typedef {Brk.BitcoinDollarsSatsPattern2} FullValuePattern
 * SumValuePattern: patterns with sum stats (sum, cumulative, average, percentiles - no base) for bitcoin/sats/dollars
 * @typedef {{bitcoin: SumStatsPattern<any>, sats: SumStatsPattern<any>, dollars: SumStatsPattern<any>}} SumValuePattern
 * AnyValuePatternType: union of all value pattern types
 * @typedef {ValuePattern | FullValuePattern} AnyValuePatternType
 * @typedef {Brk.AnyMetricPattern} AnyMetricPattern
 * @typedef {Brk.DollarsSatsPattern} ActivePricePattern
 * @typedef {Brk.AnyMetricEndpointBuilder} AnyMetricEndpoint
 * @typedef {Brk.AnyMetricData} AnyMetricData
 * @typedef {Brk.AllP2aP2pk33P2pk65P2pkhP2shP2trP2wpkhP2wshPattern} AddrCountPattern
 * Relative patterns by capability:
 * - BasicRelativePattern: minimal relative (investedCapitalIn*Pct, supplyIn*RelToOwnSupply only)
 * - GlobalRelativePattern: has RelToMarketCap metrics (netUnrealizedPnlRelToMarketCap, etc)
 * - OwnRelativePattern: has RelToOwnMarketCap metrics (netUnrealizedPnlRelToOwnMarketCap, etc)
 * - FullRelativePattern: has BOTH RelToMarketCap AND RelToOwnMarketCap
 * @typedef {Brk.InvestedSupplyPattern} BasicRelativePattern
 * @typedef {Brk.InvestedNegNetNuplSupplyUnrealizedPattern} GlobalRelativePattern
 * @typedef {Brk.InvestedNegNetSupplyUnrealizedPattern} OwnRelativePattern
 * @typedef {Brk.InvestedNegNetNuplSupplyUnrealizedPattern2} FullRelativePattern
 * @typedef {Brk.GreedInvestedInvestorNegNetPainSupplyTotalUnrealizedPattern} UnrealizedPattern
 * @typedef {Brk.GreedInvestedInvestorNegNetPainPeakSupplyTotalUnrealizedPattern} UnrealizedFullPattern
 *
 * Realized patterns
 * @typedef {Brk.CapCapitulationInvestorLossMvrvNegNetPeakProfitRealizedSellSoprTotalValuePattern} RealizedPattern
 * @typedef {Brk.CapCapitulationInvestorLossMvrvNegNetPeakProfitRealizedSellSoprTotalValuePattern2} RealizedPattern2
 * @typedef {Brk.AdjustedCapCapitulationInvestorLossMvrvNegNetPeakProfitRealizedSellSoprTotalValuePattern} RealizedPattern3
 * @typedef {Brk.AdjustedCapCapitulationInvestorLossMvrvNegNetPeakProfitRealizedSellSoprTotalValuePattern2} RealizedPattern4
 */

/**
 * @template T
 * @typedef {Brk.MetricEndpointBuilder<T>} MetricEndpoint
 */
/**
 * Stats pattern: average, min, max, percentiles (NO base)
 * @template T
 * @typedef {Brk.AverageMaxMedianMinPct10Pct25Pct75Pct90TxindexPattern<T>} StatsPattern
 */
/**
 * Base stats pattern: base, average, min, max, percentiles (NO sum/cumulative)
 * @template T
 * @typedef {Brk.AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern<T>} BaseStatsPattern
 */
/**
 * Full stats pattern: base, average, sum, cumulative, min, max, percentiles
 * @template T
 * @typedef {Brk.AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2<T>} FullStatsPattern
 */
/**
 * Sum stats pattern: average, sum, cumulative, percentiles (NO base)
 * @template T
 * @typedef {Brk.AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2<T>} SumStatsPattern
 */
/**
 * Count pattern: sum and cumulative only
 * @template T
 * @typedef {Brk.CumulativeSumPattern<T>} CountPattern
 */
/**
 * Any stats pattern union - patterns with sum/cumulative + percentiles
 * @typedef {SumStatsPattern<any> | FullStatsPattern<any> | BlockSizePattern} AnyStatsPattern
 */

/**
 *
 * @typedef {InstanceType<typeof BrkClient>["INDEXES"]} Indexes
 * @typedef {Indexes[number]} IndexName
 * @typedef {InstanceType<typeof BrkClient>["POOL_ID_TO_POOL_NAME"]} PoolIdToPoolName
 * @typedef {keyof PoolIdToPoolName} PoolId
 *
 * Tree branch types
 * @typedef {Brk.MetricsTree_Market} Market
 * @typedef {Brk.MetricsTree_Market_MovingAverage} MarketMovingAverage
 * @typedef {Brk.MetricsTree_Market_Dca} MarketDca
 * @typedef {Brk._10y2y3y4y5y6y8yPattern} PeriodCagrPattern
 * Full stats pattern union (both generic and non-generic variants)
 * @typedef {Brk.AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern | FullStatsPattern<any>} AnyFullStatsPattern
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
 * @typedef {GlobalRelativePattern | FullRelativePattern} RelativeWithMarketCap
 * @typedef {OwnRelativePattern | FullRelativePattern} RelativeWithOwnMarketCap
 * @typedef {OwnRelativePattern | FullRelativePattern | AllRelativePattern} RelativeWithOwnPnl
 * @typedef {GlobalRelativePattern | FullRelativePattern} RelativeWithNupl
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
 * @typedef {{ name: string, title: string, list: readonly CohortWithNuplPercentiles[] }} CohortGroupWithNuplPercentiles
 *
 * Cohorts with RealizedWithExtras (realizedCapRelToOwnMarketCap + realizedProfitToLossRatio)
 * @typedef {CohortAll | CohortFull | CohortWithPercentiles} CohortWithRealizedExtras
 *
 * Cohorts with circulating supply relative metrics (supplyRelToCirculatingSupply etc.)
 * These have GlobalRelativePattern or FullRelativePattern (same as RelativeWithMarketCap/RelativeWithNupl)
 * @typedef {CohortFull | CohortLongTerm | CohortWithAdjusted | CohortBasicWithMarketCap} UtxoCohortWithCirculatingSupplyRelative
 *
 * Address cohorts with circulating supply relative metrics (all address amount cohorts have these)
 * @typedef {AddressCohortObject} AddressCohortWithCirculatingSupplyRelative
 *
 * All cohorts with circulating supply relative metrics
 * @typedef {UtxoCohortWithCirculatingSupplyRelative | AddressCohortWithCirculatingSupplyRelative} CohortWithCirculatingSupplyRelative
 *
 * Generic tree node type for walking
 * @typedef {AnyMetricPattern | Record<string, unknown>} TreeNode
 *
 * Chartable index IDs (subset of IndexName that can be charted)
 * @typedef {"height" | "dateindex" | "weekindex" | "monthindex" | "quarterindex" | "semesterindex" | "yearindex" | "decadeindex"} ChartableIndex
 */
