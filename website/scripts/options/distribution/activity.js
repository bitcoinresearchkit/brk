/**
 * Activity section builders
 *
 * Structure:
 * - Volume: Sent volume (Sum, Cumulative, 14d EMA)
 * - SOPR: Spent Output Profit Ratio (30d > 7d > raw)
 * - Sell Side Risk: Risk ratio
 * - Value: Flows, Created & Destroyed, Breakdown
 * - Coins Destroyed: Coinblocks/Coindays (Sum, Cumulative)
 *
 * For cohorts WITH adjusted values: Additional Normal/Adjusted sub-sections
 */

import { Unit } from "../../utils/units.js";
import { line, baseline, dotsBaseline, dots } from "../series.js";
import { satsBtcUsd, mapCohortsWithAll, flatMapCohortsWithAll } from "../shared.js";
import { colors } from "../../utils/colors.js";

// ============================================================================
// Shared Helpers
// ============================================================================

/**
 * Create SOPR series from realized pattern (30d > 7d > raw order)
 * @param {{ sopr: AnyMetricPattern, sopr7dEma: AnyMetricPattern, sopr30dEma: AnyMetricPattern }} realized
 * @param {string} rawName - Name for the raw SOPR series
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function soprSeries(realized, rawName = "SOPR") {
  return [
    baseline({
      metric: realized.sopr30dEma,
      name: "30d EMA",
      color: colors.bi.p3,
      unit: Unit.ratio,
      base: 1,
    }),
    baseline({
      metric: realized.sopr7dEma,
      name: "7d EMA",
      color: colors.bi.p2,
      unit: Unit.ratio,
      base: 1,
    }),
    dotsBaseline({
      metric: realized.sopr,
      name: rawName,
      color: colors.bi.p1,
      unit: Unit.ratio,
      base: 1,
    }),
  ];
}

/**
 * Create grouped SOPR chart entries (Raw, 7d EMA, 30d EMA)
 * @template {{ color: Color, name: string }} T
 * @param {readonly T[]} list
 * @param {T} all
 * @param {(item: T) => AnyMetricPattern} getSopr
 * @param {(item: T) => AnyMetricPattern} getSopr7d
 * @param {(item: T) => AnyMetricPattern} getSopr30d
 * @param {(metric: string) => string} title
 * @param {string} titlePrefix
 * @returns {PartialOptionsTree}
 */
function groupedSoprCharts(
  list,
  all,
  getSopr,
  getSopr7d,
  getSopr30d,
  title,
  titlePrefix,
) {
  return [
    {
      name: "Raw",
      title: title(`${titlePrefix}SOPR`),
      bottom: mapCohortsWithAll(list, all, (item) =>
        baseline({
          metric: getSopr(item),
          name: item.name,
          color: item.color,
          unit: Unit.ratio,
          base: 1,
        }),
      ),
    },
    {
      name: "7d EMA",
      title: title(`${titlePrefix}SOPR 7d EMA`),
      bottom: mapCohortsWithAll(list, all, (item) =>
        baseline({
          metric: getSopr7d(item),
          name: item.name,
          color: item.color,
          unit: Unit.ratio,
          base: 1,
        }),
      ),
    },
    {
      name: "30d EMA",
      title: title(`${titlePrefix}SOPR 30d EMA`),
      bottom: mapCohortsWithAll(list, all, (item) =>
        baseline({
          metric: getSopr30d(item),
          name: item.name,
          color: item.color,
          unit: Unit.ratio,
          base: 1,
        }),
      ),
    },
  ];
}

/**
 * Create value breakdown tree (Profit/Loss Created/Destroyed)
 * @template {{ color: Color, name: string, tree: { realized: AnyRealizedPattern } }} T
 * @param {readonly T[]} list
 * @param {T} all
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsTree}
 */
function valueBreakdownTree(list, all, title) {
  return [
    {
      name: "Profit",
      tree: [
        {
          name: "Created",
          title: title("Profit Value Created"),
          bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
            line({
              metric: tree.realized.profitValueCreated,
              name,
              color,
              unit: Unit.usd,
            }),
          ),
        },
        {
          name: "Destroyed",
          title: title("Profit Value Destroyed"),
          bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
            line({
              metric: tree.realized.profitValueDestroyed,
              name,
              color,
              unit: Unit.usd,
            }),
          ),
        },
      ],
    },
    {
      name: "Loss",
      tree: [
        {
          name: "Created",
          title: title("Loss Value Created"),
          bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
            line({
              metric: tree.realized.lossValueCreated,
              name,
              color,
              unit: Unit.usd,
            }),
          ),
        },
        {
          name: "Destroyed",
          title: title("Loss Value Destroyed"),
          bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
            line({
              metric: tree.realized.lossValueDestroyed,
              name,
              color,
              unit: Unit.usd,
            }),
          ),
        },
      ],
    },
  ];
}

