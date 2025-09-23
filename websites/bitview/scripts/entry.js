/**
 *
 * @import { Valued,  SingleValueData, CandlestickData, OHLCTuple, Series, ISeries, HistogramData, LineData, BaselineData, LineSeriesPartialOptions, BaselineSeriesPartialOptions, HistogramSeriesPartialOptions, CandlestickSeriesPartialOptions, CreateChartElement, Chart } from "./core/chart"
 *
 * @import * as _ from "./packages/leeoniya-ufuzzy/1.0.19/dist/uFuzzy.d.ts"
 *
 * @import { SerializedChartableIndex } from "./panes/chart";
 *
 * @import { Signal, Signals, Accessor } from "./packages/solidjs-signals/wrapper";
 *
 * @import { DateIndex, DecadeIndex, DifficultyEpoch, Index, HalvingEpoch, Height, MonthIndex, P2PK33AddressIndex, P2PK65AddressIndex, P2PKHAddressIndex, P2SHAddressIndex, P2MSOutputIndex, P2AAddressIndex, P2TRAddressIndex, P2WPKHAddressIndex, P2WSHAddressIndex, TxIndex, InputIndex, OutputIndex, WeekIndex, SemesterIndex, YearIndex, MetricToIndexes, QuarterIndex, EmptyOutputIndex, OpReturnIndex, UnknownOutputIndex, EmptyAddressIndex, LoadedAddressIndex } from "./bridge/vecs"
 *
 * @import { Pools, Pool } from "./bridge/pools"
 *
 * @import { Color, ColorName, Colors } from "./core/colors"
 *
 * @import { Option, PartialChartOption, ChartOption, AnyPartialOption, ProcessedOptionAddons, OptionsTree, SimulationOption, AnySeriesBlueprint, SeriesType, AnyFetchedSeriesBlueprint, TableOption, ExplorerOption, UrlOption, PartialOptionsGroup, OptionsGroup, PartialOptionsTree } from "./core/options/partial"
 *
 * @import { WebSockets } from "./core/ws"
 *
 * @import { Unit } from "./core/serde"
 */

/**
 * @typedef {typeof import("./lazy")["default"]} Packages
 * @typedef {typeof import("./core/utils")} Utilities
 * @typedef {typeof import("./core/env")["default"]} Env
 * @typedef {typeof import("./core/elements")["default"]} Elements
 * @typedef {string} Metric
 */

// DO NOT CHANGE, Exact format is expected in `brk_bundler`
// @ts-ignore
import("./main.js");
