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
import { line, baseline, dotsBaseline, percentRatio, percentRatioDots } from "../series.js";
import {
  mapCohortsWithAll,
  flatMapCohortsWithAll,
} from "../shared.js";
import { colors } from "../../utils/colors.js";

// ============================================================================
// Shared Volume Helpers
// ============================================================================

/**
 * @param {{ sent: Brk.BaseCumulativeInSumPattern, coindaysDestroyed: Brk.BaseCumulativeSumPattern<number> }} activity
 * @param {Color} color
 * @param {(name: string) => string} title
 * @returns {PartialOptionsTree}
 */
function volumeAndCoinsTree(activity, color, title) {
  return [
    {
      name: "Volume",
      tree: [
        {
          name: "Sum",
          title: title("Sent Volume"),
          bottom: [
            line({ series: activity.sent.base, name: "Sum", color, unit: Unit.sats }),
            line({ series: activity.sent.sum._24h, name: "24h", color: colors.time._24h, unit: Unit.sats, defaultActive: false }),
            line({ series: activity.sent.sum._1w, name: "1w", color: colors.time._1w, unit: Unit.sats, defaultActive: false }),
            line({ series: activity.sent.sum._1m, name: "1m", color: colors.time._1m, unit: Unit.sats, defaultActive: false }),
            line({ series: activity.sent.sum._1y, name: "1y", color: colors.time._1y, unit: Unit.sats, defaultActive: false }),
          ],
        },
        {
          name: "Cumulative",
          title: title("Sent Volume (Total)"),
          bottom: [
            line({ series: activity.sent.cumulative, name: "All-time", color, unit: Unit.sats }),
          ],
        },
      ],
    },
    {
      name: "Coins Destroyed",
      tree: [
        {
          name: "Base",
          title: title("Coindays Destroyed"),
          bottom: [
            line({ series: activity.coindaysDestroyed.base, name: "Base", color, unit: Unit.coindays }),
            line({ series: activity.coindaysDestroyed.sum._24h, name: "24h", color: colors.time._24h, unit: Unit.coindays, defaultActive: false }),
            line({ series: activity.coindaysDestroyed.sum._1w, name: "1w", color: colors.time._1w, unit: Unit.coindays, defaultActive: false }),
            line({ series: activity.coindaysDestroyed.sum._1m, name: "1m", color: colors.time._1m, unit: Unit.coindays, defaultActive: false }),
            line({ series: activity.coindaysDestroyed.sum._1y, name: "1y", color: colors.time._1y, unit: Unit.coindays, defaultActive: false }),
          ],
        },
        {
          name: "Cumulative",
          title: title("Cumulative Coindays Destroyed"),
          bottom: [
            line({ series: activity.coindaysDestroyed.cumulative, name: "All-time", color, unit: Unit.coindays }),
          ],
        },
      ],
    },
  ];
}

/**
 * Sent in profit/loss breakdown tree (shared by full and mid-level activity)
 * @param {Brk.BaseCumulativeInSumPattern} sent
 * @param {(name: string) => string} title
 * @returns {PartialOptionsTree}
 */
