/**
 * @typedef {Object} BaseSeriesBlueprint
 * @property {string} title
 * @property {string} [key] - Optional key for persistence (derived from title if not provided)
 * @property {boolean} [defaultActive]
 *
 * @typedef {Object} BaselineSeriesBlueprintSpecific
 * @property {"Baseline"} type
 * @property {Color} [color]
 * @property {[Color, Color]} [colors]
 * @property {BaselineSeriesPartialOptions} [options]
 * @typedef {BaseSeriesBlueprint & BaselineSeriesBlueprintSpecific} BaselineSeriesBlueprint
 *
 * @typedef {Object} CandlestickSeriesBlueprintSpecific
 * @property {"Candlestick"} type
 * @property {[Color, Color]} [colors]
 * @property {CandlestickSeriesPartialOptions} [options]
 * @typedef {BaseSeriesBlueprint & CandlestickSeriesBlueprintSpecific} CandlestickSeriesBlueprint
 *
 * @typedef {Object} LineSeriesBlueprintSpecific
 * @property {"Line"} [type]
 * @property {Color} [color]
 * @property {LineSeriesPartialOptions} [options]
 * @typedef {BaseSeriesBlueprint & LineSeriesBlueprintSpecific} LineSeriesBlueprint
 *
 * @typedef {Object} HistogramSeriesBlueprintSpecific
 * @property {"Histogram"} type
 * @property {Color | [Color, Color]} [color] - Single color or [positive, negative] colors (defaults to green/red)
 * @property {HistogramSeriesPartialOptions} [options]
 * @typedef {BaseSeriesBlueprint & HistogramSeriesBlueprintSpecific} HistogramSeriesBlueprint
 *
 * @typedef {Object} DotsSeriesBlueprintSpecific
 * @property {"Dots"} type
 * @property {Color} [color]
 * @property {LineSeriesPartialOptions} [options]
 * @typedef {BaseSeriesBlueprint & DotsSeriesBlueprintSpecific} DotsSeriesBlueprint
 *
 * @typedef {BaselineSeriesBlueprint | CandlestickSeriesBlueprint | LineSeriesBlueprint | HistogramSeriesBlueprint | DotsSeriesBlueprint} AnySeriesBlueprint
 *
 * @typedef {AnySeriesBlueprint["type"]} SeriesType
 *
 * @typedef {{ metric: AnyMetricPattern, unit?: Unit }} FetchedAnySeriesOptions
 *
 * @typedef {BaselineSeriesBlueprint & FetchedAnySeriesOptions} FetchedBaselineSeriesBlueprint
 * @typedef {CandlestickSeriesBlueprint & FetchedAnySeriesOptions} FetchedCandlestickSeriesBlueprint
 * @typedef {LineSeriesBlueprint & FetchedAnySeriesOptions} FetchedLineSeriesBlueprint
 * @typedef {HistogramSeriesBlueprint & FetchedAnySeriesOptions} FetchedHistogramSeriesBlueprint
 * @typedef {DotsSeriesBlueprint & FetchedAnySeriesOptions} FetchedDotsSeriesBlueprint
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
 * @property {"link"} [kind]
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
 * ============================================================================
 * UTXO Cohort Pattern Types (based on brk client patterns)
 * ============================================================================
 *
 * Patterns with adjustedSopr + percentiles + RelToMarketCap:
 *   - ShortTermPattern (term.short)
 * @typedef {ShortTermPattern} PatternFull
 *
 * The "All" pattern is special - has adjustedSopr + percentiles but NO RelToMarketCap
 * @typedef {AllUtxoPattern} PatternAll
 *
 * Patterns with adjustedSopr only (RealizedPattern4, CostBasisPattern):
 *   - MaxAgePattern (maxAge.*)
 * @typedef {MaxAgePattern} PatternWithAdjusted
 *
 * Patterns with percentiles only (RealizedPattern2, CostBasisPattern2):
 *   - LongTermPattern (term.long)
 *   - AgeRangePattern (ageRange.*)
 * @typedef {LongTermPattern | AgeRangePattern} PatternWithPercentiles
 *
 * Patterns with RelToMarketCap in relative (RelativePattern):
 *   - BasicUtxoPattern (minAge.*, geAmount.*, ltAmount.*)
 * @typedef {BasicUtxoPattern} PatternBasicWithMarketCap
 *
 * Patterns without RelToMarketCap in relative (RelativePattern4):
 *   - EpochPattern (epoch.*, amountRange.*, year.*, type.*)
 * @typedef {EpochPattern} PatternBasicWithoutMarketCap
 *
 * Union of basic patterns (for backwards compat)
 * @typedef {PatternBasicWithMarketCap | PatternBasicWithoutMarketCap} PatternBasic
 *
 * ============================================================================
 * Cohort Object Types (by capability)
 * ============================================================================
 *
 * All cohort: adjustedSopr + percentiles but NO RelToMarketCap (special)
 * @typedef {Object} CohortAll
 * @property {string} name
 * @property {string} title
 * @property {Color} color
 * @property {PatternAll} tree
 * @property {Brk.MetricPattern1<Brk.StoredU64>} addrCount
 *
 * Full cohort: adjustedSopr + percentiles + RelToMarketCap (term.short)
 * @typedef {Object} CohortFull
 * @property {string} name
 * @property {string} title
 * @property {Color} color
 * @property {PatternFull} tree
 *
 * Cohort with adjustedSopr only (maxAge.*)
 * @typedef {Object} CohortWithAdjusted
 * @property {string} name
 * @property {string} title
 * @property {Color} color
 * @property {PatternWithAdjusted} tree
 *
 * Cohort with percentiles only (term.long, ageRange.*)
 * @typedef {Object} CohortWithPercentiles
 * @property {string} name
 * @property {string} title
 * @property {Color} color
 * @property {PatternWithPercentiles} tree
 *
 * Long term cohort (term.long) - has nupl via RelativePattern5
 * @typedef {Object} CohortLongTerm
 * @property {string} name
 * @property {string} title
 * @property {Color} color
 * @property {LongTermPattern} tree
 *
 * Age range cohort (ageRange.*) - no nupl via RelativePattern2
 * @typedef {Object} CohortAgeRange
 * @property {string} name
 * @property {string} title
 * @property {Color} color
 * @property {AgeRangePattern} tree
 *
 * Basic cohort WITH RelToMarketCap (minAge.*, geAmount.*, ltAmount.*)
 * @typedef {Object} CohortBasicWithMarketCap
 * @property {string} name
 * @property {string} title
 * @property {Color} color
 * @property {PatternBasicWithMarketCap} tree
 *
 * Basic cohort WITHOUT RelToMarketCap (epoch.*, amountRange.*, year.*, type.*)
 * @typedef {Object} CohortBasicWithoutMarketCap
 * @property {string} name
 * @property {string} title
 * @property {Color} color
 * @property {PatternBasicWithoutMarketCap} tree
 *
 * Union of basic cohort types
 * @typedef {CohortBasicWithMarketCap | CohortBasicWithoutMarketCap} CohortBasic
 *
 * ============================================================================
 * Extended Cohort Types (with address count)
 * ============================================================================
 *
 * Addressable cohort with address count (for "type" cohorts - no RelToMarketCap)
 * @typedef {CohortBasicWithoutMarketCap & { addrCount: Brk.MetricPattern1<Brk.StoredU64> }} CohortAddress
 *
 * ============================================================================
 * Cohort Group Types (by capability)
 * ============================================================================
 *
 * @typedef {Object} CohortGroupFull
 * @property {string} name
 * @property {string} title
 * @property {readonly CohortFull[]} list
 *
 * @typedef {Object} CohortGroupWithAdjusted
 * @property {string} name
 * @property {string} title
 * @property {readonly CohortWithAdjusted[]} list
 *
 * @typedef {Object} CohortGroupWithPercentiles
 * @property {string} name
 * @property {string} title
 * @property {readonly CohortWithPercentiles[]} list
 *
 * @typedef {Object} CohortGroupLongTerm
 * @property {string} name
 * @property {string} title
 * @property {readonly CohortLongTerm[]} list
 *
 * @typedef {Object} CohortGroupAgeRange
 * @property {string} name
 * @property {string} title
 * @property {readonly CohortAgeRange[]} list
 *
 * @typedef {Object} CohortGroupBasicWithMarketCap
 * @property {string} name
 * @property {string} title
 * @property {readonly CohortBasicWithMarketCap[]} list
 *
 * @typedef {Object} CohortGroupBasicWithoutMarketCap
 * @property {string} name
 * @property {string} title
 * @property {readonly CohortBasicWithoutMarketCap[]} list
 *
 * Union of basic cohort group types
 * @typedef {CohortGroupBasicWithMarketCap | CohortGroupBasicWithoutMarketCap} CohortGroupBasic
 *
 * @typedef {Object} UtxoCohortGroupObject
 * @property {string} name
 * @property {string} title
 * @property {readonly UtxoCohortObject[]} list
 *
 * @typedef {Object} AddressCohortObject
 * @property {string} name
 * @property {string} title
 * @property {Color} color
 * @property {AddressCohortPattern} tree
 *
 * @typedef {UtxoCohortObject | AddressCohortObject} CohortObject
 *
 *
 * @typedef {Object} AddressCohortGroupObject
 * @property {string} name
 * @property {string} title
 * @property {readonly AddressCohortObject[]} list
 *
 * @typedef {UtxoCohortGroupObject | AddressCohortGroupObject} CohortGroupObject
 */

// Re-export for type consumers
export {};
