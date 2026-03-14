/**
 * Profitability section builders
 *
 * Capability tiers:
 * - Full (All/STH/LTH): full unrealized with rel metrics, invested capital, sentiment;
 *   full realized with relToRcap, peakRegret, profitToLossRatio, grossPnl
 * - Mid (AgeRange/MaxAge): unrealized profit/loss/netPnl/nupl (no rel, no invested, no sentiment);
 *   realized with netPnl + delta (no relToRcap, no peakRegret)
 * - Basic (UtxoAmount, Empty, Address): nupl only unrealized;
 *   basic realized profit/loss (no netPnl, no relToRcap)
 */

import { Unit } from "../../utils/units.js";
import { line, baseline, dots, dotsBaseline } from "../series.js";
import { colors } from "../../utils/colors.js";
import { priceLine, priceLines } from "../constants.js";
import {
  mapCohorts,
  mapCohortsWithAll,
} from "../shared.js";

// ============================================================================
// Core Series Builders
// ============================================================================

/**
 * @typedef {Object} PnlSeriesConfig
 * @property {AnyMetricPattern} profit
 * @property {AnyMetricPattern} loss
 * @property {AnyMetricPattern} negLoss
 * @property {AnyMetricPattern} [gross]
 */

/**
 * @param {PnlSeriesConfig} m
 * @param {Unit} unit
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function pnlLines(m, unit) {
  const series = [
    line({ metric: m.profit, name: "Profit", color: colors.profit, unit }),
    line({ metric: m.loss, name: "Loss", color: colors.loss, unit }),
  ];
  if (m.gross) {
    series.push(line({ metric: m.gross, name: "Total", color: colors.default, unit }));
  }
  series.push(line({ metric: m.negLoss, name: "Negative Loss", color: colors.loss, unit, defaultActive: false }));
  return series;
}

/**
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
 * Unrealized P&L (USD + relToMcap) for All cohort
 * @param {Brk.MetricsTree_Cohorts_Utxo_All_Unrealized} u
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function unrealizedAll(u) {
  return [
    ...pnlLines(
      { profit: u.profit.base.usd, loss: u.loss.base.usd, negLoss: u.loss.negative, gross: u.grossPnl.usd },
      Unit.usd,
    ),
    priceLine({ unit: Unit.usd, defaultActive: false }),
    line({ metric: u.profit.relToMcap.ratio, name: "Profit", color: colors.profit, unit: Unit.pctMcap }),
    line({ metric: u.loss.relToMcap.ratio, name: "Loss", color: colors.loss, unit: Unit.pctMcap }),
    priceLine({ unit: Unit.pctMcap, defaultActive: false }),
    line({ metric: u.profit.relToOwnGross.ratio, name: "Profit", color: colors.profit, unit: Unit.pctOwnPnl }),
    line({ metric: u.loss.relToOwnGross.ratio, name: "Loss", color: colors.loss, unit: Unit.pctOwnPnl }),
    ...priceLines({ numbers: [100, 50, 0], unit: Unit.pctOwnPnl }),
  ];
}

/**
 * Unrealized P&L (USD + relToMcap + relToOwnMcap + relToOwnGross) for Full cohorts (STH/LTH)
 * @param {Brk.GrossInvestedLossNetNuplProfitSentimentPattern2} u
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function unrealizedFull(u) {
  return [
    ...pnlLines(
      { profit: u.profit.base.usd, loss: u.loss.base.usd, negLoss: u.loss.negative, gross: u.grossPnl.usd },
      Unit.usd,
    ),
    priceLine({ unit: Unit.usd, defaultActive: false }),
    line({ metric: u.profit.relToMcap.ratio, name: "Profit", color: colors.profit, unit: Unit.pctMcap }),
    line({ metric: u.loss.relToMcap.ratio, name: "Loss", color: colors.loss, unit: Unit.pctMcap }),
    priceLine({ unit: Unit.pctMcap, defaultActive: false }),
    line({ metric: u.profit.relToOwnMcap.ratio, name: "Profit", color: colors.profit, unit: Unit.pctOwnMcap }),
    line({ metric: u.loss.relToOwnMcap.ratio, name: "Loss", color: colors.loss, unit: Unit.pctOwnMcap }),
    priceLine({ unit: Unit.pctOwnMcap, defaultActive: false }),
    line({ metric: u.profit.relToOwnGross.ratio, name: "Profit", color: colors.profit, unit: Unit.pctOwnPnl }),
    line({ metric: u.loss.relToOwnGross.ratio, name: "Loss", color: colors.loss, unit: Unit.pctOwnPnl }),
    ...priceLines({ numbers: [100, 50, 0], unit: Unit.pctOwnPnl }),
  ];
}

/**
 * Unrealized P&L for LTH (loss relToMcap only + relToOwnMcap + relToOwnGross)
 * @param {Brk.GrossInvestedLossNetNuplProfitSentimentPattern2} u
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function unrealizedLongTerm(u) {
  return [
    ...pnlLines(
      { profit: u.profit.base.usd, loss: u.loss.base.usd, negLoss: u.loss.negative, gross: u.grossPnl.usd },
      Unit.usd,
    ),
    priceLine({ unit: Unit.usd, defaultActive: false }),
    line({ metric: u.loss.relToMcap.ratio, name: "Loss", color: colors.loss, unit: Unit.pctMcap }),
    line({ metric: u.profit.relToOwnMcap.ratio, name: "Profit", color: colors.profit, unit: Unit.pctOwnMcap }),
    line({ metric: u.loss.relToOwnMcap.ratio, name: "Loss", color: colors.loss, unit: Unit.pctOwnMcap }),
    priceLine({ unit: Unit.pctOwnMcap, defaultActive: false }),
    line({ metric: u.profit.relToOwnGross.ratio, name: "Profit", color: colors.profit, unit: Unit.pctOwnPnl }),
    line({ metric: u.loss.relToOwnGross.ratio, name: "Loss", color: colors.loss, unit: Unit.pctOwnPnl }),
    ...priceLines({ numbers: [100, 50, 0], unit: Unit.pctOwnPnl }),
  ];
}

/**
 * Unrealized P&L (USD only) for mid-tier cohorts (AgeRange/MaxAge)
 * @param {Brk.LossNetNuplProfitPattern} u
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function unrealizedMid(u) {
  return [
    ...pnlLines(
      { profit: u.profit.base.usd, loss: u.loss.base.usd, negLoss: u.loss.negative },
      Unit.usd,
    ),
    priceLine({ unit: Unit.usd, defaultActive: false }),
  ];
}

// ============================================================================
// Net Unrealized P&L Builders
// ============================================================================

/**
 * Net P&L for All cohort
 * @param {Brk.MetricsTree_Cohorts_Utxo_All_Unrealized} u
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function netUnrealizedAll(u) {
  return [
    netBaseline(u.netPnl.usd, Unit.usd),
    netBaseline(u.netPnl.relToOwnGross.ratio, Unit.pctOwnPnl),
  ];
}

/**
 * Net P&L for Full cohorts (STH/LTH)
 * @param {Brk.GrossInvestedLossNetNuplProfitSentimentPattern2} u
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function netUnrealizedFull(u) {
  return [
    netBaseline(u.netPnl.usd, Unit.usd),
    netBaseline(u.netPnl.relToOwnMcap.ratio, Unit.pctOwnMcap),
    netBaseline(u.netPnl.relToOwnGross.ratio, Unit.pctOwnPnl),
  ];
}

/**
 * Net P&L for mid-tier cohorts
 * @param {Brk.LossNetNuplProfitPattern} u
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function netUnrealizedMid(u) {
  return [netBaseline(u.netPnl.usd, Unit.usd)];
}

// ============================================================================
// Invested Capital, Sentiment, NUPL
// ============================================================================

/**
 * Invested capital (Full unrealized only)
 * @param {Brk.GrossInvestedLossNetNuplProfitSentimentPattern2 | Brk.MetricsTree_Cohorts_Utxo_All_Unrealized} u
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function investedCapitalSeries(u) {
  return [
    line({ metric: u.investedCapital.inProfit.usd, name: "In Profit", color: colors.profit, unit: Unit.usd }),
    line({ metric: u.investedCapital.inLoss.usd, name: "In Loss", color: colors.loss, unit: Unit.usd }),
  ];
}

/**
 * Sentiment (Full unrealized only)
 * @param {Brk.GrossInvestedLossNetNuplProfitSentimentPattern2 | Brk.MetricsTree_Cohorts_Utxo_All_Unrealized} u
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function sentimentSeries(u) {
  return [
    baseline({ metric: u.sentiment.net.usd, name: "Net Sentiment", unit: Unit.usd }),
    line({ metric: u.sentiment.greedIndex.usd, name: "Greed Index", color: colors.profit, unit: Unit.usd, defaultActive: false }),
    line({ metric: u.sentiment.painIndex.usd, name: "Pain Index", color: colors.loss, unit: Unit.usd, defaultActive: false }),
  ];
}

/**
 * NUPL series
 * @param {Brk.BpsRatioPattern} nupl
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function nuplSeries(nupl) {
  return [baseline({ metric: nupl.ratio, name: "NUPL", unit: Unit.ratio })];
}

// ============================================================================
// Realized P&L Builders — Full (All/STH/LTH)
// ============================================================================

/**
 * @param {Brk.CapGrossInvestorLossMvrvNetPeakPriceProfitSellSoprPattern | Brk.MetricsTree_Cohorts_Utxo_Lth_Realized} r
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function realizedPnlSumFull(r) {
  return [
    dots({ metric: r.profit.base.usd, name: "Profit", color: colors.profit, unit: Unit.usd }),
    dots({ metric: r.loss.negative, name: "Negative Loss", color: colors.loss, unit: Unit.usd, defaultActive: false }),
    dots({ metric: r.loss.base.usd, name: "Loss", color: colors.loss, unit: Unit.usd, defaultActive: false }),
    baseline({ metric: r.profit.relToRcap.ratio, name: "Profit", color: colors.profit, unit: Unit.pctRcap }),
    baseline({ metric: r.loss.relToRcap.ratio, name: "Loss", color: colors.loss, unit: Unit.pctRcap }),
  ];
}

/**
 * @param {Brk.CapGrossInvestorLossMvrvNetPeakPriceProfitSellSoprPattern | Brk.MetricsTree_Cohorts_Utxo_Lth_Realized} r
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function realizedNetPnlSumFull(r) {
  return [
    dotsBaseline({ metric: r.netPnl.base.usd, name: "Net", unit: Unit.usd, defaultActive: false }),
    baseline({ metric: r.netPnl.relToRcap.ratio, name: "Net", unit: Unit.pctRcap }),
  ];
}

/**
 * @param {Brk.CapGrossInvestorLossMvrvNetPeakPriceProfitSellSoprPattern | Brk.MetricsTree_Cohorts_Utxo_Lth_Realized} r
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function realizedPnlCumulativeFull(r) {
  return [
    line({ metric: r.profit.cumulative.usd, name: "Profit", color: colors.profit, unit: Unit.usd }),
    line({ metric: r.loss.cumulative.usd, name: "Loss", color: colors.loss, unit: Unit.usd }),
    line({ metric: r.loss.negative, name: "Negative Loss", color: colors.loss, unit: Unit.usd, defaultActive: false }),
    baseline({ metric: r.profit.relToRcap.ratio, name: "Profit", color: colors.profit, unit: Unit.pctRcap }),
    baseline({ metric: r.loss.relToRcap.ratio, name: "Loss", color: colors.loss, unit: Unit.pctRcap }),
  ];
}

/**
 * @param {Brk.CapGrossInvestorLossMvrvNetPeakPriceProfitSellSoprPattern | Brk.MetricsTree_Cohorts_Utxo_Lth_Realized} r
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function realized30dChangeFull(r) {
  return [
    baseline({ metric: r.netPnl.delta.change._1m.usd, name: "30d Change", unit: Unit.usd }),
    baseline({ metric: r.netPnl.change1m.relToMcap.ratio, name: "30d Change", unit: Unit.pctMcap }),
    baseline({ metric: r.netPnl.change1m.relToRcap.ratio, name: "30d Change", unit: Unit.pctRcap }),
  ];
}

/**
 * Rolling realized with P/L and ratio (full realized only)
 * @param {Brk.CapGrossInvestorLossMvrvNetPeakPriceProfitSellSoprPattern | Brk.MetricsTree_Cohorts_Utxo_Lth_Realized} r
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsTree}
 */
