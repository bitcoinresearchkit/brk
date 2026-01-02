/**
 * @import * as _ from "./modules/leeoniya-ufuzzy/1.0.19/dist/uFuzzy.d.ts"
 *
 * @import { Signal, Signals, Accessor } from "./signals";
 *
 * @import { BrkClient, CatalogTree_Computed_Distribution_UtxoCohorts as UtxoCohortTree, CatalogTree_Computed_Distribution_AddressCohorts as AddressCohortTree, CatalogTree_Computed_Distribution_UtxoCohorts_All as AllUtxoPattern, UpTo1dPattern as MaxAgePattern, _10yTo12yPattern as MinAgePattern, _0satsPattern2 as UtxoAmountPattern, _0satsPattern as AddressAmountPattern, Ratio1ySdPattern, Dollars, Price111dSmaPattern as EmaRatioPattern, Index, BlockCountPattern, BitcoinPattern, BlockSizePattern, BlockIntervalPattern, CoinbasePattern, Constant0Pattern, ActivePriceRatioPattern, _0satsPattern, PricePaidPattern2, UnclaimedRewardsPattern as ValuePattern, SentPattern as RewardPattern, Metric } from "./modules/brk-client/index.js"
 *
 * @import { Resources, MetricResource } from './resources'
 *
 * @import { Valued, SingleValueData, CandlestickData, Series, ISeries, HistogramData, LineData, BaselineData, LineSeriesPartialOptions, BaselineSeriesPartialOptions, HistogramSeriesPartialOptions, CandlestickSeriesPartialOptions, CreateChartElement, Chart } from "./chart/index"
 *
 * @import { Color, ColorName, Colors } from "./utils/colors"
 *
 * @import { WebSockets } from "./utils/ws"
 *
 * @import { Option, PartialChartOption, ChartOption, AnyPartialOption, ProcessedOptionAddons, OptionsTree, SimulationOption, AnySeriesBlueprint, SeriesType, AnyFetchedSeriesBlueprint, TableOption, ExplorerOption, UrlOption, PartialOptionsGroup, OptionsGroup, PartialOptionsTree, UtxoCohortObject, AddressCohortObject, CohortObject, UtxoCohortGroupObject, AddressCohortGroupObject, CohortGroupObject, MetricAccessor, FetchedLineSeriesBlueprint, PartialContext, AgeCohortObject, AmountCohortObject, AgeCohortGroupObject, AmountCohortGroupObject } from "./options/partial/index.js"
 *
 * @import { Unit } from "./utils/serde"
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
 * @typedef {AllUtxoPattern | MaxAgePattern | MinAgePattern | UtxoAmountPattern} UtxoCohortPattern
 * @typedef {AddressAmountPattern} AddressCohortPattern
 * @typedef {UtxoCohortPattern | AddressCohortPattern} CohortPattern
 *
 * Capability-based pattern groupings (patterns that have specific properties)
 * @typedef {AllUtxoPattern | MinAgePattern | UtxoAmountPattern} PatternWithRealizedPrice
 * @typedef {AllUtxoPattern} PatternWithFullRealized
 * @typedef {AllUtxoPattern | MinAgePattern | UtxoAmountPattern} PatternWithNupl
 * @typedef {AllUtxoPattern | MinAgePattern | UtxoAmountPattern} PatternWithPricePaidStats
 * @typedef {AllUtxoPattern | MinAgePattern | UtxoAmountPattern} PatternWithActivity
 * @typedef {AllUtxoPattern | MaxAgePattern | MinAgePattern} PatternWithPricePercentiles
 *
 * Cohort objects with specific pattern capabilities
 * @typedef {{ name: string, title: string, color: Color, tree: PatternWithRealizedPrice }} CohortWithRealizedPrice
 * @typedef {{ name: string, title: string, color: Color, tree: PatternWithFullRealized }} CohortWithFullRealized
 * @typedef {{ name: string, title: string, color: Color, tree: PatternWithNupl }} CohortWithNupl
 * @typedef {{ name: string, title: string, color: Color, tree: PatternWithPricePaidStats }} CohortWithPricePaidStats
 * @typedef {{ name: string, title: string, color: Color, tree: PatternWithActivity }} CohortWithActivity
 * @typedef {{ name: string, title: string, color: Color, tree: PatternWithPricePercentiles }} CohortWithPricePercentiles
 *
 * Tree branch types
 * @typedef {InstanceType<typeof BrkClient>["tree"]["computed"]["market"]} Market
 * @typedef {Market["movingAverage"]} MarketMovingAverage
 * @typedef {Market["dca"]} MarketDca
 *
 * Generic tree node type for walking
 * @typedef {MetricAccessor<unknown> | Record<string, unknown>} TreeNode
 */

// DO NOT CHANGE, Exact format is expected in `brk_bundler`
// @ts-ignore
import("./main.js");
