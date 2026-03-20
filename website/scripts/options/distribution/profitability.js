/**
 * Profitability section builders
 *
 * Capability tiers:
 * - Full (All/STH/LTH): full unrealized with rel series, invested capital, sentiment;
 *   full realized with relToRcap, peakRegret, profitToLossRatio, grossPnl
 * - Mid (AgeRange/MaxAge): unrealized profit/loss/netPnl/nupl (no rel, no invested, no sentiment);
 *   realized with netPnl + delta (no relToRcap, no peakRegret)
 * - Basic (UtxoAmount, Empty, Address): nupl only unrealized;
 *   basic realized profit/loss (no netPnl, no relToRcap)
 */

import { Unit } from "../../utils/units.js";
import { ROLLING_WINDOWS, line, dotted, baseline, percentRatio, percentRatioBaseline } from "../series.js";
import { colors } from "../../utils/colors.js";
import { priceLine } from "../constants.js";
import {
  mapCohortsWithAll,
  flatMapCohorts,
  flatMapCohortsWithAll,
} from "../shared.js";

// ============================================================================
// Core Series Builders
// ============================================================================

/**
 * @param {AnySeriesPattern} s
 * @param {Unit} unit
 * @returns {AnyFetchedSeriesBlueprint}
 */
function netBaseline(s, unit) {
  return baseline({ series: s, name: "Net P&L", unit });
}

// ============================================================================
// Unrealized P&L Builders
// ============================================================================

/**
 * Overview chart: net + profit + loss inverted (active), loss raw (hidden)
 * @param {{ usd: AnySeriesPattern }} profit
 * @param {{ usd: AnySeriesPattern, negative: AnySeriesPattern }} loss
 * @param {AnySeriesPattern} netPnlUsd
 * @param {(name: string) => string} title
 * @returns {PartialChartOption}
 */
function unrealizedOverview(profit, loss, netPnlUsd, title) {
  return {
    name: "Overview",
    title: title("Unrealized P&L"),
    bottom: [
      baseline({ series: netPnlUsd, name: "Net P&L", unit: Unit.usd }),
      dotted({ series: profit.usd, name: "Profit", color: colors.profit, unit: Unit.usd }),
      dotted({ series: loss.negative, name: "Neg. Loss", color: colors.loss, unit: Unit.usd }),
      dotted({ series: loss.usd, name: "Loss", color: colors.loss, unit: Unit.usd, defaultActive: false }),
      priceLine({ unit: Unit.usd }),
    ],
  };
}

/**
 * Relative P&L chart: profit + loss as % of some denominator
 * @param {{ percent: AnySeriesPattern, ratio: AnySeriesPattern }} profit
 * @param {{ percent: AnySeriesPattern, ratio: AnySeriesPattern }} loss
 * @param {string} name
 * @param {(name: string) => string} title
 * @returns {PartialChartOption}
 */
function relPnlChart(profit, loss, name, title) {
  return {
    name,
    title: title(`Unrealized P&L (${name})`),
    bottom: [
      ...percentRatio({ pattern: profit, name: "Profit", color: colors.profit }),
      ...percentRatio({ pattern: loss, name: "Loss", color: colors.loss }),
    ],
  };
}

/**
 * Unrealized tree for All cohort
 * @param {AllRelativePattern} u
 * @param {(name: string) => string} title
 * @returns {PartialOptionsTree}
 */
function unrealizedTreeAll(u, title) {
  return [
    unrealizedOverview(u.profit, u.loss, u.netPnl.usd, title),
    { name: "Net P&L", title: title("Net Unrealized P&L"), bottom: [netBaseline(u.netPnl.usd, Unit.usd)] },
    { name: "NUPL", title: title("NUPL"), bottom: nuplSeries(u.nupl) },
    { name: "Profit", title: title("Unrealized Profit"), bottom: [line({ series: u.profit.usd, name: "Profit", color: colors.profit, unit: Unit.usd })] },
    { name: "Loss", title: title("Unrealized Loss"), bottom: [line({ series: u.loss.usd, name: "Loss", color: colors.loss, unit: Unit.usd })] },
    {
      name: "Relative",
      tree: [
        {
          name: "Own P&L",
          title: title("Unrealized P&L (% of Own P&L)"),
          bottom: [
            ...percentRatioBaseline({ pattern: u.netPnl.toOwnGrossPnl, name: "Net" }),
            ...percentRatio({ pattern: u.profit.toOwnGrossPnl, name: "Profit", color: colors.profit, defaultActive: false }),
            ...percentRatio({ pattern: u.loss.toOwnGrossPnl, name: "Loss", color: colors.loss, defaultActive: false }),
          ],
        },
        relPnlChart(u.profit.toMcap, u.loss.toMcap, "Market Cap", title),
      ],
    },
  ];
}

/**
 * Unrealized tree for Full cohorts (STH)
 * @param {FullRelativePattern} u
 * @param {(name: string) => string} title
 * @returns {PartialOptionsTree}
 */
function unrealizedTreeFull(u, title) {
  return [
    unrealizedOverview(u.profit, u.loss, u.netPnl.usd, title),
    { name: "Net P&L", title: title("Net Unrealized P&L"), bottom: [netBaseline(u.netPnl.usd, Unit.usd)] },
    { name: "NUPL", title: title("NUPL"), bottom: nuplSeries(u.nupl) },
    { name: "Profit", title: title("Unrealized Profit"), bottom: [line({ series: u.profit.usd, name: "Profit", color: colors.profit, unit: Unit.usd })] },
    { name: "Loss", title: title("Unrealized Loss"), bottom: [line({ series: u.loss.usd, name: "Loss", color: colors.loss, unit: Unit.usd })] },
    {
      name: "Relative",
      tree: [
        {
          name: "Own P&L",
          title: title("Unrealized P&L (% of Own P&L)"),
          bottom: [
            ...percentRatioBaseline({ pattern: u.netPnl.toOwnGrossPnl, name: "Net" }),
            ...percentRatio({ pattern: u.profit.toOwnGrossPnl, name: "Profit", color: colors.profit, defaultActive: false }),
            ...percentRatio({ pattern: u.loss.toOwnGrossPnl, name: "Loss", color: colors.loss, defaultActive: false }),
          ],
        },
        relPnlChart(u.profit.toMcap, u.loss.toMcap, "Market Cap", title),
        relPnlChart(u.profit.toOwnMcap, u.loss.toOwnMcap, "Own Market Cap", title),
      ],
    },
  ];
}