function singleRollingRealizedTreeFull(r, title) {
  return [
    {
      name: "Profit",
      tree: [
        {
          name: "Compare",
          title: title("Rolling Realized Profit"),
          bottom: [
            line({ metric: r.profit.sum._24h.usd, name: "24h", color: colors.time._24h, unit: Unit.usd }),
            line({ metric: r.profit.sum._1w.usd, name: "7d", color: colors.time._1w, unit: Unit.usd }),
            line({ metric: r.profit.sum._1m.usd, name: "30d", color: colors.time._1m, unit: Unit.usd }),
            line({ metric: r.profit.sum._1y.usd, name: "1y", color: colors.time._1y, unit: Unit.usd }),
          ],
        },
        { name: "24h", title: title("Realized Profit (24h)"), bottom: [line({ metric: r.profit.sum._24h.usd, name: "Profit", color: colors.profit, unit: Unit.usd })] },
        { name: "7d", title: title("Realized Profit (7d)"), bottom: [line({ metric: r.profit.sum._1w.usd, name: "Profit", color: colors.profit, unit: Unit.usd })] },
        { name: "30d", title: title("Realized Profit (30d)"), bottom: [line({ metric: r.profit.sum._1m.usd, name: "Profit", color: colors.profit, unit: Unit.usd })] },
        { name: "1y", title: title("Realized Profit (1y)"), bottom: [line({ metric: r.profit.sum._1y.usd, name: "Profit", color: colors.profit, unit: Unit.usd })] },
      ],
    },
    {
      name: "Loss",
      tree: [
        {
          name: "Compare",
          title: title("Rolling Realized Loss"),
          bottom: [
            line({ metric: r.loss.sum._24h.usd, name: "24h", color: colors.time._24h, unit: Unit.usd }),
            line({ metric: r.loss.sum._1w.usd, name: "7d", color: colors.time._1w, unit: Unit.usd }),
            line({ metric: r.loss.sum._1m.usd, name: "30d", color: colors.time._1m, unit: Unit.usd }),
            line({ metric: r.loss.sum._1y.usd, name: "1y", color: colors.time._1y, unit: Unit.usd }),
          ],
        },
        { name: "24h", title: title("Realized Loss (24h)"), bottom: [line({ metric: r.loss.sum._24h.usd, name: "Loss", color: colors.loss, unit: Unit.usd })] },
        { name: "7d", title: title("Realized Loss (7d)"), bottom: [line({ metric: r.loss.sum._1w.usd, name: "Loss", color: colors.loss, unit: Unit.usd })] },
        { name: "30d", title: title("Realized Loss (30d)"), bottom: [line({ metric: r.loss.sum._1m.usd, name: "Loss", color: colors.loss, unit: Unit.usd })] },
        { name: "1y", title: title("Realized Loss (1y)"), bottom: [line({ metric: r.loss.sum._1y.usd, name: "Loss", color: colors.loss, unit: Unit.usd })] },
      ],
    },
    {
      name: "P/L Ratio",
      tree: [
        {
          name: "Compare",
          title: title("Rolling Realized P/L Ratio"),
          bottom: [
            baseline({ metric: r.profitToLossRatio._24h, name: "24h", color: colors.time._24h, unit: Unit.ratio }),
            baseline({ metric: r.profitToLossRatio._1w, name: "7d", color: colors.time._1w, unit: Unit.ratio }),
            baseline({ metric: r.profitToLossRatio._1m, name: "30d", color: colors.time._1m, unit: Unit.ratio }),
            baseline({ metric: r.profitToLossRatio._1y, name: "1y", color: colors.time._1y, unit: Unit.ratio }),
          ],
        },
        { name: "24h", title: title("Realized P/L Ratio (24h)"), bottom: [baseline({ metric: r.profitToLossRatio._24h, name: "P/L Ratio", unit: Unit.ratio })] },
        { name: "7d", title: title("Realized P/L Ratio (7d)"), bottom: [baseline({ metric: r.profitToLossRatio._1w, name: "P/L Ratio", unit: Unit.ratio })] },
        { name: "30d", title: title("Realized P/L Ratio (30d)"), bottom: [baseline({ metric: r.profitToLossRatio._1m, name: "P/L Ratio", unit: Unit.ratio })] },
        { name: "1y", title: title("Realized P/L Ratio (1y)"), bottom: [baseline({ metric: r.profitToLossRatio._1y, name: "P/L Ratio", unit: Unit.ratio })] },
      ],
    },
  ];
}

