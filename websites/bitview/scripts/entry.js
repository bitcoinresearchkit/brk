/**
 * @import * as _ from "./modules/leeoniya-ufuzzy/1.0.19/dist/uFuzzy.d.ts"
 *
 * @import { IChartApi, ISeriesApi as _ISeriesApi, SeriesDefinition, SingleValueData as _SingleValueData, CandlestickData as _CandlestickData, BaselineData as _BaselineData, HistogramData as _HistogramData, SeriesType as LCSeriesType, IPaneApi, LineSeriesPartialOptions as _LineSeriesPartialOptions, HistogramSeriesPartialOptions as _HistogramSeriesPartialOptions, BaselineSeriesPartialOptions as _BaselineSeriesPartialOptions, CandlestickSeriesPartialOptions as _CandlestickSeriesPartialOptions, WhitespaceData, DeepPartial, ChartOptions, Time, LineData as _LineData, createChart as CreateChart, LineStyle } from './modules/lightweight-charts/5.0.9/dist/typings.js'
 *
 * @import { Signal, Signals, Accessor } from "./signals.js";
 *
 * @import * as Brk from "./modules/brk-client/index.js"
 * @import { BrkClient, Index, Metric, MetricData } from "./modules/brk-client/index.js"
 *
 * @import { Resources, MetricResource } from './resources.js'
 *
 * @import { Valued, SingleValueData, CandlestickData, Series, ISeries, HistogramData, LineData, BaselineData, LineSeriesPartialOptions, BaselineSeriesPartialOptions, HistogramSeriesPartialOptions, CandlestickSeriesPartialOptions, CreateChartElement, Chart, Legend } from "./chart/index.js"
 *
 * @import { Color, ColorName, Colors } from "./utils/colors.js"
 *
 * @import { WebSockets } from "./utils/ws.js"
 *
 * @import { Option, PartialChartOption, ChartOption, AnyPartialOption, ProcessedOptionAddons, OptionsTree, SimulationOption, AnySeriesBlueprint, SeriesType, AnyFetchedSeriesBlueprint, TableOption, ExplorerOption, UrlOption, PartialOptionsGroup, OptionsGroup, PartialOptionsTree, UtxoCohortObject, AddressCohortObject, CohortObject, CohortGroupObject, FetchedLineSeriesBlueprint, FetchedBaselineSeriesBlueprint, FetchedHistogramSeriesBlueprint, PartialContext, PatternAll, PatternFull, PatternWithAdjusted, PatternWithPercentiles, PatternBasic, CohortAll, CohortFull, CohortWithAdjusted, CohortWithPercentiles, CohortBasic, CohortGroupFull, CohortGroupWithAdjusted, CohortGroupWithPercentiles, CohortGroupBasic, UtxoCohortGroupObject, AddressCohortGroupObject } from "./options/partial.js"
 *
 * @import { line as LineSeriesFn, baseline as BaselineSeriesFn, histogram as HistogramSeriesFn } from "./options/series.js"
 *
 * @import { UnitObject as Unit } from "./utils/units.js"
 *
 * @import { ChartableIndexName } from "./panes/chart/index.js";
 */

// import uFuzzy = require("./modules/leeoniya-ufuzzy/1.0.19/dist/uFuzzy.d.ts");

/**
 * @typedef {[number, number, number, number]} OHLCTuple
 *
 * Brk type aliases
 * @typedef {Brk.MetricsTree_Distribution_UtxoCohorts} UtxoCohortTree
 * @typedef {Brk.MetricsTree_Distribution_AddressCohorts} AddressCohortTree
 * @typedef {Brk.MetricsTree_Distribution_UtxoCohorts_All} AllUtxoPattern
 * @typedef {Brk.MetricsTree_Distribution_UtxoCohorts_Term_Short} ShortTermPattern
 * @typedef {Brk.MetricsTree_Distribution_UtxoCohorts_Term_Long} LongTermPattern
 * @typedef {Brk._10yPattern} MaxAgePattern
 * @typedef {Brk._10yTo12yPattern} AgeRangePattern
 * @typedef {Brk._0satsPattern2} UtxoAmountPattern
 * @typedef {Brk._0satsPattern} AddressAmountPattern
 * @typedef {Brk._100btcPattern} BasicUtxoPattern
 * @typedef {Brk._0satsPattern2} EpochPattern
 * @typedef {Brk.Ratio1ySdPattern} Ratio1ySdPattern
 * @typedef {Brk.Dollars} Dollars
 * @typedef {Brk.Price111dSmaPattern} EmaRatioPattern
 * @typedef {Brk.CoinbasePattern} CoinbasePattern
 * @typedef {Brk.ActivePriceRatioPattern} ActivePriceRatioPattern
 * @typedef {Brk.UnclaimedRewardsPattern} ValuePattern
 * @typedef {Brk.AnyMetricPattern} AnyMetricPattern
 * @typedef {Brk.AnyMetricEndpointBuilder} AnyMetricEndpoint
 * @typedef {Brk.AnyMetricData} AnyMetricData
 * @typedef {Brk.AddrCountPattern} AddrCountPattern
 * @typedef {Brk.MetricsTree_Blocks_Interval} IntervalPattern
 * @typedef {Brk.MetricsTree_Supply_Circulating} SupplyPattern
 * @typedef {Brk.RelativePattern} GlobalRelativePattern
 * @typedef {Brk.RelativePattern2} OwnRelativePattern
 * @typedef {Brk.RelativePattern5} FullRelativePattern
 * @typedef {Brk.MetricsTree_Distribution_UtxoCohorts_All_Relative} AllRelativePattern
 */

/**
 * @template T
 * @typedef {Brk.BlockCountPattern<T>} BlockCountPattern
 */
/**
 * @template T
 * @typedef {Brk.FullnessPattern<T>} FullnessPattern
 */
/**
 * @template T
 * @typedef {Brk.FeeRatePattern<T>} FeeRatePattern
 */
/**
 * @template T
 * @typedef {Brk.MetricEndpointBuilder<T>} MetricEndpoint
 */
/**
 * @template T
 * @typedef {Brk.DollarsPattern<T>} SizePattern
 */
/**
 * @template T
 * @typedef {Brk.CountPattern2<T>} CountStatsPattern
 */
/**
 * @typedef {Brk.MetricsTree_Blocks_Size} BlockSizePattern
 */
/**
 * Stats pattern union - accepts both CountStatsPattern and BlockSizePattern
 * @typedef {CountStatsPattern<any> | BlockSizePattern} AnyStatsPattern
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
 *
 * Capability-based pattern groupings (patterns that have specific properties)
 * @typedef {AllUtxoPattern | AgeRangePattern | UtxoAmountPattern} PatternWithRealizedPrice
 * @typedef {AllUtxoPattern} PatternWithFullRealized
 * @typedef {AllUtxoPattern | AgeRangePattern | UtxoAmountPattern} PatternWithNupl
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
 * Generic tree node type for walking
 * @typedef {AnyMetricPattern | Record<string, unknown>} TreeNode
 *
 * Chartable index IDs (subset of IndexName that can be charted)
 * @typedef {"height" | "dateindex" | "weekindex" | "monthindex" | "quarterindex" | "semesterindex" | "yearindex" | "decadeindex"} ChartableIndex
 */
