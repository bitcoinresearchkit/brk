/**
 * Profitability section builders
 */

import { Unit } from "../../utils/units.js";
import { line, baseline, dots, dotsBaseline } from "../series.js";
import { colors } from "../../utils/colors.js";
import { priceLine, priceLines } from "../constants.js";
import {
  satsBtcUsd,
  satsBtcUsdFrom,
  mapCohorts,
  mapCohortsWithAll,
  flatMapCohortsWithAll,
} from "../shared.js";

// ============================================================================
// Core Series Builders (Composable Primitives)
// ============================================================================

/**
 * @typedef {Object} PnlSeriesConfig
 * @property {AnyMetricPattern} profit
 * @property {AnyMetricPattern} loss
 * @property {AnyMetricPattern} negLoss
 * @property {AnyMetricPattern} [total]
 */

/**
 * Create profit/loss line series for a unit
 * @param {PnlSeriesConfig} metrics
 * @param {Unit} unit
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function pnlLines(metrics, unit) {
  const series = [
    line({
      metric: metrics.profit,
      name: "Profit",
      color: colors.profit,
      unit,
    }),
    line({ metric: metrics.loss, name: "Loss", color: colors.loss, unit }),
  ];
  if (metrics.total) {
    series.push(
      line({
        metric: metrics.total,
        name: "Total",
        color: colors.default,
        unit,
      }),
    );
  }
  series.push(
    line({
      metric: metrics.negLoss,
      name: "Negative Loss",
      color: colors.loss,
      unit,
      defaultActive: false,
    }),
  );
  return series;
}

/**
 * Create net P&L baseline
 * @param {AnyMetricPattern} metric
 * @param {Unit} unit
 * @returns {AnyFetchedSeriesBlueprint}
 */
function netBaseline(metric, unit) {
  return baseline({ metric, name: "Net P&L", unit });
}

// ============================================================================
// Unrealized P&L Builders
// ============================================================================

/**
 * @typedef {Object} UnrealizedMetrics
 * @property {AnyMetricPattern} profit
 * @property {AnyMetricPattern} loss
 * @property {AnyMetricPattern} negLoss
 * @property {AnyMetricPattern} total
 * @property {AnyMetricPattern} net
 */

/**
 * Extract unrealized metrics from tree
 * @param {{ unrealized: UnrealizedPattern }} tree
 * @returns {UnrealizedMetrics}
 */
function getUnrealizedMetrics(tree) {
  return {
    profit: tree.unrealized.unrealizedProfit,
    loss: tree.unrealized.unrealizedLoss,
    negLoss: tree.unrealized.negUnrealizedLoss,
    total: tree.unrealized.totalUnrealizedPnl,
    net: tree.unrealized.netUnrealizedPnl,
  };
}

/**
 * Base unrealized P&L (USD only)
 * @param {UnrealizedMetrics} m
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function unrealizedUsd(m) {
  return [
    ...pnlLines(
      { profit: m.profit, loss: m.loss, negLoss: m.negLoss, total: m.total },
      Unit.usd,
    ),
    priceLine({ unit: Unit.usd, defaultActive: false }),
  ];
}

/**
 * Unrealized P&L with % of Market Cap
 * @param {UnrealizedMetrics} m
 * @param {RelativeWithNupl} rel
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function unrealizedWithMarketCap(m, rel) {
  return [
    ...unrealizedUsd(m),
    ...pnlLines(
      {
        profit: rel.unrealizedProfitRelToMarketCap,
        loss: rel.unrealizedLossRelToMarketCap,
        negLoss: rel.negUnrealizedLossRelToMarketCap,
      },
      Unit.pctMcap,
    ),
    priceLine({ unit: Unit.pctMcap, defaultActive: false }),
  ];
}

/**
 * Unrealized P&L with % of Own Market Cap + % of Own P&L
 * @param {UnrealizedMetrics} m
 * @param {RelativeWithOwnMarketCap} rel
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function unrealizedWithOwnMarketCap(m, rel) {
  return [
    ...unrealizedUsd(m),
    ...pnlLines(
      {
        profit: rel.unrealizedProfitRelToOwnMarketCap,
        loss: rel.unrealizedLossRelToOwnMarketCap,
        negLoss: rel.negUnrealizedLossRelToOwnMarketCap,
      },
      Unit.pctOwnMcap,
    ),
    priceLine({ unit: Unit.pctOwnMcap, defaultActive: false }),
    ...pnlLines(
      {
        profit: rel.unrealizedProfitRelToOwnTotalUnrealizedPnl,
        loss: rel.unrealizedLossRelToOwnTotalUnrealizedPnl,
        negLoss: rel.negUnrealizedLossRelToOwnTotalUnrealizedPnl,
      },
      Unit.pctOwnPnl,
    ),
    ...priceLines({ numbers: [100, 50, 0], unit: Unit.pctOwnPnl }),
  ];
}

/**
 * Unrealized P&L for "all" cohort (% M.Cap + % Own P&L, no Own M.Cap)
 * @param {UnrealizedMetrics} m
 * @param {AllRelativePattern} rel
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function unrealizedAll(m, rel) {
  return [
    ...unrealizedUsd(m),
    ...pnlLines(
      {
        profit: rel.unrealizedProfitRelToMarketCap,
        loss: rel.unrealizedLossRelToMarketCap,
        negLoss: rel.negUnrealizedLossRelToMarketCap,
      },
      Unit.pctMcap,
    ),
    priceLine({ unit: Unit.pctMcap, defaultActive: false }),
    ...pnlLines(
      {
        profit: rel.unrealizedProfitRelToOwnTotalUnrealizedPnl,
        loss: rel.unrealizedLossRelToOwnTotalUnrealizedPnl,
        negLoss: rel.negUnrealizedLossRelToOwnTotalUnrealizedPnl,
      },
      Unit.pctOwnPnl,
    ),
    ...priceLines({ numbers: [100, 50, 0], unit: Unit.pctOwnPnl }),
  ];
}

/**
 * Unrealized P&L for Full cohorts (all relative metrics)
 * @param {UnrealizedMetrics} m
 * @param {FullRelativePattern} rel
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function unrealizedFull(m, rel) {
  return [
    ...unrealizedUsd(m),
    ...pnlLines(
      {
        profit: rel.unrealizedProfitRelToMarketCap,
        loss: rel.unrealizedLossRelToMarketCap,
        negLoss: rel.negUnrealizedLossRelToMarketCap,
      },
      Unit.pctMcap,
    ),
    priceLine({ unit: Unit.pctMcap, defaultActive: false }),
    ...pnlLines(
      {
        profit: rel.unrealizedProfitRelToOwnMarketCap,
        loss: rel.unrealizedLossRelToOwnMarketCap,
        negLoss: rel.negUnrealizedLossRelToOwnMarketCap,
      },
      Unit.pctOwnMcap,
    ),
    priceLine({ unit: Unit.pctOwnMcap, defaultActive: false }),
    ...pnlLines(
      {
        profit: rel.unrealizedProfitRelToOwnTotalUnrealizedPnl,
        loss: rel.unrealizedLossRelToOwnTotalUnrealizedPnl,
        negLoss: rel.negUnrealizedLossRelToOwnTotalUnrealizedPnl,
      },
      Unit.pctOwnPnl,
    ),
    ...priceLines({ numbers: [100, 50, 0], unit: Unit.pctOwnPnl }),
  ];
}

/**
 * Unrealized P&L for LongTerm (% M.Cap loss only + Own M.Cap + Own P&L)
 * @param {UnrealizedMetrics} m
 * @param {RelativeWithOwnMarketCap & RelativeWithNupl} rel
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function unrealizedLongTerm(m, rel) {
  return [
    ...unrealizedUsd(m),
    line({
      metric: rel.unrealizedLossRelToMarketCap,
      name: "Loss",
      color: colors.loss,
      unit: Unit.pctMcap,
    }),
    ...pnlLines(
      {
        profit: rel.unrealizedProfitRelToOwnMarketCap,
        loss: rel.unrealizedLossRelToOwnMarketCap,
        negLoss: rel.negUnrealizedLossRelToOwnMarketCap,
      },
      Unit.pctOwnMcap,
    ),
    priceLine({ unit: Unit.pctOwnMcap, defaultActive: false }),
    ...pnlLines(
      {
        profit: rel.unrealizedProfitRelToOwnTotalUnrealizedPnl,
        loss: rel.unrealizedLossRelToOwnTotalUnrealizedPnl,
        negLoss: rel.negUnrealizedLossRelToOwnTotalUnrealizedPnl,
      },
      Unit.pctOwnPnl,
    ),
    ...priceLines({ numbers: [100, 50, 0], unit: Unit.pctOwnPnl }),
  ];
}

// ============================================================================
// Net Unrealized P&L Builders
// ============================================================================

/**
 * Net P&L (USD only)
 * @param {AnyMetricPattern} net
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function netUnrealizedUsd(net) {
  return [netBaseline(net, Unit.usd)];
}

/**
 * Net P&L with % of Market Cap
 * @param {AnyMetricPattern} net
 * @param {RelativeWithNupl} rel
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function netUnrealizedWithMarketCap(net, rel) {
  return [
    netBaseline(net, Unit.usd),
    netBaseline(rel.netUnrealizedPnlRelToMarketCap, Unit.pctMcap),
  ];
}

/**
 * Net P&L with % of Own Market Cap + % of Own P&L
 * @param {AnyMetricPattern} net
 * @param {RelativeWithOwnMarketCap} rel
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function netUnrealizedWithOwnMarketCap(net, rel) {
  return [
    netBaseline(net, Unit.usd),
    netBaseline(rel.netUnrealizedPnlRelToOwnMarketCap, Unit.pctOwnMcap),
    netBaseline(rel.netUnrealizedPnlRelToOwnTotalUnrealizedPnl, Unit.pctOwnPnl),
  ];
}

/**
 * Net P&L for "all" cohort
 * @param {AnyMetricPattern} net
 * @param {AllRelativePattern} rel
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function netUnrealizedAll(net, rel) {
  return [
    netBaseline(net, Unit.usd),
    netBaseline(rel.netUnrealizedPnlRelToMarketCap, Unit.pctMcap),
    netBaseline(rel.netUnrealizedPnlRelToOwnTotalUnrealizedPnl, Unit.pctOwnPnl),
  ];
}

/**
 * Net P&L for Full cohorts
 * @param {AnyMetricPattern} net
 * @param {FullRelativePattern} rel
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function netUnrealizedFull(net, rel) {
  return [
    netBaseline(net, Unit.usd),
    netBaseline(rel.netUnrealizedPnlRelToMarketCap, Unit.pctMcap),
    netBaseline(rel.netUnrealizedPnlRelToOwnMarketCap, Unit.pctOwnMcap),
    netBaseline(rel.netUnrealizedPnlRelToOwnTotalUnrealizedPnl, Unit.pctOwnPnl),
  ];
}

// ============================================================================
// Invested Capital & Other Unrealized
// ============================================================================

/**
 * Invested capital (USD only)
 * @param {{ unrealized: UnrealizedPattern }} tree
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function investedCapitalAbsolute(tree) {
  return [
    line({
      metric: tree.unrealized.investedCapitalInProfit,
      name: "In Profit",
      color: colors.profit,
      unit: Unit.usd,
    }),
    line({
      metric: tree.unrealized.investedCapitalInLoss,
      name: "In Loss",
      color: colors.loss,
      unit: Unit.usd,
    }),
  ];
}

/**
 * Invested capital with % of Own R.Cap
 * @param {{ unrealized: UnrealizedPattern, relative: RelativeWithInvestedCapitalPct }} tree
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function investedCapitalWithPct(tree) {
  return [
    ...investedCapitalAbsolute(tree),
    baseline({
      metric: tree.relative.investedCapitalInProfitPct,
      name: "In Profit",
      color: colors.profit,
      unit: Unit.pctOwnRcap,
    }),
    baseline({
      metric: tree.relative.investedCapitalInLossPct,
      name: "In Loss",
      color: colors.loss,
      unit: Unit.pctOwnRcap,
    }),
    ...priceLines({ numbers: [100, 50], unit: Unit.pctOwnRcap }),
  ];
}

/**
 * NUPL series
 * @param {RelativeWithNupl} rel
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function nuplSeries(rel) {
  return [baseline({ metric: rel.nupl, name: "NUPL", unit: Unit.ratio })];
}

/**
 * Peak regret (USD only)
 * @param {{ unrealized: UnrealizedFullPattern }} tree
 * @param {Color} color
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function peakRegretAbsolute(tree, color) {
  return [
    line({
      metric: tree.unrealized.peakRegret,
      name: "Peak Regret",
      color,
      unit: Unit.usd,
    }),
  ];
}

/**
 * Peak regret with % of Market Cap
 * @param {{ unrealized: UnrealizedFullPattern, relative: RelativeWithPeakRegret }} tree
 * @param {Color} color
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function peakRegretWithMarketCap(tree, color) {
  return [
    line({
      metric: tree.unrealized.peakRegret,
      name: "Peak Regret",
      color,
      unit: Unit.usd,
    }),
    baseline({
      metric: tree.relative.unrealizedPeakRegretRelToMarketCap,
      name: "Rel. to Market Cap",
      color,
      unit: Unit.pctMcap,
    }),
  ];
}

/**
 * Sentiment series
 * @param {{ unrealized: UnrealizedPattern }} tree
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function sentimentSeries(tree) {
  return [
    baseline({
      metric: tree.unrealized.netSentiment,
      name: "Net Sentiment",
      unit: Unit.usd,
    }),
    line({
      metric: tree.unrealized.greedIndex,
      name: "Greed Index",
      color: colors.profit,
      unit: Unit.usd,
      defaultActive: false,
    }),
    line({
      metric: tree.unrealized.painIndex,
      name: "Pain Index",
      color: colors.loss,
      unit: Unit.usd,
      defaultActive: false,
    }),
  ];
}

/**
 * Sentiment chart for single cohort
 * @param {{ unrealized: UnrealizedPattern }} tree
 * @param {(metric: string) => string} title
 * @returns {PartialChartOption}
 */
