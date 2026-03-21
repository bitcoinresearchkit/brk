/**
 * Activity section builders
 *
 * Capabilities by cohort type:
 * - All/STH: activity (full), SOPR (rolling + adjusted), sell side risk, value (flows + breakdown), coins
 * - LTH: activity (full), SOPR (rolling), sell side risk, value (flows + breakdown), coins
 * - AgeRange/MaxAge: activity (basic), SOPR (24h only), value (no flows/breakdown), coins
 * - Others (UtxoAmount, Empty, Address): no activity, value only
 */

import { Unit } from "../../utils/units.js";
import {
  line,
  baseline,
  dotsBaseline,
  percentRatio,
  chartsFromCount,
  averagesArray,
  ROLLING_WINDOWS,
} from "../series.js";
import {
  satsBtcUsd,
  satsBtcUsdFullTree,
  mapCohortsWithAll,
  flatMapCohortsWithAll,
  groupedWindowsCumulative,
  groupedWindowsCumulativeSatsBtcUsd,
} from "../shared.js";
import { colors } from "../../utils/colors.js";

// ============================================================================
// Shared Volume Helpers
// ============================================================================

/**
 * @param {{ transferVolume: TransferVolumePattern }} activity
 * @param {Color} color
 * @param {(name: string) => string} title
 * @returns {PartialOptionsGroup}
 */
function volumeFolderWithProfitability(activity, color, title) {
  const tv = activity.transferVolume;
  return {
    name: "Volume",
    tree: [
      ...satsBtcUsdFullTree({
        pattern: tv,
        title: title("Sent Volume"),
        color,
      }),
      {
        name: "Profitability",
        tree: ROLLING_WINDOWS.map((w) => ({
          name: w.name,
          title: title(`Sent Volume Profitability (${w.title})`),
          bottom: [
            ...satsBtcUsd({
              pattern: tv.inProfit.sum[w.key],
              name: "In Profit",
              color: colors.profit,
            }),
            ...satsBtcUsd({
              pattern: tv.inLoss.sum[w.key],
              name: "In Loss",
              color: colors.loss,
            }),
          ],
        })),
      },
    ],
  };
}


// ============================================================================
// Shared SOPR Helpers
// ============================================================================

/**
 * @param {RollingWindowPattern<number>} ratio
 * @param {(name: string) => string} title
 * @param {string} [prefix]
 * @returns {PartialOptionsTree}
 */
function singleRollingSoprTree(ratio, title, prefix = "") {
  return [
    {
      name: "Compare",
      title: title(`${prefix}SOPR`),
      bottom: ROLLING_WINDOWS.map((w) =>
        baseline({
          series: ratio[w.key],
          name: w.name,
          color: w.color,
          unit: Unit.ratio,
          base: 1,
        }),
      ),
    },
    ...ROLLING_WINDOWS.map((w) => ({
      name: w.name,
      title: title(`${prefix}SOPR (${w.title})`),
      bottom: [
        baseline({
          series: ratio[w.key],
          name: "SOPR",
          unit: Unit.ratio,
          base: 1,
        }),
      ],
    })),
  ];
}

// ============================================================================
// Shared Sell Side Risk Helpers
// ============================================================================

/**
 * @param {SellSideRiskPattern} sellSideRisk
 * @param {(name: string) => string} title
 * @returns {PartialOptionsTree}
 */
function singleSellSideRiskTree(sellSideRisk, title) {
  return [
    {
      name: "Compare",
      title: title("Sell Side Risk"),
      bottom: ROLLING_WINDOWS.flatMap((w) =>
        percentRatio({
          pattern: sellSideRisk[w.key],
          name: w.name,
          color: w.color,
        }),
      ),
    },
    ...ROLLING_WINDOWS.map((w) => ({
      name: w.name,
      title: title(`Sell Side Risk (${w.title})`),
      bottom: percentRatio({
        pattern: sellSideRisk[w.key],
        name: "Risk",
        color: w.color,
      }),
    })),
  ];
}

