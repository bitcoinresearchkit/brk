/**
 * Holdings section builders
 *
 * Supply pattern capabilities by cohort type:
 * - DeltaHalfInRelTotalPattern2 (STH/LTH): inProfit + inLoss + toCirculating + toOwn
 * - SeriesTree_Cohorts_Utxo_All_Supply (All): inProfit + inLoss + toOwn (no toCirculating)
 * - DeltaHalfInRelTotalPattern (AgeRange/MaxAge/Epoch): inProfit + inLoss + toCirculating (no toOwn)
 * - DeltaHalfInTotalPattern2 (Type.*): inProfit + inLoss (no rel)
 * - DeltaHalfTotalPattern (Empty/UtxoAmount/AddrAmount): total + half only
 */

import { Unit } from "../../utils/units.js";
import {
  ROLLING_WINDOWS,
  line,
  baseline,
  sumsTreeBaseline,
  rollingPercentRatioTree,
  percentRatio,
  percentRatioBaseline,
} from "../series.js";
import {
  satsBtcUsd,
  flatMapCohorts,
  mapCohortsWithAll,
  flatMapCohortsWithAll,
} from "../shared.js";
import { colors } from "../../utils/colors.js";
import { priceLines } from "../constants.js";

/**
 * Simple supply series (total + half only, no profit/loss)
 * @param {{ total: AnyValuePattern }} supply
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function simpleSupplySeries(supply) {
  return satsBtcUsd({
    pattern: supply.total,
    name: "Total",
  });
}


/**
 * @param {readonly (UtxoCohortObject | CohortWithoutRelative)[]} list
 * @param {CohortAll} all
 * @param {(name: string) => string} title
 */
function groupedUtxoCountFolder(list, all, title) {
  return {
    name: "UTXOs",
    tree: [
      {
        name: "Count",
        title: title("UTXO Count"),
        bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
          line({ series: tree.outputs.unspentCount.base, name, color, unit: Unit.count }),
        ),
      },
      ...groupedDeltaItems(list, all, (c) => c.tree.outputs.unspentCount.delta, Unit.count, title, "UTXO Count"),
    ],
  };
}

/**
 * @param {{ absolute: { _24h: AnySeriesPattern, _1w: AnySeriesPattern, _1m: AnySeriesPattern, _1y: AnySeriesPattern }, rate: { _24h: { percent: AnySeriesPattern, ratio: AnySeriesPattern }, _1w: { percent: AnySeriesPattern, ratio: AnySeriesPattern }, _1m: { percent: AnySeriesPattern, ratio: AnySeriesPattern }, _1y: { percent: AnySeriesPattern, ratio: AnySeriesPattern } } }} delta
 * @param {Unit} unit
 * @param {(name: string) => string} title
 * @param {string} name
 * @returns {PartialOptionsTree}
 */
function singleDeltaItems(delta, unit, title, name) {
  return [
    {
      ...sumsTreeBaseline({
        windows: delta.absolute,
        title,
        metric: `${name} Change`,
        unit,
      }),
      name: "Change",
    },
    {
      ...rollingPercentRatioTree({
        windows: delta.rate,
        title,
        metric: `${name} Growth Rate`,
      }),
      name: "Growth Rate",
    },
  ];
}

/**
 * @template {{ name: string, color: Color }} T
 * @template {{ name: string, color: Color }} A
 * @param {readonly T[]} list
 * @param {A} all
 * @param {(c: T | A) => DeltaPattern} getDelta
 * @param {Unit} unit
 * @param {(name: string) => string} title
 * @param {string} name
 * @returns {PartialOptionsTree}
 */