/**
 * Unrealized tree for LTH (loss relToMcap only, has own mcap + own P&L)
 * @param {FullRelativePattern} u
 * @param {(name: string) => string} title
 * @returns {PartialOptionsTree}
 */
function unrealizedTreeLongTerm(u, title) {
  return [
    unrealizedOverview(u.profit, u.loss, u.netPnl.usd, title),
    { name: "Net P&L", title: title("Net Unrealized P&L"), bottom: [netBaseline(u.netPnl.usd, Unit.usd)] },
    { name: "NUPL", title: title("NUPL"), bottom: nuplSeries(u.nupl) },
    { name: "Profit", title: title("Unrealized Profit"), bottom: [line({ series: u.profit.usd, name: "Profit", color: colors.profit, unit: Unit.usd })] },
    { name: "Loss", title: title("Unrealized Loss"), bottom: [line({ series: u.loss.usd, name: "Loss", color: colors.loss, unit: Unit.usd })] },
    {
      name: "Relative",
      tree: [
        {
          name: "Own P&L",
          title: title("Unrealized P&L (% of Own P&L)"),
          bottom: [
            ...percentRatioBaseline({ pattern: u.netPnl.toOwnGrossPnl, name: "Net" }),
            ...percentRatio({ pattern: u.profit.toOwnGrossPnl, name: "Profit", color: colors.profit, defaultActive: false }),
            ...percentRatio({ pattern: u.loss.toOwnGrossPnl, name: "Loss", color: colors.loss, defaultActive: false }),
          ],
        },
        {
          name: "Market Cap",
          title: title("Unrealized Loss (% of Market Cap)"),
          bottom: percentRatio({ pattern: u.loss.toMcap, name: "Loss", color: colors.loss }),
        },
        relPnlChart(u.profit.toOwnMcap, u.loss.toOwnMcap, "Own Market Cap", title),
      ],
    },
  ];
}

/**
 * Unrealized tree for mid-tier cohorts (AgeRange/MaxAge — profit/loss/net, no relative)
 * @param {BasicRelativePattern} u
 * @param {(name: string) => string} title
 * @returns {PartialOptionsTree}
 */
function unrealizedTreeMid(u, title) {
  return [
    unrealizedOverview(u.profit, u.loss, u.netPnl.usd, title),
    { name: "Net P&L", title: title("Net Unrealized P&L"), bottom: [netBaseline(u.netPnl.usd, Unit.usd)] },
    { name: "NUPL", title: title("NUPL"), bottom: nuplSeries(u.nupl) },
    { name: "Profit", title: title("Unrealized Profit"), bottom: [line({ series: u.profit.usd, name: "Profit", color: colors.profit, unit: Unit.usd })] },
    { name: "Loss", title: title("Unrealized Loss"), bottom: [line({ series: u.loss.usd, name: "Loss", color: colors.loss, unit: Unit.usd })] },
  ];
}

// ============================================================================
// Invested Capital, Sentiment, NUPL
// ============================================================================

/**
 * Invested capital (Full unrealized only)
 * @param {FullRelativePattern | AllRelativePattern} u
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function investedCapitalSeries(u) {
  return [
    line({ series: u.investedCapital.inProfit.usd, name: "In Profit", color: colors.profit, unit: Unit.usd }),
    line({ series: u.investedCapital.inLoss.usd, name: "In Loss", color: colors.loss, unit: Unit.usd }),
  ];
}

/**
 * Sentiment (Full unrealized only)
 * @param {FullRelativePattern | AllRelativePattern} u
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function sentimentSeries(u) {
  return [
    baseline({ series: u.sentiment.net.usd, name: "Net Sentiment", unit: Unit.usd }),
    line({ series: u.sentiment.greedIndex.usd, name: "Greed Index", color: colors.profit, unit: Unit.usd, defaultActive: false }),
    line({ series: u.sentiment.painIndex.usd, name: "Pain Index", color: colors.loss, unit: Unit.usd, defaultActive: false }),
  ];
}

/**
 * NUPL series
 * @param {NuplPattern} nupl
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function nuplSeries(nupl) {
  return [baseline({ series: nupl.ratio, name: "NUPL", unit: Unit.ratio })];
}

// ============================================================================
// Realized P&L Helpers
// ============================================================================

/**
 * Flat metric folder: Compare + windows + Cumulative + optional % of Realized Cap
 * @param {Object} args
 * @param {{ sum: Record<string, { usd: AnySeriesPattern }>, cumulative: { usd: AnySeriesPattern }, base: { usd: AnySeriesPattern } }} args.pattern
 * @param {string} args.metricTitle
 * @param {Color} args.color
 * @param {(name: string) => string} args.title
 * @param {{ percent: AnySeriesPattern, ratio: AnySeriesPattern }} [args.toRcap]
 * @returns {PartialOptionsTree}
 */
