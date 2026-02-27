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
import {
  satsBtcUsd,
  mapCohortsWithAll,
  flatMapCohortsWithAll,
} from "../shared.js";
import { colors } from "../../utils/colors.js";

// ============================================================================
// Shared Helpers
// ============================================================================

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
          metric: tree.activity.coinblocksDestroyed.sum._24h,
          name,
          color,
          unit: Unit.coinblocks,
        }),
        line({
          metric: tree.activity.coindaysDestroyed.sum._24h,
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
// Rolling Helpers
// ============================================================================

/**
 * Rolling SOPR tree for single cohort
 * @param {Object} m
 * @param {AnyMetricPattern} m.s24h
 * @param {AnyMetricPattern} m.s7d
 * @param {AnyMetricPattern} m.s30d
 * @param {AnyMetricPattern} m.s1y
 * @param {AnyMetricPattern} m.ema24h7d
 * @param {AnyMetricPattern} m.ema24h30d
 * @param {(metric: string) => string} title
 * @param {string} prefix
 * @returns {PartialOptionsTree}
 */
function singleRollingSoprTree(m, title, prefix = "") {
  return [
    {
      name: "Compare",
      title: title(`Rolling ${prefix}SOPR`),
      bottom: [
        baseline({ metric: m.s24h, name: "24h", color: colors.time._24h, unit: Unit.ratio, base: 1 }),
        baseline({ metric: m.s7d, name: "7d", color: colors.time._1w, unit: Unit.ratio, base: 1 }),
        baseline({ metric: m.s30d, name: "30d", color: colors.time._1m, unit: Unit.ratio, base: 1 }),
        baseline({ metric: m.s1y, name: "1y", color: colors.time._1y, unit: Unit.ratio, base: 1 }),
      ],
    },
    {
      name: "24h",
      title: title(`${prefix}SOPR (24h)`),
      bottom: [
        baseline({ metric: m.ema24h30d, name: "30d EMA", color: colors.bi.p3, unit: Unit.ratio, base: 1 }),
        baseline({ metric: m.ema24h7d, name: "7d EMA", color: colors.bi.p2, unit: Unit.ratio, base: 1 }),
        dotsBaseline({ metric: m.s24h, name: "24h", color: colors.bi.p1, unit: Unit.ratio, base: 1 }),
      ],
    },
    {
      name: "7d",
      title: title(`${prefix}SOPR (7d)`),
      bottom: [baseline({ metric: m.s7d, name: "SOPR", unit: Unit.ratio, base: 1 })],
    },
    {
      name: "30d",
      title: title(`${prefix}SOPR (30d)`),
      bottom: [baseline({ metric: m.s30d, name: "SOPR", unit: Unit.ratio, base: 1 })],
    },
    {
      name: "1y",
      title: title(`${prefix}SOPR (1y)`),
      bottom: [baseline({ metric: m.s1y, name: "SOPR", unit: Unit.ratio, base: 1 })],
    },
  ];
}

/**
 * Rolling sell side risk tree for single cohort
 * @param {AnyRealizedPattern} r
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsTree}
 */
function singleRollingSellSideRiskTree(r, title) {
  return [
    {
      name: "Compare",
      title: title("Rolling Sell Side Risk"),
      bottom: [
        line({ metric: r.sellSideRiskRatio24h, name: "24h", color: colors.time._24h, unit: Unit.ratio }),
        line({ metric: r.sellSideRiskRatio7d, name: "7d", color: colors.time._1w, unit: Unit.ratio }),
        line({ metric: r.sellSideRiskRatio30d, name: "30d", color: colors.time._1m, unit: Unit.ratio }),
        line({ metric: r.sellSideRiskRatio1y, name: "1y", color: colors.time._1y, unit: Unit.ratio }),
      ],
    },
    {
      name: "24h",
      title: title("Sell Side Risk (24h)"),
      bottom: [
        line({ metric: r.sellSideRiskRatio24h30dEma, name: "30d EMA", color: colors.time._1m, unit: Unit.ratio }),
        line({ metric: r.sellSideRiskRatio24h7dEma, name: "7d EMA", color: colors.time._1w, unit: Unit.ratio }),
        dots({ metric: r.sellSideRiskRatio24h, name: "Raw", color: colors.bitcoin, unit: Unit.ratio }),
      ],
    },
    {
      name: "7d",
      title: title("Sell Side Risk (7d)"),
      bottom: [line({ metric: r.sellSideRiskRatio7d, name: "Risk", unit: Unit.ratio })],
    },
    {
      name: "30d",
      title: title("Sell Side Risk (30d)"),
      bottom: [line({ metric: r.sellSideRiskRatio30d, name: "Risk", unit: Unit.ratio })],
    },
    {
      name: "1y",
      title: title("Sell Side Risk (1y)"),
      bottom: [line({ metric: r.sellSideRiskRatio1y, name: "Risk", unit: Unit.ratio })],
    },
  ];
}

/**
 * Rolling value created/destroyed tree for single cohort
 * @param {Object} m
 * @param {AnyMetricPattern} m.created24h
 * @param {AnyMetricPattern} m.created7d
 * @param {AnyMetricPattern} m.created30d
 * @param {AnyMetricPattern} m.created1y
 * @param {AnyMetricPattern} m.destroyed24h
 * @param {AnyMetricPattern} m.destroyed7d
 * @param {AnyMetricPattern} m.destroyed30d
 * @param {AnyMetricPattern} m.destroyed1y
 * @param {(metric: string) => string} title
 * @param {string} prefix
 * @returns {PartialOptionsTree}
 */
function singleRollingValueTree(m, title, prefix = "") {
  return [
    {
      name: "Compare",
      tree: [
        {
          name: "Created",
          title: title(`Rolling ${prefix}Value Created`),
          bottom: [
            line({ metric: m.created24h, name: "24h", color: colors.time._24h, unit: Unit.usd }),
            line({ metric: m.created7d, name: "7d", color: colors.time._1w, unit: Unit.usd }),
            line({ metric: m.created30d, name: "30d", color: colors.time._1m, unit: Unit.usd }),
            line({ metric: m.created1y, name: "1y", color: colors.time._1y, unit: Unit.usd }),
          ],
        },
        {
          name: "Destroyed",
          title: title(`Rolling ${prefix}Value Destroyed`),
          bottom: [
            line({ metric: m.destroyed24h, name: "24h", color: colors.time._24h, unit: Unit.usd }),
            line({ metric: m.destroyed7d, name: "7d", color: colors.time._1w, unit: Unit.usd }),
            line({ metric: m.destroyed30d, name: "30d", color: colors.time._1m, unit: Unit.usd }),
            line({ metric: m.destroyed1y, name: "1y", color: colors.time._1y, unit: Unit.usd }),
          ],
        },
      ],
    },
    {
      name: "24h",
      title: title(`${prefix}Value Created & Destroyed (24h)`),
      bottom: [
        line({ metric: m.created24h, name: "Created", color: colors.usd, unit: Unit.usd }),
        line({ metric: m.destroyed24h, name: "Destroyed", color: colors.loss, unit: Unit.usd }),
      ],
    },
    {
      name: "7d",
      title: title(`${prefix}Value Created & Destroyed (7d)`),
      bottom: [
        line({ metric: m.created7d, name: "Created", color: colors.usd, unit: Unit.usd }),
        line({ metric: m.destroyed7d, name: "Destroyed", color: colors.loss, unit: Unit.usd }),
      ],
    },
    {
      name: "30d",
      title: title(`${prefix}Value Created & Destroyed (30d)`),
      bottom: [
        line({ metric: m.created30d, name: "Created", color: colors.usd, unit: Unit.usd }),
        line({ metric: m.destroyed30d, name: "Destroyed", color: colors.loss, unit: Unit.usd }),
      ],
    },
    {
      name: "1y",
      title: title(`${prefix}Value Created & Destroyed (1y)`),
      bottom: [
        line({ metric: m.created1y, name: "Created", color: colors.usd, unit: Unit.usd }),
        line({ metric: m.destroyed1y, name: "Destroyed", color: colors.loss, unit: Unit.usd }),
      ],
    },
  ];
}

/**
 * Rolling SOPR charts for grouped cohorts
 * @template {{ color: Color, name: string }} T
 * @param {readonly T[]} list
 * @param {T} all
 * @param {(item: T) => AnyMetricPattern} get24h
 * @param {(item: T) => AnyMetricPattern} get7d
 * @param {(item: T) => AnyMetricPattern} get30d
 * @param {(item: T) => AnyMetricPattern} get1y
 * @param {(metric: string) => string} title
 * @param {string} prefix
 * @returns {PartialOptionsTree}
 */
function groupedRollingSoprCharts(list, all, get24h, get7d, get30d, get1y, title, prefix = "") {
  return [
    {
      name: "24h",
      title: title(`${prefix}SOPR (24h)`),
      bottom: mapCohortsWithAll(list, all, (c) =>
        baseline({ metric: get24h(c), name: c.name, color: c.color, unit: Unit.ratio, base: 1 }),
      ),
    },
    {
      name: "7d",
      title: title(`${prefix}SOPR (7d)`),
      bottom: mapCohortsWithAll(list, all, (c) =>
        baseline({ metric: get7d(c), name: c.name, color: c.color, unit: Unit.ratio, base: 1 }),
      ),
    },
    {
      name: "30d",
      title: title(`${prefix}SOPR (30d)`),
      bottom: mapCohortsWithAll(list, all, (c) =>
        baseline({ metric: get30d(c), name: c.name, color: c.color, unit: Unit.ratio, base: 1 }),
      ),
    },
    {
      name: "1y",
      title: title(`${prefix}SOPR (1y)`),
      bottom: mapCohortsWithAll(list, all, (c) =>
        baseline({ metric: get1y(c), name: c.name, color: c.color, unit: Unit.ratio, base: 1 }),
      ),
    },
  ];
}

/**
 * Rolling sell side risk charts for grouped cohorts
 * @param {readonly CohortObject[]} list
 * @param {CohortObject} all
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsTree}
 */
function groupedRollingSellSideRiskCharts(list, all, title) {
  return [
    {
      name: "24h",
      title: title("Sell Side Risk (24h)"),
      bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
        line({ metric: tree.realized.sellSideRiskRatio24h, name, color, unit: Unit.ratio }),
      ),
    },
    {
      name: "7d",
      title: title("Sell Side Risk (7d)"),
      bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
        line({ metric: tree.realized.sellSideRiskRatio7d, name, color, unit: Unit.ratio }),
      ),
    },
    {
      name: "30d",
      title: title("Sell Side Risk (30d)"),
      bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
        line({ metric: tree.realized.sellSideRiskRatio30d, name, color, unit: Unit.ratio }),
      ),
    },
    {
      name: "1y",
      title: title("Sell Side Risk (1y)"),
      bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
        line({ metric: tree.realized.sellSideRiskRatio1y, name, color, unit: Unit.ratio }),
      ),
    },
  ];
}