function sentProfitLossTree(sent, title) {
  return [
    {
      name: "Sent In Profit",
      tree: [
        {
          name: "USD",
          title: title("Sent Volume In Profit"),
          bottom: [
            line({ series: sent.inProfit.base.usd, name: "Base", color: colors.profit, unit: Unit.usd }),
            line({ series: sent.inProfit.sum._24h.usd, name: "24h", color: colors.time._24h, unit: Unit.usd, defaultActive: false }),
            line({ series: sent.inProfit.sum._1w.usd, name: "1w", color: colors.time._1w, unit: Unit.usd, defaultActive: false }),
            line({ series: sent.inProfit.sum._1m.usd, name: "1m", color: colors.time._1m, unit: Unit.usd, defaultActive: false }),
            line({ series: sent.inProfit.sum._1y.usd, name: "1y", color: colors.time._1y, unit: Unit.usd, defaultActive: false }),
          ],
        },
        {
          name: "BTC",
          title: title("Sent Volume In Profit (BTC)"),
          bottom: [
            line({ series: sent.inProfit.base.btc, name: "Base", color: colors.profit, unit: Unit.btc }),
            line({ series: sent.inProfit.sum._24h.btc, name: "24h", color: colors.time._24h, unit: Unit.btc, defaultActive: false }),
            line({ series: sent.inProfit.sum._1w.btc, name: "1w", color: colors.time._1w, unit: Unit.btc, defaultActive: false }),
            line({ series: sent.inProfit.sum._1m.btc, name: "1m", color: colors.time._1m, unit: Unit.btc, defaultActive: false }),
            line({ series: sent.inProfit.sum._1y.btc, name: "1y", color: colors.time._1y, unit: Unit.btc, defaultActive: false }),
          ],
        },
        {
          name: "Sats",
          title: title("Sent Volume In Profit (Sats)"),
          bottom: [
            line({ series: sent.inProfit.base.sats, name: "Base", color: colors.profit, unit: Unit.sats }),
            line({ series: sent.inProfit.sum._24h.sats, name: "24h", color: colors.time._24h, unit: Unit.sats, defaultActive: false }),
            line({ series: sent.inProfit.sum._1w.sats, name: "1w", color: colors.time._1w, unit: Unit.sats, defaultActive: false }),
            line({ series: sent.inProfit.sum._1m.sats, name: "1m", color: colors.time._1m, unit: Unit.sats, defaultActive: false }),
            line({ series: sent.inProfit.sum._1y.sats, name: "1y", color: colors.time._1y, unit: Unit.sats, defaultActive: false }),
          ],
        },
        { name: "Cumulative", title: title("Cumulative Sent In Profit"), bottom: [
          line({ series: sent.inProfit.cumulative.usd, name: "USD", color: colors.profit, unit: Unit.usd }),
          line({ series: sent.inProfit.cumulative.btc, name: "BTC", color: colors.profit, unit: Unit.btc, defaultActive: false }),
          line({ series: sent.inProfit.cumulative.sats, name: "Sats", color: colors.profit, unit: Unit.sats, defaultActive: false }),
        ]},
      ],
    },
    {
      name: "Sent In Loss",
      tree: [
        {
          name: "USD",
          title: title("Sent Volume In Loss"),
          bottom: [
            line({ series: sent.inLoss.base.usd, name: "Base", color: colors.loss, unit: Unit.usd }),
            line({ series: sent.inLoss.sum._24h.usd, name: "24h", color: colors.time._24h, unit: Unit.usd, defaultActive: false }),
            line({ series: sent.inLoss.sum._1w.usd, name: "1w", color: colors.time._1w, unit: Unit.usd, defaultActive: false }),
            line({ series: sent.inLoss.sum._1m.usd, name: "1m", color: colors.time._1m, unit: Unit.usd, defaultActive: false }),
            line({ series: sent.inLoss.sum._1y.usd, name: "1y", color: colors.time._1y, unit: Unit.usd, defaultActive: false }),
          ],
        },
        {
          name: "BTC",
          title: title("Sent Volume In Loss (BTC)"),
          bottom: [
            line({ series: sent.inLoss.base.btc, name: "Base", color: colors.loss, unit: Unit.btc }),
            line({ series: sent.inLoss.sum._24h.btc, name: "24h", color: colors.time._24h, unit: Unit.btc, defaultActive: false }),
            line({ series: sent.inLoss.sum._1w.btc, name: "1w", color: colors.time._1w, unit: Unit.btc, defaultActive: false }),
            line({ series: sent.inLoss.sum._1m.btc, name: "1m", color: colors.time._1m, unit: Unit.btc, defaultActive: false }),
            line({ series: sent.inLoss.sum._1y.btc, name: "1y", color: colors.time._1y, unit: Unit.btc, defaultActive: false }),
          ],
        },
        {
          name: "Sats",
          title: title("Sent Volume In Loss (Sats)"),
          bottom: [
            line({ series: sent.inLoss.base.sats, name: "Base", color: colors.loss, unit: Unit.sats }),
            line({ series: sent.inLoss.sum._24h.sats, name: "24h", color: colors.time._24h, unit: Unit.sats, defaultActive: false }),
            line({ series: sent.inLoss.sum._1w.sats, name: "1w", color: colors.time._1w, unit: Unit.sats, defaultActive: false }),
            line({ series: sent.inLoss.sum._1m.sats, name: "1m", color: colors.time._1m, unit: Unit.sats, defaultActive: false }),
            line({ series: sent.inLoss.sum._1y.sats, name: "1y", color: colors.time._1y, unit: Unit.sats, defaultActive: false }),
          ],
        },
        { name: "Cumulative", title: title("Cumulative Sent In Loss"), bottom: [
          line({ series: sent.inLoss.cumulative.usd, name: "USD", color: colors.loss, unit: Unit.usd }),
          line({ series: sent.inLoss.cumulative.btc, name: "BTC", color: colors.loss, unit: Unit.btc, defaultActive: false }),
          line({ series: sent.inLoss.cumulative.sats, name: "Sats", color: colors.loss, unit: Unit.sats, defaultActive: false }),
        ]},
      ],
    },
  ];
}

/**
 * Volume and coins tree with coinyears, dormancy, and sent in profit/loss (All/STH/LTH)
 * @param {Brk.CoindaysCoinyearsDormancySentPattern} activity
 * @param {Color} color
 * @param {(name: string) => string} title
 * @returns {PartialOptionsTree}
 */
