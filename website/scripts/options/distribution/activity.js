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
} from "../shared.js";
import { colors } from "../../utils/colors.js";

// ============================================================================
// Shared Volume Helpers
// ============================================================================

/**
 * Volume folder with optional profitability (in profit + in loss per window)
 * @param {{ transferVolume: TransferVolumePattern }} activity
 * @param {Color} color
 * @param {(name: string) => string} title
 * @param {boolean} [withProfitability]
 * @returns {PartialOptionsGroup}
 */
function volumeFolder(activity, color, title, withProfitability) {
  const tv = activity.transferVolume;
  return {
    name: "Volume",
    tree: [
      ...satsBtcUsdFullTree({ pattern: tv, title: title("Sent Volume"), color }),
      ...(withProfitability ? [{
        name: "Profitability",
        tree: ROLLING_WINDOWS.map((w) => ({
          name: w.name,
          title: title(`Sent Volume Profitability (${w.title})`),
          bottom: [
            ...satsBtcUsd({ pattern: tv.inProfit.sum[w.key], name: "In Profit", color: colors.profit }),
            ...satsBtcUsd({ pattern: tv.inLoss.sum[w.key], name: "In Loss", color: colors.loss }),
          ],
        })),
      }] : []),
    ],
  };
}

/**
 * Full activity items: volume (with profitability), coindays, dormancy
 * @param {FullActivityPattern} activity
 * @param {Color} color
 * @param {(name: string) => string} title
 * @returns {PartialOptionsTree}
 */
function fullVolumeTree(activity, color, title) {
  return [
    volumeFolder(activity, color, title, true),
    {
      name: "Coindays Destroyed",
      tree: chartsFromCount({
        pattern: activity.coindaysDestroyed,
        title: title("Coindays Destroyed"),
        unit: Unit.coindays,
        color,
      }),
    },
    {
      name: "Dormancy",
      tree: averagesArray({
        windows: activity.dormancy,
        title: title("Dormancy"),
        unit: Unit.days,
      }),
    },
  ];
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
      bottom: percentRatio({ pattern: sellSideRisk[w.key], name: "Risk", color: w.color }),
    })),
  ];
}

// ============================================================================
// Single Cohort Activity Sections
// ============================================================================