/**
 * Create coins destroyed tree (Sum/Cumulative with Coinblocks/Coindays)
 * @template {{ color: Color, name: string, tree: { activity: { coinblocksDestroyed: CountPattern<any>, coindaysDestroyed: CountPattern<any> } } }} T
 * @param {readonly T[]} list
 * @param {T} all
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsTree}
 */
function coinsDestroyedTree(list, all, title) {
  return [
    {
      name: "Sum",
      title: title("Coins Destroyed"),
      bottom: flatMapCohortsWithAll(list, all, ({ name, color, tree }) => [
        line({
          metric: tree.activity.coinblocksDestroyed.sum,
          name,
          color,
          unit: Unit.coinblocks,
        }),
        line({
          metric: tree.activity.coindaysDestroyed.sum,
          name,
          color,
          unit: Unit.coindays,
        }),
      ]),
    },
    {
      name: "Cumulative",
      title: title("Cumulative Coins Destroyed"),
      bottom: flatMapCohortsWithAll(list, all, ({ name, color, tree }) => [
        line({
          metric: tree.activity.coinblocksDestroyed.cumulative,
          name,
          color,
          unit: Unit.coinblocks,
        }),
        line({
          metric: tree.activity.coindaysDestroyed.cumulative,
          name,
          color,
          unit: Unit.coindays,
        }),
      ]),
    },
  ];
}

// ============================================================================
// SOPR Helpers
// ============================================================================

/**
 * Create SOPR series for single cohort (30d > 7d > raw order)
 * @param {UtxoCohortObject | CohortWithoutRelative} cohort
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function createSingleSoprSeries(cohort) {
  return soprSeries(cohort.tree.realized);
}

/**
 * Create SOPR tree with normal and adjusted sub-sections
 * @param {CohortAll | CohortFull | CohortWithAdjusted} cohort
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsTree}
 */
function createSingleSoprTreeWithAdjusted(cohort, title) {
  const { realized } = cohort.tree;
  return [
    {
      name: "Normal",
      title: title("SOPR"),
      bottom: soprSeries(realized),
    },
    {
      name: "Adjusted",
      title: title("Adjusted SOPR"),
      bottom: soprSeries(
        {
          sopr: realized.adjustedSopr,
          sopr7dEma: realized.adjustedSopr7dEma,
          sopr30dEma: realized.adjustedSopr30dEma,
        },
        "Adjusted SOPR",
      ),
    },
  ];
}

/**
 * Create grouped SOPR tree with separate charts for each variant
 * @param {readonly (UtxoCohortObject | CohortWithoutRelative)[]} list
 * @param {CohortAll} all
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsTree}
 */
function createGroupedSoprTree(list, all, title) {
  return groupedSoprCharts(
    list,
    all,
    (c) => c.tree.realized.sopr,
    (c) => c.tree.realized.sopr7dEma,
    (c) => c.tree.realized.sopr30dEma,
    title,
    "",
  );
}

/**
 * Create grouped SOPR tree with Normal and Adjusted sub-sections
 * @param {readonly (CohortAll | CohortFull | CohortWithAdjusted)[]} list
 * @param {CohortAll | CohortFull | CohortWithAdjusted} all
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsTree}
 */
function createGroupedSoprTreeWithAdjusted(list, all, title) {
  return [
    {
      name: "Normal",
      tree: groupedSoprCharts(
        list,
        all,
        (c) => c.tree.realized.sopr,
        (c) => c.tree.realized.sopr7dEma,
        (c) => c.tree.realized.sopr30dEma,
        title,
        "",
      ),
    },
    {
      name: "Adjusted",
      tree: groupedSoprCharts(
        list,
        all,
        (c) => c.tree.realized.adjustedSopr,
        (c) => c.tree.realized.adjustedSopr7dEma,
        (c) => c.tree.realized.adjustedSopr30dEma,
        title,
        "Adjusted ",
      ),
    },
  ];
}

// ============================================================================
// Single Cohort Activity Section
// ============================================================================