function fullVolumeTree(activity, color, title) {
  return [
    ...volumeAndCoinsTree(activity, color, title),
    ...sentProfitLossTree(activity.sent, title),
    {
      name: "Coinyears Destroyed",
      title: title("Coinyears Destroyed"),
      bottom: [line({ series: activity.coinyearsDestroyed, name: "CYD", color, unit: Unit.years })],
    },
    {
      name: "Dormancy",
      title: title("Dormancy"),
      bottom: [line({ series: activity.dormancy, name: "Dormancy", color, unit: Unit.days })],
    },
  ];
}

// ============================================================================
// Shared SOPR Helpers
// ============================================================================

/**
 * @param {Brk._1m1w1y24hPattern<number>} ratio
 * @param {(name: string) => string} title
 * @param {string} [prefix]
 * @returns {PartialOptionsTree}
 */
function singleRollingSoprTree(ratio, title, prefix = "") {
  return [
    {
      name: "Compare",
      title: title(`Rolling ${prefix}SOPR`),
      bottom: [
        baseline({ series: ratio._24h, name: "24h", color: colors.time._24h, unit: Unit.ratio, base: 1 }),
        baseline({ series: ratio._1w, name: "7d", color: colors.time._1w, unit: Unit.ratio, base: 1 }),
        baseline({ series: ratio._1m, name: "30d", color: colors.time._1m, unit: Unit.ratio, base: 1 }),
        baseline({ series: ratio._1y, name: "1y", color: colors.time._1y, unit: Unit.ratio, base: 1 }),
      ],
    },
    {
      name: "24h",
      title: title(`${prefix}SOPR (24h)`),
      bottom: [dotsBaseline({ series: ratio._24h, name: "24h", unit: Unit.ratio, base: 1 })],
    },
    {
      name: "7d",
      title: title(`${prefix}SOPR (7d)`),
      bottom: [baseline({ series: ratio._1w, name: "SOPR", unit: Unit.ratio, base: 1 })],
    },
    {
      name: "30d",
      title: title(`${prefix}SOPR (30d)`),
      bottom: [baseline({ series: ratio._1m, name: "SOPR", unit: Unit.ratio, base: 1 })],
    },
    {
      name: "1y",
      title: title(`${prefix}SOPR (1y)`),
      bottom: [baseline({ series: ratio._1y, name: "SOPR", unit: Unit.ratio, base: 1 })],
    },
  ];
}

// ============================================================================
// Shared Sell Side Risk Helpers
// ============================================================================

/**
 * @param {Brk._1m1w1y24hPattern6} sellSideRisk
 * @param {(name: string) => string} title
 * @returns {PartialOptionsTree}
 */
function singleSellSideRiskTree(sellSideRisk, title) {
  return [
    {
      name: "Compare",
      title: title("Rolling Sell Side Risk"),
      bottom: [
        ...percentRatioDots({ pattern: sellSideRisk._24h, name: "24h", color: colors.time._24h }),
        ...percentRatio({ pattern: sellSideRisk._1w, name: "7d", color: colors.time._1w }),
        ...percentRatio({ pattern: sellSideRisk._1m, name: "30d", color: colors.time._1m }),
        ...percentRatio({ pattern: sellSideRisk._1y, name: "1y", color: colors.time._1y }),
      ],
    },
    {
      name: "24h",
      title: title("Sell Side Risk (24h)"),
      bottom: percentRatioDots({ pattern: sellSideRisk._24h, name: "Raw", color: colors.bitcoin }),
    },
    {
      name: "7d",
      title: title("Sell Side Risk (7d)"),
      bottom: percentRatio({ pattern: sellSideRisk._1w, name: "Risk" }),
    },
    {
      name: "30d",
      title: title("Sell Side Risk (30d)"),
      bottom: percentRatio({ pattern: sellSideRisk._1m, name: "Risk" }),
    },
    {
      name: "1y",
      title: title("Sell Side Risk (1y)"),
      bottom: percentRatio({ pattern: sellSideRisk._1y, name: "Risk" }),
    },
  ];
}

// ============================================================================
// Shared Value Helpers
// ============================================================================

/**
 * @param {Brk.BaseCumulativeSumPattern<number>} valueCreated
 * @param {Brk.BaseCumulativeSumPattern<number>} valueDestroyed
 * @param {(name: string) => string} title
 * @param {string} [prefix]
 * @returns {PartialOptionsTree}
 */