function realizedMetricFolder({ pattern, metricTitle, color, title, toRcap }) {
  return [
    {
      name: "Compare",
      title: title(`Realized ${metricTitle}`),
      bottom: ROLLING_WINDOWS.map((w) =>
        line({ series: pattern.sum[w.key].usd, name: w.name, color: w.color, unit: Unit.usd }),
      ),
    },
    ...ROLLING_WINDOWS.map((w) => ({
      name: w.name,
      title: title(`Realized ${metricTitle} (${w.title})`),
      bottom: [line({ series: pattern.sum[w.key].usd, name: metricTitle, color, unit: Unit.usd })],
    })),
    {
      name: "Cumulative",
      title: title(`Realized ${metricTitle} (Total)`),
      bottom: [line({ series: pattern.cumulative.usd, name: metricTitle, color, unit: Unit.usd })],
    },
    ...(toRcap ? [{
      name: "% of Realized Cap",
      title: title(`Realized ${metricTitle} (% of Realized Cap)`),
      bottom: percentRatioBaseline({ pattern: toRcap, name: metricTitle, color }),
    }] : []),
  ];
}

/**
 * Net P&L folder: Compare + windows + Cumulative + optional % of Rcap + Change/
 * @param {Object} args
 * @param {NetPnlFullPattern | NetPnlBasicPattern} args.netPnl
 * @param {(name: string) => string} args.title
 * @param {{ percent: AnySeriesPattern, ratio: AnySeriesPattern }} [args.toRcap]
 * @param {PartialOptionsTree} [args.extraChange] - Additional change items (% of Mcap, % of Rcap)
 * @returns {PartialOptionsGroup}
 */
function realizedNetFolder({ netPnl, title, toRcap, extraChange = [] }) {
  return {
    name: "Net P&L",
    tree: [
      {
        name: "Compare",
        title: title("Net Realized P&L"),
        bottom: ROLLING_WINDOWS.map((w) =>
          baseline({ series: netPnl.sum[w.key].usd, name: w.name, color: w.color, unit: Unit.usd }),
        ),
      },
      ...ROLLING_WINDOWS.map((w) => ({
        name: w.name,
        title: title(`Net Realized P&L (${w.title})`),
        bottom: [baseline({ series: netPnl.sum[w.key].usd, name: "Net", unit: Unit.usd })],
      })),
      {
        name: "Cumulative",
        title: title("Net Realized P&L (Total)"),
        bottom: [baseline({ series: netPnl.cumulative.usd, name: "Net", unit: Unit.usd })],
      },
      ...(toRcap ? [{
        name: "% of Realized Cap",
        title: title("Net Realized P&L (% of Realized Cap)"),
        bottom: percentRatioBaseline({ pattern: toRcap, name: "Net" }),
      }] : []),
      {
        name: "Change",
        tree: [
          {
            name: "Absolute",
            tree: [
              {
                name: "Compare",
                title: title("Net Realized P&L Change"),
                bottom: ROLLING_WINDOWS.map((w) =>
                  baseline({ series: netPnl.delta.absolute[w.key].usd, name: w.name, color: w.color, unit: Unit.usd }),
                ),
              },
              ...ROLLING_WINDOWS.map((w) => ({
                name: w.name,
                title: title(`Net Realized P&L Change (${w.title})`),
                bottom: [baseline({ series: netPnl.delta.absolute[w.key].usd, name: "Change", unit: Unit.usd })],
              })),
            ],
          },
          {
            name: "Rate",
            tree: [
              {
                name: "Compare",
                title: title("Net Realized P&L Rate"),
                bottom: ROLLING_WINDOWS.flatMap((w) =>
                  percentRatio({ pattern: netPnl.delta.rate[w.key], name: w.name, color: w.color }),
                ),
              },
              ...ROLLING_WINDOWS.map((w) => ({
                name: w.name,
                title: title(`Net Realized P&L Rate (${w.title})`),
                bottom: percentRatioBaseline({ pattern: netPnl.delta.rate[w.key], name: "Rate" }),
              })),
            ],
          },
          ...extraChange,
        ],
      },
    ],
  };
}


/**
 * Realized overview folder: one chart per window showing net + profit (dotted) + neg. loss (dotted) + loss (hidden) + gross (hidden)
 * @param {Object} args
 * @param {{ sum: Record<string, { usd: AnySeriesPattern }> }} args.profit
 * @param {{ sum: Record<string, { usd: AnySeriesPattern }>, negative: { sum: Record<string, AnySeriesPattern> } }} args.loss
 * @param {{ sum: Record<string, { usd: AnySeriesPattern }> }} args.netPnl
 * @param {{ sum: Record<string, { usd: AnySeriesPattern }> }} [args.grossPnl]
 * @param {{ sum: Record<string, { usd: AnySeriesPattern }> }} [args.peakRegret]
 * @param {(name: string) => string} args.title
 * @returns {PartialOptionsGroup}
 */
function realizedOverviewFolder({ profit, loss, netPnl, grossPnl, peakRegret, title }) {
  return {
    name: "Overview",
    tree: ROLLING_WINDOWS.map((w) => ({
      name: w.name,
      title: title(`Realized P&L (${w.title})`),
      bottom: [
        baseline({ series: netPnl.sum[w.key].usd, name: "Net P&L", unit: Unit.usd }),
        dotted({ series: profit.sum[w.key].usd, name: "Profit", color: colors.profit, unit: Unit.usd }),
        dotted({ series: loss.negative.sum[w.key], name: "Neg. Loss", color: colors.loss, unit: Unit.usd }),
        dotted({ series: loss.sum[w.key].usd, name: "Loss", color: colors.loss, unit: Unit.usd, defaultActive: false }),
        ...(grossPnl ? [dotted({ series: grossPnl.sum[w.key].usd, name: "Gross", color: colors.bitcoin, unit: Unit.usd, defaultActive: false })] : []),
        ...(peakRegret ? [dotted({ series: peakRegret.sum[w.key].usd, name: "Peak Regret", color: colors.default, unit: Unit.usd, defaultActive: false })] : []),
        priceLine({ unit: Unit.usd }),
      ],
    })),
  };
}

// ============================================================================
// Realized Subfolder Builders
// ============================================================================

/**
 * Full realized subfolder (All/STH/LTH)
 * @param {RealizedPattern | LthRealizedPattern} r
 * @param {(name: string) => string} title
 * @returns {PartialOptionsGroup}
 */