function groupedDeltaItems(list, all, getDelta, unit, title, name) {
  return [
      {
        name: "Change",
        tree: ROLLING_WINDOWS.map((w) => ({
          name: w.name,
          title: title(`${w.title} ${name} Change`),
          bottom: mapCohortsWithAll(list, all, (c) =>
            baseline({
              series: getDelta(c).absolute[w.key],
              name: c.name,
              color: c.color,
              unit,
            }),
          ),
        })),
      },
      {
        name: "Growth Rate",
        tree: ROLLING_WINDOWS.map((w) => ({
          name: w.name,
          title: title(`${w.title} ${name} Growth Rate`),
          bottom: flatMapCohortsWithAll(list, all, (c) =>
            percentRatioBaseline({
              pattern: getDelta(c).rate[w.key],
              name: c.name,
              color: c.color,
            }),
          ),
        })),
      },
  ];
}

// ============================================================================
// Single Cohort Composable Builders
// ============================================================================

/**
 * Profitability chart (in profit + in loss supply)
 * @param {{ total: AnyValuePattern, half: AnyValuePattern, inProfit: AnyValuePattern, inLoss: AnyValuePattern }} supply
 * @param {(name: string) => string} title
 * @returns {PartialChartOption}
 */
function profitabilityChart(supply, title) {
  return {
    name: "Profitability",
    title: title("Supply Profitability"),
    bottom: [
      ...satsBtcUsd({
        pattern: supply.total,
        name: "Total",
        color: colors.default,
      }),
      ...satsBtcUsd({
        pattern: supply.inProfit,
        name: "In Profit",
        color: colors.profit,
      }),
      ...satsBtcUsd({
        pattern: supply.inLoss,
        name: "In Loss",
        color: colors.loss,
      }),
      ...satsBtcUsd({
        pattern: supply.half,
        name: "Halved",
        color: colors.gray,
        style: 4,
      }),
    ],
  };
}

/**
 * @param {{ toCirculating: PercentRatioPattern, inProfit: { toCirculating: PercentRatioPattern }, inLoss: { toCirculating: PercentRatioPattern } }} supply
 * @param {(name: string) => string} title
 * @returns {PartialChartOption}
 */
function circulatingChart(supply, title) {
  return {
    name: "% of Circulating",
    title: title("Supply (% of Circulating)"),
    bottom: [
      ...percentRatio({ pattern: supply.toCirculating, name: "Total", color: colors.default }),
      ...percentRatio({ pattern: supply.inProfit.toCirculating, name: "In Profit", color: colors.profit }),
      ...percentRatio({ pattern: supply.inLoss.toCirculating, name: "In Loss", color: colors.loss }),
    ],
  };
}

/**
 * @param {{ inProfit: { toOwn: { percent: AnySeriesPattern, ratio: AnySeriesPattern } }, inLoss: { toOwn: { percent: AnySeriesPattern, ratio: AnySeriesPattern } } }} supply
 * @param {(name: string) => string} title
 * @returns {PartialChartOption}
 */
function ownSupplyChart(supply, title) {
  return {
    name: "% of Own Supply",
    title: title("Supply (% of Own)"),
    bottom: [
      ...percentRatio({ pattern: supply.inProfit.toOwn, name: "In Profit", color: colors.profit }),
      ...percentRatio({ pattern: supply.inLoss.toOwn, name: "In Loss", color: colors.loss }),
      ...priceLines({ numbers: [100, 50, 0], unit: Unit.percentage }),
    ],
  };
}

/**
 * Count folder (UTXO or Address) with value + change
 * @param {{ base: AnySeriesPattern, delta: DeltaPattern }} pattern
 * @param {string} name
 * @param {string} chartTitle
 * @param {Color} color
 * @param {(name: string) => string} title
 * @returns {PartialOptionsGroup}
 */
function countFolder(pattern, name, chartTitle, color, title) {
  return {
    name,
    tree: [
      {
        name: "Count",
        title: title(chartTitle),
        bottom: [
          line({
            series: pattern.base,
            name: "Count",
            color,
            unit: Unit.count,
          }),
        ],
      },
      ...singleDeltaItems(pattern.delta, Unit.count, title, chartTitle),
    ],
  };
}

// ============================================================================
// Single Cohort Holdings Sections
// ============================================================================