function singleRollingValueTree(valueCreated, valueDestroyed, title, prefix = "") {
  return [
    {
      name: "Compare",
      tree: [
        {
          name: "Created",
          title: title(`Rolling ${prefix}Value Created`),
          bottom: [
            line({ series: valueCreated.sum._24h, name: "24h", color: colors.time._24h, unit: Unit.usd }),
            line({ series: valueCreated.sum._1w, name: "7d", color: colors.time._1w, unit: Unit.usd }),
            line({ series: valueCreated.sum._1m, name: "30d", color: colors.time._1m, unit: Unit.usd }),
            line({ series: valueCreated.sum._1y, name: "1y", color: colors.time._1y, unit: Unit.usd }),
          ],
        },
        {
          name: "Destroyed",
          title: title(`Rolling ${prefix}Value Destroyed`),
          bottom: [
            line({ series: valueDestroyed.sum._24h, name: "24h", color: colors.time._24h, unit: Unit.usd }),
            line({ series: valueDestroyed.sum._1w, name: "7d", color: colors.time._1w, unit: Unit.usd }),
            line({ series: valueDestroyed.sum._1m, name: "30d", color: colors.time._1m, unit: Unit.usd }),
            line({ series: valueDestroyed.sum._1y, name: "1y", color: colors.time._1y, unit: Unit.usd }),
          ],
        },
      ],
    },
    {
      name: "24h",
      title: title(`${prefix}Value Created & Destroyed (24h)`),
      bottom: [
        line({ series: valueCreated.sum._24h, name: "Created", color: colors.usd, unit: Unit.usd }),
        line({ series: valueDestroyed.sum._24h, name: "Destroyed", color: colors.loss, unit: Unit.usd }),
      ],
    },
    {
      name: "7d",
      title: title(`${prefix}Value Created & Destroyed (7d)`),
      bottom: [
        line({ series: valueCreated.sum._1w, name: "Created", color: colors.usd, unit: Unit.usd }),
        line({ series: valueDestroyed.sum._1w, name: "Destroyed", color: colors.loss, unit: Unit.usd }),
      ],
    },
    {
      name: "30d",
      title: title(`${prefix}Value Created & Destroyed (30d)`),
      bottom: [
        line({ series: valueCreated.sum._1m, name: "Created", color: colors.usd, unit: Unit.usd }),
        line({ series: valueDestroyed.sum._1m, name: "Destroyed", color: colors.loss, unit: Unit.usd }),
      ],
    },
    {
      name: "1y",
      title: title(`${prefix}Value Created & Destroyed (1y)`),
      bottom: [
        line({ series: valueCreated.sum._1y, name: "Created", color: colors.usd, unit: Unit.usd }),
        line({ series: valueDestroyed.sum._1y, name: "Destroyed", color: colors.loss, unit: Unit.usd }),
      ],
    },
    {
      name: "Cumulative",
      title: title(`${prefix}Value Created & Destroyed (Total)`),
      bottom: [
        line({ series: valueCreated.cumulative, name: "Created", color: colors.usd, unit: Unit.usd }),
        line({ series: valueDestroyed.cumulative, name: "Destroyed", color: colors.loss, unit: Unit.usd }),
      ],
    },
  ];
}

/**
 * Value section for cohorts with full realized (flows + breakdown)
 * @param {Brk.BaseCumulativeDistributionRelSumValuePattern} profit
 * @param {Brk.BaseCapitulationCumulativeNegativeRelSumValuePattern} loss
 * @param {Brk.BaseCumulativeSumPattern<number>} valueCreated
 * @param {Brk.BaseCumulativeSumPattern<number>} valueDestroyed
 * @param {AnyFetchedSeriesBlueprint[]} extraValueSeries
 * @param {PartialOptionsTree} rollingTree
 * @param {(name: string) => string} title
 * @returns {PartialOptionsGroup}
 */
function fullValueSection(profit, loss, valueCreated, valueDestroyed, extraValueSeries, rollingTree, title) {
  return {
    name: "Value",
    tree: [
      {
        name: "Flows",
        title: title("Profit & Capitulation Flows"),
        bottom: [
          line({ series: profit.distributionFlow, name: "Distribution Flow", color: colors.profit, unit: Unit.usd }),
          line({ series: loss.capitulationFlow, name: "Capitulation Flow", color: colors.loss, unit: Unit.usd }),
        ],
      },
      {
        name: "Created & Destroyed",
        title: title("Value Created & Destroyed"),
        bottom: [
          line({ series: valueCreated.base, name: "Created", color: colors.usd, unit: Unit.usd }),
          line({ series: valueDestroyed.base, name: "Destroyed", color: colors.loss, unit: Unit.usd }),
          ...extraValueSeries,
        ],
      },
      {
        name: "Breakdown",
        tree: [
          {
            name: "Profit",
            title: title("Profit Value Created & Destroyed"),
            bottom: [
              line({ series: profit.valueCreated.base, name: "Created", color: colors.profit, unit: Unit.usd }),
              line({ series: profit.valueDestroyed.base, name: "Destroyed", color: colors.loss, unit: Unit.usd }),
            ],
          },
          {
            name: "Loss",
            title: title("Loss Value Created & Destroyed"),
            bottom: [
              line({ series: loss.valueCreated.base, name: "Created", color: colors.profit, unit: Unit.usd }),
              line({ series: loss.valueDestroyed.base, name: "Destroyed", color: colors.loss, unit: Unit.usd }),
            ],
          },
        ],
      },
      { name: "Rolling", tree: rollingTree },
    ],
  };
}

