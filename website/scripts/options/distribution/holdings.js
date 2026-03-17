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
import { ROLLING_WINDOWS, line, baseline, rollingWindowsTree, rollingPercentRatioTree, percentRatio } from "../series.js";
import {
  satsBtcUsd,
  mapCohorts,
  mapCohortsWithAll,
  flatMapCohortsWithAll,
} from "../shared.js";
import { colors } from "../../utils/colors.js";
import { priceLines } from "../constants.js";

/**
 * Simple supply series (total + half only, no profit/loss)
 * @param {{ total: AnyValuePattern, half: AnyValuePattern }} supply
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function simpleSupplySeries(supply) {
  return [
    ...satsBtcUsd({
      pattern: supply.total,
      name: "Total",
      color: colors.default,
    }),
    ...satsBtcUsd({
      pattern: supply.half,
      name: "Halved",
      color: colors.gray,
      style: 4,
    }),
  ];
}

/**
 * Full supply series (total, profit, loss, halved)
 * @param {{ total: AnyValuePattern, half: AnyValuePattern, inProfit: AnyValuePattern, inLoss: AnyValuePattern }} supply
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function fullSupplySeries(supply) {
  return [
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
  ];
}

/**
 * % of Own Supply series (profit/loss relative to own supply)
 * @param {{ inProfit: { toOwn: { percent: AnySeriesPattern, ratio: AnySeriesPattern } }, inLoss: { toOwn: { percent: AnySeriesPattern, ratio: AnySeriesPattern } } }} supply
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function ownSupplyPctSeries(supply) {
  return [
    line({ series: supply.inProfit.toOwn.percent, name: "In Profit", color: colors.profit, unit: Unit.pctOwn }),
    line({ series: supply.inLoss.toOwn.percent, name: "In Loss", color: colors.loss, unit: Unit.pctOwn }),
    line({ series: supply.inProfit.toOwn.ratio, name: "In Profit", color: colors.profit, unit: Unit.ratio }),
    line({ series: supply.inLoss.toOwn.ratio, name: "In Loss", color: colors.loss, unit: Unit.ratio }),
    ...priceLines({ numbers: [100, 50, 0], unit: Unit.pctOwn }),
  ];
}

/**
 * % of Circulating Supply series (total, profit, loss)
 * @param {{ toCirculating: { percent: AnySeriesPattern }, inProfit: { toCirculating: { percent: AnySeriesPattern } }, inLoss: { toCirculating: { percent: AnySeriesPattern } } }} supply
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function circulatingSupplyPctSeries(supply) {
  return [
    line({
      series: supply.toCirculating.percent,
      name: "Total",
      color: colors.default,
      unit: Unit.pctSupply,
    }),
    line({
      series: supply.inProfit.toCirculating.percent,
      name: "In Profit",
      color: colors.profit,
      unit: Unit.pctSupply,
    }),
    line({
      series: supply.inLoss.toCirculating.percent,
      name: "In Loss",
      color: colors.loss,
      unit: Unit.pctSupply,
    }),
  ];
}

/**
 * Ratio of Circulating Supply series (total, profit, loss)
 * @param {{ toCirculating: { ratio: AnySeriesPattern }, inProfit: { toCirculating: { ratio: AnySeriesPattern } }, inLoss: { toCirculating: { ratio: AnySeriesPattern } } }} supply
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function circulatingSupplyRatioSeries(supply) {
  return [
    line({
      series: supply.toCirculating.ratio,
      name: "Total",
      color: colors.default,
      unit: Unit.ratio,
    }),
    line({
      series: supply.inProfit.toCirculating.ratio,
      name: "In Profit",
      color: colors.profit,
      unit: Unit.ratio,
    }),
    line({
      series: supply.inLoss.toCirculating.ratio,
      name: "In Loss",
      color: colors.loss,
      unit: Unit.ratio,
    }),
  ];
}

/**
 * @param {readonly (UtxoCohortObject | CohortWithoutRelative)[]} list
 * @param {CohortAll} all
 * @param {(name: string) => string} title
 */
function groupedUtxoCountChart(list, all, title) {
  return {
    name: "UTXO Count",
    title: title("UTXO Count"),
    bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
      line({
        series: tree.outputs.unspentCount.base,
        name,
        color,
        unit: Unit.count,
      }),
    ),
  };
}