function realizedSubfolderFull(r, title) {
  return {
    name: "Realized",
    tree: [
      realizedOverviewFolder({ profit: r.profit, loss: r.loss, netPnl: r.netPnl, grossPnl: r.grossPnl, peakRegret: r.peakRegret, title }),
      realizedNetFolder({
        netPnl: r.netPnl,
        title,
        toRcap: r.netPnl.toRcap,
        extraChange: [
          {
            name: "% of Market Cap",
            title: title("Net Realized P&L Change (% of Market Cap)"),
            bottom: percentRatioBaseline({ pattern: r.netPnl.change1m.toMcap, name: "30d Change" }),
          },
          {
            name: "% of Realized Cap",
            title: title("Net Realized P&L Change (% of Realized Cap)"),
            bottom: percentRatioBaseline({ pattern: r.netPnl.change1m.toRcap, name: "30d Change" }),
          },
        ],
      }),
      { name: "Profit", tree: realizedMetricFolder({ pattern: r.profit, metricTitle: "Profit", color: colors.profit, title, toRcap: r.profit.toRcap }) },
      { name: "Loss", tree: realizedMetricFolder({ pattern: r.loss, metricTitle: "Loss", color: colors.loss, title, toRcap: r.loss.toRcap }) },
      { name: "Gross P&L", tree: realizedMetricFolder({ pattern: r.grossPnl, metricTitle: "Gross P&L", color: colors.bitcoin, title }) },
      {
        name: "P/L Ratio",
        tree: [
          {
            name: "Compare",
            title: title("Realized P/L Ratio"),
            bottom: ROLLING_WINDOWS.map((w) =>
              baseline({ series: r.profitToLossRatio[w.key], name: w.name, color: w.color, unit: Unit.ratio, base: 1 }),
            ),
          },
          ...ROLLING_WINDOWS.map((w) => ({
            name: w.name,
            title: title(`Realized P/L Ratio (${w.title})`),
            bottom: [baseline({ series: r.profitToLossRatio[w.key], name: "P/L Ratio", unit: Unit.ratio, base: 1 })],
          })),
        ],
      },
      {
        name: "Peak Regret",
        tree: [
          {
            name: "Compare",
            title: title("Peak Regret"),
            bottom: ROLLING_WINDOWS.map((w) =>
              line({ series: r.peakRegret.sum[w.key].usd, name: w.name, color: w.color, unit: Unit.usd }),
            ),
          },
          ...ROLLING_WINDOWS.map((w) => ({
            name: w.name,
            title: title(`Peak Regret (${w.title})`),
            bottom: [line({ series: r.peakRegret.sum[w.key].usd, name: "Peak Regret", unit: Unit.usd })],
          })),
          {
            name: "Cumulative",
            title: title("Peak Regret (Total)"),
            bottom: [line({ series: r.peakRegret.cumulative.usd, name: "Peak Regret", unit: Unit.usd })],
          },
          {
            name: "% of Realized Cap",
            title: title("Peak Regret (% of Realized Cap)"),
            bottom: percentRatioBaseline({ pattern: r.peakRegret.toRcap, name: "Peak Regret" }),
          },
        ],
      },
    ],
  };
}

/**
 * Mid realized subfolder (AgeRange/MaxAge — has netPnl + delta, no relToRcap/peakRegret)
 * @param {MidRealizedPattern} r
 * @param {(name: string) => string} title
 * @returns {PartialOptionsGroup}
 */
function realizedSubfolderMid(r, title) {
  return {
    name: "Realized",
    tree: [
      realizedOverviewFolder({ profit: r.profit, loss: r.loss, netPnl: r.netPnl, title }),
      realizedNetFolder({ netPnl: r.netPnl, title }),
      { name: "Profit", tree: realizedMetricFolder({ pattern: r.profit, metricTitle: "Profit", color: colors.profit, title }) },
      { name: "Loss", tree: realizedMetricFolder({ pattern: r.loss, metricTitle: "Loss", color: colors.loss, title }) },
    ],
  };
}

/**
 * Basic realized subfolder (no netPnl, no relToRcap)
 * @param {BasicRealizedPattern} r
 * @param {(name: string) => string} title
 * @returns {PartialOptionsGroup}
 */
function realizedSubfolderBasic(r, title) {
  return {
    name: "Realized",
    tree: [
      { name: "Profit", tree: realizedMetricFolder({ pattern: r.profit, metricTitle: "Profit", color: colors.profit, title }) },
      { name: "Loss", tree: realizedMetricFolder({ pattern: r.loss, metricTitle: "Loss", color: colors.loss, title }) },
    ],
  };
}

// ============================================================================
// Single Cohort Section Builders
// ============================================================================