// ============================================================================
// Single Cohort Activity Sections
// ============================================================================

/**
 * Single activity tree items shared between WithAdjusted and basic
 * @param {CohortAll | CohortFull | CohortLongTerm} cohort
 * @param {(name: string) => string} title
 * @param {PartialOptionsGroup} soprFolder
 * @returns {PartialOptionsTree}
 */
function singleFullActivityTree(cohort, title, soprFolder) {
  const { tree, color } = cohort;
  return [
    volumeFolderWithProfitability(tree.activity, color, title),
    soprFolder,
    { name: "Coindays Destroyed", tree: chartsFromCount({ pattern: tree.activity.coindaysDestroyed, title: title("Coindays Destroyed"), unit: Unit.coindays, color }) },
    { name: "Dormancy", tree: averagesArray({ windows: tree.activity.dormancy, title: title("Dormancy"), unit: Unit.days }) },
    { name: "Sell Side Risk", tree: singleSellSideRiskTree(tree.realized.sellSideRiskRatio, title) },
  ];
}

/** @param {{ cohort: CohortAll | CohortFull, title: (name: string) => string }} args */
export function createActivitySectionWithAdjusted({ cohort, title }) {
  const sopr = cohort.tree.realized.sopr;
  return {
    name: "Activity",
    tree: singleFullActivityTree(cohort, title, {
      name: "SOPR",
      tree: [
        ...singleRollingSoprTree(sopr.ratio, title),
        { name: "Adjusted", tree: singleRollingSoprTree(sopr.adjusted.ratio, title, "Adjusted ") },
      ],
    }),
  };
}

/** @param {{ cohort: CohortFull | CohortLongTerm, title: (name: string) => string }} args */
export function createActivitySection({ cohort, title }) {
  return {
    name: "Activity",
    tree: singleFullActivityTree(cohort, title, {
      name: "SOPR",
      tree: singleRollingSoprTree(cohort.tree.realized.sopr.ratio, title),
    }),
  };
}

/**
 * Activity section for cohorts with activity but basic realized (AgeRange/MaxAge — 24h SOPR only)
 * @param {{ cohort: CohortAgeRange | CohortWithAdjusted, title: (name: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createActivitySectionWithActivity({ cohort, title }) {
  const { tree, color } = cohort;
  const sopr = tree.realized.sopr;

  return {
    name: "Activity",
    tree: [
      volumeFolderWithProfitability(tree.activity, color, title),
      {
        name: "Coindays Destroyed",
        tree: chartsFromCount({
          pattern: tree.activity.coindaysDestroyed,
          title: title("Coindays Destroyed"),
          unit: Unit.coindays,
          color,
        }),
      },
      {
        name: "SOPR",
        title: title("SOPR (24h)"),
        bottom: [
          dotsBaseline({
            series: sopr.ratio._24h,
            name: "SOPR",
            unit: Unit.ratio,
            base: 1,
          }),
        ],
      },
    ],
  };
}

/**
 * Minimal activity section: volume only
 * @param {{ cohort: CohortBasicWithMarketCap | CohortBasicWithoutMarketCap | CohortWithoutRelative | CohortAddr | AddrCohortObject, title: (name: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createActivitySectionMinimal({ cohort, title }) {
  return {
    name: "Activity",
    tree: satsBtcUsdFullTree({
      pattern: cohort.tree.activity.transferVolume,
      title: title("Volume"),
    }),
  };
}

/**
 * Grouped minimal activity: volume
 * @param {{ list: readonly (UtxoCohortObject | CohortWithoutRelative | CohortAddr | AddrCohortObject)[], all: CohortAll, title: (name: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createGroupedActivitySectionMinimal({ list, all, title }) {
  return {
    name: "Activity",
    tree: ROLLING_WINDOWS.map((w) => ({
      name: w.name,
      title: title(`Volume (${w.title})`),
      bottom: flatMapCohortsWithAll(list, all, ({ name, color, tree }) =>
        satsBtcUsd({
          pattern: tree.activity.transferVolume.sum[w.key],
          name,
          color,
        }),
      ),
    })),
  };
}

/**
 * Grouped volume folder with In Profit/Loss subfolders
 * @template {{ name: string, color: Color }} T
 * @template {{ name: string, color: Color }} A
 * @param {readonly T[]} list
 * @param {A} all
 * @param {(name: string) => string} title
 * @param {(c: T | A) => { sum: Record<string, AnyValuePattern>, cumulative: AnyValuePattern, inProfit: { sum: Record<string, AnyValuePattern>, cumulative: AnyValuePattern }, inLoss: { sum: Record<string, AnyValuePattern>, cumulative: AnyValuePattern } }} getTransferVolume
 * @returns {PartialOptionsGroup}
 */
