/**
 * Holdings section builders
 *
 * Supply pattern capabilities by cohort type:
 * - DeltaHalfInRelTotalPattern2 (STH/LTH): inProfit + inLoss + relToCirculating + relToOwn
 * - MetricsTree_Cohorts_Utxo_All_Supply (All): inProfit + inLoss + relToOwn (no relToCirculating)
 * - DeltaHalfInRelTotalPattern (AgeRange/MaxAge/Epoch): inProfit + inLoss + relToCirculating (no relToOwn)
 * - DeltaHalfInTotalPattern2 (Type.*): inProfit + inLoss (no rel)
 * - DeltaHalfTotalPattern (Empty/UtxoAmount/AddrAmount): total + half only
 */

import { Unit } from "../../utils/units.js";
import { line, baseline } from "../series.js";
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
 * @param {{ inProfit: { relToOwn: { percent: AnyMetricPattern } }, inLoss: { relToOwn: { percent: AnyMetricPattern } } }} supply
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function ownSupplyPctSeries(supply) {
  return [
    line({
      metric: supply.inProfit.relToOwn.percent,
      name: "In Profit",
      color: colors.profit,
      unit: Unit.pctOwn,
    }),
    line({
      metric: supply.inLoss.relToOwn.percent,
      name: "In Loss",
      color: colors.loss,
      unit: Unit.pctOwn,
    }),
    ...priceLines({ numbers: [100, 50, 0], unit: Unit.pctOwn }),
  ];
}

/**
 * % of Circulating Supply series (total, profit, loss)
 * @param {{ relToCirculating: { percent: AnyMetricPattern }, inProfit: { relToCirculating: { percent: AnyMetricPattern } }, inLoss: { relToCirculating: { percent: AnyMetricPattern } } }} supply
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function circulatingSupplyPctSeries(supply) {
  return [
    line({
      metric: supply.relToCirculating.percent,
      name: "Total",
      color: colors.default,
      unit: Unit.pctSupply,
    }),
    line({
      metric: supply.inProfit.relToCirculating.percent,
      name: "In Profit",
      color: colors.profit,
      unit: Unit.pctSupply,
    }),
    line({
      metric: supply.inLoss.relToCirculating.percent,
      name: "In Loss",
      color: colors.loss,
      unit: Unit.pctSupply,
    }),
  ];
}

/**
 * @param {readonly (UtxoCohortObject | CohortWithoutRelative)[]} list
 * @param {CohortAll} all
 * @param {(metric: string) => string} title
 */
function groupedUtxoCountChart(list, all, title) {
  return {
    name: "UTXO Count",
    title: title("UTXO Count"),
    bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
      line({
        metric: tree.outputs.unspentCount.inner,
        name,
        color,
        unit: Unit.count,
      }),
    ),
  };
}

/**
 * @param {readonly (UtxoCohortObject | CohortWithoutRelative)[]} list
 * @param {CohortAll} all
 * @param {(metric: string) => string} title
 */
function grouped30dSupplyChangeChart(list, all, title) {
  return {
    name: "Supply",
    title: title("Supply 30d Change"),
    bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
      baseline({
        metric: tree.supply.delta.change._1m,
        name,
        color,
        unit: Unit.sats,
      }),
    ),
  };
}

/**
 * @param {readonly (UtxoCohortObject | CohortWithoutRelative)[]} list
 * @param {CohortAll} all
 * @param {(metric: string) => string} title
 */
function grouped30dUtxoCountChangeChart(list, all, title) {
  return {
    name: "UTXO Count",
    title: title("UTXO Count 30d Change"),
    bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
      baseline({
        metric: tree.outputs.unspentCount.delta.change._1m,
        name,
        unit: Unit.count,
        color,
      }),
    ),
  };
}

/**
 * @param {UtxoCohortObject | CohortWithoutRelative} cohort
 * @param {(metric: string) => string} title
 * @returns {PartialChartOption}
 */
function singleUtxoCountChart(cohort, title) {
  return {
    name: "UTXO Count",
    title: title("UTXO Count"),
    bottom: [
      line({
        metric: cohort.tree.outputs.unspentCount.inner,
        name: "UTXO Count",
        color: cohort.color,
        unit: Unit.count,
      }),
    ],
  };
}

/**
 * @param {UtxoCohortObject | CohortWithoutRelative} cohort
 * @param {(metric: string) => string} title
 * @returns {PartialChartOption}
 */