/**
 * @param {{ absolute: { _24h: AnySeriesPattern, _1w: AnySeriesPattern, _1m: AnySeriesPattern, _1y: AnySeriesPattern }, rate: { _24h: { percent: AnySeriesPattern, ratio: AnySeriesPattern }, _1w: { percent: AnySeriesPattern, ratio: AnySeriesPattern }, _1m: { percent: AnySeriesPattern, ratio: AnySeriesPattern }, _1y: { percent: AnySeriesPattern, ratio: AnySeriesPattern } } }} delta
 * @param {Unit} unit
 * @param {(name: string) => string} title
 * @param {string} name
 * @returns {PartialOptionsGroup}
 */
function singleDeltaTree(delta, unit, title, name) {
  return {
    name,
    tree: [
      { ...rollingWindowsTree({ windows: delta.absolute, title: title(`${name} Change`), unit, series: baseline }), name: "Absolute" },
      { ...rollingPercentRatioTree({ windows: delta.rate, title: title(`${name} Rate`) }), name: "Rate" },
    ],
  };
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
 * @returns {PartialOptionsGroup}
 */
function groupedDeltaTree(list, all, getDelta, unit, title, name) {
  return {
    name,
    tree: [
      {
        name: "Absolute",
        tree: ROLLING_WINDOWS.map((w) => ({
          name: w.name,
          title: title(`${name} Change (${w.name})`),
          bottom: mapCohortsWithAll(list, all, (c) =>
            baseline({ series: getDelta(c).absolute[w.key], name: c.name, color: c.color, unit }),
          ),
        })),
      },
      {
        name: "Rate",
        tree: ROLLING_WINDOWS.map((w) => ({
          name: w.name,
          title: title(`${name} Rate (${w.name})`),
          bottom: flatMapCohortsWithAll(list, all, (c) =>
            percentRatio({ pattern: getDelta(c).rate[w.key], name: c.name, color: c.color }),
          ),
        })),
      },
    ],
  };
}

/**
 * @param {UtxoCohortObject | CohortWithoutRelative} cohort
 * @param {(name: string) => string} title
 * @returns {PartialChartOption}
 */
function singleUtxoCountChart(cohort, title) {
  return {
    name: "UTXO Count",
    title: title("UTXO Count"),
    bottom: [
      line({
        series: cohort.tree.outputs.unspentCount.base,
        name: "UTXO Count",
        color: cohort.color,
        unit: Unit.count,
      }),
    ],
  };
}


/**
 * @param {CohortAll | CohortAddr | AddrCohortObject} cohort
 * @param {(name: string) => string} title
 * @returns {PartialChartOption}
 */
function singleAddressCountChart(cohort, title) {
  return {
    name: "Address Count",
    title: title("Address Count"),
    bottom: [
      line({
        series: cohort.addressCount.base,
        name: "Address Count",
        color: cohort.color,
        unit: Unit.count,
      }),
    ],
  };
}


// ============================================================================
// Single Cohort Holdings Sections
// ============================================================================

/**
 * Basic holdings (total + half only, no supply breakdown)
 * For: CohortWithoutRelative, CohortBasicWithMarketCap, CohortBasicWithoutMarketCap
 * @param {{ cohort: UtxoCohortObject | CohortWithoutRelative, title: (name: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createHoldingsSection({ cohort, title }) {
  return {
    name: "Holdings",
    tree: [
      {
        name: "Supply",
        title: title("Supply"),
        bottom: simpleSupplySeries(cohort.tree.supply),
      },
      singleUtxoCountChart(cohort, title),
      {
        name: "Change",
        tree: [
          singleDeltaTree(cohort.tree.supply.delta, Unit.sats, title, "Supply"),
          singleDeltaTree(cohort.tree.outputs.unspentCount.delta, Unit.count, title, "UTXO Count"),
        ],
      },
    ],
  };
}

/**
 * Holdings for CohortAll (has inProfit/inLoss with toOwn but no toCirculating)
 * @param {{ cohort: CohortAll, title: (name: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createHoldingsSectionAll({ cohort, title }) {
  const { supply } = cohort.tree;
  return {
    name: "Holdings",
    tree: [
      {
        name: "Supply",
        title: title("Supply"),
        bottom: [
          ...fullSupplySeries(supply),
          ...ownSupplyPctSeries(supply),
        ],
      },
      singleUtxoCountChart(cohort, title),
      singleAddressCountChart(cohort, title),
      {
        name: "Change",
        tree: [
          singleDeltaTree(cohort.tree.supply.delta, Unit.sats, title, "Supply"),
          singleDeltaTree(cohort.tree.outputs.unspentCount.delta, Unit.count, title, "UTXO Count"),
          singleDeltaTree(cohort.addressCount.delta, Unit.count, title, "Address Count"),
        ],
      },
    ],
  };
}

/**
 * Holdings with full relative series (toCirculating + toOwn)
 * For: CohortFull, CohortLongTerm (have DeltaHalfInRelTotalPattern2)
 * @param {{ cohort: CohortFull | CohortLongTerm, title: (name: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createHoldingsSectionWithRelative({ cohort, title }) {
  const { supply } = cohort.tree;
  return {
    name: "Holdings",
    tree: [
      {
        name: "Supply",
        tree: [
          {
            name: "Overview",
            title: title("Supply"),
            bottom: [
              ...fullSupplySeries(supply),
              ...circulatingSupplyPctSeries(supply),
              ...ownSupplyPctSeries(supply),
            ],
          },
          {
            name: "Ratio",
            title: title("Supply (% of Circulating)"),
            bottom: circulatingSupplyRatioSeries(supply),
          },
        ],
      },
      singleUtxoCountChart(cohort, title),
      {
        name: "Change",
        tree: [
          singleDeltaTree(cohort.tree.supply.delta, Unit.sats, title, "Supply"),
          singleDeltaTree(cohort.tree.outputs.unspentCount.delta, Unit.count, title, "UTXO Count"),
        ],
      },
    ],
  };
}

/**
 * Holdings with inProfit/inLoss + toCirculating (no toOwn)
 * For: CohortWithAdjusted, CohortAgeRange (have DeltaHalfInRelTotalPattern)
 * @param {{ cohort: CohortWithAdjusted | CohortAgeRange, title: (name: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createHoldingsSectionWithOwnSupply({ cohort, title }) {
  const { supply } = cohort.tree;
  return {
    name: "Holdings",
    tree: [
      {
        name: "Supply",
        tree: [
          {
            name: "Overview",
            title: title("Supply"),
            bottom: [
              ...fullSupplySeries(supply),
              ...circulatingSupplyPctSeries(supply),
            ],
          },
          {
            name: "Ratio",
            title: title("Supply (% of Circulating)"),
            bottom: circulatingSupplyRatioSeries(supply),
          },
        ],
      },
      singleUtxoCountChart(cohort, title),
      {
        name: "Change",
        tree: [
          singleDeltaTree(cohort.tree.supply.delta, Unit.sats, title, "Supply"),
          singleDeltaTree(cohort.tree.outputs.unspentCount.delta, Unit.count, title, "UTXO Count"),
        ],
      },
    ],
  };
}

/**
 * Holdings with inProfit/inLoss (no rel, no address count)
 * For: CohortWithoutRelative (p2ms, unknown, empty)
 * @param {{ cohort: CohortWithoutRelative, title: (name: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createHoldingsSectionWithProfitLoss({ cohort, title }) {
  return {
    name: "Holdings",
    tree: [
      {
        name: "Supply",
        title: title("Supply"),
        bottom: fullSupplySeries(cohort.tree.supply),
      },
      singleUtxoCountChart(cohort, title),
      {
        name: "Change",
        tree: [
          singleDeltaTree(cohort.tree.supply.delta, Unit.sats, title, "Supply"),
          singleDeltaTree(cohort.tree.outputs.unspentCount.delta, Unit.count, title, "UTXO Count"),
        ],
      },
    ],
  };
}

/**
 * Holdings for CohortAddr (has inProfit/inLoss but no rel, plus address count)
 * @param {{ cohort: CohortAddr, title: (name: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createHoldingsSectionAddress({ cohort, title }) {
  return {
    name: "Holdings",
    tree: [
      {
        name: "Supply",
        title: title("Supply"),
        bottom: fullSupplySeries(cohort.tree.supply),
      },
      singleUtxoCountChart(cohort, title),
      singleAddressCountChart(cohort, title),
      {
        name: "Change",
        tree: [
          singleDeltaTree(cohort.tree.supply.delta, Unit.sats, title, "Supply"),
          singleDeltaTree(cohort.tree.outputs.unspentCount.delta, Unit.count, title, "UTXO Count"),
          singleDeltaTree(cohort.addressCount.delta, Unit.count, title, "Address Count"),
        ],
      },
    ],
  };
}

/**
 * Holdings for address amount cohorts (no inProfit/inLoss, has address count)
 * @param {{ cohort: AddrCohortObject, title: (name: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createHoldingsSectionAddressAmount({ cohort, title }) {
  return {
    name: "Holdings",
    tree: [
      {
        name: "Supply",
        title: title("Supply"),
        bottom: simpleSupplySeries(cohort.tree.supply),
      },
      singleUtxoCountChart(cohort, title),
      singleAddressCountChart(cohort, title),
      {
        name: "Change",
        tree: [
          singleDeltaTree(cohort.tree.supply.delta, Unit.sats, title, "Supply"),
          singleDeltaTree(cohort.tree.outputs.unspentCount.delta, Unit.count, title, "UTXO Count"),
          singleDeltaTree(cohort.addressCount.delta, Unit.count, title, "Address Count"),
        ],
      },
    ],
  };
}

// ============================================================================
// Grouped Cohort Holdings Sections
// ============================================================================

/**
 * @param {{ list: readonly CohortAddr[], all: CohortAll, title: (name: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createGroupedHoldingsSectionAddress({ list, all, title }) {
  return {
    name: "Holdings",
    tree: [
      {
        name: "Supply",
        tree: [
          {
            name: "Total",
            title: title("Supply"),
            bottom: flatMapCohortsWithAll(list, all, ({ name, color, tree }) =>
              satsBtcUsd({ pattern: tree.supply.total, name, color }),
            ),
          },
          {
            name: "In Profit",
            title: title("Supply In Profit"),
            bottom: flatMapCohortsWithAll(list, all, ({ name, color, tree }) =>
              satsBtcUsd({
                pattern: tree.supply.inProfit,
                name,
                color,
              }),
            ),
          },
          {
            name: "In Loss",
            title: title("Supply In Loss"),
            bottom: flatMapCohortsWithAll(list, all, ({ name, color, tree }) =>
              satsBtcUsd({
                pattern: tree.supply.inLoss,
                name,
                color,
              }),
            ),
          },
        ],
      },
      groupedUtxoCountChart(list, all, title),
      {
        name: "Address Count",
        title: title("Address Count"),
        bottom: mapCohortsWithAll(list, all, ({ name, color, addressCount }) =>
          line({ series: addressCount.base, name, color, unit: Unit.count }),
        ),
      },
      {
        name: "Change",
        tree: [
          groupedDeltaTree(list, all, (c) => c.tree.supply.delta, Unit.sats, title, "Supply"),
          groupedDeltaTree(list, all, (c) => c.tree.outputs.unspentCount.delta, Unit.count, title, "UTXO Count"),
          groupedDeltaTree(list, all, (c) => c.addressCount.delta, Unit.count, title, "Address Count"),
        ],
      },
    ],
  };
}

/**
 * Grouped holdings for address amount cohorts (no inProfit/inLoss, has address count)
 * @param {{ list: readonly AddrCohortObject[], all: CohortAll, title: (name: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createGroupedHoldingsSectionAddressAmount({
  list,
  all,
  title,
}) {
  return {
    name: "Holdings",
    tree: [
      {
        name: "Supply",
        tree: [
          {
            name: "Total",
            title: title("Supply"),
            bottom: flatMapCohortsWithAll(list, all, ({ name, color, tree }) =>
              satsBtcUsd({ pattern: tree.supply.total, name, color }),
            ),
          },
        ],
      },
      groupedUtxoCountChart(list, all, title),
      {
        name: "Address Count",
        title: title("Address Count"),
        bottom: mapCohortsWithAll(list, all, ({ name, color, addressCount }) =>
          line({ series: addressCount.base, name, color, unit: Unit.count }),
        ),
      },
      {
        name: "Change",
        tree: [
          groupedDeltaTree(list, all, (c) => c.tree.supply.delta, Unit.sats, title, "Supply"),
          groupedDeltaTree(list, all, (c) => c.tree.outputs.unspentCount.delta, Unit.count, title, "UTXO Count"),
          groupedDeltaTree(list, all, (c) => c.addressCount.delta, Unit.count, title, "Address Count"),
        ],
      },
    ],
  };
}

/**
 * Basic grouped holdings (total + half only)
 * @param {{ list: readonly (UtxoCohortObject | CohortWithoutRelative)[], all: CohortAll, title: (name: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createGroupedHoldingsSection({ list, all, title }) {
  return {
    name: "Holdings",
    tree: [
      {
        name: "Supply",
        tree: [
          {
            name: "Total",
            title: title("Supply"),
            bottom: flatMapCohortsWithAll(list, all, ({ name, color, tree }) =>
              satsBtcUsd({ pattern: tree.supply.total, name, color }),
            ),
          },
        ],
      },
      groupedUtxoCountChart(list, all, title),
      {
        name: "Change",
        tree: [
          groupedDeltaTree(list, all, (c) => c.tree.supply.delta, Unit.sats, title, "Supply"),
          groupedDeltaTree(list, all, (c) => c.tree.outputs.unspentCount.delta, Unit.count, title, "UTXO Count"),
        ],
      },
    ],
  };
}

/**
 * Grouped holdings with inProfit/inLoss (no rel, no address count)
 * For: CohortWithoutRelative (p2ms, unknown, empty)
 * @param {{ list: readonly CohortWithoutRelative[], all: CohortAll, title: (name: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createGroupedHoldingsSectionWithProfitLoss({
  list,
  all,
  title,
}) {
  return {
    name: "Holdings",
    tree: [
      {
        name: "Supply",
        tree: [
          {
            name: "Total",
            title: title("Supply"),
            bottom: flatMapCohortsWithAll(list, all, ({ name, color, tree }) =>
              satsBtcUsd({ pattern: tree.supply.total, name, color }),
            ),
          },
          {
            name: "In Profit",
            title: title("Supply In Profit"),
            bottom: flatMapCohortsWithAll(list, all, ({ name, color, tree }) =>
              satsBtcUsd({
                pattern: tree.supply.inProfit,
                name,
                color,
              }),
            ),
          },
          {
            name: "In Loss",
            title: title("Supply In Loss"),
            bottom: flatMapCohortsWithAll(list, all, ({ name, color, tree }) =>
              satsBtcUsd({
                pattern: tree.supply.inLoss,
                name,
                color,
              }),
            ),
          },
        ],
      },
      groupedUtxoCountChart(list, all, title),
      {
        name: "Change",
        tree: [
          groupedDeltaTree(list, all, (c) => c.tree.supply.delta, Unit.sats, title, "Supply"),
          groupedDeltaTree(list, all, (c) => c.tree.outputs.unspentCount.delta, Unit.count, title, "UTXO Count"),
        ],
      },
    ],
  };
}

/**
 * Grouped holdings with inProfit/inLoss + toCirculating (no toOwn)
 * For: CohortWithAdjusted, CohortAgeRange
 * @param {{ list: readonly (CohortWithAdjusted | CohortAgeRange)[], all: CohortAll, title: (name: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createGroupedHoldingsSectionWithOwnSupply({
  list,
  all,
  title,
}) {
  return {
    name: "Holdings",
    tree: [
      {
        name: "Supply",
        tree: [
          {
            name: "Total",
            title: title("Supply"),
            bottom: [
              ...flatMapCohortsWithAll(list, all, ({ name, color, tree }) =>
                satsBtcUsd({ pattern: tree.supply.total, name, color }),
              ),
              ...mapCohorts(list, ({ name, color, tree }) =>
                line({
                  series: tree.supply.toCirculating.percent,
                  name,
                  color,
                  unit: Unit.pctSupply,
                }),
              ),
            ],
          },
          {
            name: "In Profit",
            title: title("Supply In Profit"),
            bottom: [
              ...flatMapCohortsWithAll(list, all, ({ name, color, tree }) =>
                satsBtcUsd({
                  pattern: tree.supply.inProfit,
                  name,
                  color,
                }),
              ),
              ...mapCohorts(list, ({ name, color, tree }) =>
                line({
                  series: tree.supply.inProfit.toCirculating.percent,
                  name,
                  color,
                  unit: Unit.pctSupply,
                }),
              ),
            ],
          },
          {
            name: "In Loss",
            title: title("Supply In Loss"),
            bottom: [
              ...flatMapCohortsWithAll(list, all, ({ name, color, tree }) =>
                satsBtcUsd({
                  pattern: tree.supply.inLoss,
                  name,
                  color,
                }),
              ),
              ...mapCohorts(list, ({ name, color, tree }) =>
                line({
                  series: tree.supply.inLoss.toCirculating.percent,
                  name,
                  color,
                  unit: Unit.pctSupply,
                }),
              ),
            ],
          },
        ],
      },
      groupedUtxoCountChart(list, all, title),
      {
        name: "Change",
        tree: [
          groupedDeltaTree(list, all, (c) => c.tree.supply.delta, Unit.sats, title, "Supply"),
          groupedDeltaTree(list, all, (c) => c.tree.outputs.unspentCount.delta, Unit.count, title, "UTXO Count"),
        ],
      },
    ],
  };
}

/**
 * Grouped holdings with full relative series (toCirculating + toOwn)
 * For: CohortFull, CohortLongTerm
 * @param {{ list: readonly (CohortFull | CohortLongTerm)[], all: CohortAll, title: (name: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createGroupedHoldingsSectionWithRelative({ list, all, title }) {
  return {
    name: "Holdings",
    tree: [
      {
        name: "Supply",
        tree: [
          {
            name: "Total",
            title: title("Supply"),
            bottom: [
              ...flatMapCohortsWithAll(list, all, ({ name, color, tree }) =>
                satsBtcUsd({ pattern: tree.supply.total, name, color }),
              ),
              ...mapCohorts(list, ({ name, color, tree }) =>
                line({
                  series: tree.supply.toCirculating.percent,
                  name,
                  color,
                  unit: Unit.pctSupply,
                }),
              ),
            ],
          },
          {
            name: "In Profit",
            title: title("Supply In Profit"),
            bottom: [
              ...flatMapCohortsWithAll(list, all, ({ name, color, tree }) =>
                satsBtcUsd({
                  pattern: tree.supply.inProfit,
                  name,
                  color,
                }),
              ),
              ...mapCohorts(list, ({ name, color, tree }) =>
                line({
                  series: tree.supply.inProfit.toCirculating.percent,
                  name,
                  color,
                  unit: Unit.pctSupply,
                }),
              ),
              ...mapCohortsWithAll(list, all, ({ name, color, tree }) =>
                line({
                  series: tree.supply.inProfit.toOwn.percent,
                  name,
                  color,
                  unit: Unit.pctOwn,
                }),
              ),
              ...priceLines({ numbers: [100, 50, 0], unit: Unit.pctOwn }),
            ],
          },
          {
            name: "In Loss",
            title: title("Supply In Loss"),
            bottom: [
              ...flatMapCohortsWithAll(list, all, ({ name, color, tree }) =>
                satsBtcUsd({
                  pattern: tree.supply.inLoss,
                  name,
                  color,
                }),
              ),
              ...mapCohorts(list, ({ name, color, tree }) =>
                line({
                  series: tree.supply.inLoss.toCirculating.percent,
                  name,
                  color,
                  unit: Unit.pctSupply,
                }),
              ),
              ...mapCohortsWithAll(list, all, ({ name, color, tree }) =>
                line({
                  series: tree.supply.inLoss.toOwn.percent,
                  name,
                  color,
                  unit: Unit.pctOwn,
                }),
              ),
              ...priceLines({ numbers: [100, 50, 0], unit: Unit.pctOwn }),
            ],
          },
        ],
      },
      groupedUtxoCountChart(list, all, title),
      {
        name: "Change",
        tree: [
          groupedDeltaTree(list, all, (c) => c.tree.supply.delta, Unit.sats, title, "Supply"),
          groupedDeltaTree(list, all, (c) => c.tree.outputs.unspentCount.delta, Unit.count, title, "UTXO Count"),
        ],
      },
    ],
  };
}