function groupedVolumeFolder(list, all, title, getTransferVolume) {
  return {
    name: "Volume",
    tree: [
      ...groupedWindowsCumulativeSatsBtcUsd({ list, all, title, metricTitle: "Sent Volume", getMetric: (c) => getTransferVolume(c) }),
      { name: "In Profit", tree: groupedWindowsCumulativeSatsBtcUsd({ list, all, title, metricTitle: "Sent In Profit", getMetric: (c) => getTransferVolume(c).inProfit }) },
      { name: "In Loss", tree: groupedWindowsCumulativeSatsBtcUsd({ list, all, title, metricTitle: "Sent In Loss", getMetric: (c) => getTransferVolume(c).inLoss }) },
    ],
  };
}

// ============================================================================
// Grouped SOPR Helpers
// ============================================================================

/**
 * @template {{ color: Color, name: string }} T
 * @template {{ color: Color, name: string }} A
 * @param {readonly T[]} list
 * @param {A} all
 * @param {(item: T | A) => { _24h: AnySeriesPattern, _1w: AnySeriesPattern, _1m: AnySeriesPattern, _1y: AnySeriesPattern }} getRatio
 * @param {(name: string) => string} title
 * @param {string} [prefix]
 * @returns {PartialOptionsTree}
 */
function groupedSoprCharts(list, all, getRatio, title, prefix = "") {
  return ROLLING_WINDOWS.map((w) => ({
    name: w.name,
    title: title(`${prefix}SOPR (${w.title})`),
    bottom: mapCohortsWithAll(list, all, (c) =>
      baseline({
        series: getRatio(c)[w.key],
        name: c.name,
        color: c.color,
        unit: Unit.ratio,
        base: 1,
      }),
    ),
  }));
}

// ============================================================================
// Grouped Value/Flow Helpers
// ============================================================================

/**
 * @template {{ color: Color, name: string }} T
 * @template {{ color: Color, name: string }} A
 * @param {readonly T[]} list
 * @param {A} all
 * @param {readonly { name: string, title: string, getCreated: (item: T | A) => AnySeriesPattern, getDestroyed: (item: T | A) => AnySeriesPattern }[]} windows
 * @param {(name: string) => string} title
 * @param {string} [prefix]
 * @returns {PartialOptionsTree}
 */

// ============================================================================
// Grouped Activity Sections
// ============================================================================

/**
 * Grouped activity tree items shared between WithAdjusted and basic
 * @param {readonly (CohortFull | CohortLongTerm)[]} list
 * @param {CohortAll} all
 * @param {(name: string) => string} title
 * @param {PartialOptionsGroup} soprFolder
 * @returns {PartialOptionsTree}
 */
function groupedFullActivityTree(list, all, title, soprFolder) {
  return [
    groupedVolumeFolder(list, all, title, (c) => c.tree.activity.transferVolume),
    soprFolder,
    ...groupedActivitySharedItems(list, all, title),
  ];
}