/**
 * @param {{ cohort: UtxoCohortObject | CohortWithoutRelative, title: (name: string) => string }} args
 * @returns {PartialOptionsTree}
 */
export function createHoldingsSection({ cohort, title }) {
  const { supply } = cohort.tree;
  return [
    {
      name: "Supply",
      tree: [
        {
          name: "Total",
          title: title("Supply"),
          bottom: simpleSupplySeries(supply),
        },
        ...singleDeltaItems(supply.delta, Unit.sats, title, "Supply"),
      ],
    },
    countFolder(cohort.tree.outputs.unspentCount, "UTXOs", "UTXO Count", cohort.color, title),
  ];
}

/**
 * @param {{ cohort: CohortAll, title: (name: string) => string }} args
 * @returns {PartialOptionsTree}
 */
export function createHoldingsSectionAll({ cohort, title }) {
  const { supply } = cohort.tree;
  return [
    {
      name: "Supply",
      tree: [
        {
          name: "Total",
          title: title("Supply"),
          bottom: simpleSupplySeries(supply),
        },
        profitabilityChart(supply, title),
        ownSupplyChart(supply, title),
        ...singleDeltaItems(supply.delta, Unit.sats, title, "Supply"),
      ],
    },
    countFolder(cohort.tree.outputs.unspentCount, "UTXOs", "UTXO Count", cohort.color, title),
    countFolder(cohort.addressCount, "Addresses", "Address Count", cohort.color, title),
  ];
}

/**
 * @param {{ cohort: CohortFull | CohortLongTerm, title: (name: string) => string }} args
 * @returns {PartialOptionsTree}
 */
export function createHoldingsSectionWithRelative({ cohort, title }) {
  const { supply } = cohort.tree;
  return [
    {
      name: "Supply",
      tree: [
        {
          name: "Total",
          title: title("Supply"),
          bottom: simpleSupplySeries(supply),
        },
        profitabilityChart(supply, title),
        circulatingChart(supply, title),
        ownSupplyChart(supply, title),
        ...singleDeltaItems(supply.delta, Unit.sats, title, "Supply"),
      ],
    },
    countFolder(cohort.tree.outputs.unspentCount, "UTXOs", "UTXO Count", cohort.color, title),
  ];
}

/**
 * @param {{ cohort: CohortWithAdjusted | CohortAgeRange, title: (name: string) => string }} args
 * @returns {PartialOptionsTree}
 */
export function createHoldingsSectionWithOwnSupply({ cohort, title }) {
  const { supply } = cohort.tree;
  return [
    {
      name: "Supply",
      tree: [
        {
          name: "Total",
          title: title("Supply"),
          bottom: simpleSupplySeries(supply),
        },
        profitabilityChart(supply, title),
        circulatingChart(supply, title),
        ...singleDeltaItems(supply.delta, Unit.sats, title, "Supply"),
      ],
    },
    countFolder(cohort.tree.outputs.unspentCount, "UTXOs", "UTXO Count", cohort.color, title),
  ];
}

/**
 * @param {{ cohort: CohortWithoutRelative, title: (name: string) => string }} args
 * @returns {PartialOptionsTree}
 */
export function createHoldingsSectionWithProfitLoss({ cohort, title }) {
  const { supply } = cohort.tree;
  return [
    {
      name: "Supply",
      tree: [
        {
          name: "Total",
          title: title("Supply"),
          bottom: simpleSupplySeries(supply),
        },
        profitabilityChart(supply, title),
        ...singleDeltaItems(supply.delta, Unit.sats, title, "Supply"),
      ],
    },
    countFolder(cohort.tree.outputs.unspentCount, "UTXOs", "UTXO Count", cohort.color, title),
  ];
}

/**
 * @param {{ cohort: CohortAddr, title: (name: string) => string }} args
 * @returns {PartialOptionsTree}
 */