/**
 * Basic profitability section (NUPL only unrealized, basic realized)
 * @param {{ cohort: UtxoCohortObject, title: (name: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createProfitabilitySection({ cohort, title }) {
  const { tree } = cohort;
  return {
    name: "Profitability",
    tree: [
      {
        name: "Unrealized",
        tree: [
          { name: "NUPL", title: title("NUPL"), bottom: nuplSeries(tree.unrealized.nupl) },
        ],
      },
      realizedSubfolderBasic(tree.realized, title),
    ],
  };
}

/**
 * Profitability section with unrealized P&L + NUPL (no netPnl, no rel)
 * For: CohortWithoutRelative (p2ms, unknown, empty)
 * @param {{ cohort: CohortWithoutRelative, title: (name: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createProfitabilitySectionWithProfitLoss({ cohort, title }) {
  const u = cohort.tree.unrealized;
  return {
    name: "Profitability",
    tree: [
      {
        name: "Unrealized",
        tree: [
          { name: "NUPL", title: title("NUPL"), bottom: nuplSeries(u.nupl) },
          { name: "Profit", title: title("Unrealized Profit"), bottom: [line({ series: u.profit.usd, name: "Profit", color: colors.profit, unit: Unit.usd })] },
          { name: "Loss", title: title("Unrealized Loss"), bottom: [line({ series: u.loss.usd, name: "Loss", color: colors.loss, unit: Unit.usd })] },
        ],
      },
      realizedSubfolderBasic(cohort.tree.realized, title),
    ],
  };
}

/**
 * Section for All cohort
 * @param {{ cohort: CohortAll, title: (name: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createProfitabilitySectionAll({ cohort, title }) {
  const u = cohort.tree.unrealized;
  const r = cohort.tree.realized;
  return {
    name: "Profitability",
    tree: [
      { name: "Unrealized", tree: unrealizedTreeAll(u, title) },
      realizedSubfolderFull(r, title),
      {
        name: "Invested Capital",
        title: title("Invested Capital In Profit & Loss"),
        bottom: investedCapitalSeries(u),
      },
      { name: "Sentiment", title: title("Market Sentiment"), bottom: sentimentSeries(u) },
    ],
  };
}

/**
 * Section for Full cohorts (STH)
 * @param {{ cohort: CohortFull, title: (name: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createProfitabilitySectionFull({ cohort, title }) {
  const u = cohort.tree.unrealized;
  const r = cohort.tree.realized;
  return {
    name: "Profitability",
    tree: [
      { name: "Unrealized", tree: unrealizedTreeFull(u, title) },
      realizedSubfolderFull(r, title),
      {
        name: "Invested Capital",
        title: title("Invested Capital In Profit & Loss"),
        bottom: investedCapitalSeries(u),
      },
      { name: "Sentiment", title: title("Market Sentiment"), bottom: sentimentSeries(u) },
    ],
  };
}

/**
 * Section with NUPL (basic cohorts with market cap — NuplPattern unrealized)
 * @param {{ cohort: CohortBasicWithMarketCap, title: (name: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createProfitabilitySectionWithNupl({ cohort, title }) {
  const { tree } = cohort;
  return {
    name: "Profitability",
    tree: [
      {
        name: "Unrealized",
        tree: [
          { name: "NUPL", title: title("NUPL"), bottom: nuplSeries(tree.unrealized.nupl) },
        ],
      },
      realizedSubfolderBasic(tree.realized, title),
    ],
  };
}

/**
 * Section for LongTerm cohort
 * @param {{ cohort: CohortLongTerm, title: (name: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createProfitabilitySectionLongTerm({ cohort, title }) {
  const u = cohort.tree.unrealized;
  const r = cohort.tree.realized;
  return {
    name: "Profitability",
    tree: [
      { name: "Unrealized", tree: unrealizedTreeLongTerm(u, title) },
      realizedSubfolderFull(r, title),
      {
        name: "Invested Capital",
        title: title("Invested Capital In Profit & Loss"),
        bottom: investedCapitalSeries(u),
      },
      { name: "Sentiment", title: title("Market Sentiment"), bottom: sentimentSeries(u) },
    ],
  };
}

/**
 * Section for AgeRange cohorts (mid-tier: has unrealized profit/loss/netPnl, mid realized)
 * @param {{ cohort: CohortAgeRange, title: (name: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createProfitabilitySectionWithInvestedCapitalPct({ cohort, title }) {
  const u = cohort.tree.unrealized;
  const r = cohort.tree.realized;
  return {
    name: "Profitability",
    tree: [
      { name: "Unrealized", tree: unrealizedTreeMid(u, title) },
      realizedSubfolderMid(r, title),
    ],
  };
}

/**
 * Section with invested capital % but no unrealized relative (basic cohorts)
 * @param {{ cohort: CohortBasicWithoutMarketCap, title: (name: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createProfitabilitySectionBasicWithInvestedCapitalPct({ cohort, title }) {
  const { tree } = cohort;
  return {
    name: "Profitability",
    tree: [
      {
        name: "Unrealized",
        tree: [
          { name: "NUPL", title: title("NUPL"), bottom: nuplSeries(tree.unrealized.nupl) },
        ],
      },
      realizedSubfolderBasic(tree.realized, title),
    ],
  };
}

/**
 * Section for CohortAddr (has unrealized profit/loss + NUPL, basic realized)
 * @param {{ cohort: CohortAddr, title: (name: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createProfitabilitySectionAddress({ cohort, title }) {
  const u = cohort.tree.unrealized;
  return {
    name: "Profitability",
    tree: [
      {
        name: "Unrealized",
        tree: [
          { name: "NUPL", title: title("NUPL"), bottom: nuplSeries(u.nupl) },
          { name: "Profit", title: title("Unrealized Profit"), bottom: [line({ series: u.profit.usd, name: "Profit", color: colors.profit, unit: Unit.usd })] },
          { name: "Loss", title: title("Unrealized Loss"), bottom: [line({ series: u.loss.usd, name: "Loss", color: colors.loss, unit: Unit.usd })] },
        ],
      },
      realizedSubfolderBasic(cohort.tree.realized, title),
    ],
  };
}

// ============================================================================
// Grouped Cohort Helpers
// ============================================================================

/**
 * Grouped realized P&L sum (basic — all cohorts have profit/loss)
 * @template {{ name: string, color: Color, tree: { realized: { profit: RealizedProfitLossPattern, loss: RealizedProfitLossPattern } } }} T
 * @template {{ name: string, color: Color, tree: { realized: { profit: RealizedProfitLossPattern, loss: RealizedProfitLossPattern } } }} A
 * @param {readonly T[]} list
 * @param {A} all
 * @param {(name: string) => string} title
 * @returns {PartialOptionsTree}
 */
function groupedRealizedPnlSum(list, all, title) {
  return [
    {
      name: "Profit",
      title: title("Realized Profit"),
      bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
        line({ series: tree.realized.profit.base.usd, name, color, unit: Unit.usd }),
      ),
    },
    {
      name: "Loss",
      title: title("Realized Loss"),
      bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
        line({ series: tree.realized.loss.base.usd, name, color, unit: Unit.usd }),
      ),
    },
  ];
}