/** @param {{ list: readonly CohortFull[], all: CohortAll, title: (name: string) => string }} args */
export function createGroupedActivitySectionWithAdjusted({ list, all, title }) {
  return {
    name: "Activity",
    tree: groupedFullActivityTree(list, all, title, {
      name: "SOPR",
      tree: [
        ...groupedSoprCharts(list, all, (c) => c.tree.realized.sopr.ratio, title),
        { name: "Adjusted", tree: groupedSoprCharts(list, all, (c) => c.tree.realized.sopr.adjusted.ratio, title, "Adjusted ") },
      ],
    }),
  };
}

/** @param {{ list: readonly (CohortFull | CohortLongTerm)[], all: CohortAll, title: (name: string) => string }} args */
export function createGroupedActivitySection({ list, all, title }) {
  return {
    name: "Activity",
    tree: groupedFullActivityTree(list, all, title, {
      name: "SOPR",
      tree: groupedSoprCharts(list, all, (c) => c.tree.realized.sopr.ratio, title),
    }),
  };
}

/**
 * Shared grouped activity items: coindays, dormancy, sell side risk
 * @param {readonly (CohortFull | CohortLongTerm)[]} list
 * @param {CohortAll} all
 * @param {(name: string) => string} title
 * @returns {PartialOptionsTree}
 */
function groupedActivitySharedItems(list, all, title) {
  return [
    {
      name: "Coindays Destroyed",
      tree: groupedWindowsCumulative({
        list, all, title, metricTitle: "Coindays Destroyed",
        getWindowSeries: (c, key) => c.tree.activity.coindaysDestroyed.sum[key],
        getCumulativeSeries: (c) => c.tree.activity.coindaysDestroyed.cumulative,
        seriesFn: line, unit: Unit.coindays,
      }),
    },
    {
      name: "Dormancy",
      tree: ROLLING_WINDOWS.map((w) => ({
        name: w.name,
        title: title(`Dormancy (${w.title})`),
        bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
          line({ series: tree.activity.dormancy[w.key], name, color, unit: Unit.days }),
        ),
      })),
    },
    {
      name: "Sell Side Risk",
      tree: ROLLING_WINDOWS.map((w) => ({
        name: w.name,
        title: title(`Sell Side Risk (${w.title})`),
        bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
          line({ series: tree.realized.sellSideRiskRatio[w.key].ratio, name, color, unit: Unit.ratio }),
        ),
      })),
    },
  ];
}


/**
 * Grouped activity for cohorts with activity but basic realized (AgeRange/MaxAge)
 * @param {{ list: readonly (CohortAgeRange | CohortWithAdjusted)[], all: CohortAll, title: (name: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createGroupedActivitySectionWithActivity({ list, all, title }) {
  return {
    name: "Activity",
    tree: [
      {
        name: "Volume",
        tree: ROLLING_WINDOWS.map((w) => ({
          name: w.name,
          title: title(`Sent Volume (${w.title})`),
          bottom: flatMapCohortsWithAll(list, all, ({ name, color, tree }) =>
            satsBtcUsd({
              pattern: tree.activity.transferVolume.sum[w.key],
              name,
              color,
            }),
          ),
        })),
      },
      {
        name: "SOPR",
        title: title("SOPR (24h)"),
        bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
          baseline({
            series: tree.realized.sopr.ratio._24h,
            name,
            color,
            unit: Unit.ratio,
            base: 1,
          }),
        ),
      },
      {
        name: "Coindays Destroyed",
        tree: [
          ...ROLLING_WINDOWS.map((w) => ({
            name: w.name,
            title: title(`Coindays Destroyed (${w.title})`),
            bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
              line({
                series: tree.activity.coindaysDestroyed.sum[w.key],
                name,
                color,
                unit: Unit.coindays,
              }),
            ),
          })),
          {
            name: "Cumulative",
            title: title("Cumulative Coindays Destroyed"),
            bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
              line({
                series: tree.activity.coindaysDestroyed.cumulative,
                name,
                color,
                unit: Unit.coindays,
              }),
            ),
          },
        ],
      },
    ],
  };
}