/**
 * Rolling realized profit/loss sums (basic — no P/L ratio)
 * @param {Brk.BaseCumulativeSumPattern3} profit
 * @param {Brk.BaseCumulativeSumPattern3} loss
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsTree}
 */
function singleRollingRealizedTreeBasic(profit, loss, title) {
  return [
    {
      name: "Profit",
      tree: [
        { name: "24h", title: title("Realized Profit (24h)"), bottom: [line({ metric: profit.sum._24h.usd, name: "Profit", color: colors.profit, unit: Unit.usd })] },
        { name: "7d", title: title("Realized Profit (7d)"), bottom: [line({ metric: profit.sum._1w.usd, name: "Profit", color: colors.profit, unit: Unit.usd })] },
        { name: "30d", title: title("Realized Profit (30d)"), bottom: [line({ metric: profit.sum._1m.usd, name: "Profit", color: colors.profit, unit: Unit.usd })] },
        { name: "1y", title: title("Realized Profit (1y)"), bottom: [line({ metric: profit.sum._1y.usd, name: "Profit", color: colors.profit, unit: Unit.usd })] },
      ],
    },
    {
      name: "Loss",
      tree: [
        { name: "24h", title: title("Realized Loss (24h)"), bottom: [line({ metric: loss.sum._24h.usd, name: "Loss", color: colors.loss, unit: Unit.usd })] },
        { name: "7d", title: title("Realized Loss (7d)"), bottom: [line({ metric: loss.sum._1w.usd, name: "Loss", color: colors.loss, unit: Unit.usd })] },
        { name: "30d", title: title("Realized Loss (30d)"), bottom: [line({ metric: loss.sum._1m.usd, name: "Loss", color: colors.loss, unit: Unit.usd })] },
        { name: "1y", title: title("Realized Loss (1y)"), bottom: [line({ metric: loss.sum._1y.usd, name: "Loss", color: colors.loss, unit: Unit.usd })] },
      ],
    },
  ];
}