/**
 * Full activity with adjusted SOPR (All/STH)
 * @param {{ cohort: CohortAll | CohortFull, title: (name: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createActivitySectionWithAdjusted({ cohort, title }) {
  const { tree, color } = cohort;
  const r = tree.realized;
  const sopr = r.sopr;

  return {
    name: "Activity",
    tree: [
      ...fullVolumeTree(tree.activity, color, title),
      {
        name: "SOPR",
        tree: [
          ...singleRollingSoprTree(sopr.ratio, title),
          {
            name: "Adjusted",
            tree: singleRollingSoprTree(
              sopr.adjusted.ratio,
              title,
              "Adjusted ",
            ),
          },
        ],
      },
      {
        name: "Sell Side Risk",
        tree: singleSellSideRiskTree(r.sellSideRiskRatio, title),
      },
    ],
  };
}

/**
 * Activity section for cohorts with rolling SOPR + sell side risk (LTH, also CohortFull | CohortLongTerm)
 * @param {{ cohort: CohortFull | CohortLongTerm, title: (name: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createActivitySection({ cohort, title }) {
  const { tree, color } = cohort;
  const r = tree.realized;
  const sopr = r.sopr;

  return {
    name: "Activity",
    tree: [
      ...fullVolumeTree(tree.activity, color, title),
      {
        name: "SOPR",
        tree: singleRollingSoprTree(sopr.ratio, title),
      },
      {
        name: "Sell Side Risk",
        tree: singleSellSideRiskTree(r.sellSideRiskRatio, title),
      },
    ],
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
      volumeFolder(tree.activity, color, title, true),
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
        satsBtcUsd({ pattern: tree.activity.transferVolume.sum[w.key], name, color }),
      ),
    })),
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
 * @param {{ list: readonly CohortFull[], all: CohortAll, title: (name: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createGroupedActivitySectionWithAdjusted({ list, all, title }) {
  return {
    name: "Activity",
    tree: [
      {
        name: "Volume",
        tree: [
          ...ROLLING_WINDOWS.map((w) => ({
            name: w.name,
            title: title(`Sent Volume (${w.title})`),
            bottom: flatMapCohortsWithAll(list, all, ({ name, color, tree }) =>
              satsBtcUsd({ pattern: tree.activity.transferVolume.sum[w.key], name, color }),
            ),
          })),
          {
            name: "Cumulative",
            title: title("Cumulative Sent Volume"),
            bottom: flatMapCohortsWithAll(list, all, ({ name, color, tree }) =>
              satsBtcUsd({ pattern: tree.activity.transferVolume.cumulative, name, color }),
            ),
          },
          {
            name: "In Profit",
            tree: [
              ...ROLLING_WINDOWS.map((w) => ({
                name: w.name,
                title: title(`Sent In Profit (${w.title})`),
                bottom: flatMapCohortsWithAll(list, all, ({ name, color, tree }) =>
                  satsBtcUsd({ pattern: tree.activity.transferVolume.inProfit.sum[w.key], name, color }),
                ),
              })),
              {
                name: "Cumulative",
                title: title("Cumulative Sent In Profit"),
                bottom: flatMapCohortsWithAll(list, all, ({ name, color, tree }) =>
                  satsBtcUsd({ pattern: tree.activity.transferVolume.inProfit.cumulative, name, color }),
                ),
              },
            ],
          },
          {
            name: "In Loss",
            tree: [
              ...ROLLING_WINDOWS.map((w) => ({
                name: w.name,
                title: title(`Sent In Loss (${w.title})`),
                bottom: flatMapCohortsWithAll(list, all, ({ name, color, tree }) =>
                  satsBtcUsd({ pattern: tree.activity.transferVolume.inLoss.sum[w.key], name, color }),
                ),
              })),
              {
                name: "Cumulative",
                title: title("Cumulative Sent In Loss"),
                bottom: flatMapCohortsWithAll(list, all, ({ name, color, tree }) =>
                  satsBtcUsd({ pattern: tree.activity.transferVolume.inLoss.cumulative, name, color }),
                ),
              },
            ],
          },
        ],
      },
      {
        name: "Coindays Destroyed",
        tree: [
          ...ROLLING_WINDOWS.map((w) => ({
            name: w.name,
            title: title(`Coindays Destroyed (${w.title})`),
            bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
              line({ series: tree.activity.coindaysDestroyed.sum[w.key], name, color, unit: Unit.coindays }),
            ),
          })),
          {
            name: "Cumulative",
            title: title("Cumulative Coindays Destroyed"),
            bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
              line({ series: tree.activity.coindaysDestroyed.cumulative, name, color, unit: Unit.coindays }),
            ),
          },
        ],
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
        name: "SOPR",
        tree: [
          ...groupedSoprCharts(list, all, (c) => c.tree.realized.sopr.ratio, title),
          {
            name: "Adjusted",
            tree: groupedSoprCharts(list, all, (c) => c.tree.realized.sopr.adjusted.ratio, title, "Adjusted "),
          },
        ],
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
    ],
  };
}

/**
 * Grouped activity for cohorts with rolling SOPR + sell side risk (LTH-like)
 * @param {{ list: readonly (CohortFull | CohortLongTerm)[], all: CohortAll, title: (name: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createGroupedActivitySection({ list, all, title }) {
  return {
    name: "Activity",
    tree: [
      {
        name: "Volume",
        tree: [
          ...ROLLING_WINDOWS.map((w) => ({
            name: w.name,
            title: title(`Sent Volume (${w.title})`),
            bottom: flatMapCohortsWithAll(list, all, ({ name, color, tree }) =>
              satsBtcUsd({ pattern: tree.activity.transferVolume.sum[w.key], name, color }),
            ),
          })),
          {
            name: "Cumulative",
            title: title("Cumulative Sent Volume"),
            bottom: flatMapCohortsWithAll(list, all, ({ name, color, tree }) =>
              satsBtcUsd({ pattern: tree.activity.transferVolume.cumulative, name, color }),
            ),
          },
          {
            name: "In Profit",
            tree: [
              ...ROLLING_WINDOWS.map((w) => ({
                name: w.name,
                title: title(`Sent In Profit (${w.title})`),
                bottom: flatMapCohortsWithAll(list, all, ({ name, color, tree }) =>
                  satsBtcUsd({ pattern: tree.activity.transferVolume.inProfit.sum[w.key], name, color }),
                ),
              })),
              {
                name: "Cumulative",
                title: title("Cumulative Sent In Profit"),
                bottom: flatMapCohortsWithAll(list, all, ({ name, color, tree }) =>
                  satsBtcUsd({ pattern: tree.activity.transferVolume.inProfit.cumulative, name, color }),
                ),
              },
            ],
          },
          {
            name: "In Loss",
            tree: [
              ...ROLLING_WINDOWS.map((w) => ({
                name: w.name,
                title: title(`Sent In Loss (${w.title})`),
                bottom: flatMapCohortsWithAll(list, all, ({ name, color, tree }) =>
                  satsBtcUsd({ pattern: tree.activity.transferVolume.inLoss.sum[w.key], name, color }),
                ),
              })),
              {
                name: "Cumulative",
                title: title("Cumulative Sent In Loss"),
                bottom: flatMapCohortsWithAll(list, all, ({ name, color, tree }) =>
                  satsBtcUsd({ pattern: tree.activity.transferVolume.inLoss.cumulative, name, color }),
                ),
              },
            ],
          },
        ],
      },
      {
        name: "Coindays Destroyed",
        tree: [
          ...ROLLING_WINDOWS.map((w) => ({
            name: w.name,
            title: title(`Coindays Destroyed (${w.title})`),
            bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
              line({ series: tree.activity.coindaysDestroyed.sum[w.key], name, color, unit: Unit.coindays }),
            ),
          })),
          {
            name: "Cumulative",
            title: title("Cumulative Coindays Destroyed"),
            bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
              line({ series: tree.activity.coindaysDestroyed.cumulative, name, color, unit: Unit.coindays }),
            ),
          },
        ],
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
        name: "SOPR",
        tree: groupedSoprCharts(list, all, (c) => c.tree.realized.sopr.ratio, title),
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
    ],
  };
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
            satsBtcUsd({ pattern: tree.activity.transferVolume.sum[w.key], name, color }),
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
              line({ series: tree.activity.coindaysDestroyed.sum[w.key], name, color, unit: Unit.coindays }),
            ),
          })),
          {
            name: "Cumulative",
            title: title("Cumulative Coindays Destroyed"),
            bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
              line({ series: tree.activity.coindaysDestroyed.cumulative, name, color, unit: Unit.coindays }),
            ),
          },
        ],
      },
    ],
  };
}