function sentimentChart(tree, title) {
  return {
    name: "Sentiment",
    title: title("Market Sentiment"),
    bottom: sentimentSeries(tree),
  };
}

/**
 * Volume subfolder for single cohort
 * @param {{ realized: AnyRealizedPattern }} tree
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsGroup}
 */
function volumeSubfolder(tree, title) {
  return { name: "Volume", tree: sentInPnlTree(tree, title) };
}

// ============================================================================
// Realized P&L Builders
// ============================================================================

/**
 * Realized P&L sum series
 * @param {{ realized: AnyRealizedPattern }} tree
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function realizedPnlSum(tree) {
  const r = tree.realized;
  return [
    line({
      metric: r.realizedProfit7dEma,
      name: "Profit 7d EMA",
      color: colors.profit,
      unit: Unit.usd,
    }),
    line({
      metric: r.realizedLoss7dEma,
      name: "Loss 7d EMA",
      color: colors.loss,
      unit: Unit.usd,
    }),
    dots({
      metric: r.realizedProfit.height,
      name: "Profit",
      color: colors.profit,
      unit: Unit.usd,
      defaultActive: false,
    }),
    dots({
      metric: r.negRealizedLoss,
      name: "Negative Loss",
      color: colors.loss,
      unit: Unit.usd,
      defaultActive: false,
    }),
    dots({
      metric: r.realizedLoss.height,
      name: "Loss",
      color: colors.loss,
      unit: Unit.usd,
      defaultActive: false,
    }),
    dots({
      metric: r.realizedValue,
      name: "Value",
      color: colors.default,
      unit: Unit.usd,
      defaultActive: false,
    }),
    baseline({
      metric: r.realizedProfitRelToRealizedCap,
      name: "Profit",
      color: colors.profit,
      unit: Unit.pctRcap,
    }),
    baseline({
      metric: r.realizedLossRelToRealizedCap,
      name: "Loss",
      color: colors.loss,
      unit: Unit.pctRcap,
    }),
  ];
}

/**
 * Realized Net P&L sum series
 * @param {{ realized: AnyRealizedPattern }} tree
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function realizedNetPnlSum(tree) {
  const r = tree.realized;
  return [
    baseline({
      metric: r.netRealizedPnl7dEma,
      name: "Net 7d EMA",
      unit: Unit.usd,
    }),
    dotsBaseline({
      metric: r.netRealizedPnl.height,
      name: "Net",
      unit: Unit.usd,
      defaultActive: false,
    }),
    baseline({
      metric: r.netRealizedPnlRelToRealizedCap,
      name: "Net",
      unit: Unit.pctRcap,
    }),
  ];
}

/**
 * Realized P&L cumulative series
 * @param {{ realized: AnyRealizedPattern }} tree
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function realizedPnlCumulative(tree) {
  const r = tree.realized;
  return [
    line({
      metric: r.realizedProfit.cumulative,
      name: "Profit",
      color: colors.profit,
      unit: Unit.usd,
    }),
    line({
      metric: r.realizedLoss.cumulative,
      name: "Loss",
      color: colors.loss,
      unit: Unit.usd,
    }),
    line({
      metric: r.negRealizedLoss,
      name: "Negative Loss",
      color: colors.loss,
      unit: Unit.usd,
      defaultActive: false,
    }),
    baseline({
      metric: r.realizedProfitRelToRealizedCap,
      name: "Profit",
      color: colors.profit,
      unit: Unit.pctRcap,
    }),
    baseline({
      metric: r.realizedLossRelToRealizedCap,
      name: "Loss",
      color: colors.loss,
      unit: Unit.pctRcap,
    }),
  ];
}

/**
 * Realized Net P&L cumulative series
 * @param {{ realized: AnyRealizedPattern }} tree
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function realizedNetPnlCumulative(tree) {
  const r = tree.realized;
  return [
    baseline({
      metric: r.netRealizedPnl.cumulative,
      name: "Net",
      unit: Unit.usd,
    }),
    baseline({
      metric: r.netRealizedPnlRelToRealizedCap,
      name: "Net",
      unit: Unit.pctRcap,
    }),
  ];
}

/**
 * Realized 30d change series
 * @param {{ realized: AnyRealizedPattern }} tree
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function realized30dChange(tree) {
  const r = tree.realized;
  return [
    baseline({
      metric: r.netRealizedPnlCumulative30dDelta,
      name: "30d Change",
      unit: Unit.usd,
    }),
    baseline({
      metric: r.netRealizedPnlCumulative30dDeltaRelToMarketCap,
      name: "30d Change",
      unit: Unit.pctMcap,
    }),
    baseline({
      metric: r.netRealizedPnlCumulative30dDeltaRelToRealizedCap,
      name: "30d Change",
      unit: Unit.pctRcap,
    }),
  ];
}

/**
 * Sent in profit/loss tree
 * @param {{ realized: AnyRealizedPattern }} tree
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsTree}
 */