function single30dSupplyChangeChart(cohort, title) {
  return {
    name: "Supply",
    title: title("Supply 30d Change"),
    bottom: [
      baseline({
        metric: cohort.tree.supply.delta.change._1m,
        name: "30d Change",
        unit: Unit.sats,
      }),
    ],
  };
}

/**
 * @param {UtxoCohortObject | CohortWithoutRelative} cohort
 * @param {(metric: string) => string} title
 * @returns {PartialChartOption}
 */
function single30dUtxoCountChangeChart(cohort, title) {
  return {
    name: "UTXO Count",
    title: title("UTXO Count 30d Change"),
    bottom: [
      baseline({
        metric: cohort.tree.outputs.unspentCount.delta.change._1m,
        name: "30d Change",
        unit: Unit.count,
      }),
    ],
  };
}

/**
 * @param {CohortAll | CohortAddress | AddressCohortObject} cohort
 * @param {(metric: string) => string} title
 * @returns {PartialChartOption}
 */
function singleAddressCountChart(cohort, title) {
  return {
    name: "Address Count",
    title: title("Address Count"),
    bottom: [
      line({
        metric: cohort.addressCount.inner,
        name: "Address Count",
        color: cohort.color,
        unit: Unit.count,
      }),
    ],
  };
}

/**
 * @param {CohortAll | CohortAddress | AddressCohortObject} cohort
 * @param {(metric: string) => string} title
 * @returns {PartialChartOption}
 */
