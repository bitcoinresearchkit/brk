/**
 * @import * as _ from "./modules/leeoniya-ufuzzy/1.0.19/dist/uFuzzy.d.ts"
 *
 * @import { IChartApi, ISeriesApi as _ISeriesApi, SeriesDefinition, SingleValueData as _SingleValueData, CandlestickData as _CandlestickData, BaselineData as _BaselineData, HistogramData as _HistogramData, SeriesType as LCSeriesType, IPaneApi, LineSeriesPartialOptions as _LineSeriesPartialOptions, HistogramSeriesPartialOptions as _HistogramSeriesPartialOptions, BaselineSeriesPartialOptions as _BaselineSeriesPartialOptions, CandlestickSeriesPartialOptions as _CandlestickSeriesPartialOptions, WhitespaceData, DeepPartial, ChartOptions, Time, LineData as _LineData, createChart as CreateChart, LineStyle } from './modules/lightweight-charts/5.0.9/dist/typings.js'
 *
 * @import { Signal, Signals, Accessor } from "./signals.js";
 *
 * @import { BrkClient, CatalogTree_Distribution_UtxoCohorts as UtxoCohortTree, CatalogTree_Distribution_AddressCohorts as AddressCohortTree, CatalogTree_Distribution_UtxoCohorts_All as AllUtxoPattern, CatalogTree_Distribution_UtxoCohorts_Term_Short as ShortTermPattern, CatalogTree_Distribution_UtxoCohorts_Term_Long as LongTermPattern, _10yPattern as MaxAgePattern, _10yTo12yPattern as AgeRangePattern, _0satsPattern2 as UtxoAmountPattern, _0satsPattern as AddressAmountPattern, _100btcPattern as BasicUtxoPattern, _0satsPattern2 as EpochPattern, Ratio1ySdPattern, Dollars, Price111dSmaPattern as EmaRatioPattern, Index, BlockCountPattern, SizePattern, FullnessPattern, FeeRatePattern, CoinbasePattern, ActivePriceRatioPattern, _0satsPattern, UnclaimedRewardsPattern as ValuePattern, Metric, MetricPattern, AnyMetricPattern, MetricEndpoint, MetricData, AnyMetricEndpoint, AnyMetricData, AddrCountPattern, CatalogTree_Blocks_Interval as IntervalPattern, _24hCoinbaseSumPattern as SupplyPattern, RelativePattern, RelativePattern2, RelativePattern5, CatalogTree_Distribution_UtxoCohorts_All_Relative as AllRelativePattern } from "./modules/brk-client/index.js"
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
 * @typedef {typeof import("./lazy")["default"]} Modules
 * @typedef {[number, number, number, number]} OHLCTuple
 *
 * @typedef {InstanceType<typeof BrkClient>["INDEXES"]} Indexes
 * @typedef {Indexes[number]} IndexName
 * @typedef {InstanceType<typeof BrkClient>["POOL_ID_TO_POOL_NAME"]} PoolIdToPoolName
 * @typedef {keyof PoolIdToPoolName} PoolId
 *
 * Pattern unions by cohort type
 * @typedef {AllUtxoPattern | AgeRangePattern | UtxoAmountPattern} UtxoCohortPattern
 * @typedef {AddressAmountPattern} AddressCohortPattern
 * @typedef {UtxoCohortPattern | AddressCohortPattern} CohortPattern
 *
 * Relative pattern capability types
 * @typedef {RelativePattern | RelativePattern5} RelativeWithMarketCap
 * @typedef {RelativePattern2 | RelativePattern5} RelativeWithOwnMarketCap
 * @typedef {RelativePattern2 | RelativePattern5 | AllRelativePattern} RelativeWithOwnPnl
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
 * Tree branch types
 * @typedef {InstanceType<typeof BrkClient>["tree"]["market"]} Market
 * @typedef {Market["movingAverage"]} MarketMovingAverage
 * @typedef {Market["dca"]} MarketDca
 *
 * Generic tree node type for walking
 * @typedef {AnyMetricPattern | Record<string, unknown>} TreeNode
 *
 * Chartable index IDs (subset of IndexName that can be charted)
 * @typedef {"height" | "dateindex" | "weekindex" | "monthindex" | "quarterindex" | "semesterindex" | "yearindex" | "decadeindex"} ChartableIndex
 */