// ============================================================================
// Realized Subfolder Builders
// ============================================================================

/**
 * Full realized subfolder (All/STH/LTH)
 * @param {Brk.CapGrossInvestorLossMvrvNetPeakPriceProfitSellSoprPattern | Brk.MetricsTree_Cohorts_Utxo_Lth_Realized} r
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsGroup}
 */
function realizedSubfolderFull(r, title) {
  return {
    name: "Realized",
    tree: [
      { name: "P&L", title: title("Realized P&L"), bottom: realizedPnlSumFull(r) },
      { name: "Net", title: title("Net Realized P&L"), bottom: realizedNetPnlSumFull(r) },
      { name: "30d Change", title: title("Realized P&L 30d Change"), bottom: realized30dChangeFull(r) },
      {
        name: "Total",
        title: title("Total Realized P&L"),
        bottom: [line({ metric: r.grossPnl.cumulative.usd, name: "Total", unit: Unit.usd, color: colors.bitcoin })],
      },
      {
        name: "P/L Ratio",
        title: title("Realized Profit/Loss Ratio"),
        bottom: [baseline({ metric: r.profitToLossRatio._1y, name: "P/L Ratio", unit: Unit.ratio })],
      },
      {
        name: "Peak Regret",
        title: title("Realized Peak Regret"),
        bottom: [
          line({ metric: r.peakRegret.base, name: "Peak Regret", unit: Unit.usd }),
        ],
      },
      { name: "Rolling", tree: singleRollingRealizedTreeFull(r, title) },
      {
        name: "Cumulative",
        tree: [
          { name: "P&L", title: title("Cumulative Realized P&L"), bottom: realizedPnlCumulativeFull(r) },
          {
            name: "Net",
            title: title("Cumulative Net Realized P&L"),
            bottom: [
              baseline({ metric: r.netPnl.cumulative.usd, name: "Net", unit: Unit.usd }),
              baseline({ metric: r.netPnl.relToRcap.ratio, name: "Net", unit: Unit.pctRcap }),
            ],
          },
          {
            name: "Peak Regret",
            title: title("Cumulative Realized Peak Regret"),
            bottom: [
              line({ metric: r.peakRegret.cumulative, name: "Peak Regret", unit: Unit.usd }),
              line({ metric: r.peakRegret.relToRcap.ratio, name: "Peak Regret", unit: Unit.pctRcap }),
            ],
          },
        ],
      },
    ],
  };
}