/**
 * Rolling value created/destroyed charts for grouped cohorts
 * @template {{ color: Color, name: string }} T
 * @param {readonly T[]} list
 * @param {T} all
 * @param {readonly { name: string, getCreated: (item: T) => AnyMetricPattern, getDestroyed: (item: T) => AnyMetricPattern }[]} windows
 * @param {(metric: string) => string} title
 * @param {string} prefix
 * @returns {PartialOptionsTree}
 */
function groupedRollingValueCharts(list, all, windows, title, prefix = "") {
  return [
    {
      name: "Created",
      tree: windows.map((w) => ({
        name: w.name,
        title: title(`${prefix}Value Created (${w.name})`),
        bottom: mapCohortsWithAll(list, all, (item) =>
          line({ metric: w.getCreated(item), name: item.name, color: item.color, unit: Unit.usd }),
        ),
      })),
    },
    {
      name: "Destroyed",
      tree: windows.map((w) => ({
        name: w.name,
        title: title(`${prefix}Value Destroyed (${w.name})`),
        bottom: mapCohortsWithAll(list, all, (item) =>
          line({ metric: w.getDestroyed(item), name: item.name, color: item.color, unit: Unit.usd }),
        ),
      })),
    },
  ];
}

// ============================================================================
// SOPR Helpers
// ============================================================================