function sentInPnlTree(tree, title) {
  const r = tree.realized;
  return [
    {
      name: "Sum",
      title: title("Sent In Profit & Loss"),
      bottom: [
        ...satsBtcUsd({
          pattern: r.sentInProfit14dEma,
          name: "In Profit 14d EMA",
          color: colors.profit,
          defaultActive: false,
        }),
        ...satsBtcUsd({
          pattern: r.sentInLoss14dEma,
          name: "In Loss 14d EMA",
          color: colors.loss,
          defaultActive: false,
        }),
        ...satsBtcUsdFrom({
          source: r.sentInProfit,
          key: "base",
          name: "In Profit",
          color: colors.profit,
        }),
        ...satsBtcUsdFrom({
          source: r.sentInLoss,
          key: "base",
          name: "In Loss",
          color: colors.loss,
        }),
      ],
    },
    {
      name: "Cumulative",
      title: title("Cumulative Sent In Profit & Loss"),
      bottom: [
        ...satsBtcUsdFrom({
          source: r.sentInProfit,
          key: "cumulative",
          name: "In Profit",
          color: colors.profit,
        }),
        ...satsBtcUsdFrom({
          source: r.sentInLoss,
          key: "cumulative",
          name: "In Loss",
          color: colors.loss,
        }),
      ],
    },
  ];
}

// ============================================================================
// Rolling Realized Helpers
// ============================================================================

/**
 * Rolling realized value tree for single cohort (available on all realized patterns)
 * @param {AnyRealizedPattern} r
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsTree}
 */
function singleRollingRealizedValueTree(r, title) {
  return [
    {
      name: "Compare",
      title: title("Rolling Realized Value"),
      bottom: [
        line({ metric: r.realizedValue24h, name: "24h", color: colors.time._24h, unit: Unit.usd }),
        line({ metric: r.realizedValue7d, name: "7d", color: colors.time._1w, unit: Unit.usd }),
        line({ metric: r.realizedValue30d, name: "30d", color: colors.time._1m, unit: Unit.usd }),
        line({ metric: r.realizedValue1y, name: "1y", color: colors.time._1y, unit: Unit.usd }),
      ],
    },
    { name: "24h", title: title("Realized Value (24h)"), bottom: [line({ metric: r.realizedValue24h, name: "Value", unit: Unit.usd })] },
    { name: "7d", title: title("Realized Value (7d)"), bottom: [line({ metric: r.realizedValue7d, name: "Value", unit: Unit.usd })] },
    { name: "30d", title: title("Realized Value (30d)"), bottom: [line({ metric: r.realizedValue30d, name: "Value", unit: Unit.usd })] },
    { name: "1y", title: title("Realized Value (1y)"), bottom: [line({ metric: r.realizedValue1y, name: "Value", unit: Unit.usd })] },
  ];
}

/**
 * Rolling realized tree with P/L for single cohort (for RealizedWithExtras patterns)
 * @param {RealizedWithExtras} r
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsTree}
 */
function singleRollingRealizedTreeWithExtras(r, title) {
  return [
    {
      name: "Value",
      tree: singleRollingRealizedValueTree(r, title),
    },
    {
      name: "Profit",
      tree: [
        {
          name: "Compare",
          title: title("Rolling Realized Profit"),
          bottom: [
            line({ metric: r.realizedProfit24h, name: "24h", color: colors.time._24h, unit: Unit.usd }),
            line({ metric: r.realizedProfit7d, name: "7d", color: colors.time._1w, unit: Unit.usd }),
            line({ metric: r.realizedProfit30d, name: "30d", color: colors.time._1m, unit: Unit.usd }),
            line({ metric: r.realizedProfit1y, name: "1y", color: colors.time._1y, unit: Unit.usd }),
          ],
        },
        { name: "24h", title: title("Realized Profit (24h)"), bottom: [line({ metric: r.realizedProfit24h, name: "Profit", color: colors.profit, unit: Unit.usd })] },
        { name: "7d", title: title("Realized Profit (7d)"), bottom: [line({ metric: r.realizedProfit7d, name: "Profit", color: colors.profit, unit: Unit.usd })] },
        { name: "30d", title: title("Realized Profit (30d)"), bottom: [line({ metric: r.realizedProfit30d, name: "Profit", color: colors.profit, unit: Unit.usd })] },
        { name: "1y", title: title("Realized Profit (1y)"), bottom: [line({ metric: r.realizedProfit1y, name: "Profit", color: colors.profit, unit: Unit.usd })] },
      ],
    },
    {
      name: "Loss",
      tree: [
        {
          name: "Compare",
          title: title("Rolling Realized Loss"),
          bottom: [
            line({ metric: r.realizedLoss24h, name: "24h", color: colors.time._24h, unit: Unit.usd }),
            line({ metric: r.realizedLoss7d, name: "7d", color: colors.time._1w, unit: Unit.usd }),
            line({ metric: r.realizedLoss30d, name: "30d", color: colors.time._1m, unit: Unit.usd }),
            line({ metric: r.realizedLoss1y, name: "1y", color: colors.time._1y, unit: Unit.usd }),
          ],
        },
        { name: "24h", title: title("Realized Loss (24h)"), bottom: [line({ metric: r.realizedLoss24h, name: "Loss", color: colors.loss, unit: Unit.usd })] },
        { name: "7d", title: title("Realized Loss (7d)"), bottom: [line({ metric: r.realizedLoss7d, name: "Loss", color: colors.loss, unit: Unit.usd })] },
        { name: "30d", title: title("Realized Loss (30d)"), bottom: [line({ metric: r.realizedLoss30d, name: "Loss", color: colors.loss, unit: Unit.usd })] },
        { name: "1y", title: title("Realized Loss (1y)"), bottom: [line({ metric: r.realizedLoss1y, name: "Loss", color: colors.loss, unit: Unit.usd })] },
      ],
    },
    {
      name: "P/L Ratio",
      tree: [
        {
          name: "Compare",
          title: title("Rolling Realized P/L Ratio"),
          bottom: [
            baseline({ metric: r.realizedProfitToLossRatio24h, name: "24h", color: colors.time._24h, unit: Unit.ratio }),
            baseline({ metric: r.realizedProfitToLossRatio7d, name: "7d", color: colors.time._1w, unit: Unit.ratio }),
            baseline({ metric: r.realizedProfitToLossRatio30d, name: "30d", color: colors.time._1m, unit: Unit.ratio }),
            baseline({ metric: r.realizedProfitToLossRatio1y, name: "1y", color: colors.time._1y, unit: Unit.ratio }),
          ],
        },
        { name: "24h", title: title("Realized P/L Ratio (24h)"), bottom: [baseline({ metric: r.realizedProfitToLossRatio24h, name: "P/L Ratio", unit: Unit.ratio })] },
        { name: "7d", title: title("Realized P/L Ratio (7d)"), bottom: [baseline({ metric: r.realizedProfitToLossRatio7d, name: "P/L Ratio", unit: Unit.ratio })] },
        { name: "30d", title: title("Realized P/L Ratio (30d)"), bottom: [baseline({ metric: r.realizedProfitToLossRatio30d, name: "P/L Ratio", unit: Unit.ratio })] },
        { name: "1y", title: title("Realized P/L Ratio (1y)"), bottom: [baseline({ metric: r.realizedProfitToLossRatio1y, name: "P/L Ratio", unit: Unit.ratio })] },
      ],
    },
  ];
}