/**
 * Base activity section builder for single cohorts
 * @param {Object} args
 * @param {UtxoCohortObject | CohortWithoutRelative} args.cohort
 * @param {(metric: string) => string} args.title
 * @param {AnyFetchedSeriesBlueprint[]} [args.valueMetrics] - Optional additional value metrics
 * @param {PartialOptionsTree} [args.soprTree] - Optional SOPR tree override
 * @returns {PartialOptionsGroup}
 */
export function createActivitySection({
  cohort,
  title,
  valueMetrics = [],
  soprTree,
}) {
  const { tree, color } = cohort;

  return {
    name: "Activity",
    tree: [
      {
        name: "Volume",
        tree: [
          {
            name: "Sum",
            title: title("Sent Volume"),
            bottom: [
              line({
                metric: tree.activity.sent14dEma.sats,
                name: "14d EMA",
                color: colors.indicator.main,
                unit: Unit.sats,
                defaultActive: false,
              }),
              line({
                metric: tree.activity.sent14dEma.bitcoin,
                name: "14d EMA",
                color: colors.indicator.main,
                unit: Unit.btc,
                defaultActive: false,
              }),
              line({
                metric: tree.activity.sent14dEma.dollars,
                name: "14d EMA",
                color: colors.indicator.main,
                unit: Unit.usd,
                defaultActive: false,
              }),
              line({
                metric: tree.activity.sent.sats.sum,
                name: "sum",
                color,
                unit: Unit.sats,
              }),
              line({
                metric: tree.activity.sent.bitcoin.sum,
                name: "sum",
                color,
                unit: Unit.btc,
              }),
              line({
                metric: tree.activity.sent.dollars.sum,
                name: "sum",
                color,
                unit: Unit.usd,
              }),
            ],
          },
          {
            name: "Cumulative",
            title: title("Sent Volume (Total)"),
            bottom: [
              line({
                metric: tree.activity.sent.sats.cumulative,
                name: "all-time",
                color,
                unit: Unit.sats,
              }),
              line({
                metric: tree.activity.sent.bitcoin.cumulative,
                name: "all-time",
                color,
                unit: Unit.btc,
              }),
              line({
                metric: tree.activity.sent.dollars.cumulative,
                name: "all-time",
                color,
                unit: Unit.usd,
              }),
            ],
          },
        ],
      },
      soprTree
        ? { name: "SOPR", tree: soprTree }
        : {
            name: "SOPR",
            title: title("SOPR"),
            bottom: createSingleSoprSeries(cohort),
          },
      {
        name: "Sell Side Risk",
        title: title("Sell Side Risk Ratio"),
        bottom: createSingleSellSideRiskSeries(tree),
      },
      {
        name: "Value",
        tree: [
          {
            name: "Flows",
            title: title("Profit & Capitulation Flows"),
            bottom: createSingleCapitulationProfitFlowSeries(tree),
          },
          {
            name: "Created & Destroyed",
            title: title("Value Created & Destroyed"),
            bottom: [
              ...createSingleValueCreatedDestroyedSeries(tree),
              ...valueMetrics,
            ],
          },
          {
            name: "Breakdown",
            tree: [
              {
                name: "Profit",
                title: title("Profit Value Created & Destroyed"),
                bottom: [
                  line({
                    metric: tree.realized.profitValueCreated,
                    name: "Created",
                    color: colors.profit,
                    unit: Unit.usd,
                  }),
                  line({
                    metric: tree.realized.profitValueDestroyed,
                    name: "Destroyed",
                    color: colors.loss,
                    unit: Unit.usd,
                  }),
                ],
              },
              {
                name: "Loss",
                title: title("Loss Value Created & Destroyed"),
                bottom: [
                  line({
                    metric: tree.realized.lossValueCreated,
                    name: "Created",
                    color: colors.profit,
                    unit: Unit.usd,
                  }),
                  line({
                    metric: tree.realized.lossValueDestroyed,
                    name: "Destroyed",
                    color: colors.loss,
                    unit: Unit.usd,
                  }),
                ],
              },
            ],
          },
        ],
      },
      {
        name: "Coins Destroyed",
        tree: [
          {
            name: "Sum",
            title: title("Coins Destroyed"),
            bottom: [
              line({
                metric: tree.activity.coinblocksDestroyed.sum,
                name: "Coinblocks",
                color,
                unit: Unit.coinblocks,
              }),
              line({
                metric: tree.activity.coindaysDestroyed.sum,
                name: "Coindays",
                color,
                unit: Unit.coindays,
              }),
            ],
          },
          {
            name: "Cumulative",
            title: title("Cumulative Coins Destroyed"),
            bottom: [
              line({
                metric: tree.activity.coinblocksDestroyed.cumulative,
                name: "Coinblocks",
                color,
                unit: Unit.coinblocks,
              }),
              line({
                metric: tree.activity.coindaysDestroyed.cumulative,
                name: "Coindays",
                color,
                unit: Unit.coindays,
              }),
            ],
          },
        ],
      },
    ],
  };
}