function single30dAddressCountChangeChart(cohort, title) {
  return {
    name: "Address Count",
    title: title("Address Count 30d Change"),
    bottom: [
      baseline({
        metric: cohort.addressCount.delta.change._1m,
        name: "30d Change",
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
 * @param {{ cohort: UtxoCohortObject | CohortWithoutRelative, title: (metric: string) => string }} args
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
        name: "30d Changes",
        tree: [
          single30dSupplyChangeChart(cohort, title),
          single30dUtxoCountChangeChart(cohort, title),
        ],
      },
    ],
  };
}

/**
 * Holdings for CohortAll (has inProfit/inLoss with relToOwn but no relToCirculating)
 * @param {{ cohort: CohortAll, title: (metric: string) => string }} args
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
        name: "30d Changes",
        tree: [
          single30dSupplyChangeChart(cohort, title),
          single30dUtxoCountChangeChart(cohort, title),
          single30dAddressCountChangeChart(cohort, title),
        ],
      },
    ],
  };
}

/**
 * Holdings with full relative metrics (relToCirculating + relToOwn)
 * For: CohortFull, CohortLongTerm (have DeltaHalfInRelTotalPattern2)
 * @param {{ cohort: CohortFull | CohortLongTerm, title: (metric: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createHoldingsSectionWithRelative({ cohort, title }) {
  const { supply } = cohort.tree;
  return {
    name: "Holdings",
    tree: [
      {
        name: "Supply",
        title: title("Supply"),
        bottom: [
          ...fullSupplySeries(supply),
          ...circulatingSupplyPctSeries(supply),
          ...ownSupplyPctSeries(supply),
        ],
      },
      singleUtxoCountChart(cohort, title),
      {
        name: "30d Changes",
        tree: [
          single30dSupplyChangeChart(cohort, title),
          single30dUtxoCountChangeChart(cohort, title),
        ],
      },
    ],
  };
}

/**
 * Holdings with inProfit/inLoss + relToCirculating (no relToOwn)
 * For: CohortWithAdjusted, CohortAgeRange (have DeltaHalfInRelTotalPattern)
 * @param {{ cohort: CohortWithAdjusted | CohortAgeRange, title: (metric: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createHoldingsSectionWithOwnSupply({ cohort, title }) {
  const { supply } = cohort.tree;
  return {
    name: "Holdings",
    tree: [
      {
        name: "Supply",
        title: title("Supply"),
        bottom: [
          ...fullSupplySeries(supply),
          ...circulatingSupplyPctSeries(supply),
        ],
      },
      singleUtxoCountChart(cohort, title),
      {
        name: "30d Changes",
        tree: [
          single30dSupplyChangeChart(cohort, title),
          single30dUtxoCountChangeChart(cohort, title),
        ],
      },
    ],
  };
}

/**
 * Holdings for CohortAddress (has inProfit/inLoss but no rel, plus address count)
 * @param {{ cohort: CohortAddress, title: (metric: string) => string }} args
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
        name: "30d Changes",
        tree: [
          single30dSupplyChangeChart(cohort, title),
          single30dUtxoCountChangeChart(cohort, title),
          single30dAddressCountChangeChart(cohort, title),
        ],
      },
    ],
  };
}

/**
 * Holdings for address amount cohorts (no inProfit/inLoss, has address count)
 * @param {{ cohort: AddressCohortObject, title: (metric: string) => string }} args
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
        name: "30d Changes",
        tree: [
          single30dSupplyChangeChart(cohort, title),
          single30dUtxoCountChangeChart(cohort, title),
          single30dAddressCountChangeChart(cohort, title),
        ],
      },
    ],
  };
}

// ============================================================================
// Grouped Cohort Holdings Sections
// ============================================================================

/**
 * @param {{ list: readonly CohortAddress[], all: CohortAll, title: (metric: string) => string }} args
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
          line({ metric: addressCount.inner, name, color, unit: Unit.count }),
        ),
      },
      {
        name: "30d Changes",
        tree: [
          grouped30dSupplyChangeChart(list, all, title),
          grouped30dUtxoCountChangeChart(list, all, title),
          {
            name: "Address Count",
            title: title("Address Count 30d Change"),
            bottom: mapCohortsWithAll(list, all, ({ name, color, addressCount }) =>
              baseline({
                metric: addressCount.delta.change._1m,
                name,
                unit: Unit.count,
                color,
              }),
            ),
          },
        ],
      },
    ],
  };
}

/**
 * Grouped holdings for address amount cohorts (no inProfit/inLoss, has address count)
 * @param {{ list: readonly AddressCohortObject[], all: CohortAll, title: (metric: string) => string }} args
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
          line({ metric: addressCount.inner, name, color, unit: Unit.count }),
        ),
      },
      {
        name: "30d Changes",
        tree: [
          grouped30dSupplyChangeChart(list, all, title),
          grouped30dUtxoCountChangeChart(list, all, title),
          {
            name: "Address Count",
            title: title("Address Count 30d Change"),
            bottom: mapCohortsWithAll(list, all, ({ name, color, addressCount }) =>
              baseline({
                metric: addressCount.delta.change._1m,
                name,
                unit: Unit.count,
                color,
              }),
            ),
          },
        ],
      },
    ],
  };
}

/**
 * Basic grouped holdings (total + half only)
 * @param {{ list: readonly (UtxoCohortObject | CohortWithoutRelative)[], all: CohortAll, title: (metric: string) => string }} args
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
        name: "30d Changes",
        tree: [
          grouped30dSupplyChangeChart(list, all, title),
          grouped30dUtxoCountChangeChart(list, all, title),
        ],
      },
    ],
  };
}

/**
 * Grouped holdings with inProfit/inLoss + relToCirculating (no relToOwn)
 * For: CohortWithAdjusted, CohortAgeRange
 * @param {{ list: readonly (CohortWithAdjusted | CohortAgeRange)[], all: CohortAll, title: (metric: string) => string }} args
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
                  metric: tree.supply.relToCirculating.percent,
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
                  metric: tree.supply.inProfit.relToCirculating.percent,
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
                  metric: tree.supply.inLoss.relToCirculating.percent,
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
        name: "30d Changes",
        tree: [
          grouped30dSupplyChangeChart(list, all, title),
          grouped30dUtxoCountChangeChart(list, all, title),
        ],
      },
    ],
  };
}

/**
 * Grouped holdings with full relative metrics (relToCirculating + relToOwn)
 * For: CohortFull, CohortLongTerm
 * @param {{ list: readonly (CohortFull | CohortLongTerm)[], all: CohortAll, title: (metric: string) => string }} args
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
                  metric: tree.supply.relToCirculating.percent,
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
                  metric: tree.supply.inProfit.relToCirculating.percent,
                  name,
                  color,
                  unit: Unit.pctSupply,
                }),
              ),
              ...mapCohortsWithAll(list, all, ({ name, color, tree }) =>
                line({
                  metric: tree.supply.inProfit.relToOwn.percent,
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
                  metric: tree.supply.inLoss.relToCirculating.percent,
                  name,
                  color,
                  unit: Unit.pctSupply,
                }),
              ),
              ...mapCohortsWithAll(list, all, ({ name, color, tree }) =>
                line({
                  metric: tree.supply.inLoss.relToOwn.percent,
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
        name: "30d Changes",
        tree: [
          grouped30dSupplyChangeChart(list, all, title),
          grouped30dUtxoCountChangeChart(list, all, title),
        ],
      },
    ],
  };
}