/**
 * Grouped rolling realized value charts (available on all realized patterns)
 * @param {readonly CohortObject[]} list
 * @param {CohortObject} all
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsTree}
 */
function groupedRollingRealizedValueCharts(list, all, title) {
  return [
    { name: "24h", title: title("Realized Value (24h)"), bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) => line({ metric: tree.realized.realizedValue24h, name, color, unit: Unit.usd })) },
    { name: "7d", title: title("Realized Value (7d)"), bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) => line({ metric: tree.realized.realizedValue7d, name, color, unit: Unit.usd })) },
    { name: "30d", title: title("Realized Value (30d)"), bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) => line({ metric: tree.realized.realizedValue30d, name, color, unit: Unit.usd })) },
    { name: "1y", title: title("Realized Value (1y)"), bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) => line({ metric: tree.realized.realizedValue1y, name, color, unit: Unit.usd })) },
  ];
}

/**
 * Grouped rolling realized charts with P/L (for RealizedWithExtras cohorts)
 * @param {readonly (CohortAgeRange | CohortLongTerm | CohortAll | CohortFull)[]} list
 * @param {CohortAll} all
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsTree}
 */
function groupedRollingRealizedChartsWithExtras(list, all, title) {
  return [
    {
      name: "Value",
      tree: groupedRollingRealizedValueCharts(list, all, title),
    },
    {
      name: "Profit",
      tree: [
        { name: "24h", title: title("Realized Profit (24h)"), bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) => line({ metric: tree.realized.realizedProfit24h, name, color, unit: Unit.usd })) },
        { name: "7d", title: title("Realized Profit (7d)"), bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) => line({ metric: tree.realized.realizedProfit7d, name, color, unit: Unit.usd })) },
        { name: "30d", title: title("Realized Profit (30d)"), bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) => line({ metric: tree.realized.realizedProfit30d, name, color, unit: Unit.usd })) },
        { name: "1y", title: title("Realized Profit (1y)"), bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) => line({ metric: tree.realized.realizedProfit1y, name, color, unit: Unit.usd })) },
      ],
    },
    {
      name: "Loss",
      tree: [
        { name: "24h", title: title("Realized Loss (24h)"), bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) => line({ metric: tree.realized.realizedLoss24h, name, color, unit: Unit.usd })) },
        { name: "7d", title: title("Realized Loss (7d)"), bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) => line({ metric: tree.realized.realizedLoss7d, name, color, unit: Unit.usd })) },
        { name: "30d", title: title("Realized Loss (30d)"), bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) => line({ metric: tree.realized.realizedLoss30d, name, color, unit: Unit.usd })) },
        { name: "1y", title: title("Realized Loss (1y)"), bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) => line({ metric: tree.realized.realizedLoss1y, name, color, unit: Unit.usd })) },
      ],
    },
    {
      name: "P/L Ratio",
      tree: [
        { name: "24h", title: title("Realized P/L Ratio (24h)"), bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) => baseline({ metric: tree.realized.realizedProfitToLossRatio24h, name, color, unit: Unit.ratio })) },
        { name: "7d", title: title("Realized P/L Ratio (7d)"), bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) => baseline({ metric: tree.realized.realizedProfitToLossRatio7d, name, color, unit: Unit.ratio })) },
        { name: "30d", title: title("Realized P/L Ratio (30d)"), bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) => baseline({ metric: tree.realized.realizedProfitToLossRatio30d, name, color, unit: Unit.ratio })) },
        { name: "1y", title: title("Realized P/L Ratio (1y)"), bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) => baseline({ metric: tree.realized.realizedProfitToLossRatio1y, name, color, unit: Unit.ratio })) },
      ],
    },
  ];
}

// ============================================================================
// Realized Subfolder Builders
// ============================================================================

/**
 * Base realized subfolder (no P/L ratio)
 * @param {{ realized: AnyRealizedPattern }} tree
 * @param {(metric: string) => string} title
 * @param {PartialOptionsTree} [rollingTree]
 * @returns {PartialOptionsGroup}
 */
function realizedSubfolder(tree, title, rollingTree) {
  const r = tree.realized;
  return {
    name: "Realized",
    tree: [
      {
        name: "P&L",
        title: title("Realized P&L"),
        bottom: realizedPnlSum(tree),
      },
      {
        name: "Net",
        title: title("Net Realized P&L"),
        bottom: realizedNetPnlSum(tree),
      },
      {
        name: "30d Change",
        title: title("Realized P&L 30d Change"),
        bottom: realized30dChange(tree),
      },
      {
        name: "Total",
        title: title("Total Realized P&L"),
        bottom: [
          line({
            metric: r.totalRealizedPnl,
            name: "Total",
            unit: Unit.usd,
            color: colors.bitcoin,
          }),
        ],
      },
      {
        name: "Peak Regret",
        title: title("Realized Peak Regret"),
        bottom: [
          line({
            metric: r.peakRegret.height,
            name: "Peak Regret",
            unit: Unit.usd,
          }),
        ],
      },
      {
        name: "Rolling",
        tree: rollingTree ?? singleRollingRealizedValueTree(r, title),
      },
      {
        name: "Cumulative",
        tree: [
          {
            name: "P&L",
            title: title("Cumulative Realized P&L"),
            bottom: realizedPnlCumulative(tree),
          },
          {
            name: "Net",
            title: title("Cumulative Net Realized P&L"),
            bottom: realizedNetPnlCumulative(tree),
          },
          {
            name: "Peak Regret",
            title: title("Cumulative Realized Peak Regret"),
            bottom: [
              line({
                metric: r.peakRegret.cumulative,
                name: "Peak Regret",
                unit: Unit.usd,
              }),
              line({
                metric: r.peakRegretRelToRealizedCap,
                name: "Peak Regret",
                unit: Unit.pctRcap,
              }),
            ],
          },
        ],
      },
    ],
  };
}

/**
 * Realized subfolder with P/L ratio and rolling P/L
 * @param {{ realized: RealizedWithExtras }} tree
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsGroup}
 */
function realizedSubfolderWithExtras(tree, title) {
  const r = tree.realized;
  const base = realizedSubfolder(tree, title, singleRollingRealizedTreeWithExtras(r, title));
  // Insert P/L Ratio after Total (index 3)
  base.tree.splice(4, 0, {
    name: "P/L Ratio",
    title: title("Realized Profit/Loss Ratio"),
    bottom: [
      baseline({
        metric: r.realizedProfitToLossRatio1y,
        name: "P/L Ratio",
        unit: Unit.ratio,
      }),
    ],
  });
  return base;
}

// ============================================================================
// Single Cohort Section Builders
// ============================================================================