/**
 * Create SOPR tree with normal and adjusted sub-sections
 * @param {CohortAll | CohortFull | CohortWithAdjusted} cohort
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsTree}
 */
function createSingleSoprTreeWithAdjusted(cohort, title) {
  const r = cohort.tree.realized;
  return [
    {
      name: "Normal",
      tree: singleRollingSoprTree(
        { s24h: r.sopr24h, s7d: r.sopr7d, s30d: r.sopr30d, s1y: r.sopr1y, ema24h7d: r.sopr24h7dEma, ema24h30d: r.sopr24h30dEma },
        title,
      ),
    },
    {
      name: "Adjusted",
      tree: singleRollingSoprTree(
        { s24h: r.adjustedSopr24h, s7d: r.adjustedSopr7d, s30d: r.adjustedSopr30d, s1y: r.adjustedSopr1y, ema24h7d: r.adjustedSopr24h7dEma, ema24h30d: r.adjustedSopr24h30dEma },
        title,
        "Adjusted ",
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
      tree: [
        ...groupedSoprCharts(
          list,
          all,
          (c) => c.tree.realized.sopr,
          (c) => c.tree.realized.sopr7dEma,
          (c) => c.tree.realized.sopr30dEma,
          title,
          "",
        ),
        {
          name: "Rolling",
          tree: groupedRollingSoprCharts(
            list,
            all,
            (c) => c.tree.realized.sopr24h,
            (c) => c.tree.realized.sopr7d,
            (c) => c.tree.realized.sopr30d,
            (c) => c.tree.realized.sopr1y,
            title,
          ),
        },
      ],
    },
    {
      name: "Adjusted",
      tree: [
        ...groupedSoprCharts(
          list,
          all,
          (c) => c.tree.realized.adjustedSopr,
          (c) => c.tree.realized.adjustedSopr7dEma,
          (c) => c.tree.realized.adjustedSopr30dEma,
          title,
          "Adjusted ",
        ),
        {
          name: "Rolling",
          tree: groupedRollingSoprCharts(
            list,
            all,
            (c) => c.tree.realized.adjustedSopr24h,
            (c) => c.tree.realized.adjustedSopr7d,
            (c) => c.tree.realized.adjustedSopr30d,
            (c) => c.tree.realized.adjustedSopr1y,
            title,
            "Adjusted ",
          ),
        },
      ],
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
 * @param {PartialOptionsTree} [args.valueRollingTree] - Optional value rolling tree override
 * @returns {PartialOptionsGroup}
 */
export function createActivitySection({
  cohort,
  title,
  valueMetrics = [],
  soprTree,
  valueRollingTree,
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
                metric: tree.activity.sent14dEma.btc,
                name: "14d EMA",
                color: colors.indicator.main,
                unit: Unit.btc,
                defaultActive: false,
              }),
              line({
                metric: tree.activity.sent14dEma.usd,
                name: "14d EMA",
                color: colors.indicator.main,
                unit: Unit.usd,
                defaultActive: false,
              }),
              line({
                metric: tree.activity.sent.base.sats,
                name: "sum",
                color,
                unit: Unit.sats,
              }),
              line({
                metric: tree.activity.sent.base.btc,
                name: "sum",
                color,
                unit: Unit.btc,
              }),
              line({
                metric: tree.activity.sent.base.usd,
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
                metric: tree.activity.sent.cumulative.sats,
                name: "all-time",
                color,
                unit: Unit.sats,
              }),
              line({
                metric: tree.activity.sent.cumulative.btc,
                name: "all-time",
                color,
                unit: Unit.btc,
              }),
              line({
                metric: tree.activity.sent.cumulative.usd,
                name: "all-time",
                color,
                unit: Unit.usd,
              }),
            ],
          },
        ],
      },
      {
        name: "SOPR",
        tree:
          soprTree ??
          singleRollingSoprTree(
            { s24h: tree.realized.sopr24h, s7d: tree.realized.sopr7d, s30d: tree.realized.sopr30d, s1y: tree.realized.sopr1y, ema24h7d: tree.realized.sopr24h7dEma, ema24h30d: tree.realized.sopr24h30dEma },
            title,
          ),
      },
      {
        name: "Sell Side Risk",
        tree: singleRollingSellSideRiskTree(tree.realized, title),
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
          {
            name: "Rolling",
            tree:
              valueRollingTree ??
              singleRollingValueTree(
                {
                  created24h: tree.realized.valueCreated24h, created7d: tree.realized.valueCreated7d,
                  created30d: tree.realized.valueCreated30d, created1y: tree.realized.valueCreated1y,
                  destroyed24h: tree.realized.valueDestroyed24h, destroyed7d: tree.realized.valueDestroyed7d,
                  destroyed30d: tree.realized.valueDestroyed30d, destroyed1y: tree.realized.valueDestroyed1y,
                },
                title,
              ),
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
                metric: tree.activity.coinblocksDestroyed.sum._24h,
                name: "Coinblocks",
                color,
                unit: Unit.coinblocks,
              }),
              line({
                metric: tree.activity.coindaysDestroyed.sum._24h,
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
    valueRollingTree: [
      {
        name: "Normal",
        tree: singleRollingValueTree(
          {
            created24h: tree.realized.valueCreated24h, created7d: tree.realized.valueCreated7d,
            created30d: tree.realized.valueCreated30d, created1y: tree.realized.valueCreated1y,
            destroyed24h: tree.realized.valueDestroyed24h, destroyed7d: tree.realized.valueDestroyed7d,
            destroyed30d: tree.realized.valueDestroyed30d, destroyed1y: tree.realized.valueDestroyed1y,
          },
          title,
        ),
      },
      {
        name: "Adjusted",
        tree: singleRollingValueTree(
          {
            created24h: tree.realized.adjustedValueCreated24h, created7d: tree.realized.adjustedValueCreated7d,
            created30d: tree.realized.adjustedValueCreated30d, created1y: tree.realized.adjustedValueCreated1y,
            destroyed24h: tree.realized.adjustedValueDestroyed24h, destroyed7d: tree.realized.adjustedValueDestroyed7d,
            destroyed30d: tree.realized.adjustedValueDestroyed30d, destroyed1y: tree.realized.adjustedValueDestroyed1y,
          },
          title,
          "Adjusted ",
        ),
      },
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
                  sats: tree.activity.sent.base.sats,
                  btc: tree.activity.sent.base.btc,
                  usd: tree.activity.sent.base.usd,
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
        tree: soprTree ?? [
          ...createGroupedSoprTree(list, all, title),
          {
            name: "Rolling",
            tree: groupedRollingSoprCharts(
              list,
              all,
              (c) => c.tree.realized.sopr24h,
              (c) => c.tree.realized.sopr7d,
              (c) => c.tree.realized.sopr30d,
              (c) => c.tree.realized.sopr1y,
              title,
            ),
          },
        ],
      },
      {
        name: "Sell Side Risk",
        tree: groupedRollingSellSideRiskCharts(list, all, title),
      },
      {
        name: "Value",
        tree: valueTree ?? [
          ...createGroupedValueTree(list, all, title),
          {
            name: "Rolling",
            tree: groupedRollingValueCharts(
              list,
              all,
              [
                { name: "24h", getCreated: (c) => c.tree.realized.valueCreated24h, getDestroyed: (c) => c.tree.realized.valueDestroyed24h },
                { name: "7d", getCreated: (c) => c.tree.realized.valueCreated7d, getDestroyed: (c) => c.tree.realized.valueDestroyed7d },
                { name: "30d", getCreated: (c) => c.tree.realized.valueCreated30d, getDestroyed: (c) => c.tree.realized.valueDestroyed30d },
                { name: "1y", getCreated: (c) => c.tree.realized.valueCreated1y, getDestroyed: (c) => c.tree.realized.valueDestroyed1y },
              ],
              title,
            ),
          },
        ],
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
    {
      name: "Rolling",
      tree: [
        {
          name: "Normal",
          tree: groupedRollingValueCharts(
            list,
            all,
            [
              { name: "24h", getCreated: (c) => c.tree.realized.valueCreated24h, getDestroyed: (c) => c.tree.realized.valueDestroyed24h },
              { name: "7d", getCreated: (c) => c.tree.realized.valueCreated7d, getDestroyed: (c) => c.tree.realized.valueDestroyed7d },
              { name: "30d", getCreated: (c) => c.tree.realized.valueCreated30d, getDestroyed: (c) => c.tree.realized.valueDestroyed30d },
              { name: "1y", getCreated: (c) => c.tree.realized.valueCreated1y, getDestroyed: (c) => c.tree.realized.valueDestroyed1y },
            ],
            title,
          ),
        },
        {
          name: "Adjusted",
          tree: groupedRollingValueCharts(
            list,
            all,
            [
              { name: "24h", getCreated: (c) => c.tree.realized.adjustedValueCreated24h, getDestroyed: (c) => c.tree.realized.adjustedValueDestroyed24h },
              { name: "7d", getCreated: (c) => c.tree.realized.adjustedValueCreated7d, getDestroyed: (c) => c.tree.realized.adjustedValueDestroyed7d },
              { name: "30d", getCreated: (c) => c.tree.realized.adjustedValueCreated30d, getDestroyed: (c) => c.tree.realized.adjustedValueDestroyed30d },
              { name: "1y", getCreated: (c) => c.tree.realized.adjustedValueCreated1y, getDestroyed: (c) => c.tree.realized.adjustedValueDestroyed1y },
            ],
            title,
            "Adjusted ",
          ),
        },
      ],
    },
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