/**
 * Grouped realized P&L sum with extras (full cohorts)
 * @param {readonly (CohortAll | CohortFull | CohortLongTerm)[]} list
 * @param {CohortAll} all
 * @param {(name: string) => string} title
 * @returns {PartialOptionsTree}
 */
function groupedRealizedPnlSumFull(list, all, title) {
  return [
    ...groupedRealizedPnlSum(list, all, title),
    {
      name: "Total",
      title: title("Total Realized P&L"),
      bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
        line({ series: tree.realized.grossPnl.cumulative.usd, name, color, unit: Unit.usd }),
      ),
    },
    {
      name: "P/L Ratio",
      title: title("Realized Profit/Loss Ratio"),
      bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
        baseline({ series: tree.realized.profitToLossRatio._1y, name, color, unit: Unit.ratio, base: 1 }),
      ),
    },
  ];
}

/**
 * Grouped rolling realized charts (basic — profit/loss sums only)
 * @template {{ name: string, color: Color, tree: { realized: { profit: RealizedProfitLossPattern, loss: RealizedProfitLossPattern } } }} T
 * @template {{ name: string, color: Color, tree: { realized: { profit: RealizedProfitLossPattern, loss: RealizedProfitLossPattern } } }} A
 * @param {readonly T[]} list
 * @param {A} all
 * @param {(name: string) => string} title
 * @returns {PartialOptionsTree}
 */
function groupedRollingRealizedCharts(list, all, title) {
  return [
    {
      name: "Profit",
      tree: ROLLING_WINDOWS.map((w) => ({
        name: w.name,
        title: title(`Realized Profit (${w.title})`),
        bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) => line({ series: tree.realized.profit.sum[w.key].usd, name, color, unit: Unit.usd })),
      })),
    },
    {
      name: "Loss",
      tree: ROLLING_WINDOWS.map((w) => ({
        name: w.name,
        title: title(`Realized Loss (${w.title})`),
        bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) => line({ series: tree.realized.loss.sum[w.key].usd, name, color, unit: Unit.usd })),
      })),
    },
  ];
}

/**
 * Grouped rolling realized with P/L ratio (full cohorts)
 * @param {readonly (CohortAll | CohortFull | CohortLongTerm)[]} list
 * @param {CohortAll} all
 * @param {(name: string) => string} title
 * @returns {PartialOptionsTree}
 */
function groupedRollingRealizedChartsFull(list, all, title) {
  return [
    ...groupedRollingRealizedCharts(list, all, title),
    {
      name: "P/L Ratio",
      tree: ROLLING_WINDOWS.map((w) => ({
        name: w.name,
        title: title(`Realized P/L Ratio (${w.title})`),
        bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
          baseline({ series: tree.realized.profitToLossRatio[w.key], name, color, unit: Unit.ratio, base: 1 }),
        ),
      })),
    },
  ];
}

/**
 * Grouped realized subfolder (basic)
 * @template {{ name: string, color: Color, tree: { realized: { profit: RealizedProfitLossPattern, loss: RealizedProfitLossPattern } } }} T
 * @template {{ name: string, color: Color, tree: { realized: { profit: RealizedProfitLossPattern, loss: RealizedProfitLossPattern } } }} A
 * @param {readonly T[]} list
 * @param {A} all
 * @param {(name: string) => string} title
 * @returns {PartialOptionsGroup}
 */
function groupedRealizedSubfolder(list, all, title) {
  return {
    name: "Realized",
    tree: [
      { name: "P&L", tree: groupedRealizedPnlSum(list, all, title) },
      { name: "Rolling", tree: groupedRollingRealizedCharts(list, all, title) },
      {
        name: "Cumulative",
        tree: [
          {
            name: "Profit",
            title: title("Cumulative Realized Profit"),
            bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
              line({ series: tree.realized.profit.cumulative.usd, name, color, unit: Unit.usd }),
            ),
          },
          {
            name: "Loss",
            title: title("Cumulative Realized Loss"),
            bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
              line({ series: tree.realized.loss.cumulative.usd, name, color, unit: Unit.usd }),
            ),
          },
        ],
      },
    ],
  };
}

/**
 * Grouped net realized P&L delta (Absolute + Rate with all rolling windows)
 * @param {readonly (CohortAll | CohortFull | CohortLongTerm)[]} list
 * @param {CohortAll} all
 * @param {(name: string) => string} title
 * @returns {PartialOptionsGroup}
 */
function groupedRealizedNetPnlDeltaTree(list, all, title) {
  return {
    name: "Change",
    tree: [
      {
        name: "Absolute",
        tree: [
          {
            name: "Compare",
            title: title("Net Realized P&L Change"),
            bottom: ROLLING_WINDOWS.flatMap((w) =>
              mapCohortsWithAll(list, all, ({ name, tree }) =>
                baseline({ series: tree.realized.netPnl.delta.absolute[w.key].usd, name: `${name} (${w.name})`, color: w.color, unit: Unit.usd }),
              ),
            ),
          },
          ...ROLLING_WINDOWS.map((w) => ({
            name: w.name,
            title: title(`Net Realized P&L Change (${w.title})`),
            bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
              baseline({ series: tree.realized.netPnl.delta.absolute[w.key].usd, name, color, unit: Unit.usd }),
            ),
          })),
        ],
      },
      {
        name: "Rate",
        tree: [
          {
            name: "Compare",
            title: title("Net Realized P&L Rate"),
            bottom: ROLLING_WINDOWS.flatMap((w) =>
              flatMapCohortsWithAll(list, all, ({ name, tree }) =>
                percentRatio({ pattern: tree.realized.netPnl.delta.rate[w.key], name: `${name} (${w.name})`, color: w.color }),
              ),
            ),
          },
          ...ROLLING_WINDOWS.map((w) => ({
            name: w.name,
            title: title(`Net Realized P&L Rate (${w.title})`),
            bottom: flatMapCohortsWithAll(list, all, ({ name, color, tree }) =>
              percentRatio({ pattern: tree.realized.netPnl.delta.rate[w.key], name, color }),
            ),
          })),
        ],
      },
    ],
  };
}