/**
 * Basic profitability section (USD only unrealized)
 * @param {{ cohort: UtxoCohortObject | CohortWithoutRelative, title: (metric: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createProfitabilitySection({ cohort, title }) {
  const { tree } = cohort;
  const m = getUnrealizedMetrics(tree);
  return {
    name: "Profitability",
    tree: [
      {
        name: "Unrealized",
        tree: [
          {
            name: "P&L",
            title: title("Unrealized P&L"),
            bottom: unrealizedUsd(m),
          },
          {
            name: "Net P&L",
            title: title("Net Unrealized P&L"),
            bottom: netUnrealizedUsd(m.net),
          },
        ],
      },
      realizedSubfolder(tree, title),
      volumeSubfolder(tree, title),
      {
        name: "Invested Capital",
        tree: [
          {
            name: "Absolute",
            title: title("Invested Capital In Profit & Loss"),
            bottom: investedCapitalAbsolute(tree),
          },
        ],
      },
      sentimentChart(tree, title),
    ],
  };
}

/**
 * Section with invested capital % but no unrealized relative (basic cohorts)
 * @param {{ cohort: CohortBasicWithoutMarketCap, title: (metric: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createProfitabilitySectionBasicWithInvestedCapitalPct({
  cohort,
  title,
}) {
  const { tree } = cohort;
  const m = getUnrealizedMetrics(tree);
  return {
    name: "Profitability",
    tree: [
      {
        name: "Unrealized",
        tree: [
          {
            name: "P&L",
            title: title("Unrealized P&L"),
            bottom: unrealizedUsd(m),
          },
          {
            name: "Net P&L",
            title: title("Net Unrealized P&L"),
            bottom: netUnrealizedUsd(m.net),
          },
        ],
      },
      realizedSubfolder(tree, title),
      volumeSubfolder(tree, title),
      {
        name: "Invested Capital",
        title: title("Invested Capital In Profit & Loss"),
        bottom: investedCapitalWithPct(tree),
      },
      sentimentChart(tree, title),
    ],
  };
}

/**
 * Section for ageRange cohorts (Own M.Cap + Own P&L + peak regret)
 * @param {{ cohort: CohortAgeRange, title: (metric: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createProfitabilitySectionWithInvestedCapitalPct({
  cohort,
  title,
}) {
  const { tree, color } = cohort;
  const m = getUnrealizedMetrics(tree);
  return {
    name: "Profitability",
    tree: [
      {
        name: "Unrealized",
        tree: [
          {
            name: "P&L",
            title: title("Unrealized P&L"),
            bottom: unrealizedWithOwnMarketCap(m, tree.relative),
          },
          {
            name: "Net P&L",
            title: title("Net Unrealized P&L"),
            bottom: netUnrealizedWithOwnMarketCap(m.net, tree.relative),
          },
          {
            name: "Peak Regret",
            title: title("Unrealized Peak Regret"),
            bottom: peakRegretAbsolute(tree, color),
          },
        ],
      },
      realizedSubfolderWithExtras(tree, title),
      volumeSubfolder(tree, title),
      {
        name: "Invested Capital",
        title: title("Invested Capital In Profit & Loss"),
        bottom: investedCapitalWithPct(tree),
      },
      sentimentChart(tree, title),
    ],
  };
}

/**
 * Section with NUPL (basic cohorts with market cap)
 * @param {{ cohort: CohortBasicWithMarketCap, title: (metric: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createProfitabilitySectionWithNupl({ cohort, title }) {
  const { tree } = cohort;
  const m = getUnrealizedMetrics(tree);
  return {
    name: "Profitability",
    tree: [
      {
        name: "Unrealized",
        tree: [
          {
            name: "P&L",
            title: title("Unrealized P&L"),
            bottom: unrealizedWithMarketCap(m, tree.relative),
          },
          {
            name: "Net P&L",
            title: title("Net Unrealized P&L"),
            bottom: netUnrealizedWithMarketCap(m.net, tree.relative),
          },
          {
            name: "NUPL",
            title: title("NUPL"),
            bottom: nuplSeries(tree.relative),
          },
        ],
      },
      realizedSubfolder(tree, title),
      volumeSubfolder(tree, title),
      {
        name: "Invested Capital",
        title: title("Invested Capital In Profit & Loss"),
        bottom: investedCapitalWithPct(tree),
      },
      sentimentChart(tree, title),
    ],
  };
}

/**
 * Section for LongTerm cohort
 * @param {{ cohort: CohortLongTerm, title: (metric: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createProfitabilitySectionLongTerm({ cohort, title }) {
  const { tree, color } = cohort;
  const m = getUnrealizedMetrics(tree);
  return {
    name: "Profitability",
    tree: [
      {
        name: "Unrealized",
        tree: [
          {
            name: "P&L",
            title: title("Unrealized P&L"),
            bottom: unrealizedLongTerm(m, tree.relative),
          },
          {
            name: "Net P&L",
            title: title("Net Unrealized P&L"),
            bottom: netUnrealizedWithOwnMarketCap(m.net, tree.relative),
          },
          {
            name: "NUPL",
            title: title("NUPL"),
            bottom: nuplSeries(tree.relative),
          },
          {
            name: "Peak Regret",
            title: title("Unrealized Peak Regret"),
            bottom: peakRegretWithMarketCap(tree, color),
          },
        ],
      },
      realizedSubfolderWithExtras(tree, title),
      volumeSubfolder(tree, title),
      {
        name: "Invested Capital",
        title: title("Invested Capital In Profit & Loss"),
        bottom: investedCapitalWithPct(tree),
      },
      sentimentChart(tree, title),
    ],
  };
}

/**
 * Section for Full cohorts (all relative metrics)
 * @param {{ cohort: CohortFull, title: (metric: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createProfitabilitySectionFull({ cohort, title }) {
  const { tree, color } = cohort;
  const m = getUnrealizedMetrics(tree);
  return {
    name: "Profitability",
    tree: [
      {
        name: "Unrealized",
        tree: [
          {
            name: "P&L",
            title: title("Unrealized P&L"),
            bottom: unrealizedFull(m, tree.relative),
          },
          {
            name: "Net P&L",
            title: title("Net Unrealized P&L"),
            bottom: netUnrealizedFull(m.net, tree.relative),
          },
          {
            name: "NUPL",
            title: title("NUPL"),
            bottom: nuplSeries(tree.relative),
          },
          {
            name: "Peak Regret",
            title: title("Unrealized Peak Regret"),
            bottom: peakRegretWithMarketCap(tree, color),
          },
        ],
      },
      realizedSubfolderWithExtras(tree, title),
      volumeSubfolder(tree, title),
      {
        name: "Invested Capital",
        title: title("Invested Capital In Profit & Loss"),
        bottom: investedCapitalWithPct(tree),
      },
      sentimentChart(tree, title),
    ],
  };
}

/**
 * Section for "all" cohort
 * @param {{ cohort: CohortAll, title: (metric: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createProfitabilitySectionAll({ cohort, title }) {
  const { tree, color } = cohort;
  const m = getUnrealizedMetrics(tree);
  return {
    name: "Profitability",
    tree: [
      {
        name: "Unrealized",
        tree: [
          {
            name: "P&L",
            title: title("Unrealized P&L"),
            bottom: unrealizedAll(m, tree.relative),
          },
          {
            name: "Net P&L",
            title: title("Net Unrealized P&L"),
            bottom: netUnrealizedAll(m.net, tree.relative),
          },
          {
            name: "NUPL",
            title: title("NUPL"),
            bottom: nuplSeries(tree.relative),
          },
          {
            name: "Peak Regret",
            title: title("Unrealized Peak Regret"),
            bottom: peakRegretWithMarketCap(tree, color),
          },
        ],
      },
      realizedSubfolderWithExtras(tree, title),
      volumeSubfolder(tree, title),
      {
        name: "Invested Capital",
        title: title("Invested Capital In Profit & Loss"),
        bottom: investedCapitalWithPct(tree),
      },
      sentimentChart(tree, title),
    ],
  };
}

/**
 * Section with Peak Regret + NUPL (minAge cohorts)
 * @param {{ cohort: CohortMinAge, title: (metric: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createProfitabilitySectionWithPeakRegret({ cohort, title }) {
  const { tree, color } = cohort;
  const m = getUnrealizedMetrics(tree);
  return {
    name: "Profitability",
    tree: [
      {
        name: "Unrealized",
        tree: [
          {
            name: "P&L",
            title: title("Unrealized P&L"),
            bottom: unrealizedWithMarketCap(m, tree.relative),
          },
          {
            name: "Net P&L",
            title: title("Net Unrealized P&L"),
            bottom: netUnrealizedWithMarketCap(m.net, tree.relative),
          },
          {
            name: "NUPL",
            title: title("NUPL"),
            bottom: nuplSeries(tree.relative),
          },
          {
            name: "Peak Regret",
            title: title("Unrealized Peak Regret"),
            bottom: peakRegretWithMarketCap(tree, color),
          },
        ],
      },
      realizedSubfolder(tree, title),
      volumeSubfolder(tree, title),
      {
        name: "Invested Capital",
        title: title("Invested Capital In Profit & Loss"),
        bottom: investedCapitalWithPct(tree),
      },
      sentimentChart(tree, title),
    ],
  };
}

// ============================================================================
// Grouped Cohort Helpers
// ============================================================================

/**
 * Grouped P&L charts (USD only)
 * @param {readonly CohortObject[]} list
 * @param {CohortAll} all
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsTree}
 */