/**
 * Simple value section (created & destroyed + rolling)
 * @param {Brk.BaseCumulativeSumPattern<number>} valueCreated
 * @param {Brk.BaseCumulativeSumPattern<number>} valueDestroyed
 * @param {(name: string) => string} title
 * @returns {PartialOptionsGroup}
 */
function simpleValueSection(valueCreated, valueDestroyed, title) {
  return {
    name: "Value",
    tree: [
      {
        name: "Created & Destroyed",
        title: title("Value Created & Destroyed"),
        bottom: [
          line({ series: valueCreated.base, name: "Created", color: colors.usd, unit: Unit.usd }),
          line({ series: valueDestroyed.base, name: "Destroyed", color: colors.loss, unit: Unit.usd }),
        ],
      },
      {
        name: "Rolling",
        tree: singleRollingValueTree(valueCreated, valueDestroyed, title),
      },
    ],
  };
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
          {
            name: "Normal",
            tree: singleRollingSoprTree(sopr.ratio, title),
          },
          {
            name: "Adjusted",
            tree: singleRollingSoprTree(sopr.adjusted.ratio, title, "Adjusted "),
          },
        ],
      },
      { name: "Sell Side Risk", tree: singleSellSideRiskTree(r.sellSideRiskRatio, title) },
      fullValueSection(
        r.profit, r.loss,
        sopr.valueCreated, sopr.valueDestroyed,
        [
          line({ series: sopr.adjusted.valueCreated.base, name: "Adjusted Created", color: colors.adjustedCreated, unit: Unit.usd, defaultActive: false }),
          line({ series: sopr.adjusted.valueDestroyed.base, name: "Adjusted Destroyed", color: colors.adjustedDestroyed, unit: Unit.usd, defaultActive: false }),
        ],
        [
          {
            name: "Normal",
            tree: singleRollingValueTree(sopr.valueCreated, sopr.valueDestroyed, title),
          },
          {
            name: "Adjusted",
            tree: singleRollingValueTree(sopr.adjusted.valueCreated, sopr.adjusted.valueDestroyed, title, "Adjusted "),
          },
        ],
        title,
      ),
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
      { name: "Sell Side Risk", tree: singleSellSideRiskTree(r.sellSideRiskRatio, title) },
      fullValueSection(
        r.profit, r.loss,
        sopr.valueCreated, sopr.valueDestroyed,
        [],
        singleRollingValueTree(sopr.valueCreated, sopr.valueDestroyed, title),
        title,
      ),
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
      ...volumeAndCoinsTree(tree.activity, color, title),
      ...sentProfitLossTree(tree.activity.sent, title),
      {
        name: "SOPR",
        title: title("SOPR (24h)"),
        bottom: [dotsBaseline({ series: sopr.ratio._24h, name: "SOPR", unit: Unit.ratio, base: 1 })],
      },
      simpleValueSection(sopr.valueCreated, sopr.valueDestroyed, title),
    ],
  };
}

/**
 * Minimal activity section for cohorts without activity field (value only)
 * @param {{ cohort: CohortBasicWithMarketCap | CohortBasicWithoutMarketCap | CohortWithoutRelative | CohortAddress | AddressCohortObject, title: (name: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createActivitySectionMinimal({ cohort, title }) {
  const sopr = cohort.tree.realized.sopr;

  return {
    name: "Activity",
    tree: [
      simpleValueSection(sopr.valueCreated, sopr.valueDestroyed, title),
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
 * @param {(item: T | A) => AnySeriesPattern} getRaw
 * @param {(item: T | A) => AnySeriesPattern} get7d
 * @param {(item: T | A) => AnySeriesPattern} get30d
 * @param {(name: string) => string} title
 * @param {string} [prefix]
 * @returns {PartialOptionsTree}
 */
function groupedSoprCharts(list, all, getRaw, get7d, get30d, title, prefix = "") {
  return [
    {
      name: "Raw",
      title: title(`${prefix}SOPR`),
      bottom: mapCohortsWithAll(list, all, (item) =>
        baseline({ series: getRaw(item), name: item.name, color: item.color, unit: Unit.ratio, base: 1 }),
      ),
    },
    {
      name: "7d",
      title: title(`${prefix}SOPR (7d)`),
      bottom: mapCohortsWithAll(list, all, (item) =>
        baseline({ series: get7d(item), name: item.name, color: item.color, unit: Unit.ratio, base: 1 }),
      ),
    },
    {
      name: "30d",
      title: title(`${prefix}SOPR (30d)`),
      bottom: mapCohortsWithAll(list, all, (item) =>
        baseline({ series: get30d(item), name: item.name, color: item.color, unit: Unit.ratio, base: 1 }),
      ),
    },
  ];
}