/**
 * Grouped realized subfolder for full cohorts
 * @param {readonly (CohortAll | CohortFull | CohortLongTerm)[]} list
 * @param {CohortAll} all
 * @param {(name: string) => string} title
 * @returns {PartialOptionsGroup}
 */
function groupedRealizedSubfolderFull(list, all, title) {
  return {
    name: "Realized",
    tree: [
      { name: "P&L", tree: groupedRealizedPnlSumFull(list, all, title) },
      {
        name: "Net",
        title: title("Net Realized P&L"),
        bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
          baseline({ series: tree.realized.netPnl.base.usd, name, color, unit: Unit.usd }),
        ),
      },
      groupedRealizedNetPnlDeltaTree(list, all, title),
      { name: "Rolling", tree: groupedRollingRealizedChartsFull(list, all, title) },
      {
        name: "Cumulative",
        tree: [
          {
            name: "Profit",
            title: title("Cumulative Realized Profit"),
            bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
              line({ series: tree.realized.profit.cumulative.usd, name, color, unit: Unit.usd }),
            ),
          },
          {
            name: "Loss",
            title: title("Cumulative Realized Loss"),
            bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
              line({ series: tree.realized.loss.cumulative.usd, name, color, unit: Unit.usd }),
            ),
          },
          {
            name: "Net",
            title: title("Cumulative Net Realized P&L"),
            bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
              baseline({ series: tree.realized.netPnl.cumulative.usd, name, color, unit: Unit.usd }),
            ),
          },
        ],
      },
    ],
  };
}

/**
 * Grouped NUPL chart
 * @template {{ name: string, color: Color, tree: { unrealized: { nupl: NuplPattern } } }} T
 * @template {{ name: string, color: Color, tree: { unrealized: { nupl: NuplPattern } } }} A
 * @param {readonly T[]} list
 * @param {A} all
 * @param {(name: string) => string} title
 * @returns {PartialOptionsTree}
 */
function groupedNuplCharts(list, all, title) {
  return [
    {
      name: "NUPL",
      title: title("NUPL"),
      bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
        baseline({ series: tree.unrealized.nupl.ratio, name, color, unit: Unit.ratio }),
      ),
    },
  ];
}

/**
 * Grouped unrealized: Net → NUPL → Profit → Loss (no relative)
 * @param {readonly (CohortAgeRange | CohortWithAdjusted)[]} list
 * @param {CohortAll} all
 * @param {(name: string) => string} title
 * @returns {PartialOptionsTree}
 */
function groupedUnrealizedMid(list, all, title) {
  return [
    {
      name: "Net P&L",
      title: title("Net Unrealized P&L"),
      bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
        baseline({ series: tree.unrealized.netPnl.usd, name, color, unit: Unit.usd }),
      ),
    },
    ...groupedNuplCharts(list, all, title),
    {
      name: "Profit",
      title: title("Unrealized Profit"),
      bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
        line({ series: tree.unrealized.profit.usd, name, color, unit: Unit.usd }),
      ),
    },
    {
      name: "Loss",
      title: title("Unrealized Loss"),
      bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
        line({ series: tree.unrealized.loss.usd, name, color, unit: Unit.usd }),
      ),
    },
  ];
}

/**
 * Grouped unrealized: Net → NUPL → Profit → Loss → Relative(Market Cap)
 * @param {readonly (CohortFull | CohortLongTerm)[]} list
 * @param {CohortAll} all
 * @param {(name: string) => string} title
 * @returns {PartialOptionsTree}
 */
function groupedUnrealizedWithMarketCap(list, all, title) {
  return [
    ...groupedUnrealizedMid(list, all, title),
    {
      name: "% of Market Cap",
      tree: [
        {
          name: "Profit",
          title: title("Unrealized Profit (% of Market Cap)"),
          bottom: flatMapCohortsWithAll(list, all, ({ name, color, tree }) =>
            percentRatio({ pattern: tree.unrealized.profit.toMcap, name, color }),
          ),
        },
        {
          name: "Loss",
          title: title("Unrealized Loss (% of Market Cap)"),
          bottom: flatMapCohortsWithAll(list, all, ({ name, color, tree }) =>
            percentRatio({ pattern: tree.unrealized.loss.toMcap, name, color }),
          ),
        },
      ],
    },
  ];
}

/**
 * Grouped unrealized for LongTerm: Net → NUPL → Profit → Loss → Relative(Own P&L, Market Cap, Own Mcap)
 * @param {readonly CohortLongTerm[]} list
 * @param {CohortAll} all
 * @param {(name: string) => string} title
 * @returns {PartialOptionsTree}
 */