/**
 * Mid realized subfolder (AgeRange/MaxAge — has netPnl + delta, no relToRcap/peakRegret)
 * @param {Brk.CapLossMvrvNetPriceProfitSoprPattern} r
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsGroup}
 */
function realizedSubfolderMid(r, title) {
  return {
    name: "Realized",
    tree: [
      {
        name: "P&L",
        title: title("Realized P&L"),
        bottom: [
          dots({ metric: r.profit.base.usd, name: "Profit", color: colors.profit, unit: Unit.usd }),
          dots({ metric: r.loss.negative, name: "Negative Loss", color: colors.loss, unit: Unit.usd, defaultActive: false }),
          dots({ metric: r.loss.base.usd, name: "Loss", color: colors.loss, unit: Unit.usd, defaultActive: false }),
        ],
      },
      {
        name: "Net",
        title: title("Net Realized P&L"),
        bottom: [dotsBaseline({ metric: r.netPnl.base.usd, name: "Net", unit: Unit.usd })],
      },
      {
        name: "30d Change",
        title: title("Realized P&L 30d Change"),
        bottom: [baseline({ metric: r.netPnl.delta.change._1m.usd, name: "30d Change", unit: Unit.usd })],
      },
      { name: "Rolling", tree: singleRollingRealizedTreeBasic(r.profit, r.loss, title) },
      {
        name: "Cumulative",
        tree: [
          {
            name: "P&L",
            title: title("Cumulative Realized P&L"),
            bottom: [
              line({ metric: r.profit.cumulative.usd, name: "Profit", color: colors.profit, unit: Unit.usd }),
              line({ metric: r.loss.cumulative.usd, name: "Loss", color: colors.loss, unit: Unit.usd }),
            ],
          },
          {
            name: "Net",
            title: title("Cumulative Net Realized P&L"),
            bottom: [baseline({ metric: r.netPnl.cumulative.usd, name: "Net", unit: Unit.usd })],
          },
        ],
      },
    ],
  };
}

/**
 * Basic realized subfolder (no netPnl, no relToRcap)
 * @param {Brk.CapLossMvrvPriceProfitSoprPattern} r
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsGroup}
 */
function realizedSubfolderBasic(r, title) {
  return {
    name: "Realized",
    tree: [
      {
        name: "P&L",
        title: title("Realized P&L"),
        bottom: [
          dots({ metric: r.profit.base.usd, name: "Profit", color: colors.profit, unit: Unit.usd }),
          dots({ metric: r.loss.base.usd, name: "Loss", color: colors.loss, unit: Unit.usd, defaultActive: false }),
        ],
      },
      { name: "Rolling", tree: singleRollingRealizedTreeBasic(r.profit, r.loss, title) },
      {
        name: "Cumulative",
        title: title("Cumulative Realized P&L"),
        bottom: [
          line({ metric: r.profit.cumulative.usd, name: "Profit", color: colors.profit, unit: Unit.usd }),
          line({ metric: r.loss.cumulative.usd, name: "Loss", color: colors.loss, unit: Unit.usd }),
        ],
      },
    ],
  };
}

// ============================================================================
// Single Cohort Section Builders
// ============================================================================

/**
 * Basic profitability section (NUPL only unrealized, basic realized)
 * @param {{ cohort: UtxoCohortObject | CohortWithoutRelative, title: (metric: string) => string }} args
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
 * Section for All cohort
 * @param {{ cohort: CohortAll, title: (metric: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createProfitabilitySectionAll({ cohort, title }) {
  const u = cohort.tree.unrealized;
  const r = cohort.tree.realized;
  return {
    name: "Profitability",
    tree: [
      {
        name: "Unrealized",
        tree: [
          { name: "P&L", title: title("Unrealized P&L"), bottom: unrealizedAll(u) },
          { name: "Net P&L", title: title("Net Unrealized P&L"), bottom: netUnrealizedAll(u) },
          { name: "NUPL", title: title("NUPL"), bottom: nuplSeries(u.nupl) },
        ],
      },
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
 * @param {{ cohort: CohortFull, title: (metric: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createProfitabilitySectionFull({ cohort, title }) {
  const u = cohort.tree.unrealized;
  const r = cohort.tree.realized;
  return {
    name: "Profitability",
    tree: [
      {
        name: "Unrealized",
        tree: [
          { name: "P&L", title: title("Unrealized P&L"), bottom: unrealizedFull(u) },
          { name: "Net P&L", title: title("Net Unrealized P&L"), bottom: netUnrealizedFull(u) },
          { name: "NUPL", title: title("NUPL"), bottom: nuplSeries(u.nupl) },
        ],
      },
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
 * @param {{ cohort: CohortBasicWithMarketCap, title: (metric: string) => string }} args
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
 * @param {{ cohort: CohortLongTerm, title: (metric: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createProfitabilitySectionLongTerm({ cohort, title }) {
  const u = cohort.tree.unrealized;
  const r = cohort.tree.realized;
  return {
    name: "Profitability",
    tree: [
      {
        name: "Unrealized",
        tree: [
          { name: "P&L", title: title("Unrealized P&L"), bottom: unrealizedLongTerm(u) },
          { name: "Net P&L", title: title("Net Unrealized P&L"), bottom: netUnrealizedFull(u) },
          { name: "NUPL", title: title("NUPL"), bottom: nuplSeries(u.nupl) },
        ],
      },
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
 * @param {{ cohort: CohortAgeRange, title: (metric: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createProfitabilitySectionWithInvestedCapitalPct({ cohort, title }) {
  const u = cohort.tree.unrealized;
  const r = cohort.tree.realized;
  return {
    name: "Profitability",
    tree: [
      {
        name: "Unrealized",
        tree: [
          { name: "P&L", title: title("Unrealized P&L"), bottom: unrealizedMid(u) },
          { name: "Net P&L", title: title("Net Unrealized P&L"), bottom: netUnrealizedMid(u) },
          { name: "NUPL", title: title("NUPL"), bottom: nuplSeries(u.nupl) },
        ],
      },
      realizedSubfolderMid(r, title),
    ],
  };
}

/**
 * Section with invested capital % but no unrealized relative (basic cohorts)
 * @param {{ cohort: CohortBasicWithoutMarketCap, title: (metric: string) => string }} args
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

// ============================================================================
// Grouped Cohort Helpers
// ============================================================================

/**
 * Grouped realized P&L sum (basic — all cohorts have profit/loss)
 * @template {{ name: string, color: Color, tree: { realized: { profit: Brk.BaseCumulativeSumPattern3, loss: Brk.BaseCumulativeSumPattern3 } } }} T
 * @template {{ name: string, color: Color, tree: { realized: { profit: Brk.BaseCumulativeSumPattern3, loss: Brk.BaseCumulativeSumPattern3 } } }} A
 * @param {readonly T[]} list
 * @param {A} all
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsTree}
 */
