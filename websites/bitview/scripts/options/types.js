/**
 * @typedef {Object} BaseSeriesBlueprint
 * @property {string} title
 * @property {boolean} [defaultActive]
 *
 * @typedef {Object} BaselineSeriesBlueprintSpecific
 * @property {"Baseline"} type
 * @property {Color} [color]
 * @property {[Color, Color]} [colors]
 * @property {BaselineSeriesPartialOptions} [options]
 * @property {Accessor<BaselineData[]>} [data]
 * @typedef {BaseSeriesBlueprint & BaselineSeriesBlueprintSpecific} BaselineSeriesBlueprint
 *
 * @typedef {Object} CandlestickSeriesBlueprintSpecific
 * @property {"Candlestick"} type
 * @property {Color} [color]
 * @property {CandlestickSeriesPartialOptions} [options]
 * @property {Accessor<CandlestickData[]>} [data]
 * @typedef {BaseSeriesBlueprint & CandlestickSeriesBlueprintSpecific} CandlestickSeriesBlueprint
 *
 * @typedef {Object} LineSeriesBlueprintSpecific
 * @property {"Line"} [type]
 * @property {Color} [color]
 * @property {LineSeriesPartialOptions} [options]
 * @property {Accessor<LineData[]>} [data]
 * @typedef {BaseSeriesBlueprint & LineSeriesBlueprintSpecific} LineSeriesBlueprint
 *
 * @typedef {Object} HistogramSeriesBlueprintSpecific
 * @property {"Histogram"} type
 * @property {Color | [Color, Color]} [color] - Single color or [positive, negative] colors (defaults to green/red)
 * @property {HistogramSeriesPartialOptions} [options]
 * @property {Accessor<HistogramData[]>} [data]
 * @typedef {BaseSeriesBlueprint & HistogramSeriesBlueprintSpecific} HistogramSeriesBlueprint
 *
 * @typedef {BaselineSeriesBlueprint | CandlestickSeriesBlueprint | LineSeriesBlueprint | HistogramSeriesBlueprint} AnySeriesBlueprint
 *
 * @typedef {AnySeriesBlueprint["type"]} SeriesType
 *
 * @typedef {{ metric: AnyMetricPattern, unit?: Unit }} FetchedAnySeriesOptions
 *
 * @typedef {BaselineSeriesBlueprint & FetchedAnySeriesOptions} FetchedBaselineSeriesBlueprint
 * @typedef {CandlestickSeriesBlueprint & FetchedAnySeriesOptions} FetchedCandlestickSeriesBlueprint
 * @typedef {LineSeriesBlueprint & FetchedAnySeriesOptions} FetchedLineSeriesBlueprint
 * @typedef {HistogramSeriesBlueprint & FetchedAnySeriesOptions} FetchedHistogramSeriesBlueprint
 * @typedef {AnySeriesBlueprint & FetchedAnySeriesOptions} AnyFetchedSeriesBlueprint
 *
 * @typedef {Object} PartialOption
 * @property {string} name
 *
 * @typedef {Object} ProcessedOptionAddons
 * @property {string} title
 * @property {string[]} path
 *
 * @typedef {Object} PartialExplorerOptionSpecific
 * @property {"explorer"} kind
 * @property {string} title
 *
 * @typedef {PartialOption & PartialExplorerOptionSpecific} PartialExplorerOption
 *
 * @typedef {Required<PartialExplorerOption> & ProcessedOptionAddons} ExplorerOption
 *
 * @typedef {Object} PartialChartOptionSpecific
 * @property {"chart"} [kind]
 * @property {string} title
 * @property {AnyFetchedSeriesBlueprint[]} [top]
 * @property {AnyFetchedSeriesBlueprint[]} [bottom]
 *
 * @typedef {PartialOption & PartialChartOptionSpecific} PartialChartOption
 *
 * @typedef {Object} ProcessedChartOptionAddons
 * @property {Map<Unit, AnyFetchedSeriesBlueprint[]>} top
 * @property {Map<Unit, AnyFetchedSeriesBlueprint[]>} bottom
 *
 * @typedef {Required<Omit<PartialChartOption, "top" | "bottom">> & ProcessedChartOptionAddons & ProcessedOptionAddons} ChartOption
 *
 * @typedef {Object} PartialTableOptionSpecific
 * @property {"table"} kind
 * @property {string} title
 *
 * @typedef {PartialOption & PartialTableOptionSpecific} PartialTableOption
 *
 * @typedef {Required<PartialTableOption> & ProcessedOptionAddons} TableOption
 *
 * @typedef {Object} PartialSimulationOptionSpecific
 * @property {"simulation"} kind
 * @property {string} title
 *
 * @typedef {PartialOption & PartialSimulationOptionSpecific} PartialSimulationOption
 *
 * @typedef {Required<PartialSimulationOption> & ProcessedOptionAddons} SimulationOption
 *
 * @typedef {Object} PartialUrlOptionSpecific
 * @property {"url"} [kind]
 * @property {() => string} url
 * @property {string} title
 * @property {boolean} [qrcode]
 *
 * @typedef {PartialOption & PartialUrlOptionSpecific} PartialUrlOption
 *
 * @typedef {Required<PartialUrlOption> & ProcessedOptionAddons} UrlOption
 *
 * @typedef {PartialExplorerOption | PartialChartOption | PartialTableOption | PartialSimulationOption | PartialUrlOption} AnyPartialOption
 *
 * @typedef {ExplorerOption | ChartOption | TableOption | SimulationOption | UrlOption} Option
 *
 * @typedef {(AnyPartialOption | PartialOptionsGroup)[]} PartialOptionsTree
 *
 * @typedef {Object} PartialOptionsGroup
 * @property {string} name
 * @property {PartialOptionsTree} tree
 *
 * @typedef {Object} OptionsGroup
 * @property {string} name
 * @property {OptionsTree} tree
 *
 * @typedef {(Option | OptionsGroup)[]} OptionsTree
 *
 * @typedef {Object} UtxoCohortObject
 * @property {string} name
 * @property {string} title
 * @property {Color} color
 * @property {UtxoCohortPattern} tree
 *
 * Age cohorts (term, maxAge, minAge, ageRange, epoch) - have cost basis percentiles
 * @typedef {Object} AgeCohortObject
 * @property {string} name
 * @property {string} title
 * @property {Color} color
 * @property {PatternWithCostBasisPercentiles} tree
 *
 * Amount cohorts (geAmount, ltAmount, amountRange, type) - no cost basis percentiles
 * @typedef {Object} AmountCohortObject
 * @property {string} name
 * @property {string} title
 * @property {Color} color
 * @property {UtxoAmountPattern} tree
 *
 * @typedef {Object} AddressCohortObject
 * @property {string} name
 * @property {string} title
 * @property {Color} color
 * @property {AddressCohortPattern} tree
 *
 * @typedef {UtxoCohortObject | AddressCohortObject} CohortObject
 *
 * @typedef {Object} UtxoCohortGroupObject
 * @property {string} name
 * @property {string} title
 * @property {readonly UtxoCohortObject[]} list
 *
 * @typedef {Object} AgeCohortGroupObject
 * @property {string} name
 * @property {string} title
 * @property {readonly AgeCohortObject[]} list
 *
 * @typedef {Object} AmountCohortGroupObject
 * @property {string} name
 * @property {string} title
 * @property {readonly AmountCohortObject[]} list
 *
 * @typedef {Object} AddressCohortGroupObject
 * @property {string} name
 * @property {string} title
 * @property {readonly AddressCohortObject[]} list
 *
 * @typedef {UtxoCohortGroupObject | AddressCohortGroupObject} CohortGroupObject
 *
 * @typedef {Object} PartialContext
 * @property {Colors} colors
 * @property {BrkClient} brk
 * @property {(args: { metric: AnyMetricPattern, name: string, unit: Unit, color?: Color, defaultActive?: boolean, options?: LineSeriesPartialOptions }) => AnyFetchedSeriesBlueprint} s
 * @property {(pattern: BlockCountPattern<any>, title: string, color?: Color) => AnyFetchedSeriesBlueprint[]} fromBlockCount
 * @property {(pattern: FullnessPattern<any>, title: string, color?: Color) => AnyFetchedSeriesBlueprint[]} fromBitcoin
 * @property {(pattern: SizePattern<any>, title: string, color?: Color) => AnyFetchedSeriesBlueprint[]} fromBlockSize
 * @property {(pattern: SizePattern<any>, title: string, unit: Unit) => AnyFetchedSeriesBlueprint[]} fromSizePattern
 * @property {(pattern: FullnessPattern<any>, title: string, unit: Unit) => AnyFetchedSeriesBlueprint[]} fromFullnessPattern
 * @property {(pattern: FeeRatePattern<any>, title: string, unit: Unit) => AnyFetchedSeriesBlueprint[]} fromFeeRatePattern
 * @property {(pattern: CoinbasePattern, title: string) => AnyFetchedSeriesBlueprint[]} fromCoinbasePattern
 * @property {(pattern: ValuePattern, title: string, sumColor?: Color, cumulativeColor?: Color) => AnyFetchedSeriesBlueprint[]} fromValuePattern
 * @property {(pattern: BlockCountPattern<any>, title: string, unit: Unit, sumColor?: Color, cumulativeColor?: Color) => AnyFetchedSeriesBlueprint[]} fromBlockCountWithUnit
 * @property {(pattern: IntervalPattern, title: string, unit: Unit, color?: Color) => AnyFetchedSeriesBlueprint[]} fromIntervalPattern
 * @property {(args: { number?: number, name?: string, defaultActive?: boolean, lineStyle?: LineStyle, color?: Color, unit: Unit }) => FetchedLineSeriesBlueprint} createPriceLine
 * @property {(args: { numbers: number[], unit: Unit }) => FetchedLineSeriesBlueprint[]} createPriceLines
 * @property {(args: { constant: AnyMetricPattern, name: string, unit: Unit, color?: Color, lineStyle?: number, defaultActive?: boolean }) => FetchedLineSeriesBlueprint} line
 */

// Re-export for type consumers
export {};