function groupedPnlCharts(list, all, title) {
  return [
    {
      name: "Profit",
      title: title("Unrealized Profit"),
      bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
        line({
          metric: tree.unrealized.unrealizedProfit,
          name,
          color,
          unit: Unit.usd,
        }),
      ),
    },
    {
      name: "Loss",
      title: title("Unrealized Loss"),
      bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
        line({
          metric: tree.unrealized.negUnrealizedLoss,
          name,
          color,
          unit: Unit.usd,
        }),
      ),
    },
    {
      name: "Net P&L",
      title: title("Net Unrealized P&L"),
      bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
        baseline({
          metric: tree.unrealized.netUnrealizedPnl,
          name,
          color,
          unit: Unit.usd,
        }),
      ),
    },
  ];
}

/**
 * Grouped P&L with % of Market Cap
 * @param {readonly (CohortFull | CohortBasicWithMarketCap | CohortMinAge | CohortLongTerm)[]} list
 * @param {CohortAll} all
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsTree}
 */
function groupedPnlChartsWithMarketCap(list, all, title) {
  return [
    {
      name: "Profit",
      title: title("Unrealized Profit"),
      bottom: [
        ...mapCohortsWithAll(list, all, ({ name, color, tree }) =>
          line({
            metric: tree.unrealized.unrealizedProfit,
            name,
            color,
            unit: Unit.usd,
          }),
        ),
        ...mapCohortsWithAll(list, all, ({ name, color, tree }) =>
          baseline({
            metric: tree.relative.unrealizedProfitRelToMarketCap,
            name,
            color,
            unit: Unit.pctMcap,
          }),
        ),
      ],
    },
    {
      name: "Loss",
      title: title("Unrealized Loss"),
      bottom: [
        ...mapCohortsWithAll(list, all, ({ name, color, tree }) =>
          line({
            metric: tree.unrealized.negUnrealizedLoss,
            name,
            color,
            unit: Unit.usd,
          }),
        ),
        ...mapCohortsWithAll(list, all, ({ name, color, tree }) =>
          baseline({
            metric: tree.relative.negUnrealizedLossRelToMarketCap,
            name,
            color,
            unit: Unit.pctMcap,
          }),
        ),
      ],
    },
    {
      name: "Net P&L",
      title: title("Net Unrealized P&L"),
      bottom: [
        ...mapCohortsWithAll(list, all, ({ name, color, tree }) =>
          baseline({
            metric: tree.unrealized.netUnrealizedPnl,
            name,
            color,
            unit: Unit.usd,
          }),
        ),
        ...mapCohortsWithAll(list, all, ({ name, color, tree }) =>
          baseline({
            metric: tree.relative.netUnrealizedPnlRelToMarketCap,
            name,
            color,
            unit: Unit.pctMcap,
          }),
        ),
      ],
    },
  ];
}

/**
 * Grouped P&L with % of Own Market Cap
 * @param {readonly CohortAgeRange[]} list
 * @param {CohortAll} all
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsTree}
 */
function groupedPnlChartsWithOwnMarketCap(list, all, title) {
  return [
    {
      name: "Profit",
      title: title("Unrealized Profit"),
      bottom: [
        ...mapCohortsWithAll(list, all, ({ name, color, tree }) =>
          line({
            metric: tree.unrealized.unrealizedProfit,
            name,
            color,
            unit: Unit.usd,
          }),
        ),
        // OwnMarketCap properties don't exist on CohortAll - use mapCohorts
        ...mapCohorts(list, ({ name, color, tree }) =>
          line({
            metric: tree.relative.unrealizedProfitRelToOwnMarketCap,
            name,
            color,
            unit: Unit.pctOwnMcap,
          }),
        ),
        ...mapCohortsWithAll(list, all, ({ name, color, tree }) =>
          line({
            metric: tree.relative.unrealizedProfitRelToOwnTotalUnrealizedPnl,
            name,
            color,
            unit: Unit.pctOwnPnl,
          }),
        ),
      ],
    },
    {
      name: "Loss",
      title: title("Unrealized Loss"),
      bottom: [
        ...mapCohortsWithAll(list, all, ({ name, color, tree }) =>
          line({
            metric: tree.unrealized.negUnrealizedLoss,
            name,
            color,
            unit: Unit.usd,
          }),
        ),
        // OwnMarketCap properties don't exist on CohortAll - use mapCohorts
        ...mapCohorts(list, ({ name, color, tree }) =>
          line({
            metric: tree.relative.negUnrealizedLossRelToOwnMarketCap,
            name,
            color,
            unit: Unit.pctOwnMcap,
          }),
        ),
        ...mapCohortsWithAll(list, all, ({ name, color, tree }) =>
          line({
            metric: tree.relative.negUnrealizedLossRelToOwnTotalUnrealizedPnl,
            name,
            color,
            unit: Unit.pctOwnPnl,
          }),
        ),
      ],
    },
    {
      name: "Net P&L",
      title: title("Net Unrealized P&L"),
      bottom: [
        ...mapCohortsWithAll(list, all, ({ name, color, tree }) =>
          baseline({
            metric: tree.unrealized.netUnrealizedPnl,
            name,
            color,
            unit: Unit.usd,
          }),
        ),
        // OwnMarketCap properties don't exist on CohortAll - use mapCohorts
        ...mapCohorts(list, ({ name, color, tree }) =>
          baseline({
            metric: tree.relative.netUnrealizedPnlRelToOwnMarketCap,
            name,
            color,
            unit: Unit.pctOwnMcap,
          }),
        ),
        ...mapCohortsWithAll(list, all, ({ name, color, tree }) =>
          baseline({
            metric: tree.relative.netUnrealizedPnlRelToOwnTotalUnrealizedPnl,
            name,
            color,
            unit: Unit.pctOwnPnl,
          }),
        ),
      ],
    },
  ];
}

/**
 * Grouped P&L for LongTerm cohorts
 * @param {readonly CohortLongTerm[]} list
 * @param {CohortAll} all
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsTree}
 */
function groupedPnlChartsLongTerm(list, all, title) {
  return [
    {
      name: "Profit",
      title: title("Unrealized Profit"),
      bottom: [
        ...mapCohortsWithAll(list, all, ({ name, color, tree }) =>
          line({
            metric: tree.unrealized.unrealizedProfit,
            name,
            color,
            unit: Unit.usd,
          }),
        ),
        // OwnMarketCap properties don't exist on CohortAll - use mapCohorts
        ...mapCohorts(list, ({ name, color, tree }) =>
          line({
            metric: tree.relative.unrealizedProfitRelToOwnMarketCap,
            name,
            color,
            unit: Unit.pctOwnMcap,
          }),
        ),
        ...mapCohortsWithAll(list, all, ({ name, color, tree }) =>
          line({
            metric: tree.relative.unrealizedProfitRelToOwnTotalUnrealizedPnl,
            name,
            color,
            unit: Unit.pctOwnPnl,
          }),
        ),
      ],
    },
    {
      name: "Loss",
      title: title("Unrealized Loss"),
      bottom: [
        ...mapCohortsWithAll(list, all, ({ name, color, tree }) =>
          line({
            metric: tree.unrealized.negUnrealizedLoss,
            name,
            color,
            unit: Unit.usd,
          }),
        ),
        ...mapCohortsWithAll(list, all, ({ name, color, tree }) =>
          line({
            metric: tree.relative.unrealizedLossRelToMarketCap,
            name,
            color,
            unit: Unit.pctMcap,
          }),
        ),
        // OwnMarketCap properties don't exist on CohortAll - use mapCohorts
        ...mapCohorts(list, ({ name, color, tree }) =>
          line({
            metric: tree.relative.negUnrealizedLossRelToOwnMarketCap,
            name,
            color,
            unit: Unit.pctOwnMcap,
          }),
        ),
        ...mapCohortsWithAll(list, all, ({ name, color, tree }) =>
          line({
            metric: tree.relative.negUnrealizedLossRelToOwnTotalUnrealizedPnl,
            name,
            color,
            unit: Unit.pctOwnPnl,
          }),
        ),
      ],
    },
    {
      name: "Net P&L",
      title: title("Net Unrealized P&L"),
      bottom: [
        ...mapCohortsWithAll(list, all, ({ name, color, tree }) =>
          baseline({
            metric: tree.unrealized.netUnrealizedPnl,
            name,
            color,
            unit: Unit.usd,
          }),
        ),
        // OwnMarketCap properties don't exist on CohortAll - use mapCohorts
        ...mapCohorts(list, ({ name, color, tree }) =>
          baseline({
            metric: tree.relative.netUnrealizedPnlRelToOwnMarketCap,
            name,
            color,
            unit: Unit.pctOwnMcap,
          }),
        ),
        ...mapCohortsWithAll(list, all, ({ name, color, tree }) =>
          baseline({
            metric: tree.relative.netUnrealizedPnlRelToOwnTotalUnrealizedPnl,
            name,
            color,
            unit: Unit.pctOwnPnl,
          }),
        ),
      ],
    },
  ];
}