/**
 * Activity section with adjusted values (for cohorts with RealizedPattern3/4)
 * @param {{ cohort: CohortAll | CohortFull | CohortWithAdjusted, title: (metric: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createActivitySectionWithAdjusted({ cohort, title }) {
  const { tree } = cohort;
  return createActivitySection({
    cohort,
    title,
    soprTree: createSingleSoprTreeWithAdjusted(cohort, title),
    valueMetrics: [
      line({
        metric: tree.realized.adjustedValueCreated,
        name: "Adjusted Created",
        color: colors.adjustedCreated,
        unit: Unit.usd,
        defaultActive: false,
      }),
      line({
        metric: tree.realized.adjustedValueDestroyed,
        name: "Adjusted Destroyed",
        color: colors.adjustedDestroyed,
        unit: Unit.usd,
        defaultActive: false,
      }),
    ],
  });
}

// ============================================================================
// Grouped Cohort Activity Section
// ============================================================================

/**
 * Create grouped flows tree (Profit Flow, Capitulation Flow)
 * @template {{ color: Color, name: string, tree: { realized: AnyRealizedPattern } }} T
 * @param {readonly T[]} list
 * @param {T} all
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsTree}
 */
function groupedFlowsTree(list, all, title) {
  return [
    {
      name: "Profit",
      title: title("Profit Flow"),
      bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
        line({
          metric: tree.realized.profitFlow,
          name,
          color,
          unit: Unit.usd,
        }),
      ),
    },
    {
      name: "Capitulation",
      title: title("Capitulation Flow"),
      bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
        line({
          metric: tree.realized.capitulationFlow,
          name,
          color,
          unit: Unit.usd,
        }),
      ),
    },
  ];
}

/**
 * Create grouped value tree (Flows, Created, Destroyed, Breakdown)
 * @template {{ color: Color, name: string, tree: { realized: AnyRealizedPattern } }} T
 * @param {readonly T[]} list
 * @param {T} all
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsTree}
 */
function createGroupedValueTree(list, all, title) {
  return [
    { name: "Flows", tree: groupedFlowsTree(list, all, title) },
    {
      name: "Created",
      title: title("Value Created"),
      bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
        line({
          metric: tree.realized.valueCreated,
          name,
          color,
          unit: Unit.usd,
        }),
      ),
    },
    {
      name: "Destroyed",
      title: title("Value Destroyed"),
      bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
        line({
          metric: tree.realized.valueDestroyed,
          name,
          color,
          unit: Unit.usd,
        }),
      ),
    },
    { name: "Breakdown", tree: valueBreakdownTree(list, all, title) },
  ];
}

/**
 * Grouped activity section builder
 * @param {{ list: readonly (UtxoCohortObject | CohortWithoutRelative)[], all: CohortAll, title: (metric: string) => string, soprTree?: PartialOptionsTree, valueTree?: PartialOptionsTree }} args
 * @returns {PartialOptionsGroup}
 */
export function createGroupedActivitySection({
  list,
  all,
  title,
  soprTree,
  valueTree,
}) {
  return {
    name: "Activity",
    tree: [
      {
        name: "Volume",
        tree: [
          {
            name: "14d EMA",
            title: title("Sent Volume 14d EMA"),
            bottom: flatMapCohortsWithAll(list, all, ({ name, color, tree }) =>
              satsBtcUsd({ pattern: tree.activity.sent14dEma, name, color }),
            ),
          },
          {
            name: "Sum",
            title: title("Sent Volume"),
            bottom: flatMapCohortsWithAll(list, all, ({ name, color, tree }) =>
              satsBtcUsd({
                pattern: {
                  sats: tree.activity.sent.sats.sum,
                  bitcoin: tree.activity.sent.bitcoin.sum,
                  dollars: tree.activity.sent.dollars.sum,
                },
                name,
                color,
              }),
            ),
          },
        ],
      },
      {
        name: "SOPR",
        tree: soprTree ?? createGroupedSoprTree(list, all, title),
      },
      {
        name: "Sell Side Risk",
        title: title("Sell Side Risk Ratio"),
        bottom: createGroupedSellSideRiskSeries(list, all),
      },
      {
        name: "Value",
        tree: valueTree ?? createGroupedValueTree(list, all, title),
      },
      { name: "Coins Destroyed", tree: coinsDestroyedTree(list, all, title) },
    ],
  };
}

