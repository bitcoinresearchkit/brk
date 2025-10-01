/**
 * @import * as _ from "./modules/leeoniya-ufuzzy/1.0.19/dist/uFuzzy.d.ts"
 *
 * @import { Signal, Signals, Accessor } from "./modules/brk-signals/index";
 *
 * @import { BRK } from "./modules/brk-client/index.js"
 * @import { Metric, MetricToIndexes } from "./modules/brk-client/metrics"
 * @import { IndexName } from "./modules/brk-client/generated/metrics"
 * @import { PoolId, PoolIdToPoolName } from "./modules/brk-client/generated/pools"
 *
 * @import { Resources, MetricResource } from './modules/brk-resources/index.js'
 *
 * @import { Valued, SingleValueData, CandlestickData, Series, ISeries, HistogramData, LineData, BaselineData, LineSeriesPartialOptions, BaselineSeriesPartialOptions, HistogramSeriesPartialOptions, CandlestickSeriesPartialOptions, CreateChartElement, Chart } from "./core/chart/index"
 *
 * @import { Color, ColorName, Colors } from "./core/colors"
 *
 * @import { WebSockets } from "./core/ws"
 *
 * @import { Option, PartialChartOption, ChartOption, AnyPartialOption, ProcessedOptionAddons, OptionsTree, SimulationOption, AnySeriesBlueprint, SeriesType, AnyFetchedSeriesBlueprint, TableOption, ExplorerOption, UrlOption, PartialOptionsGroup, OptionsGroup, PartialOptionsTree } from "./core/options/partial"
 *
 * @import { Unit } from "./core/serde"
 *
 * @import { ChartableIndexName } from "./panes/chart/index.js";
 */

// import uFuzzy = require("./modules/leeoniya-ufuzzy/1.0.19/dist/uFuzzy.d.ts");

/**
 * @typedef {typeof import("./lazy")["default"]} Modules
 * @typedef {[number, number, number, number]} OHLCTuple
 */

// DO NOT CHANGE, Exact format is expected in `brk_bundler`
// @ts-ignore
import("./main.js");