/**
 * Grouped invested capital (absolute only)
 * @param {readonly CohortObject[]} list
 * @param {CohortAll} all
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsTree}
 */
function groupedInvestedCapitalAbsolute(list, all, title) {
  return [
    {
      name: "In Profit",
      title: title("Invested Capital In Profit"),
      bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
        line({
          metric: tree.unrealized.investedCapitalInProfit,
          name,
          color,
          unit: Unit.usd,
        }),
      ),
    },
    {
      name: "In Loss",
      title: title("Invested Capital In Loss"),
      bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
        line({
          metric: tree.unrealized.investedCapitalInLoss,
          name,
          color,
          unit: Unit.usd,
        }),
      ),
    },
  ];
}

/**
 * Grouped invested capital with %
 * @param {readonly (CohortBasicWithoutMarketCap | CohortAgeRange | CohortFull | CohortBasicWithMarketCap | CohortLongTerm | CohortMinAge)[]} list
 * @param {CohortAll} all
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsTree}
 */
function groupedInvestedCapital(list, all, title) {
  return [
    {
      name: "In Profit",
      title: title("Invested Capital In Profit"),
      bottom: [
        ...mapCohortsWithAll(list, all, ({ name, color, tree }) =>
          line({
            metric: tree.unrealized.investedCapitalInProfit,
            name,
            color,
            unit: Unit.usd,
          }),
        ),
        ...mapCohortsWithAll(list, all, ({ name, color, tree }) =>
          baseline({
            metric: tree.relative.investedCapitalInProfitPct,
            name,
            color,
            unit: Unit.pctOwnRcap,
          }),
        ),
        ...priceLines({ numbers: [100, 50], unit: Unit.pctOwnRcap }),
      ],
    },
    {
      name: "In Loss",
      title: title("Invested Capital In Loss"),
      bottom: [
        ...mapCohortsWithAll(list, all, ({ name, color, tree }) =>
          line({
            metric: tree.unrealized.investedCapitalInLoss,
            name,
            color,
            unit: Unit.usd,
          }),
        ),
        ...mapCohortsWithAll(list, all, ({ name, color, tree }) =>
          baseline({
            metric: tree.relative.investedCapitalInLossPct,
            name,
            color,
            unit: Unit.pctOwnRcap,
          }),
        ),
        ...priceLines({ numbers: [100, 50], unit: Unit.pctOwnRcap }),
      ],
    },
  ];
}

/**
 * Grouped realized P&L sum
 * @param {readonly CohortObject[]} list
 * @param {CohortAll} all
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsTree}
 */
function groupedRealizedPnlSum(list, all, title) {
  return [
    {
      name: "Profit",
      title: title("Realized Profit"),
      bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
        line({
          metric: tree.realized.realizedProfit.height,
          name,
          color,
          unit: Unit.usd,
        }),
      ),
    },
    {
      name: "Loss",
      title: title("Realized Loss"),
      bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
        line({
          metric: tree.realized.negRealizedLoss,
          name,
          color,
          unit: Unit.usd,
        }),
      ),
    },
    {
      name: "Total",
      title: title("Total Realized P&L"),
      bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
        line({
          metric: tree.realized.totalRealizedPnl,
          name,
          color,
          unit: Unit.usd,
        }),
      ),
    },
    {
      name: "Value",
      title: title("Realized Value"),
      bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
        line({
          metric: tree.realized.realizedValue,
          name,
          color,
          unit: Unit.usd,
        }),
      ),
    },
  ];
}

/**
 * Grouped realized P&L sum with P/L ratio
 * @param {readonly (CohortAgeRange | CohortLongTerm | CohortAll | CohortFull)[]} list
 * @param {CohortAll} all
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsTree}
 */
function groupedRealizedPnlSumWithExtras(list, all, title) {
  return [
    ...groupedRealizedPnlSum(list, all, title),
    {
      name: "P/L Ratio",
      title: title("Realized Profit/Loss Ratio"),
      bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
        baseline({
          metric: tree.realized.realizedProfitToLossRatio1y,
          name,
          color,
          unit: Unit.ratio,
        }),
      ),
    },
  ];
}

/**
 * Grouped realized cumulative
 * @param {readonly CohortObject[]} list
 * @param {CohortAll} all
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsTree}
 */
function groupedRealizedPnlCumulative(list, all, title) {
  return [
    {
      name: "Profit",
      title: title("Cumulative Realized Profit"),
      bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
        line({
          metric: tree.realized.realizedProfit.cumulative,
          name,
          color,
          unit: Unit.usd,
        }),
      ),
    },
    {
      name: "Loss",
      title: title("Cumulative Realized Loss"),
      bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
        line({
          metric: tree.realized.realizedLoss.cumulative,
          name,
          color,
          unit: Unit.usd,
        }),
      ),
    },
  ];
}

/**
 * Grouped sent in P/L
 * @param {readonly CohortObject[]} list
 * @param {CohortAll} all
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsTree}
 */
function groupedSentInPnl(list, all, title) {
  return [
    {
      name: "Sum",
      tree: [
        {
          name: "In Profit",
          title: title("Sent In Profit"),
          bottom: [
            ...flatMapCohortsWithAll(list, all, ({ name, color, tree }) =>
              satsBtcUsd({
                pattern: tree.realized.sentInProfit14dEma,
                name: `${name} 14d EMA`,
                color,
                defaultActive: false,
              }),
            ),
            ...flatMapCohortsWithAll(list, all, ({ name, color, tree }) =>
              satsBtcUsdFrom({
                source: tree.realized.sentInProfit,
                key: "base",
                name,
                color,
              }),
            ),
          ],
        },
        {
          name: "In Loss",
          title: title("Sent In Loss"),
          bottom: [
            ...flatMapCohortsWithAll(list, all, ({ name, color, tree }) =>
              satsBtcUsd({
                pattern: tree.realized.sentInLoss14dEma,
                name: `${name} 14d EMA`,
                color,
                defaultActive: false,
              }),
            ),
            ...flatMapCohortsWithAll(list, all, ({ name, color, tree }) =>
              satsBtcUsdFrom({
                source: tree.realized.sentInLoss,
                key: "base",
                name,
                color,
              }),
            ),
          ],
        },
      ],
    },
    {
      name: "Cumulative",
      tree: [
        {
          name: "In Profit",
          title: title("Cumulative Sent In Profit"),
          bottom: flatMapCohortsWithAll(list, all, ({ name, color, tree }) =>
            satsBtcUsdFrom({
              source: tree.realized.sentInProfit,
              key: "cumulative",
              name,
              color,
            }),
          ),
        },
        {
          name: "In Loss",
          title: title("Cumulative Sent In Loss"),
          bottom: flatMapCohortsWithAll(list, all, ({ name, color, tree }) =>
            satsBtcUsdFrom({
              source: tree.realized.sentInLoss,
              key: "cumulative",
              name,
              color,
            }),
          ),
        },
      ],
    },
  ];
}

/**
 * Grouped sentiment
 * @param {readonly CohortObject[]} list
 * @param {CohortAll} all
 * @param {(metric: string) => string} title
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
          baseline({
            metric: tree.unrealized.netSentiment,
            name,
            color,
            unit: Unit.usd,
          }),
        ),
      },
      {
        name: "Greed",
        title: title("Greed Index"),
        bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
          line({
            metric: tree.unrealized.greedIndex,
            name,
            color,
            unit: Unit.usd,
          }),
        ),
      },
      {
        name: "Pain",
        title: title("Pain Index"),
        bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
          line({
            metric: tree.unrealized.painIndex,
            name,
            color,
            unit: Unit.usd,
          }),
        ),
      },
    ],
  };
}

/**
 * Grouped realized subfolder
 * @param {readonly CohortObject[]} list
 * @param {CohortAll} all
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsGroup}
 */