function groupedUnrealizedLongTerm(list, all, title) {
  return [
    ...groupedUnrealizedMid(list, all, title),
    {
      name: "Relative",
      tree: [
        {
          name: "Own P&L",
          tree: [
            {
              name: "Net",
              title: title("Net Unrealized P&L (% of Own P&L)"),
              bottom: flatMapCohortsWithAll(list, all, ({ name, color, tree }) =>
                percentRatioBaseline({ pattern: tree.unrealized.netPnl.toOwnGrossPnl, name, color }),
              ),
            },
            {
              name: "Profit",
              title: title("Unrealized Profit (% of Own P&L)"),
              bottom: flatMapCohortsWithAll(list, all, ({ name, color, tree }) =>
                percentRatio({ pattern: tree.unrealized.profit.toOwnGrossPnl, name, color }),
              ),
            },
            {
              name: "Loss",
              title: title("Unrealized Loss (% of Own P&L)"),
              bottom: flatMapCohortsWithAll(list, all, ({ name, color, tree }) =>
                percentRatio({ pattern: tree.unrealized.loss.toOwnGrossPnl, name, color }),
              ),
            },
          ],
        },
        {
          name: "Market Cap",
          title: title("Unrealized Loss (% of Market Cap)"),
          bottom: flatMapCohortsWithAll(list, all, ({ name, color, tree }) =>
            percentRatio({ pattern: tree.unrealized.loss.toMcap, name, color }),
          ),
        },
        {
          name: "Own Market Cap",
          tree: [
            {
              name: "Net",
              title: title("Net Unrealized P&L (% of Own Market Cap)"),
              bottom: flatMapCohorts(list, ({ name, color, tree }) =>
                percentRatioBaseline({ pattern: tree.unrealized.netPnl.toOwnMcap, name, color }),
              ),
            },
            {
              name: "Profit",
              title: title("Unrealized Profit (% of Own Market Cap)"),
              bottom: flatMapCohorts(list, ({ name, color, tree }) =>
                percentRatio({ pattern: tree.unrealized.profit.toOwnMcap, name, color }),
              ),
            },
            {
              name: "Loss",
              title: title("Unrealized Loss (% of Own Market Cap)"),
              bottom: flatMapCohorts(list, ({ name, color, tree }) =>
                percentRatio({ pattern: tree.unrealized.loss.toOwnMcap, name, color }),
              ),
            },
          ],
        },
      ],
    },
  ];
}

/**
 * Grouped sentiment (full unrealized only)
 * @param {readonly (CohortAll | CohortFull | CohortLongTerm)[]} list
 * @param {CohortAll} all
 * @param {(name: string) => string} title
 * @returns {PartialOptionsGroup}
 */
function groupedSentiment(list, all, title) {
  return {
    name: "Sentiment",
    tree: [
      {
        name: "Net",
        title: title("Net Sentiment"),
        bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
          baseline({ series: tree.unrealized.sentiment.net.usd, name, color, unit: Unit.usd }),
        ),
      },
      {
        name: "Greed",
        title: title("Greed Index"),
        bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
          line({ series: tree.unrealized.sentiment.greedIndex.usd, name, color, unit: Unit.usd }),
        ),
      },
      {
        name: "Pain",
        title: title("Pain Index"),
        bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
          line({ series: tree.unrealized.sentiment.painIndex.usd, name, color, unit: Unit.usd }),
        ),
      },
    ],
  };
}

// ============================================================================
// Grouped Section Builders
// ============================================================================

/**
 * Grouped profitability section (basic — NUPL only)
 * @param {{ list: readonly (UtxoCohortObject | CohortWithoutRelative)[], all: CohortAll, title: (name: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createGroupedProfitabilitySection({ list, all, title }) {
  return {
    name: "Profitability",
    tree: [
      { name: "Unrealized", tree: groupedNuplCharts(list, all, title) },
      groupedRealizedSubfolder(list, all, title),
    ],
  };
}

/**
 * Grouped profitability with unrealized profit/loss + NUPL
 * For: CohortWithoutRelative (p2ms, unknown, empty)
 * @param {{ list: readonly CohortWithoutRelative[], all: CohortAll, title: (name: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createGroupedProfitabilitySectionWithProfitLoss({ list, all, title }) {
  return {
    name: "Profitability",
    tree: [
      {
        name: "Unrealized",
        tree: [
          ...groupedNuplCharts(list, all, title),
          {
            name: "Profit",
            title: title("Unrealized Profit"),
            bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
              line({ series: tree.unrealized.profit.usd, name, color, unit: Unit.usd }),
            ),
          },
          {
            name: "Loss",
            title: title("Unrealized Loss"),
            bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
              line({ series: tree.unrealized.loss.usd, name, color, unit: Unit.usd }),
            ),
          },
        ],
      },
      groupedRealizedSubfolder(list, all, title),
    ],
  };
}

/**
 * Grouped section with invested capital % (basic cohorts — uses NUPL only)
 * @param {{ list: readonly CohortBasicWithoutMarketCap[], all: CohortAll, title: (name: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createGroupedProfitabilitySectionBasicWithInvestedCapitalPct({ list, all, title }) {
  return {
    name: "Profitability",
    tree: [
      { name: "Unrealized", tree: groupedNuplCharts(list, all, title) },
      groupedRealizedSubfolder(list, all, title),
    ],
  };
}

/**
 * Grouped section for ageRange/maxAge cohorts
 * @param {{ list: readonly (CohortAgeRange | CohortWithAdjusted)[], all: CohortAll, title: (name: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createGroupedProfitabilitySectionWithInvestedCapitalPct({ list, all, title }) {
  return {
    name: "Profitability",
    tree: [
      { name: "Unrealized", tree: groupedUnrealizedMid(list, all, title) },
      groupedRealizedSubfolder(list, all, title),
    ],
  };
}

/**
 * Grouped section with NUPL + relToMcap
 * @param {{ list: readonly (CohortFull | CohortLongTerm)[], all: CohortAll, title: (name: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createGroupedProfitabilitySectionWithNupl({ list, all, title }) {
  return {
    name: "Profitability",
    tree: [
      { name: "Unrealized", tree: groupedUnrealizedWithMarketCap(list, all, title) },
      groupedRealizedSubfolder(list, all, title),
    ],
  };
}

/**
 * Grouped section for LongTerm cohorts
 * @param {{ list: readonly CohortLongTerm[], all: CohortAll, title: (name: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createGroupedProfitabilitySectionLongTerm({ list, all, title }) {
  return {
    name: "Profitability",
    tree: [
      { name: "Unrealized", tree: groupedUnrealizedLongTerm(list, all, title) },
      groupedRealizedSubfolderFull(list, all, title),
      groupedSentiment(list, all, title),
    ],
  };
}