export function createHoldingsSectionAddress({ cohort, title }) {
  const { supply } = cohort.tree;
  return [
    {
      name: "Supply",
      tree: [
        {
          name: "Total",
          title: title("Supply"),
          bottom: simpleSupplySeries(supply),
        },
        profitabilityChart(supply, title),
        ...singleDeltaItems(supply.delta, Unit.sats, title, "Supply"),
      ],
    },
    countFolder(cohort.tree.outputs.unspentCount, "UTXOs", "UTXO Count", cohort.color, title),
    countFolder(cohort.addressCount, "Addresses", "Address Count", cohort.color, title),
  ];
}

/**
 * @param {{ cohort: AddrCohortObject, title: (name: string) => string }} args
 * @returns {PartialOptionsTree}
 */
export function createHoldingsSectionAddressAmount({ cohort, title }) {
  const { supply } = cohort.tree;
  return [
    {
      name: "Supply",
      tree: [
        {
          name: "Total",
          title: title("Supply"),
          bottom: simpleSupplySeries(supply),
        },
        ...singleDeltaItems(supply.delta, Unit.sats, title, "Supply"),
      ],
    },
    countFolder(cohort.tree.outputs.unspentCount, "UTXOs", "UTXO Count", cohort.color, title),
    countFolder(cohort.addressCount, "Addresses", "Address Count", cohort.color, title),
  ];
}

// ============================================================================
// Grouped Cohort Supply Helpers
// ============================================================================

/**
 * @template {{ name: string, color: Color, tree: { supply: { total: AnyValuePattern } } }} T
 * @param {readonly T[]} list
 * @param {CohortAll} all
 * @param {(name: string) => string} title
 * @returns {PartialChartOption}
 */
function groupedSupplyTotal(list, all, title) {
  return { name: "Total", title: title("Supply"), bottom: flatMapCohortsWithAll(list, all, ({ name, color, tree }) => satsBtcUsd({ pattern: tree.supply.total, name, color })) };
}

/**
 * @template {{ name: string, color: Color, tree: { supply: { inProfit: AnyValuePattern, inLoss: AnyValuePattern } } }} T
 * @param {readonly T[]} list
 * @param {CohortAll} all
 * @param {(name: string) => string} title
 * @returns {PartialOptionsTree}
 */
function groupedSupplyProfitLoss(list, all, title) {
  return [
    { name: "In Profit", title: title("Supply In Profit"), bottom: flatMapCohortsWithAll(list, all, ({ name, color, tree }) => satsBtcUsd({ pattern: tree.supply.inProfit, name, color })) },
    { name: "In Loss", title: title("Supply In Loss"), bottom: flatMapCohortsWithAll(list, all, ({ name, color, tree }) => satsBtcUsd({ pattern: tree.supply.inLoss, name, color })) },
  ];
}

// ============================================================================
// Grouped Cohort Holdings Sections
// ============================================================================

/**
 * @param {{ list: readonly CohortAddr[], all: CohortAll, title: (name: string) => string }} args
 * @returns {PartialOptionsTree}
 */
export function createGroupedHoldingsSectionAddress({ list, all, title }) {
  return [
    {
      name: "Supply",
      tree: [
        groupedSupplyTotal(list, all, title),
        ...groupedSupplyProfitLoss(list, all, title),
        ...groupedDeltaItems(list, all, (c) => c.tree.supply.delta, Unit.sats, title, "Supply"),
      ],
    },
    groupedUtxoCountFolder(list, all, title),
    {
      name: "Addresses",
      tree: [
        {
          name: "Count",
          title: title("Address Count"),
          bottom: mapCohortsWithAll(list, all, ({ name, color, addressCount }) =>
            line({ series: addressCount.base, name, color, unit: Unit.count }),
          ),
        },
        ...groupedDeltaItems(list, all, (c) => c.addressCount.delta, Unit.count, title, "Address Count"),
      ],
    },
  ];
}

/**
 * Grouped holdings for address amount cohorts (no inProfit/inLoss, has address count)
 * @param {{ list: readonly AddrCohortObject[], all: CohortAll, title: (name: string) => string }} args
 * @returns {PartialOptionsTree}
 */