/**
 * @template {{ color: Color, name: string }} T
 * @template {{ color: Color, name: string }} A
 * @param {readonly T[]} list
 * @param {A} all
 * @param {(item: T | A) => AnySeriesPattern} get24h
 * @param {(item: T | A) => AnySeriesPattern} get7d
 * @param {(item: T | A) => AnySeriesPattern} get30d
 * @param {(item: T | A) => AnySeriesPattern} get1y
 * @param {(name: string) => string} title
 * @param {string} [prefix]
 * @returns {PartialOptionsTree}
 */
function groupedRollingSoprCharts(list, all, get24h, get7d, get30d, get1y, title, prefix = "") {
  return [
    {
      name: "24h",
      title: title(`${prefix}SOPR (24h)`),
      bottom: mapCohortsWithAll(list, all, (c) =>
        baseline({ series: get24h(c), name: c.name, color: c.color, unit: Unit.ratio, base: 1 }),
      ),
    },
    {
      name: "7d",
      title: title(`${prefix}SOPR (7d)`),
      bottom: mapCohortsWithAll(list, all, (c) =>
        baseline({ series: get7d(c), name: c.name, color: c.color, unit: Unit.ratio, base: 1 }),
      ),
    },
    {
      name: "30d",
      title: title(`${prefix}SOPR (30d)`),
      bottom: mapCohortsWithAll(list, all, (c) =>
        baseline({ series: get30d(c), name: c.name, color: c.color, unit: Unit.ratio, base: 1 }),
      ),
    },
    {
      name: "1y",
      title: title(`${prefix}SOPR (1y)`),
      bottom: mapCohortsWithAll(list, all, (c) =>
        baseline({ series: get1y(c), name: c.name, color: c.color, unit: Unit.ratio, base: 1 }),
      ),
    },
  ];
}

// ============================================================================
// Grouped Value/Flow Helpers
// ============================================================================

/**
 * @template {{ color: Color, name: string }} T
 * @template {{ color: Color, name: string }} A
 * @param {readonly T[]} list
 * @param {A} all
 * @param {readonly { name: string, getCreated: (item: T | A) => AnySeriesPattern, getDestroyed: (item: T | A) => AnySeriesPattern }[]} windows
 * @param {(name: string) => string} title
 * @param {string} [prefix]
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
          line({ series: w.getCreated(item), name: item.name, color: item.color, unit: Unit.usd }),
        ),
      })),
    },
    {
      name: "Destroyed",
      tree: windows.map((w) => ({
        name: w.name,
        title: title(`${prefix}Value Destroyed (${w.name})`),
        bottom: mapCohortsWithAll(list, all, (item) =>
          line({ series: w.getDestroyed(item), name: item.name, color: item.color, unit: Unit.usd }),
        ),
      })),
    },
  ];
}

/**
 * @param {readonly (CohortFull | CohortLongTerm | CohortAll)[]} list
 * @param {CohortAll} all
 */