/**
 * Create grouped value tree with adjusted values (Flows, Normal, Adjusted, Breakdown)
 * @param {readonly (CohortAll | CohortFull | CohortWithAdjusted)[]} list
 * @param {CohortAll | CohortFull | CohortWithAdjusted} all
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsTree}
 */
function createGroupedValueTreeWithAdjusted(list, all, title) {
  return [
    { name: "Flows", tree: groupedFlowsTree(list, all, title) },
    {
      name: "Normal",
      tree: [
        {
          name: "Created",
          title: title("Value Created"),
          bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
            line({
              metric: tree.realized.valueCreated,
              name,
              color,
              unit: Unit.usd,
            }),
          ),
        },
        {
          name: "Destroyed",
          title: title("Value Destroyed"),
          bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
            line({
              metric: tree.realized.valueDestroyed,
              name,
              color,
              unit: Unit.usd,
            }),
          ),
        },
      ],
    },
    {
      name: "Adjusted",
      tree: [
        {
          name: "Created",
          title: title("Adjusted Value Created"),
          bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
            line({
              metric: tree.realized.adjustedValueCreated,
              name,
              color,
              unit: Unit.usd,
            }),
          ),
        },
        {
          name: "Destroyed",
          title: title("Adjusted Value Destroyed"),
          bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
            line({
              metric: tree.realized.adjustedValueDestroyed,
              name,
              color,
              unit: Unit.usd,
            }),
          ),
        },
      ],
    },
    { name: "Breakdown", tree: valueBreakdownTree(list, all, title) },
  ];
}

/**
 * Grouped activity section with adjusted values (for cohorts with RealizedPattern3/4)
 * @param {{ list: readonly (CohortAll | CohortFull | CohortWithAdjusted)[], all: CohortAll, title: (metric: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createGroupedActivitySectionWithAdjusted({ list, all, title }) {
  return createGroupedActivitySection({
    list,
    all,
    title,
    soprTree: createGroupedSoprTreeWithAdjusted(list, all, title),
    valueTree: createGroupedValueTreeWithAdjusted(list, all, title),
  });
}

/**
 * Create sell side risk ratio series for single cohort
 * @param {{ realized: AnyRealizedPattern }} tree
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function createSingleSellSideRiskSeries(tree) {
  return [
    line({
      metric: tree.realized.sellSideRiskRatio30dEma,
      name: "30d EMA",
      color: colors.time._1m,
      unit: Unit.ratio,
    }),
    line({
      metric: tree.realized.sellSideRiskRatio7dEma,
      name: "7d EMA",
      color: colors.time._1w,
      unit: Unit.ratio,
    }),
    dots({
      metric: tree.realized.sellSideRiskRatio,
      name: "Raw",
      color: colors.bitcoin,
      unit: Unit.ratio,
    }),
  ];
}

/**
 * Create sell side risk ratio series for grouped cohorts
 * @param {readonly CohortObject[]} list
 * @param {CohortObject} all
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function createGroupedSellSideRiskSeries(list, all) {
  return flatMapCohortsWithAll(list, all, ({ name, color, tree }) => [
    line({
      metric: tree.realized.sellSideRiskRatio,
      name,
      color,
      unit: Unit.ratio,
    }),
  ]);
}

/**
 * Create value created & destroyed series for single cohort
 * @param {{ realized: AnyRealizedPattern }} tree
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function createSingleValueCreatedDestroyedSeries(tree) {
  return [
    line({
      metric: tree.realized.valueCreated,
      name: "Created",
      color: colors.usd,
      unit: Unit.usd,
    }),
    line({
      metric: tree.realized.valueDestroyed,
      name: "Destroyed",
      color: colors.loss,
      unit: Unit.usd,
    }),
  ];
}

/**
 * Create capitulation & profit flow series for single cohort
 * @param {{ realized: AnyRealizedPattern }} tree
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function createSingleCapitulationProfitFlowSeries(tree) {
  return [
    line({
      metric: tree.realized.profitFlow,
      name: "Profit Flow",
      color: colors.profit,
      unit: Unit.usd,
    }),
    line({
      metric: tree.realized.capitulationFlow,
      name: "Capitulation Flow",
      color: colors.loss,
      unit: Unit.usd,
    }),
  ];
}