function groupedRealizedPnlSum(list, all, title) {
  return [
    {
      name: "Profit",
      title: title("Realized Profit"),
      bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
        line({ metric: tree.realized.profit.base.usd, name, color, unit: Unit.usd }),
      ),
    },
    {
      name: "Loss",
      title: title("Realized Loss"),
      bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
        line({ metric: tree.realized.loss.base.usd, name, color, unit: Unit.usd }),
      ),
    },
  ];
}

/**
 * Grouped realized P&L sum with extras (full cohorts)
 * @param {readonly (CohortAll | CohortFull | CohortLongTerm)[]} list
 * @param {CohortAll} all
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsTree}
 */
function groupedRealizedPnlSumFull(list, all, title) {
  return [
    ...groupedRealizedPnlSum(list, all, title),
    {
      name: "Total",
      title: title("Total Realized P&L"),
      bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
        line({ metric: tree.realized.grossPnl.cumulative.usd, name, color, unit: Unit.usd }),
      ),
    },
    {
      name: "P/L Ratio",
      title: title("Realized Profit/Loss Ratio"),
      bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
        baseline({ metric: tree.realized.profitToLossRatio._1y, name, color, unit: Unit.ratio }),
      ),
    },
  ];
}

/**
 * Grouped rolling realized charts (basic — profit/loss sums only)
 * @template {{ name: string, color: Color, tree: { realized: { profit: Brk.BaseCumulativeSumPattern3, loss: Brk.BaseCumulativeSumPattern3 } } }} T
 * @template {{ name: string, color: Color, tree: { realized: { profit: Brk.BaseCumulativeSumPattern3, loss: Brk.BaseCumulativeSumPattern3 } } }} A
 * @param {readonly T[]} list
 * @param {A} all
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsTree}
 */
function groupedRollingRealizedCharts(list, all, title) {
  return [
    {
      name: "Profit",
      tree: [
        { name: "24h", title: title("Realized Profit (24h)"), bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) => line({ metric: tree.realized.profit.sum._24h.usd, name, color, unit: Unit.usd })) },
        { name: "7d", title: title("Realized Profit (7d)"), bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) => line({ metric: tree.realized.profit.sum._1w.usd, name, color, unit: Unit.usd })) },
        { name: "30d", title: title("Realized Profit (30d)"), bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) => line({ metric: tree.realized.profit.sum._1m.usd, name, color, unit: Unit.usd })) },
        { name: "1y", title: title("Realized Profit (1y)"), bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) => line({ metric: tree.realized.profit.sum._1y.usd, name, color, unit: Unit.usd })) },
      ],
    },
    {
      name: "Loss",
      tree: [
        { name: "24h", title: title("Realized Loss (24h)"), bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) => line({ metric: tree.realized.loss.sum._24h.usd, name, color, unit: Unit.usd })) },
        { name: "7d", title: title("Realized Loss (7d)"), bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) => line({ metric: tree.realized.loss.sum._1w.usd, name, color, unit: Unit.usd })) },
        { name: "30d", title: title("Realized Loss (30d)"), bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) => line({ metric: tree.realized.loss.sum._1m.usd, name, color, unit: Unit.usd })) },
        { name: "1y", title: title("Realized Loss (1y)"), bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) => line({ metric: tree.realized.loss.sum._1y.usd, name, color, unit: Unit.usd })) },
      ],
    },
  ];
}

/**
 * Grouped rolling realized with P/L ratio (full cohorts)
 * @param {readonly (CohortAll | CohortFull | CohortLongTerm)[]} list
 * @param {CohortAll} all
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsTree}
 */