function valueWindows(list, all) {
  return [
    { name: "24h", getCreated: (/** @type {typeof list[number] | typeof all} */ c) => c.tree.realized.sopr.valueCreated.sum._24h, getDestroyed: (/** @type {typeof list[number] | typeof all} */ c) => c.tree.realized.sopr.valueDestroyed.sum._24h },
    { name: "7d", getCreated: (/** @type {typeof list[number] | typeof all} */ c) => c.tree.realized.sopr.valueCreated.sum._1w, getDestroyed: (/** @type {typeof list[number] | typeof all} */ c) => c.tree.realized.sopr.valueDestroyed.sum._1w },
    { name: "30d", getCreated: (/** @type {typeof list[number] | typeof all} */ c) => c.tree.realized.sopr.valueCreated.sum._1m, getDestroyed: (/** @type {typeof list[number] | typeof all} */ c) => c.tree.realized.sopr.valueDestroyed.sum._1m },
    { name: "1y", getCreated: (/** @type {typeof list[number] | typeof all} */ c) => c.tree.realized.sopr.valueCreated.sum._1y, getDestroyed: (/** @type {typeof list[number] | typeof all} */ c) => c.tree.realized.sopr.valueDestroyed.sum._1y },
  ];
}

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
        title: title("Sent Volume"),
        bottom: flatMapCohortsWithAll(list, all, ({ name, color, tree }) => [
          line({ series: tree.activity.sent.sum._24h, name, color, unit: Unit.sats }),
        ]),
      },
      {
        name: "SOPR",
        tree: [
          {
            name: "Normal",
            tree: [
              ...groupedSoprCharts(
                list, all,
                (c) => c.tree.realized.sopr.ratio._24h,
                (c) => c.tree.realized.sopr.ratio._1w,
                (c) => c.tree.realized.sopr.ratio._1m,
                title,
              ),
              {
                name: "Rolling",
                tree: groupedRollingSoprCharts(
                  list, all,
                  (c) => c.tree.realized.sopr.ratio._24h,
                  (c) => c.tree.realized.sopr.ratio._1w,
                  (c) => c.tree.realized.sopr.ratio._1m,
                  (c) => c.tree.realized.sopr.ratio._1y,
                  title,
                ),
              },
            ],
          },
          {
            name: "Adjusted",
            tree: [
              ...groupedSoprCharts(
                list, all,
                (c) => c.tree.realized.sopr.adjusted.ratio._24h,
                (c) => c.tree.realized.sopr.adjusted.ratio._1w,
                (c) => c.tree.realized.sopr.adjusted.ratio._1m,
                title,
                "Adjusted ",
              ),
              {
                name: "Rolling",
                tree: groupedRollingSoprCharts(
                  list, all,
                  (c) => c.tree.realized.sopr.adjusted.ratio._24h,
                  (c) => c.tree.realized.sopr.adjusted.ratio._1w,
                  (c) => c.tree.realized.sopr.adjusted.ratio._1m,
                  (c) => c.tree.realized.sopr.adjusted.ratio._1y,
                  title,
                  "Adjusted ",
                ),
              },
            ],
          },
        ],
      },
      {
        name: "Sell Side Risk",
        tree: [
          { name: "24h", title: title("Sell Side Risk (24h)"), bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) => line({ series: tree.realized.sellSideRiskRatio._24h.ratio, name, color, unit: Unit.ratio })) },
          { name: "7d", title: title("Sell Side Risk (7d)"), bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) => line({ series: tree.realized.sellSideRiskRatio._1w.ratio, name, color, unit: Unit.ratio })) },
          { name: "30d", title: title("Sell Side Risk (30d)"), bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) => line({ series: tree.realized.sellSideRiskRatio._1m.ratio, name, color, unit: Unit.ratio })) },
          { name: "1y", title: title("Sell Side Risk (1y)"), bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) => line({ series: tree.realized.sellSideRiskRatio._1y.ratio, name, color, unit: Unit.ratio })) },
        ],
      },
      {
        name: "Value",
        tree: [
          {
            name: "Flows",
            tree: [
              { name: "Distribution", title: title("Distribution Flow"), bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) => line({ series: tree.realized.profit.distributionFlow, name, color, unit: Unit.usd })) },
              { name: "Capitulation", title: title("Capitulation Flow"), bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) => line({ series: tree.realized.loss.capitulationFlow, name, color, unit: Unit.usd })) },
            ],
          },
          { name: "Created", title: title("Value Created"), bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) => line({ series: tree.realized.sopr.valueCreated.base, name, color, unit: Unit.usd })) },
          { name: "Destroyed", title: title("Value Destroyed"), bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) => line({ series: tree.realized.sopr.valueDestroyed.base, name, color, unit: Unit.usd })) },
          {
            name: "Rolling",
            tree: [
              {
                name: "Normal",
                tree: groupedRollingValueCharts(list, all, valueWindows(list, all), title),
              },
              {
                name: "Adjusted",
                tree: groupedRollingValueCharts(
                  list, all,
                  [
                    { name: "24h", getCreated: (c) => c.tree.realized.sopr.adjusted.valueCreated.sum._24h, getDestroyed: (c) => c.tree.realized.sopr.adjusted.valueDestroyed.sum._24h },
                    { name: "7d", getCreated: (c) => c.tree.realized.sopr.adjusted.valueCreated.sum._1w, getDestroyed: (c) => c.tree.realized.sopr.adjusted.valueDestroyed.sum._1w },
                    { name: "30d", getCreated: (c) => c.tree.realized.sopr.adjusted.valueCreated.sum._1m, getDestroyed: (c) => c.tree.realized.sopr.adjusted.valueDestroyed.sum._1m },
                    { name: "1y", getCreated: (c) => c.tree.realized.sopr.adjusted.valueCreated.sum._1y, getDestroyed: (c) => c.tree.realized.sopr.adjusted.valueDestroyed.sum._1y },
                  ],
                  title,
                  "Adjusted ",
                ),
              },
            ],
          },
        ],
      },
      {
        name: "Coins Destroyed",
        title: title("Coindays Destroyed"),
        bottom: flatMapCohortsWithAll(list, all, ({ name, color, tree }) => [
          line({ series: tree.activity.coindaysDestroyed.sum._24h, name, color, unit: Unit.coindays }),
        ]),
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
        title: title("Sent Volume"),
        bottom: flatMapCohortsWithAll(list, all, ({ name, color, tree }) => [
          line({ series: tree.activity.sent.sum._24h, name, color, unit: Unit.sats }),
        ]),
      },
      {
        name: "SOPR",
        tree: [
          ...groupedSoprCharts(
            list, all,
            (c) => c.tree.realized.sopr.ratio._24h,
            (c) => c.tree.realized.sopr.ratio._1w,
            (c) => c.tree.realized.sopr.ratio._1m,
            title,
          ),
          {
            name: "Rolling",
            tree: groupedRollingSoprCharts(
              list, all,
              (c) => c.tree.realized.sopr.ratio._24h,
              (c) => c.tree.realized.sopr.ratio._1w,
              (c) => c.tree.realized.sopr.ratio._1m,
              (c) => c.tree.realized.sopr.ratio._1y,
              title,
            ),
          },
        ],
      },
      {
        name: "Sell Side Risk",
        tree: [
          { name: "24h", title: title("Sell Side Risk (24h)"), bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) => line({ series: tree.realized.sellSideRiskRatio._24h.ratio, name, color, unit: Unit.ratio })) },
          { name: "7d", title: title("Sell Side Risk (7d)"), bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) => line({ series: tree.realized.sellSideRiskRatio._1w.ratio, name, color, unit: Unit.ratio })) },
          { name: "30d", title: title("Sell Side Risk (30d)"), bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) => line({ series: tree.realized.sellSideRiskRatio._1m.ratio, name, color, unit: Unit.ratio })) },
          { name: "1y", title: title("Sell Side Risk (1y)"), bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) => line({ series: tree.realized.sellSideRiskRatio._1y.ratio, name, color, unit: Unit.ratio })) },
        ],
      },
      {
        name: "Value",
        tree: [
          {
            name: "Flows",
            tree: [
              { name: "Distribution", title: title("Distribution Flow"), bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) => line({ series: tree.realized.profit.distributionFlow, name, color, unit: Unit.usd })) },
              { name: "Capitulation", title: title("Capitulation Flow"), bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) => line({ series: tree.realized.loss.capitulationFlow, name, color, unit: Unit.usd })) },
            ],
          },
          { name: "Created", title: title("Value Created"), bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) => line({ series: tree.realized.sopr.valueCreated.base, name, color, unit: Unit.usd })) },
          { name: "Destroyed", title: title("Value Destroyed"), bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) => line({ series: tree.realized.sopr.valueDestroyed.base, name, color, unit: Unit.usd })) },
          {
            name: "Rolling",
            tree: groupedRollingValueCharts(list, all, valueWindows(list, all), title),
          },
        ],
      },
      {
        name: "Coins Destroyed",
        title: title("Coindays Destroyed"),
        bottom: flatMapCohortsWithAll(list, all, ({ name, color, tree }) => [
          line({ series: tree.activity.coindaysDestroyed.sum._24h, name, color, unit: Unit.coindays }),
        ]),
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
        title: title("Sent Volume"),
        bottom: flatMapCohortsWithAll(list, all, ({ name, color, tree }) => [
          line({ series: tree.activity.sent.sum._24h, name, color, unit: Unit.sats }),
        ]),
      },
      {
        name: "SOPR",
        title: title("SOPR (24h)"),
        bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
          baseline({ series: tree.realized.sopr.ratio._24h, name, color, unit: Unit.ratio, base: 1 }),
        ),
      },
      {
        name: "Value",
        tree: [
          { name: "Created", title: title("Value Created"), bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) => line({ series: tree.realized.sopr.valueCreated.base, name, color, unit: Unit.usd })) },
          { name: "Destroyed", title: title("Value Destroyed"), bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) => line({ series: tree.realized.sopr.valueDestroyed.base, name, color, unit: Unit.usd })) },
        ],
      },
      {
        name: "Coins Destroyed",
        title: title("Coindays Destroyed"),
        bottom: flatMapCohortsWithAll(list, all, ({ name, color, tree }) => [
          line({ series: tree.activity.coindaysDestroyed.sum._24h, name, color, unit: Unit.coindays }),
        ]),
      },
    ],
  };
}

/**
 * Grouped minimal activity (value only, no activity field)
 * @param {{ list: readonly (UtxoCohortObject | CohortWithoutRelative | CohortAddress | AddressCohortObject)[], all: CohortAll, title: (name: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createGroupedActivitySectionMinimal({ list, all, title }) {
  return {
    name: "Activity",
    tree: [
      { name: "Value Created", title: title("Value Created"), bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) => line({ series: tree.realized.sopr.valueCreated.base, name, color, unit: Unit.usd })) },
      { name: "Value Destroyed", title: title("Value Destroyed"), bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) => line({ series: tree.realized.sopr.valueDestroyed.base, name, color, unit: Unit.usd })) },
    ],
  };
}