export function createGroupedHoldingsSectionAddressAmount({ list, all, title }) {
  return [
    {
      name: "Supply",
      tree: [
        groupedSupplyTotal(list, all, title),
        ...groupedDeltaItems(list, all, (c) => c.tree.supply.delta, Unit.sats, title, "Supply"),
      ],
    },
    groupedUtxoCountFolder(list, all, title),
    {
      name: "Addresses",
      tree: [
        {
          name: "Count",
          title: title("Address Count"),
          bottom: mapCohortsWithAll(list, all, ({ name, color, addressCount }) =>
            line({ series: addressCount.base, name, color, unit: Unit.count }),
          ),
        },
        ...groupedDeltaItems(list, all, (c) => c.addressCount.delta, Unit.count, title, "Address Count"),
      ],
    },
  ];
}

/** @param {{ list: readonly (UtxoCohortObject | CohortWithoutRelative)[], all: CohortAll, title: (name: string) => string }} args */
export function createGroupedHoldingsSection({ list, all, title }) {
  return [
    {
      name: "Supply",
      tree: [
        groupedSupplyTotal(list, all, title),
        ...groupedDeltaItems(list, all, (c) => c.tree.supply.delta, Unit.sats, title, "Supply"),
      ],
    },
    groupedUtxoCountFolder(list, all, title),
  ];
}

/** @param {{ list: readonly CohortWithoutRelative[], all: CohortAll, title: (name: string) => string }} args */
export function createGroupedHoldingsSectionWithProfitLoss({ list, all, title }) {
  return [
    {
      name: "Supply",
      tree: [
        groupedSupplyTotal(list, all, title),
        ...groupedSupplyProfitLoss(list, all, title),
        ...groupedDeltaItems(list, all, (c) => c.tree.supply.delta, Unit.sats, title, "Supply"),
      ],
    },
    groupedUtxoCountFolder(list, all, title),
  ];
}

/** @param {{ list: readonly (CohortWithAdjusted | CohortAgeRange)[], all: CohortAll, title: (name: string) => string }} args */
export function createGroupedHoldingsSectionWithOwnSupply({ list, all, title }) {
  return [
    {
      name: "Supply",
      tree: [
        groupedSupplyTotal(list, all, title),
        ...groupedSupplyProfitLoss(list, all, title),
        { name: "% of Circulating", title: title("Supply (% of Circulating)"), bottom: flatMapCohorts(list, ({ name, color, tree }) => percentRatio({ pattern: tree.supply.toCirculating, name, color })) },
        ...groupedDeltaItems(list, all, (c) => c.tree.supply.delta, Unit.sats, title, "Supply"),
      ],
    },
    groupedUtxoCountFolder(list, all, title),
  ];
}

/**
 * Grouped holdings with full relative series (toCirculating + toOwn)
 * For: CohortFull, CohortLongTerm
 * @param {{ list: readonly (CohortFull | CohortLongTerm)[], all: CohortAll, title: (name: string) => string }} args
 * @returns {PartialOptionsTree}
 */
export function createGroupedHoldingsSectionWithRelative({ list, all, title }) {
  return [
    {
      name: "Supply",
      tree: [
        groupedSupplyTotal(list, all, title),
        ...groupedSupplyProfitLoss(list, all, title),
        { name: "% of Circulating", title: title("Supply (% of Circulating)"), bottom: flatMapCohorts(list, ({ name, color, tree }) => percentRatio({ pattern: tree.supply.toCirculating, name, color })) },
        { name: "% of Own Supply", title: title("Supply (% of Own)"), bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) => line({ series: tree.supply.inProfit.toOwn.percent, name, color, unit: Unit.percentage })) },
        ...groupedDeltaItems(list, all, (c) => c.tree.supply.delta, Unit.sats, title, "Supply"),
      ],
    },
    groupedUtxoCountFolder(list, all, title),
  ];
}