function groupedRollingRealizedChartsFull(list, all, title) {
  return [
    ...groupedRollingRealizedCharts(list, all, title),
    {
      name: "P/L Ratio",
      tree: [
        { name: "24h", title: title("Realized P/L Ratio (24h)"), bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) => baseline({ metric: tree.realized.profitToLossRatio._24h, name, color, unit: Unit.ratio })) },
        { name: "7d", title: title("Realized P/L Ratio (7d)"), bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) => baseline({ metric: tree.realized.profitToLossRatio._1w, name, color, unit: Unit.ratio })) },
        { name: "30d", title: title("Realized P/L Ratio (30d)"), bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) => baseline({ metric: tree.realized.profitToLossRatio._1m, name, color, unit: Unit.ratio })) },
        { name: "1y", title: title("Realized P/L Ratio (1y)"), bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) => baseline({ metric: tree.realized.profitToLossRatio._1y, name, color, unit: Unit.ratio })) },
      ],
    },
  ];
}

/**
 * Grouped realized subfolder (basic)
 * @template {{ name: string, color: Color, tree: { realized: { profit: Brk.BaseCumulativeSumPattern3, loss: Brk.BaseCumulativeSumPattern3 } } }} T
 * @template {{ name: string, color: Color, tree: { realized: { profit: Brk.BaseCumulativeSumPattern3, loss: Brk.BaseCumulativeSumPattern3 } } }} A
 * @param {readonly T[]} list
 * @param {A} all
 * @param {(metric: string) => string} title
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
              line({ metric: tree.realized.profit.cumulative.usd, name, color, unit: Unit.usd }),
            ),
          },
          {
            name: "Loss",
            title: title("Cumulative Realized Loss"),
            bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
              line({ metric: tree.realized.loss.cumulative.usd, name, color, unit: Unit.usd }),
            ),
          },
        ],
      },
    ],
  };
}

/**
 * Grouped realized subfolder for full cohorts
 * @param {readonly (CohortAll | CohortFull | CohortLongTerm)[]} list
 * @param {CohortAll} all
 * @param {(metric: string) => string} title
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
          baseline({ metric: tree.realized.netPnl.base.usd, name, color, unit: Unit.usd }),
        ),
      },
      {
        name: "30d Change",
        title: title("Realized P&L 30d Change"),
        bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
          baseline({ metric: tree.realized.netPnl.delta.change._1m.usd, name, color, unit: Unit.usd }),
        ),
      },
      { name: "Rolling", tree: groupedRollingRealizedChartsFull(list, all, title) },
      {
        name: "Cumulative",
        tree: [
          {
            name: "Profit",
            title: title("Cumulative Realized Profit"),
            bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
              line({ metric: tree.realized.profit.cumulative.usd, name, color, unit: Unit.usd }),
            ),
          },
          {
            name: "Loss",
            title: title("Cumulative Realized Loss"),
            bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
              line({ metric: tree.realized.loss.cumulative.usd, name, color, unit: Unit.usd }),
            ),
          },
          {
            name: "Net",
            title: title("Cumulative Net Realized P&L"),
            bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
              baseline({ metric: tree.realized.netPnl.cumulative.usd, name, color, unit: Unit.usd }),
            ),
          },
        ],
      },
    ],
  };
}

/**
 * Grouped unrealized P&L (USD only — for all cohorts that at least have nupl)
 * @template {{ name: string, color: Color, tree: { unrealized: { nupl: Brk.BpsRatioPattern } } }} T
 * @template {{ name: string, color: Color, tree: { unrealized: { nupl: Brk.BpsRatioPattern } } }} A
 * @param {readonly T[]} list
 * @param {A} all
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsTree}
 */
function groupedNuplCharts(list, all, title) {
  return [
    {
      name: "NUPL",
      title: title("NUPL"),
      bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
        baseline({ metric: tree.unrealized.nupl.ratio, name, color, unit: Unit.ratio }),
      ),
    },
  ];
}

/**
 * Grouped unrealized for full cohorts with relToMcap
 * @param {readonly (CohortFull | CohortLongTerm)[]} list
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
          line({ metric: tree.unrealized.profit.base.usd, name, color, unit: Unit.usd }),
        ),
        ...mapCohortsWithAll(list, all, ({ name, color, tree }) =>
          baseline({ metric: tree.unrealized.profit.relToMcap.ratio, name, color, unit: Unit.pctMcap }),
        ),
      ],
    },
    {
      name: "Loss",
      title: title("Unrealized Loss"),
      bottom: [
        ...mapCohortsWithAll(list, all, ({ name, color, tree }) =>
          line({ metric: tree.unrealized.loss.base.usd, name, color, unit: Unit.usd }),
        ),
        ...mapCohortsWithAll(list, all, ({ name, color, tree }) =>
          baseline({ metric: tree.unrealized.loss.relToMcap.ratio, name, color, unit: Unit.pctMcap }),
        ),
      ],
    },
    {
      name: "Net P&L",
      title: title("Net Unrealized P&L"),
      bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
        baseline({ metric: tree.unrealized.netPnl.usd, name, color, unit: Unit.usd }),
      ),
    },
  ];
}

/**
 * Grouped unrealized for AgeRange/MaxAge (profit/loss without relToMcap)
 * @param {readonly (CohortAgeRange | CohortWithAdjusted)[]} list
 * @param {CohortAll} all
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsTree}
 */
function groupedPnlChartsWithOwnMarketCap(list, all, title) {
  return [
    {
      name: "Profit",
      title: title("Unrealized Profit"),
      bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
        line({ metric: tree.unrealized.profit.base.usd, name, color, unit: Unit.usd }),
      ),
    },
    {
      name: "Loss",
      title: title("Unrealized Loss"),
      bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
        line({ metric: tree.unrealized.loss.base.usd, name, color, unit: Unit.usd }),
      ),
    },
    {
      name: "Net P&L",
      title: title("Net Unrealized P&L"),
      bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
        baseline({ metric: tree.unrealized.netPnl.usd, name, color, unit: Unit.usd }),
      ),
    },
  ];
}