function groupedRealizedSubfolder(list, all, title) {
  return {
    name: "Realized",
    tree: [
      { name: "P&L", tree: groupedRealizedPnlSum(list, all, title) },
      {
        name: "Net",
        title: title("Net Realized P&L"),
        bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
          baseline({
            metric: tree.realized.netRealizedPnl.height,
            name,
            color,
            unit: Unit.usd,
          }),
        ),
      },
      {
        name: "30d Change",
        title: title("Realized P&L 30d Change"),
        bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
          baseline({
            metric: tree.realized.netRealizedPnlCumulative30dDelta,
            name,
            color,
            unit: Unit.usd,
          }),
        ),
      },
      {
        name: "Rolling",
        tree: groupedRollingRealizedValueCharts(list, all, title),
      },
      {
        name: "Cumulative",
        tree: [
          { name: "P&L", tree: groupedRealizedPnlCumulative(list, all, title) },
          {
            name: "Net",
            title: title("Cumulative Net Realized P&L"),
            bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
              baseline({
                metric: tree.realized.netRealizedPnl.cumulative,
                name,
                color,
                unit: Unit.usd,
              }),
            ),
          },
        ],
      },
    ],
  };
}

/**
 * Grouped realized with extras
 * @param {readonly (CohortAgeRange | CohortLongTerm | CohortAll | CohortFull)[]} list
 * @param {CohortAll} all
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsGroup}
 */
function groupedRealizedSubfolderWithExtras(list, all, title) {
  return {
    name: "Realized",
    tree: [
      { name: "P&L", tree: groupedRealizedPnlSumWithExtras(list, all, title) },
      {
        name: "Net",
        title: title("Net Realized P&L"),
        bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
          baseline({
            metric: tree.realized.netRealizedPnl.height,
            name,
            color,
            unit: Unit.usd,
          }),
        ),
      },
      {
        name: "30d Change",
        title: title("Realized P&L 30d Change"),
        bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
          baseline({
            metric: tree.realized.netRealizedPnlCumulative30dDelta,
            name,
            color,
            unit: Unit.usd,
          }),
        ),
      },
      {
        name: "Rolling",
        tree: groupedRollingRealizedChartsWithExtras(list, all, title),
      },
      {
        name: "Cumulative",
        tree: [
          { name: "P&L", tree: groupedRealizedPnlCumulative(list, all, title) },
          {
            name: "Net",
            title: title("Cumulative Net Realized P&L"),
            bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
              baseline({
                metric: tree.realized.netRealizedPnl.cumulative,
                name,
                color,
                unit: Unit.usd,
              }),
            ),
          },
        ],
      },
    ],
  };
}

// ============================================================================
// Grouped Section Builders
// ============================================================================

/**
 * Grouped profitability section (basic)
 * @param {{ list: readonly (UtxoCohortObject | CohortWithoutRelative)[], all: CohortAll, title: (metric: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createGroupedProfitabilitySection({ list, all, title }) {
  return {
    name: "Profitability",
    tree: [
      { name: "Unrealized", tree: groupedPnlCharts(list, all, title) },
      groupedRealizedSubfolder(list, all, title),
      { name: "Volume", tree: groupedSentInPnl(list, all, title) },
      {
        name: "Invested Capital",
        tree: groupedInvestedCapitalAbsolute(list, all, title),
      },
      groupedSentiment(list, all, title),
    ],
  };
}

/**
 * Grouped section with invested capital % (basic cohorts)
 * @param {{ list: readonly CohortBasicWithoutMarketCap[], all: CohortAll, title: (metric: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createGroupedProfitabilitySectionBasicWithInvestedCapitalPct({
  list,
  all,
  title,
}) {
  return {
    name: "Profitability",
    tree: [
      { name: "Unrealized", tree: groupedPnlCharts(list, all, title) },
      groupedRealizedSubfolder(list, all, title),
      { name: "Volume", tree: groupedSentInPnl(list, all, title) },
      {
        name: "Invested Capital",
        tree: groupedInvestedCapital(list, all, title),
      },
      groupedSentiment(list, all, title),
    ],
  };
}

/**
 * Grouped section for ageRange cohorts
 * @param {{ list: readonly CohortAgeRange[], all: CohortAll, title: (metric: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createGroupedProfitabilitySectionWithInvestedCapitalPct({
  list,
  all,
  title,
}) {
  return {
    name: "Profitability",
    tree: [
      {
        name: "Unrealized",
        tree: [
          ...groupedPnlChartsWithOwnMarketCap(list, all, title),
          {
            name: "Peak Regret",
            title: title("Unrealized Peak Regret"),
            bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
              line({
                metric: tree.unrealized.peakRegret,
                name,
                color,
                unit: Unit.usd,
              }),
            ),
          },
        ],
      },
      groupedRealizedSubfolderWithExtras(list, all, title),
      { name: "Volume", tree: groupedSentInPnl(list, all, title) },
      {
        name: "Invested Capital",
        tree: groupedInvestedCapital(list, all, title),
      },
      groupedSentiment(list, all, title),
    ],
  };
}

/**
 * Grouped section with NUPL
 * @param {{ list: readonly (CohortFull | CohortBasicWithMarketCap)[], all: CohortAll, title: (metric: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createGroupedProfitabilitySectionWithNupl({
  list,
  all,
  title,
}) {
  return {
    name: "Profitability",
    tree: [
      {
        name: "Unrealized",
        tree: [
          ...groupedPnlChartsWithMarketCap(list, all, title),
          {
            name: "NUPL",
            title: title("NUPL"),
            bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
              baseline({
                metric: tree.relative.nupl,
                name,
                color,
                unit: Unit.ratio,
              }),
            ),
          },
        ],
      },
      groupedRealizedSubfolder(list, all, title),
      { name: "Volume", tree: groupedSentInPnl(list, all, title) },
      {
        name: "Invested Capital",
        tree: groupedInvestedCapital(list, all, title),
      },
      groupedSentiment(list, all, title),
    ],
  };
}

/**
 * Grouped section for LongTerm cohorts
 * @param {{ list: readonly CohortLongTerm[], all: CohortAll, title: (metric: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createGroupedProfitabilitySectionLongTerm({
  list,
  all,
  title,
}) {
  return {
    name: "Profitability",
    tree: [
      {
        name: "Unrealized",
        tree: [
          ...groupedPnlChartsLongTerm(list, all, title),
          {
            name: "NUPL",
            title: title("NUPL"),
            bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
              baseline({
                metric: tree.relative.nupl,
                name,
                color,
                unit: Unit.ratio,
              }),
            ),
          },
          {
            name: "Peak Regret",
            title: title("Unrealized Peak Regret"),
            bottom: [
              ...mapCohortsWithAll(list, all, ({ name, color, tree }) =>
                line({
                  metric: tree.unrealized.peakRegret,
                  name,
                  color,
                  unit: Unit.usd,
                }),
              ),
              ...mapCohortsWithAll(list, all, ({ name, color, tree }) =>
                baseline({
                  metric: tree.relative.unrealizedPeakRegretRelToMarketCap,
                  name,
                  color,
                  unit: Unit.pctMcap,
                }),
              ),
            ],
          },
        ],
      },
      groupedRealizedSubfolderWithExtras(list, all, title),
      { name: "Volume", tree: groupedSentInPnl(list, all, title) },
      {
        name: "Invested Capital",
        tree: groupedInvestedCapital(list, all, title),
      },
      groupedSentiment(list, all, title),
    ],
  };
}

/**
 * Grouped section with Peak Regret + NUPL (minAge cohorts)
 * @param {{ list: readonly CohortMinAge[], all: CohortAll, title: (metric: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createGroupedProfitabilitySectionWithPeakRegret({
  list,
  all,
  title,
}) {
  return {
    name: "Profitability",
    tree: [
      {
        name: "Unrealized",
        tree: [
          ...groupedPnlChartsWithMarketCap(list, all, title),
          {
            name: "NUPL",
            title: title("NUPL"),
            bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
              baseline({
                metric: tree.relative.nupl,
                name,
                color,
                unit: Unit.ratio,
              }),
            ),
          },
          {
            name: "Peak Regret",
            title: title("Unrealized Peak Regret"),
            bottom: [
              ...mapCohortsWithAll(list, all, ({ name, color, tree }) =>
                line({
                  metric: tree.unrealized.peakRegret,
                  name,
                  color,
                  unit: Unit.usd,
                }),
              ),
              ...mapCohortsWithAll(list, all, ({ name, color, tree }) =>
                baseline({
                  metric: tree.relative.unrealizedPeakRegretRelToMarketCap,
                  name,
                  color,
                  unit: Unit.pctMcap,
                }),
              ),
            ],
          },
        ],
      },
      groupedRealizedSubfolder(list, all, title),
      { name: "Volume", tree: groupedSentInPnl(list, all, title) },
      {
        name: "Invested Capital",
        tree: groupedInvestedCapital(list, all, title),
      },
      groupedSentiment(list, all, title),
    ],
  };
}