/**
 * Grouped unrealized for LongTerm (profit/loss with relToOwnMcap + relToOwnGross)
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
          line({ metric: tree.unrealized.profit.base.usd, name, color, unit: Unit.usd }),
        ),
        ...mapCohorts(list, ({ name, color, tree }) =>
          line({ metric: tree.unrealized.profit.relToOwnMcap.ratio, name, color, unit: Unit.pctOwnMcap }),
        ),
        ...mapCohortsWithAll(list, all, ({ name, color, tree }) =>
          line({ metric: tree.unrealized.profit.relToOwnGross.ratio, name, color, unit: Unit.pctOwnPnl }),
        ),
      ],
    },
    {
      name: "Loss",
      title: title("Unrealized Loss"),
      bottom: [
        ...mapCohortsWithAll(list, all, ({ name, color, tree }) =>
          line({ metric: tree.unrealized.loss.base.usd, name, color, unit: Unit.usd }),
        ),
        ...mapCohortsWithAll(list, all, ({ name, color, tree }) =>
          line({ metric: tree.unrealized.loss.relToMcap.ratio, name, color, unit: Unit.pctMcap }),
        ),
        ...mapCohorts(list, ({ name, color, tree }) =>
          line({ metric: tree.unrealized.loss.relToOwnMcap.ratio, name, color, unit: Unit.pctOwnMcap }),
        ),
        ...mapCohortsWithAll(list, all, ({ name, color, tree }) =>
          line({ metric: tree.unrealized.loss.relToOwnGross.ratio, name, color, unit: Unit.pctOwnPnl }),
        ),
      ],
    },
    {
      name: "Net P&L",
      title: title("Net Unrealized P&L"),
      bottom: [
        ...mapCohortsWithAll(list, all, ({ name, color, tree }) =>
          baseline({ metric: tree.unrealized.netPnl.usd, name, color, unit: Unit.usd }),
        ),
        ...mapCohorts(list, ({ name, color, tree }) =>
          baseline({ metric: tree.unrealized.netPnl.relToOwnMcap.ratio, name, color, unit: Unit.pctOwnMcap }),
        ),
        ...mapCohortsWithAll(list, all, ({ name, color, tree }) =>
          baseline({ metric: tree.unrealized.netPnl.relToOwnGross.ratio, name, color, unit: Unit.pctOwnPnl }),
        ),
      ],
    },
  ];
}

/**
 * Grouped sentiment (full unrealized only)
 * @param {readonly (CohortAll | CohortFull | CohortLongTerm)[]} list
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
          baseline({ metric: tree.unrealized.sentiment.net.usd, name, color, unit: Unit.usd }),
        ),
      },
      {
        name: "Greed",
        title: title("Greed Index"),
        bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
          line({ metric: tree.unrealized.sentiment.greedIndex.usd, name, color, unit: Unit.usd }),
        ),
      },
      {
        name: "Pain",
        title: title("Pain Index"),
        bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
          line({ metric: tree.unrealized.sentiment.painIndex.usd, name, color, unit: Unit.usd }),
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
 * @param {{ list: readonly (UtxoCohortObject | CohortWithoutRelative)[], all: CohortAll, title: (metric: string) => string }} args
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
 * Grouped section with invested capital % (basic cohorts — uses NUPL only)
 * @param {{ list: readonly CohortBasicWithoutMarketCap[], all: CohortAll, title: (metric: string) => string }} args
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
 * @param {{ list: readonly (CohortAgeRange | CohortWithAdjusted)[], all: CohortAll, title: (metric: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createGroupedProfitabilitySectionWithInvestedCapitalPct({ list, all, title }) {
  return {
    name: "Profitability",
    tree: [
      {
        name: "Unrealized",
        tree: [
          ...groupedPnlChartsWithOwnMarketCap(list, all, title),
          ...groupedNuplCharts(list, all, title),
        ],
      },
      groupedRealizedSubfolder(list, all, title),
    ],
  };
}

/**
 * Grouped section with NUPL + relToMcap
 * @param {{ list: readonly (CohortFull | CohortLongTerm)[], all: CohortAll, title: (metric: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createGroupedProfitabilitySectionWithNupl({ list, all, title }) {
  return {
    name: "Profitability",
    tree: [
      {
        name: "Unrealized",
        tree: [
          ...groupedPnlChartsWithMarketCap(list, all, title),
          ...groupedNuplCharts(list, all, title),
        ],
      },
      groupedRealizedSubfolder(list, all, title),
    ],
  };
}

/**
 * Grouped section for LongTerm cohorts
 * @param {{ list: readonly CohortLongTerm[], all: CohortAll, title: (metric: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createGroupedProfitabilitySectionLongTerm({ list, all, title }) {
  return {
    name: "Profitability",
    tree: [
      {
        name: "Unrealized",
        tree: [
          ...groupedPnlChartsLongTerm(list, all, title),
          ...groupedNuplCharts(list, all, title),
        ],
      },
      groupedRealizedSubfolderFull(list, all, title),
      groupedSentiment(list, all, title),
    ],
  };
}
