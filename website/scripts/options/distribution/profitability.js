/**
 * Profitability section builders
 *
 * Structure:
 *   Profitability
 *     Unrealized (Paper Gains/Losses)
 *       - P&L: Profit, Loss, Net, Total
 *       - NUPL: Net Unrealized Profit/Loss Ratio (for cohorts with nupl)
 *     Realized (Locked-In Gains/Losses)
 *       - Sum: P&L + Net (USD, % of R.Cap)
 *       - Cumulative: P&L + Net + 30d Change (USD, % of R.Cap, % of M.Cap)
 *     Volume (Sent in Profit/Loss)
 *       - Sum + Cumulative
 *     Invested Capital: In Profit + In Loss (USD, % of R.Cap)
 *     Peak Regret (Opportunity Cost vs ATH)
 *     Sentiment (Market Sentiment - Greed, Pain, Net)
 */

import { Unit } from "../../utils/units.js";
import { line, baseline, dots, dotsBaseline } from "../series.js";
import { colors } from "../../utils/colors.js";
import { priceLine, priceLines } from "../constants.js";
import { satsBtcUsd, satsBtcUsdFrom } from "../shared.js";

// ============================================================================
// Single Cohort Helpers
// ============================================================================

/**
 * Create unrealized P&L series (USD only - for cohorts without relative data)
 * @param {{ unrealized: UnrealizedPattern }} tree
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function createUnrealizedPnlSeries(tree) {
  return [
    line({
      metric: tree.unrealized.unrealizedProfit,
      name: "Profit",
      color: colors.profit,
      unit: Unit.usd,
    }),
    line({
      metric: tree.unrealized.unrealizedLoss,
      name: "Loss",
      color: colors.loss,
      unit: Unit.usd,
    }),
    line({
      metric: tree.unrealized.totalUnrealizedPnl,
      name: "Total",
      color: colors.default,
      unit: Unit.usd,
    }),
    line({
      metric: tree.unrealized.negUnrealizedLoss,
      name: "Negative Loss",
      color: colors.loss,
      unit: Unit.usd,
      defaultActive: false,
    }),
    priceLine({
      unit: Unit.usd,
      defaultActive: false,
    }),
  ];
}

/**
 * Create unrealized P&L series with % of Own Market Cap (for ageRange cohorts)
 * @param {{ unrealized: UnrealizedPattern, relative: RelativeWithOwnMarketCap }} tree
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function createUnrealizedPnlSeriesWithOwnMarketCap(tree) {
  return [
    // USD
    line({
      metric: tree.unrealized.unrealizedProfit,
      name: "Profit",
      color: colors.profit,
      unit: Unit.usd,
    }),
    line({
      metric: tree.unrealized.unrealizedLoss,
      name: "Loss",
      color: colors.loss,
      unit: Unit.usd,
    }),
    line({
      metric: tree.unrealized.totalUnrealizedPnl,
      name: "Total",
      color: colors.default,
      unit: Unit.usd,
    }),
    line({
      metric: tree.unrealized.negUnrealizedLoss,
      name: "Negative Loss",
      color: colors.loss,
      unit: Unit.usd,
      defaultActive: false,
    }),
    priceLine({
      unit: Unit.usd,
      defaultActive: false,
    }),
    // % of Own Market Cap
    line({
      metric: tree.relative.unrealizedProfitRelToOwnMarketCap,
      name: "Profit",
      color: colors.profit,
      unit: Unit.pctOwnMcap,
    }),
    line({
      metric: tree.relative.unrealizedLossRelToOwnMarketCap,
      name: "Loss",
      color: colors.loss,
      unit: Unit.pctOwnMcap,
    }),
    line({
      metric: tree.relative.negUnrealizedLossRelToOwnMarketCap,
      name: "Negative Loss",
      color: colors.loss,
      unit: Unit.pctOwnMcap,
      defaultActive: false,
    }),
    priceLine({
      unit: Unit.pctOwnMcap,
      defaultActive: false,
    }),
    // % of Own P&L
    line({
      metric: tree.relative.unrealizedProfitRelToOwnTotalUnrealizedPnl,
      name: "Profit",
      color: colors.profit,
      unit: Unit.pctOwnPnl,
    }),
    line({
      metric: tree.relative.unrealizedLossRelToOwnTotalUnrealizedPnl,
      name: "Loss",
      color: colors.loss,
      unit: Unit.pctOwnPnl,
    }),
    line({
      metric: tree.relative.negUnrealizedLossRelToOwnTotalUnrealizedPnl,
      name: "Negative Loss",
      color: colors.loss,
      unit: Unit.pctOwnPnl,
      defaultActive: false,
    }),
    ...priceLines({
      numbers: [100, 50, 0],
      unit: Unit.pctOwnPnl,
    }),
  ];
}

/**
 * Create net unrealized P&L series with % of Own Market Cap (for ageRange cohorts)
 * @param {{ unrealized: UnrealizedPattern, relative: RelativeWithOwnMarketCap }} tree
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function createNetUnrealizedPnlSeriesWithOwnMarketCap(tree) {
  return [
    baseline({
      metric: tree.unrealized.netUnrealizedPnl,
      name: "Net P&L",
      unit: Unit.usd,
    }),
    // % of Own Market Cap
    baseline({
      metric: tree.relative.netUnrealizedPnlRelToOwnMarketCap,
      name: "Net P&L",
      unit: Unit.pctOwnMcap,
    }),
    // % of Own P&L
    baseline({
      metric: tree.relative.netUnrealizedPnlRelToOwnTotalUnrealizedPnl,
      name: "Net P&L",
      unit: Unit.pctOwnPnl,
    }),
  ];
}

/**
 * Create unrealized P&L series with % of Market Cap (USD + % of M.Cap)
 * @param {{ unrealized: UnrealizedPattern, relative: RelativeWithNupl }} tree
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function createUnrealizedPnlSeriesWithMarketCap(tree) {
  return [
    // USD
    line({
      metric: tree.unrealized.unrealizedProfit,
      name: "Profit",
      color: colors.profit,
      unit: Unit.usd,
    }),
    line({
      metric: tree.unrealized.unrealizedLoss,
      name: "Loss",
      color: colors.loss,
      unit: Unit.usd,
    }),
    line({
      metric: tree.unrealized.totalUnrealizedPnl,
      name: "Total",
      color: colors.default,
      unit: Unit.usd,
    }),
    line({
      metric: tree.unrealized.negUnrealizedLoss,
      name: "Negative Loss",
      color: colors.loss,
      unit: Unit.usd,
      defaultActive: false,
    }),
    priceLine({
      unit: Unit.usd,
      defaultActive: false,
    }),
    // % of Market Cap
    line({
      metric: tree.relative.unrealizedProfitRelToMarketCap,
      name: "Profit",
      color: colors.profit,
      unit: Unit.pctMcap,
    }),
    line({
      metric: tree.relative.unrealizedLossRelToMarketCap,
      name: "Loss",
      color: colors.loss,
      unit: Unit.pctMcap,
    }),
    line({
      metric: tree.relative.negUnrealizedLossRelToMarketCap,
      name: "Negative Loss",
      color: colors.loss,
      unit: Unit.pctMcap,
      defaultActive: false,
    }),
    priceLine({
      unit: Unit.pctMcap,
      defaultActive: false,
    }),
  ];
}

/**
 * Create unrealized P&L series for "all" cohort (USD + % of M.Cap + % of Own P&L)
 * @param {{ unrealized: UnrealizedPattern, relative: AllRelativePattern }} tree
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function createUnrealizedPnlSeriesAll(tree) {
  return [
    // USD
    line({
      metric: tree.unrealized.unrealizedProfit,
      name: "Profit",
      color: colors.profit,
      unit: Unit.usd,
    }),
    line({
      metric: tree.unrealized.unrealizedLoss,
      name: "Loss",
      color: colors.loss,
      unit: Unit.usd,
    }),
    line({
      metric: tree.unrealized.totalUnrealizedPnl,
      name: "Total",
      color: colors.default,
      unit: Unit.usd,
    }),
    line({
      metric: tree.unrealized.negUnrealizedLoss,
      name: "Negative Loss",
      color: colors.loss,
      unit: Unit.usd,
      defaultActive: false,
    }),
    priceLine({
      unit: Unit.usd,
      defaultActive: false,
    }),
    // % of Market Cap
    line({
      metric: tree.relative.unrealizedProfitRelToMarketCap,
      name: "Profit",
      color: colors.profit,
      unit: Unit.pctMcap,
    }),
    line({
      metric: tree.relative.unrealizedLossRelToMarketCap,
      name: "Loss",
      color: colors.loss,
      unit: Unit.pctMcap,
    }),
    line({
      metric: tree.relative.negUnrealizedLossRelToMarketCap,
      name: "Negative Loss",
      color: colors.loss,
      unit: Unit.pctMcap,
      defaultActive: false,
    }),
    priceLine({
      unit: Unit.pctMcap,
      defaultActive: false,
    }),
    // % of Own P&L
    line({
      metric: tree.relative.unrealizedProfitRelToOwnTotalUnrealizedPnl,
      name: "Profit",
      color: colors.profit,
      unit: Unit.pctOwnPnl,
    }),
    line({
      metric: tree.relative.unrealizedLossRelToOwnTotalUnrealizedPnl,
      name: "Loss",
      color: colors.loss,
      unit: Unit.pctOwnPnl,
    }),
    line({
      metric: tree.relative.negUnrealizedLossRelToOwnTotalUnrealizedPnl,
      name: "Negative Loss",
      color: colors.loss,
      unit: Unit.pctOwnPnl,
      defaultActive: false,
    }),
    ...priceLines({
      numbers: [100, 50, 0],
      unit: Unit.pctOwnPnl,
    }),
  ];
}

/**
 * Create net unrealized P&L series for "all" cohort (USD + % of M.Cap + % of Own P&L)
 * @param {{ unrealized: UnrealizedPattern, relative: AllRelativePattern }} tree
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function createNetUnrealizedPnlSeriesAll(tree) {
  return [
    baseline({
      metric: tree.unrealized.netUnrealizedPnl,
      name: "Net P&L",
      unit: Unit.usd,
    }),
    // % of Market Cap
    baseline({
      metric: tree.relative.netUnrealizedPnlRelToMarketCap,
      name: "Net P&L",
      unit: Unit.pctMcap,
    }),
    // % of Own P&L
    baseline({
      metric: tree.relative.netUnrealizedPnlRelToOwnTotalUnrealizedPnl,
      name: "Net P&L",
      unit: Unit.pctOwnPnl,
    }),
  ];
}

/**
 * Create net unrealized P&L series with % of Market Cap (USD + % of M.Cap)
 * @param {{ unrealized: UnrealizedPattern, relative: RelativeWithNupl }} tree
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function createNetUnrealizedPnlSeriesWithMarketCap(tree) {
  return [
    baseline({
      metric: tree.unrealized.netUnrealizedPnl,
      name: "Net P&L",
      unit: Unit.usd,
    }),
    // % of Market Cap
    baseline({
      metric: tree.relative.netUnrealizedPnlRelToMarketCap,
      name: "Net P&L",
      unit: Unit.pctMcap,
    }),
  ];
}

/**
 * Create net unrealized P&L series
 * @param {{ unrealized: UnrealizedPattern }} tree
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function createNetUnrealizedPnlSeries(tree) {
  return [
    baseline({
      metric: tree.unrealized.netUnrealizedPnl,
      name: "Net P&L",
      unit: Unit.usd,
    }),
  ];
}

/**
 * Create NUPL series
 * @param {RelativeWithNupl} relative
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function createNuplSeries(relative) {
  return [
    baseline({
      metric: relative.nupl,
      name: "NUPL",
      unit: Unit.ratio,
    }),
  ];
}

/**
 * Create invested capital series (absolute only - for cohorts without relative data)
 * @param {{ unrealized: UnrealizedPattern }} tree
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function createInvestedCapitalAbsoluteSeries(tree) {
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
 * Create invested capital series (USD + % of R.Cap)
 * @param {{ unrealized: UnrealizedPattern, relative: RelativeWithInvestedCapitalPct }} tree
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function createInvestedCapitalSeries(tree) {
  return [
    // USD
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
    // % of Own R.Cap
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
    ...priceLines({
      numbers: [100, 50],
      unit: Unit.pctOwnRcap,
    }),
  ];
}

/**
 * Create peak regret series
 * @param {{ unrealized: UnrealizedFullPattern }} tree
 * @param {Color} color
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function createPeakRegretSeries(tree, color) {
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
 * Create peak regret series with RelToMarketCap
 * @param {{ unrealized: UnrealizedFullPattern, relative: RelativeWithPeakRegret }} tree
 * @param {Color} color
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function createPeakRegretSeriesWithMarketCap(tree, color) {
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
 * Create sentiment series for single cohort
 * @param {{ unrealized: UnrealizedPattern }} tree
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function createSentimentSeries(tree) {
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

// ============================================================================
// Realized P&L Helpers
// ============================================================================

/**
 * Create realized P&L sum series (Profit, Loss only - no Net)
 * @param {{ realized: AnyRealizedPattern }} tree
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function createRealizedPnlSumSeries(tree) {
  return [
    line({
      metric: tree.realized.realizedProfit7dEma,
      name: "Profit 7d EMA",
      color: colors.profit,
      unit: Unit.usd,
    }),
    line({
      metric: tree.realized.realizedLoss7dEma,
      name: "Loss 7d EMA",
      color: colors.loss,
      unit: Unit.usd,
    }),
    dots({
      metric: tree.realized.realizedProfit.sum,
      name: "Profit",
      color: colors.profit,
      unit: Unit.usd,
      defaultActive: false,
    }),
    dots({
      metric: tree.realized.negRealizedLoss.sum,
      name: "Negative Loss",
      color: colors.loss,
      unit: Unit.usd,
      defaultActive: false,
    }),
    dots({
      metric: tree.realized.realizedLoss.sum,
      name: "Loss",
      color: colors.loss,
      unit: Unit.usd,
      defaultActive: false,
    }),
    dots({
      metric: tree.realized.realizedValue,
      name: "Value",
      color: colors.default,
      unit: Unit.usd,
      defaultActive: false,
    }),
    // % of R.Cap
    baseline({
      metric: tree.realized.realizedProfitRelToRealizedCap.sum,
      name: "Profit",
      color: colors.profit,
      unit: Unit.pctRcap,
    }),
    baseline({
      metric: tree.realized.realizedLossRelToRealizedCap.sum,
      name: "Loss",
      color: colors.loss,
      unit: Unit.pctRcap,
    }),
  ];
}

/**
 * Create realized Net P&L sum series (baseline)
 * @param {{ realized: AnyRealizedPattern }} tree
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function createRealizedNetPnlSumSeries(tree) {
  return [
    baseline({
      metric: tree.realized.netRealizedPnl7dEma,
      name: "Net 7d EMA",
      unit: Unit.usd,
    }),
    dotsBaseline({
      metric: tree.realized.netRealizedPnl.sum,
      name: "Net",
      unit: Unit.usd,
      defaultActive: false,
    }),
    // % of R.Cap
    baseline({
      metric: tree.realized.netRealizedPnlRelToRealizedCap.sum,
      name: "Net",
      unit: Unit.pctRcap,
    }),
  ];
}

/**
 * Create realized P&L cumulative series (Profit, Loss only - no Net)
 * @param {{ realized: AnyRealizedPattern }} tree
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function createRealizedPnlCumulativeSeries(tree) {
  return [
    line({
      metric: tree.realized.realizedProfit.cumulative,
      name: "Profit",
      color: colors.profit,
      unit: Unit.usd,
    }),
    line({
      metric: tree.realized.realizedLoss.cumulative,
      name: "Loss",
      color: colors.loss,
      unit: Unit.usd,
    }),
    line({
      metric: tree.realized.negRealizedLoss.cumulative,
      name: "Negative Loss",
      color: colors.loss,
      unit: Unit.usd,
      defaultActive: false,
    }),
    // % of R.Cap
    baseline({
      metric: tree.realized.realizedProfitRelToRealizedCap.cumulative,
      name: "Profit",
      color: colors.profit,
      unit: Unit.pctRcap,
    }),
    baseline({
      metric: tree.realized.realizedLossRelToRealizedCap.cumulative,
      name: "Loss",
      color: colors.loss,
      unit: Unit.pctRcap,
    }),
  ];
}

/**
 * Create realized Net P&L cumulative series (baseline)
 * @param {{ realized: AnyRealizedPattern }} tree
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function createRealizedNetPnlCumulativeSeries(tree) {
  return [
    baseline({
      metric: tree.realized.netRealizedPnl.cumulative,
      name: "Net",
      unit: Unit.usd,
    }),
    // % of R.Cap
    baseline({
      metric: tree.realized.netRealizedPnlRelToRealizedCap.cumulative,
      name: "Net",
      unit: Unit.pctRcap,
    }),
  ];
}

/**
 * Create realized 30d change series
 * @param {{ realized: AnyRealizedPattern }} tree
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function createRealized30dChangeSeries(tree) {
  return [
    baseline({
      metric: tree.realized.netRealizedPnlCumulative30dDelta,
      name: "30d Change",
      unit: Unit.usd,
    }),
    // % of M.Cap
    baseline({
      metric: tree.realized.netRealizedPnlCumulative30dDeltaRelToMarketCap,
      name: "30d Change",
      unit: Unit.pctMcap,
    }),
    // % of R.Cap
    baseline({
      metric: tree.realized.netRealizedPnlCumulative30dDeltaRelToRealizedCap,
      name: "30d Change",
      unit: Unit.pctRcap,
    }),
  ];
}

/**
 * Create sent in profit/loss tree for single cohort
 * @param {{ realized: AnyRealizedPattern }} tree
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsTree}
 */
function createSentInPnlTree(tree, title) {
  return [
    {
      name: "Sum",
      title: title("Sent In Profit & Loss"),
      bottom: [
        ...satsBtcUsd({
          pattern: tree.realized.sentInProfit14dEma,
          name: "In Profit 14d EMA",
          color: colors.profit,
          defaultActive: false,
        }),
        ...satsBtcUsd({
          pattern: tree.realized.sentInLoss14dEma,
          name: "In Loss 14d EMA",
          color: colors.loss,
          defaultActive: false,
        }),
        ...satsBtcUsdFrom({
          source: tree.realized.sentInProfit,
          key: "sum",
          name: "In Profit",
          color: colors.profit,
        }),
        ...satsBtcUsdFrom({
          source: tree.realized.sentInLoss,
          key: "sum",
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
          source: tree.realized.sentInProfit,
          key: "cumulative",
          name: "In Profit",
          color: colors.profit,
        }),
        ...satsBtcUsdFrom({
          source: tree.realized.sentInLoss,
          key: "cumulative",
          name: "In Loss",
          color: colors.loss,
        }),
      ],
    },
  ];
}

/**
 * Create realized subfolder for single cohort
 * @param {{ realized: AnyRealizedPattern }} tree
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsGroup}
 */
function createRealizedSubfolder(tree, title) {
  return {
    name: "Realized",
    tree: [
      {
        name: "P&L",
        title: title("Realized P&L"),
        bottom: createRealizedPnlSumSeries(tree),
      },
      {
        name: "Net",
        title: title("Net Realized P&L"),
        bottom: createRealizedNetPnlSumSeries(tree),
      },
      {
        name: "30d Change",
        title: title("Realized P&L 30d Change"),
        bottom: createRealized30dChangeSeries(tree),
      },
      {
        name: "Total",
        title: title("Total Realized P&L"),
        bottom: [
          line({
            metric: tree.realized.totalRealizedPnl,
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
            metric: tree.realized.peakRegret.sum,
            name: "Peak Regret",
            unit: Unit.usd,
          }),
        ],
      },
      {
        name: "Cumulative",
        tree: [
          {
            name: "P&L",
            title: title("Cumulative Realized P&L"),
            bottom: createRealizedPnlCumulativeSeries(tree),
          },
          {
            name: "Net",
            title: title("Cumulative Net Realized P&L"),
            bottom: createRealizedNetPnlCumulativeSeries(tree),
          },
          {
            name: "Peak Regret",
            title: title("Cumulative Realized Peak Regret"),
            bottom: [
              line({
                metric: tree.realized.peakRegret.cumulative,
                name: "Peak Regret",
                unit: Unit.usd,
              }),
              // % of R.Cap
              line({
                metric: tree.realized.peakRegretRelToRealizedCap,
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
 * Create realized subfolder for cohorts with RealizedWithExtras (has P/L Ratio)
 * @param {{ realized: RealizedWithExtras }} tree
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsGroup}
 */
function createRealizedSubfolderWithExtras(tree, title) {
  return {
    name: "Realized",
    tree: [
      {
        name: "P&L",
        title: title("Realized P&L"),
        bottom: createRealizedPnlSumSeries(tree),
      },
      {
        name: "Net",
        title: title("Net Realized P&L"),
        bottom: createRealizedNetPnlSumSeries(tree),
      },
      {
        name: "30d Change",
        title: title("Realized P&L 30d Change"),
        bottom: createRealized30dChangeSeries(tree),
      },
      {
        name: "Total",
        title: title("Total Realized P&L"),
        bottom: [
          line({
            metric: tree.realized.totalRealizedPnl,
            name: "Total",
            unit: Unit.usd,
            color: colors.bitcoin,
          }),
        ],
      },
      {
        name: "P/L Ratio",
        title: title("Realized Profit/Loss Ratio"),
        bottom: [
          baseline({
            metric: tree.realized.realizedProfitToLossRatio,
            name: "P/L Ratio",
            unit: Unit.ratio,
          }),
        ],
      },
      {
        name: "Peak Regret",
        title: title("Realized Peak Regret"),
        bottom: [
          line({
            metric: tree.realized.peakRegret.sum,
            name: "Peak Regret",
            unit: Unit.usd,
          }),
        ],
      },
      {
        name: "Cumulative",
        tree: [
          {
            name: "P&L",
            title: title("Cumulative Realized P&L"),
            bottom: createRealizedPnlCumulativeSeries(tree),
          },
          {
            name: "Net",
            title: title("Cumulative Net Realized P&L"),
            bottom: createRealizedNetPnlCumulativeSeries(tree),
          },
          {
            name: "Peak Regret",
            title: title("Cumulative Realized Peak Regret"),
            bottom: [
              line({
                metric: tree.realized.peakRegret.cumulative,
                name: "Peak Regret",
                unit: Unit.usd,
              }),
              // % of R.Cap
              line({
                metric: tree.realized.peakRegretRelToRealizedCap,
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
 * Create volume subfolder for single cohort
 * @param {{ realized: AnyRealizedPattern }} tree
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsGroup}
 */
function createVolumeSubfolder(tree, title) {
  return {
    name: "Volume",
    tree: createSentInPnlTree(tree, title),
  };
}

// ============================================================================
// Single Cohort Section Builders
// ============================================================================

/**
 * Create basic profitability section (all cohorts have these)
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
          {
            name: "P&L",
            title: title("Unrealized P&L"),
            bottom: createUnrealizedPnlSeries(tree),
          },
          {
            name: "Net P&L",
            title: title("Net Unrealized P&L"),
            bottom: createNetUnrealizedPnlSeries(tree),
          },
        ],
      },
      createRealizedSubfolder(tree, title),
      createVolumeSubfolder(tree, title),
      {
        name: "Invested Capital",
        tree: [
          {
            name: "Absolute",
            title: title("Invested Capital In Profit & Loss"),
            bottom: createInvestedCapitalAbsoluteSeries(tree),
          },
        ],
      },
      {
        name: "Sentiment",
        title: title("Market Sentiment"),
        bottom: createSentimentSeries(tree),
      },
    ],
  };
}

/**
 * Create profitability section with invested capital pct only (for basic cohorts)
 * Has invested capital % but no unrealized P&L relative metrics
 * @param {{ cohort: CohortBasicWithoutMarketCap, title: (metric: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createProfitabilitySectionBasicWithInvestedCapitalPct({
  cohort,
  title,
}) {
  const { tree } = cohort;
  return {
    name: "Profitability",
    tree: [
      {
        name: "Unrealized",
        tree: [
          {
            name: "P&L",
            title: title("Unrealized P&L"),
            bottom: createUnrealizedPnlSeries(tree),
          },
          {
            name: "Net P&L",
            title: title("Net Unrealized P&L"),
            bottom: createNetUnrealizedPnlSeries(tree),
          },
        ],
      },
      createRealizedSubfolder(tree, title),
      createVolumeSubfolder(tree, title),
      {
        name: "Invested Capital",
        title: title("Invested Capital In Profit & Loss"),
        bottom: createInvestedCapitalSeries(tree),
      },
      {
        name: "Sentiment",
        title: title("Market Sentiment"),
        bottom: createSentimentSeries(tree),
      },
    ],
  };
}

/**
 * Create profitability section with invested capital pct (for ageRange cohorts)
 * Has invested capital % and unrealized P&L % of Own Market Cap
 * @param {{ cohort: CohortAgeRange, title: (metric: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createProfitabilitySectionWithInvestedCapitalPct({
  cohort,
  title,
}) {
  const { tree, color } = cohort;
  return {
    name: "Profitability",
    tree: [
      {
        name: "Unrealized",
        tree: [
          {
            name: "P&L",
            title: title("Unrealized P&L"),
            bottom: createUnrealizedPnlSeriesWithOwnMarketCap(tree),
          },
          {
            name: "Net P&L",
            title: title("Net Unrealized P&L"),
            bottom: createNetUnrealizedPnlSeriesWithOwnMarketCap(tree),
          },
          {
            name: "Peak Regret",
            title: title("Unrealized Peak Regret"),
            bottom: createPeakRegretSeries(tree, color),
          },
        ],
      },
      createRealizedSubfolderWithExtras(tree, title),
      createVolumeSubfolder(tree, title),
      {
        name: "Invested Capital",
        title: title("Invested Capital In Profit & Loss"),
        bottom: createInvestedCapitalSeries(tree),
      },
      {
        name: "Sentiment",
        title: title("Market Sentiment"),
        bottom: createSentimentSeries(tree),
      },
    ],
  };
}

/**
 * Create profitability section with NUPL (for cohorts with RelativeWithNupl)
 * CohortBasicWithMarketCap
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
          {
            name: "P&L",
            title: title("Unrealized P&L"),
            bottom: createUnrealizedPnlSeriesWithMarketCap(tree),
          },
          {
            name: "Net P&L",
            title: title("Net Unrealized P&L"),
            bottom: createNetUnrealizedPnlSeriesWithMarketCap(tree),
          },
          {
            name: "NUPL",
            title: title("NUPL"),
            bottom: createNuplSeries(tree.relative),
          },
        ],
      },
      createRealizedSubfolder(tree, title),
      createVolumeSubfolder(tree, title),
      {
        name: "Invested Capital",
        title: title("Invested Capital In Profit & Loss"),
        bottom: createInvestedCapitalSeries(tree),
      },
      {
        name: "Sentiment",
        title: title("Market Sentiment"),
        bottom: createSentimentSeries(tree),
      },
    ],
  };
}

/**
 * Create unrealized P&L series for LongTerm cohort (USD + % M.Cap + % Own M.Cap + % Own P&L)
 * @param {{ unrealized: UnrealizedFullPattern, relative: RelativeWithOwnMarketCap & RelativeWithNupl }} tree
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function createUnrealizedPnlSeriesLongTerm(tree) {
  return [
    // USD
    line({
      metric: tree.unrealized.unrealizedProfit,
      name: "Profit",
      color: colors.profit,
      unit: Unit.usd,
    }),
    line({
      metric: tree.unrealized.unrealizedLoss,
      name: "Loss",
      color: colors.loss,
      unit: Unit.usd,
    }),
    line({
      metric: tree.unrealized.totalUnrealizedPnl,
      name: "Total",
      color: colors.default,
      unit: Unit.usd,
    }),
    line({
      metric: tree.unrealized.negUnrealizedLoss,
      name: "Negative Loss",
      color: colors.loss,
      unit: Unit.usd,
      defaultActive: false,
    }),
    priceLine({
      unit: Unit.usd,
      defaultActive: false,
    }),
    // % of Market Cap (only loss available for LongTerm)
    line({
      metric: tree.relative.unrealizedLossRelToMarketCap,
      name: "Loss",
      color: colors.loss,
      unit: Unit.pctMcap,
    }),
    // % of Own Market Cap
    line({
      metric: tree.relative.unrealizedProfitRelToOwnMarketCap,
      name: "Profit",
      color: colors.profit,
      unit: Unit.pctOwnMcap,
    }),
    line({
      metric: tree.relative.unrealizedLossRelToOwnMarketCap,
      name: "Loss",
      color: colors.loss,
      unit: Unit.pctOwnMcap,
    }),
    line({
      metric: tree.relative.negUnrealizedLossRelToOwnMarketCap,
      name: "Negative Loss",
      color: colors.loss,
      unit: Unit.pctOwnMcap,
      defaultActive: false,
    }),
    priceLine({
      unit: Unit.pctOwnMcap,
      defaultActive: false,
    }),
    // % of Own P&L
    line({
      metric: tree.relative.unrealizedProfitRelToOwnTotalUnrealizedPnl,
      name: "Profit",
      color: colors.profit,
      unit: Unit.pctOwnPnl,
    }),
    line({
      metric: tree.relative.unrealizedLossRelToOwnTotalUnrealizedPnl,
      name: "Loss",
      color: colors.loss,
      unit: Unit.pctOwnPnl,
    }),
    line({
      metric: tree.relative.negUnrealizedLossRelToOwnTotalUnrealizedPnl,
      name: "Negative Loss",
      color: colors.loss,
      unit: Unit.pctOwnPnl,
      defaultActive: false,
    }),
    ...priceLines({
      numbers: [100, 50, 0],
      unit: Unit.pctOwnPnl,
    }),
  ];
}

/**
 * Create net unrealized P&L series for LongTerm cohort (USD + % Own M.Cap + % Own P&L)
 * @param {{ unrealized: UnrealizedPattern, relative: RelativeWithOwnMarketCap }} tree
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function createNetUnrealizedPnlSeriesLongTerm(tree) {
  return [
    baseline({
      metric: tree.unrealized.netUnrealizedPnl,
      name: "Net P&L",
      unit: Unit.usd,
    }),
    // % of Own Market Cap
    baseline({
      metric: tree.relative.netUnrealizedPnlRelToOwnMarketCap,
      name: "Net P&L",
      unit: Unit.pctOwnMcap,
    }),
    // % of Own P&L
    baseline({
      metric: tree.relative.netUnrealizedPnlRelToOwnTotalUnrealizedPnl,
      name: "Net P&L",
      unit: Unit.pctOwnPnl,
    }),
  ];
}

/**
 * Create profitability section for LongTerm cohort (has own market cap + NUPL + peak regret + P/L ratio)
 * @param {{ cohort: CohortLongTerm, title: (metric: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createProfitabilitySectionLongTerm({ cohort, title }) {
  const { tree, color } = cohort;
  return {
    name: "Profitability",
    tree: [
      {
        name: "Unrealized",
        tree: [
          {
            name: "P&L",
            title: title("Unrealized P&L"),
            bottom: createUnrealizedPnlSeriesLongTerm(tree),
          },
          {
            name: "Net P&L",
            title: title("Net Unrealized P&L"),
            bottom: createNetUnrealizedPnlSeriesLongTerm(tree),
          },
          {
            name: "NUPL",
            title: title("NUPL"),
            bottom: createNuplSeries(tree.relative),
          },
          {
            name: "Peak Regret",
            title: title("Unrealized Peak Regret"),
            bottom: createPeakRegretSeriesWithMarketCap(tree, color),
          },
        ],
      },
      createRealizedSubfolderWithExtras(tree, title),
      createVolumeSubfolder(tree, title),
      {
        name: "Invested Capital",
        title: title("Invested Capital In Profit & Loss"),
        bottom: createInvestedCapitalSeries(tree),
      },
      {
        name: "Sentiment",
        title: title("Market Sentiment"),
        bottom: createSentimentSeries(tree),
      },
    ],
  };
}

/**
 * Create unrealized P&L series for Full cohorts (USD + % M.Cap + % Own M.Cap + % Own P&L)
 * @param {{ unrealized: UnrealizedFullPattern, relative: FullRelativePattern }} tree
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function createUnrealizedPnlSeriesFull(tree) {
  return [
    // USD
    line({
      metric: tree.unrealized.unrealizedProfit,
      name: "Profit",
      color: colors.profit,
      unit: Unit.usd,
    }),
    line({
      metric: tree.unrealized.unrealizedLoss,
      name: "Loss",
      color: colors.loss,
      unit: Unit.usd,
    }),
    line({
      metric: tree.unrealized.totalUnrealizedPnl,
      name: "Total",
      color: colors.default,
      unit: Unit.usd,
    }),
    line({
      metric: tree.unrealized.negUnrealizedLoss,
      name: "Negative Loss",
      color: colors.loss,
      unit: Unit.usd,
      defaultActive: false,
    }),
    priceLine({
      unit: Unit.usd,
      defaultActive: false,
    }),
    // % of Market Cap
    line({
      metric: tree.relative.unrealizedProfitRelToMarketCap,
      name: "Profit",
      color: colors.profit,
      unit: Unit.pctMcap,
    }),
    line({
      metric: tree.relative.unrealizedLossRelToMarketCap,
      name: "Loss",
      color: colors.loss,
      unit: Unit.pctMcap,
    }),
    line({
      metric: tree.relative.negUnrealizedLossRelToMarketCap,
      name: "Negative Loss",
      color: colors.loss,
      unit: Unit.pctMcap,
      defaultActive: false,
    }),
    priceLine({
      unit: Unit.pctMcap,
      defaultActive: false,
    }),
    // % of Own Market Cap
    line({
      metric: tree.relative.unrealizedProfitRelToOwnMarketCap,
      name: "Profit",
      color: colors.profit,
      unit: Unit.pctOwnMcap,
    }),
    line({
      metric: tree.relative.unrealizedLossRelToOwnMarketCap,
      name: "Loss",
      color: colors.loss,
      unit: Unit.pctOwnMcap,
    }),
    line({
      metric: tree.relative.negUnrealizedLossRelToOwnMarketCap,
      name: "Negative Loss",
      color: colors.loss,
      unit: Unit.pctOwnMcap,
      defaultActive: false,
    }),
    priceLine({
      unit: Unit.pctOwnMcap,
      defaultActive: false,
    }),
    // % of Own P&L
    line({
      metric: tree.relative.unrealizedProfitRelToOwnTotalUnrealizedPnl,
      name: "Profit",
      color: colors.profit,
      unit: Unit.pctOwnPnl,
    }),
    line({
      metric: tree.relative.unrealizedLossRelToOwnTotalUnrealizedPnl,
      name: "Loss",
      color: colors.loss,
      unit: Unit.pctOwnPnl,
    }),
    line({
      metric: tree.relative.negUnrealizedLossRelToOwnTotalUnrealizedPnl,
      name: "Negative Loss",
      color: colors.loss,
      unit: Unit.pctOwnPnl,
      defaultActive: false,
    }),
    ...priceLines({
      numbers: [100, 50, 0],
      unit: Unit.pctOwnPnl,
    }),
  ];
}

/**
 * Create net unrealized P&L series for Full cohorts (USD + % M.Cap + % Own M.Cap + % Own P&L)
 * @param {{ unrealized: UnrealizedPattern, relative: FullRelativePattern }} tree
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function createNetUnrealizedPnlSeriesFull(tree) {
  return [
    baseline({
      metric: tree.unrealized.netUnrealizedPnl,
      name: "Net P&L",
      unit: Unit.usd,
    }),
    // % of Market Cap
    baseline({
      metric: tree.relative.netUnrealizedPnlRelToMarketCap,
      name: "Net P&L",
      unit: Unit.pctMcap,
    }),
    // % of Own Market Cap
    baseline({
      metric: tree.relative.netUnrealizedPnlRelToOwnMarketCap,
      name: "Net P&L",
      unit: Unit.pctOwnMcap,
    }),
    // % of Own P&L
    baseline({
      metric: tree.relative.netUnrealizedPnlRelToOwnTotalUnrealizedPnl,
      name: "Net P&L",
      unit: Unit.pctOwnPnl,
    }),
  ];
}

/**
 * Create profitability section for Full cohorts (USD + % M.Cap + % Own M.Cap + % Own P&L + Peak Regret)
 * @param {{ cohort: CohortFull, title: (metric: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createProfitabilitySectionFull({ cohort, title }) {
  const { tree, color } = cohort;
  return {
    name: "Profitability",
    tree: [
      {
        name: "Unrealized",
        tree: [
          {
            name: "P&L",
            title: title("Unrealized P&L"),
            bottom: createUnrealizedPnlSeriesFull(tree),
          },
          {
            name: "Net P&L",
            title: title("Net Unrealized P&L"),
            bottom: createNetUnrealizedPnlSeriesFull(tree),
          },
          {
            name: "NUPL",
            title: title("NUPL"),
            bottom: createNuplSeries(tree.relative),
          },
          {
            name: "Peak Regret",
            title: title("Unrealized Peak Regret"),
            bottom: createPeakRegretSeriesWithMarketCap(tree, color),
          },
        ],
      },
      createRealizedSubfolderWithExtras(tree, title),
      createVolumeSubfolder(tree, title),
      {
        name: "Invested Capital",
        title: title("Invested Capital In Profit & Loss"),
        bottom: createInvestedCapitalSeries(tree),
      },
      {
        name: "Sentiment",
        title: title("Market Sentiment"),
        bottom: createSentimentSeries(tree),
      },
    ],
  };
}

/**
 * Create profitability section with NUPL + Peak Regret (CohortAll has special All pattern)
 * @param {{ cohort: CohortAll, title: (metric: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createProfitabilitySectionAll({ cohort, title }) {
  const { tree, color } = cohort;
  return {
    name: "Profitability",
    tree: [
      {
        name: "Unrealized",
        tree: [
          {
            name: "P&L",
            title: title("Unrealized P&L"),
            bottom: createUnrealizedPnlSeriesAll(tree),
          },
          {
            name: "Net P&L",
            title: title("Net Unrealized P&L"),
            bottom: createNetUnrealizedPnlSeriesAll(tree),
          },
          {
            name: "NUPL",
            title: title("NUPL"),
            bottom: createNuplSeries(tree.relative),
          },
          {
            name: "Peak Regret",
            title: title("Unrealized Peak Regret"),
            bottom: createPeakRegretSeriesWithMarketCap(tree, color),
          },
        ],
      },
      createRealizedSubfolderWithExtras(tree, title),
      createVolumeSubfolder(tree, title),
      {
        name: "Invested Capital",
        title: title("Invested Capital In Profit & Loss"),
        bottom: createInvestedCapitalSeries(tree),
      },
      {
        name: "Sentiment",
        title: title("Market Sentiment"),
        bottom: createSentimentSeries(tree),
      },
    ],
  };
}

/**
 * Create profitability section with Peak Regret + NUPL (CohortMinAge has GlobalPeakRelativePattern)
 * @param {{ cohort: CohortMinAge, title: (metric: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createProfitabilitySectionWithPeakRegret({ cohort, title }) {
  const { tree, color } = cohort;
  return {
    name: "Profitability",
    tree: [
      {
        name: "Unrealized",
        tree: [
          {
            name: "P&L",
            title: title("Unrealized P&L"),
            bottom: createUnrealizedPnlSeriesWithMarketCap(tree),
          },
          {
            name: "Net P&L",
            title: title("Net Unrealized P&L"),
            bottom: createNetUnrealizedPnlSeriesWithMarketCap(tree),
          },
          {
            name: "NUPL",
            title: title("NUPL"),
            bottom: createNuplSeries(tree.relative),
          },
          {
            name: "Peak Regret",
            title: title("Unrealized Peak Regret"),
            bottom: createPeakRegretSeriesWithMarketCap(tree, color),
          },
        ],
      },
      createRealizedSubfolder(tree, title),
      createVolumeSubfolder(tree, title),
      {
        name: "Invested Capital",
        title: title("Invested Capital In Profit & Loss"),
        bottom: createInvestedCapitalSeries(tree),
      },
      {
        name: "Sentiment",
        title: title("Market Sentiment"),
        bottom: createSentimentSeries(tree),
      },
    ],
  };
}

// ============================================================================
// Grouped Cohort Helpers
// ============================================================================

/**
 * Create grouped P&L charts (Profit, Loss, Net P&L) - USD only
 * @template {{ color: Color, name: string, tree: { unrealized: UnrealizedPattern } }} T
 * @param {readonly T[]} list
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsTree}
 */
function groupedPnlCharts(list, title) {
  return [
    {
      name: "Profit",
      title: title("Unrealized Profit"),
      bottom: list.map(({ color, name, tree }) =>
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
      bottom: list.map(({ color, name, tree }) =>
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
      bottom: list.map(({ color, name, tree }) =>
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
 * Create grouped P&L charts with % of Market Cap (Profit, Loss, Net P&L)
 * @template {{ color: Color, name: string, tree: { unrealized: UnrealizedPattern, relative: RelativeWithNupl } }} T
 * @param {readonly T[]} list
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsTree}
 */
function groupedPnlChartsWithMarketCap(list, title) {
  return [
    {
      name: "Profit",
      title: title("Unrealized Profit"),
      bottom: [
        // USD
        ...list.map(({ color, name, tree }) =>
          line({
            metric: tree.unrealized.unrealizedProfit,
            name,
            color,
            unit: Unit.usd,
          }),
        ),
        // % of Market Cap
        ...list.map(({ color, name, tree }) =>
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
        // USD
        ...list.map(({ color, name, tree }) =>
          line({
            metric: tree.unrealized.negUnrealizedLoss,
            name,
            color,
            unit: Unit.usd,
          }),
        ),
        // % of Market Cap
        ...list.map(({ color, name, tree }) =>
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
        // USD
        ...list.map(({ color, name, tree }) =>
          baseline({
            metric: tree.unrealized.netUnrealizedPnl,
            name,
            color,
            unit: Unit.usd,
          }),
        ),
        // % of Market Cap
        ...list.map(({ color, name, tree }) =>
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
 * Create grouped P&L charts with % of Own Market Cap (for ageRange cohorts)
 * @template {{ color: Color, name: string, tree: { unrealized: UnrealizedPattern, relative: RelativeWithOwnMarketCap } }} T
 * @param {readonly T[]} list
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsTree}
 */
function groupedPnlChartsWithOwnMarketCap(list, title) {
  return [
    {
      name: "Profit",
      title: title("Unrealized Profit"),
      bottom: [
        // USD
        ...list.map(({ color, name, tree }) =>
          line({
            metric: tree.unrealized.unrealizedProfit,
            name,
            color,
            unit: Unit.usd,
          }),
        ),
        // % of Own Market Cap
        ...list.map(({ color, name, tree }) =>
          line({
            metric: tree.relative.unrealizedProfitRelToOwnMarketCap,
            name,
            color,
            unit: Unit.pctOwnMcap,
          }),
        ),
        // % of Own P&L
        ...list.map(({ color, name, tree }) =>
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
        // USD
        ...list.map(({ color, name, tree }) =>
          line({
            metric: tree.unrealized.negUnrealizedLoss,
            name,
            color,
            unit: Unit.usd,
          }),
        ),
        // % of Own Market Cap
        ...list.map(({ color, name, tree }) =>
          line({
            metric: tree.relative.negUnrealizedLossRelToOwnMarketCap,
            name,
            color,
            unit: Unit.pctOwnMcap,
          }),
        ),
        // % of Own P&L
        ...list.map(({ color, name, tree }) =>
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
        // USD
        ...list.map(({ color, name, tree }) =>
          baseline({
            metric: tree.unrealized.netUnrealizedPnl,
            name,
            color,
            unit: Unit.usd,
          }),
        ),
        // % of Own Market Cap
        ...list.map(({ color, name, tree }) =>
          baseline({
            metric: tree.relative.netUnrealizedPnlRelToOwnMarketCap,
            name,
            color,
            unit: Unit.pctOwnMcap,
          }),
        ),
        // % of Own P&L
        ...list.map(({ color, name, tree }) =>
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
 * Create grouped invested capital charts (absolute only - for cohorts without relative data)
 * @template {{ color: Color, name: string, tree: { unrealized: UnrealizedPattern } }} T
 * @param {readonly T[]} list
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsTree}
 */
function groupedInvestedCapitalAbsoluteCharts(list, title) {
  return [
    {
      name: "In Profit",
      title: title("Invested Capital In Profit"),
      bottom: list.map(({ color, name, tree }) =>
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
      bottom: list.map(({ color, name, tree }) =>
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
 * Create grouped invested capital charts (USD + % of R.Cap)
 * @template {{ color: Color, name: string, tree: { unrealized: UnrealizedPattern, relative: RelativeWithInvestedCapitalPct } }} T
 * @param {readonly T[]} list
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsTree}
 */
function groupedInvestedCapitalCharts(list, title) {
  return [
    {
      name: "In Profit",
      title: title("Invested Capital In Profit"),
      bottom: [
        // USD
        ...list.map(({ color, name, tree }) =>
          line({
            metric: tree.unrealized.investedCapitalInProfit,
            name,
            color,
            unit: Unit.usd,
          }),
        ),
        // % of Own R.Cap
        ...list.map(({ color, name, tree }) =>
          baseline({
            metric: tree.relative.investedCapitalInProfitPct,
            name,
            color,
            unit: Unit.pctOwnRcap,
          }),
        ),
        ...priceLines({
          numbers: [100, 50],
          unit: Unit.pctOwnRcap,
        }),
      ],
    },
    {
      name: "In Loss",
      title: title("Invested Capital In Loss"),
      bottom: [
        // USD
        ...list.map(({ color, name, tree }) =>
          line({
            metric: tree.unrealized.investedCapitalInLoss,
            name,
            color,
            unit: Unit.usd,
          }),
        ),
        // % of Own R.Cap
        ...list.map(({ color, name, tree }) =>
          baseline({
            metric: tree.relative.investedCapitalInLossPct,
            name,
            color,
            unit: Unit.pctOwnRcap,
          }),
        ),
        ...priceLines({
          numbers: [100, 50],
          unit: Unit.pctOwnRcap,
        }),
      ],
    },
  ];
}

/**
 * Create grouped realized P&L sum charts
 * @template {{ color: Color, name: string, tree: { realized: AnyRealizedPattern } }} T
 * @param {readonly T[]} list
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsTree}
 */
function groupedRealizedPnlSumCharts(list, title) {
  return [
    {
      name: "Profit",
      title: title("Realized Profit"),
      bottom: list.map(({ color, name, tree }) =>
        line({
          metric: tree.realized.realizedProfit.sum,
          name,
          color,
          unit: Unit.usd,
        }),
      ),
    },
    {
      name: "Loss",
      title: title("Realized Loss"),
      bottom: list.map(({ color, name, tree }) =>
        line({
          metric: tree.realized.negRealizedLoss.sum,
          name,
          color,
          unit: Unit.usd,
        }),
      ),
    },
    {
      name: "Total",
      title: title("Total Realized P&L"),
      bottom: list.map(({ color, name, tree }) =>
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
      bottom: list.map(({ color, name, tree }) =>
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
 * Create grouped realized P&L sum charts with extras (has P/L Ratio)
 * @template {{ color: Color, name: string, tree: { realized: RealizedWithExtras } }} T
 * @param {readonly T[]} list
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsTree}
 */
function groupedRealizedPnlSumChartsWithExtras(list, title) {
  return [
    ...groupedRealizedPnlSumCharts(list, title),
    {
      name: "P/L Ratio",
      title: title("Realized Profit/Loss Ratio"),
      bottom: list.map(({ color, name, tree }) =>
        baseline({
          metric: tree.realized.realizedProfitToLossRatio,
          name,
          color,
          unit: Unit.ratio,
        }),
      ),
    },
  ];
}

/**
 * Create grouped realized Net P&L sum chart
 * @template {{ color: Color, name: string, tree: { realized: AnyRealizedPattern } }} T
 * @param {readonly T[]} list
 * @param {(metric: string) => string} title
 * @returns {PartialChartOption}
 */
function groupedRealizedNetPnlSumChart(list, title) {
  return {
    name: "Net",
    title: title("Net Realized P&L"),
    bottom: list.map(({ color, name, tree }) =>
      baseline({
        metric: tree.realized.netRealizedPnl.sum,
        name,
        color,
        unit: Unit.usd,
      }),
    ),
  };
}

/**
 * Create grouped realized P&L cumulative charts (Profit, Loss only)
 * @template {{ color: Color, name: string, tree: { realized: AnyRealizedPattern } }} T
 * @param {readonly T[]} list
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsTree}
 */
function groupedRealizedPnlCumulativeCharts(list, title) {
  return [
    {
      name: "Profit",
      title: title("Cumulative Realized Profit"),
      bottom: list.map(({ color, name, tree }) =>
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
      bottom: list.map(({ color, name, tree }) =>
        line({
          metric: tree.realized.negRealizedLoss.cumulative,
          name,
          color,
          unit: Unit.usd,
        }),
      ),
    },
  ];
}

/**
 * Create grouped realized Net P&L cumulative chart
 * @template {{ color: Color, name: string, tree: { realized: AnyRealizedPattern } }} T
 * @param {readonly T[]} list
 * @param {(metric: string) => string} title
 * @returns {PartialChartOption}
 */
function groupedRealizedNetPnlCumulativeChart(list, title) {
  return {
    name: "Net",
    title: title("Cumulative Net Realized P&L"),
    bottom: list.map(({ color, name, tree }) =>
      baseline({
        metric: tree.realized.netRealizedPnl.cumulative,
        name,
        color,
        unit: Unit.usd,
      }),
    ),
  };
}

/**
 * Create grouped sent in P/L sum tree
 * @template {{ color: Color, name: string, tree: { realized: AnyRealizedPattern } }} T
 * @param {readonly T[]} list
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsTree}
 */
function groupedSentInPnlSumTree(list, title) {
  return [
    {
      name: "In Profit",
      title: title("Sent In Profit"),
      bottom: [
        ...list.flatMap(({ color, name, tree }) =>
          satsBtcUsd({
            pattern: tree.realized.sentInProfit14dEma,
            name: `${name} 14d EMA`,
            color,
            defaultActive: false,
          }),
        ),
        ...list.flatMap(({ color, name, tree }) =>
          satsBtcUsdFrom({
            source: tree.realized.sentInProfit,
            key: "sum",
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
        ...list.flatMap(({ color, name, tree }) =>
          satsBtcUsd({
            pattern: tree.realized.sentInLoss14dEma,
            name: `${name} 14d EMA`,
            color,
            defaultActive: false,
          }),
        ),
        ...list.flatMap(({ color, name, tree }) =>
          satsBtcUsdFrom({
            source: tree.realized.sentInLoss,
            key: "sum",
            name,
            color,
          }),
        ),
      ],
    },
  ];
}

/**
 * Create grouped sent in P/L cumulative tree
 * @template {{ color: Color, name: string, tree: { realized: AnyRealizedPattern } }} T
 * @param {readonly T[]} list
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsTree}
 */
function groupedSentInPnlCumulativeTree(list, title) {
  return [
    {
      name: "In Profit",
      title: title("Cumulative Sent In Profit"),
      bottom: list.flatMap(({ color, name, tree }) =>
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
      bottom: list.flatMap(({ color, name, tree }) =>
        satsBtcUsdFrom({
          source: tree.realized.sentInLoss,
          key: "cumulative",
          name,
          color,
        }),
      ),
    },
  ];
}

/**
 * Create grouped realized subfolder
 * @template {{ color: Color, name: string, tree: { realized: AnyRealizedPattern } }} T
 * @param {readonly T[]} list
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsGroup}
 */
function groupedRealizedSubfolder(list, title) {
  return {
    name: "Realized",
    tree: [
      {
        name: "P&L",
        tree: groupedRealizedPnlSumCharts(list, title),
      },
      groupedRealizedNetPnlSumChart(list, title),
      {
        name: "30d Change",
        title: title("Realized P&L 30d Change"),
        bottom: list.map(({ color, name, tree }) =>
          baseline({
            metric: tree.realized.netRealizedPnlCumulative30dDelta,
            name,
            color,
            unit: Unit.usd,
          }),
        ),
      },
      {
        name: "Cumulative",
        tree: [
          {
            name: "P&L",
            tree: groupedRealizedPnlCumulativeCharts(list, title),
          },
          groupedRealizedNetPnlCumulativeChart(list, title),
        ],
      },
    ],
  };
}

/**
 * Create grouped realized subfolder with extras (has P/L Ratio)
 * @template {{ color: Color, name: string, tree: { realized: RealizedWithExtras } }} T
 * @param {readonly T[]} list
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsGroup}
 */
function groupedRealizedSubfolderWithExtras(list, title) {
  return {
    name: "Realized",
    tree: [
      {
        name: "P&L",
        tree: groupedRealizedPnlSumChartsWithExtras(list, title),
      },
      groupedRealizedNetPnlSumChart(list, title),
      {
        name: "30d Change",
        title: title("Realized P&L 30d Change"),
        bottom: list.map(({ color, name, tree }) =>
          baseline({
            metric: tree.realized.netRealizedPnlCumulative30dDelta,
            name,
            color,
            unit: Unit.usd,
          }),
        ),
      },
      {
        name: "Cumulative",
        tree: [
          {
            name: "P&L",
            tree: groupedRealizedPnlCumulativeCharts(list, title),
          },
          groupedRealizedNetPnlCumulativeChart(list, title),
        ],
      },
    ],
  };
}

/**
 * Create grouped volume subfolder
 * @template {{ color: Color, name: string, tree: { realized: AnyRealizedPattern } }} T
 * @param {readonly T[]} list
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsGroup}
 */
function groupedVolumeSubfolder(list, title) {
  return {
    name: "Volume",
    tree: [
      {
        name: "Sum",
        tree: groupedSentInPnlSumTree(list, title),
      },
      {
        name: "Cumulative",
        tree: groupedSentInPnlCumulativeTree(list, title),
      },
    ],
  };
}

/**
 * Create grouped sentiment folder
 * @template {{ color: Color, name: string, tree: { unrealized: UnrealizedPattern } }} T
 * @param {readonly T[]} list
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsGroup}
 */
function groupedSentimentFolder(list, title) {
  return {
    name: "Sentiment",
    tree: [
      {
        name: "Net",
        title: title("Net Sentiment"),
        bottom: list.map(({ color, name, tree }) =>
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
        bottom: list.map(({ color, name, tree }) =>
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
        bottom: list.map(({ color, name, tree }) =>
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

// ============================================================================
// Grouped Cohort Section Builders
// ============================================================================

/**
 * Create grouped profitability section (basic)
 * @template {readonly (UtxoCohortObject | CohortWithoutRelative)[]} T
 * @param {{ list: T, title: (metric: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createGroupedProfitabilitySection({ list, title }) {
  return {
    name: "Profitability",
    tree: [
      {
        name: "Unrealized",
        tree: groupedPnlCharts(list, title),
      },
      groupedRealizedSubfolder(list, title),
      groupedVolumeSubfolder(list, title),
      {
        name: "Invested Capital",
        tree: groupedInvestedCapitalAbsoluteCharts(list, title),
      },
      groupedSentimentFolder(list, title),
    ],
  };
}

/**
 * Create grouped profitability section with invested capital pct only (for basic cohorts)
 * @param {{ list: readonly CohortBasicWithoutMarketCap[], title: (metric: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createGroupedProfitabilitySectionBasicWithInvestedCapitalPct({
  list,
  title,
}) {
  return {
    name: "Profitability",
    tree: [
      {
        name: "Unrealized",
        tree: groupedPnlCharts(list, title),
      },
      groupedRealizedSubfolder(list, title),
      groupedVolumeSubfolder(list, title),
      {
        name: "Invested Capital",
        tree: groupedInvestedCapitalCharts(list, title),
      },
      groupedSentimentFolder(list, title),
    ],
  };
}

/**
 * Create grouped profitability section with invested capital pct (for ageRange cohorts)
 * Has unrealized P&L % of Own Market Cap
 * @param {{ list: readonly CohortAgeRange[], title: (metric: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createGroupedProfitabilitySectionWithInvestedCapitalPct({
  list,
  title,
}) {
  return {
    name: "Profitability",
    tree: [
      {
        name: "Unrealized",
        tree: [
          ...groupedPnlChartsWithOwnMarketCap(list, title),
          {
            name: "Peak Regret",
            title: title("Unrealized Peak Regret"),
            bottom: list.map(({ color, name, tree }) =>
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
      groupedRealizedSubfolderWithExtras(list, title),
      groupedVolumeSubfolder(list, title),
      {
        name: "Invested Capital",
        tree: groupedInvestedCapitalCharts(list, title),
      },
      groupedSentimentFolder(list, title),
    ],
  };
}

/**
 * Create grouped profitability section with NUPL
 * @param {{ list: readonly (CohortFull | CohortBasicWithMarketCap)[], title: (metric: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createGroupedProfitabilitySectionWithNupl({ list, title }) {
  return {
    name: "Profitability",
    tree: [
      {
        name: "Unrealized",
        tree: [
          ...groupedPnlChartsWithMarketCap(list, title),
          {
            name: "NUPL",
            title: title("NUPL"),
            bottom: list.map(({ color, name, tree }) =>
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
      groupedRealizedSubfolder(list, title),
      groupedVolumeSubfolder(list, title),
      {
        name: "Invested Capital",
        tree: groupedInvestedCapitalCharts(list, title),
      },
      groupedSentimentFolder(list, title),
    ],
  };
}

/**
 * Create grouped P&L charts with % of Own Market Cap for LongTerm cohorts
 * @param {readonly CohortLongTerm[]} list
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsTree}
 */
function groupedPnlChartsLongTerm(list, title) {
  return [
    {
      name: "Profit",
      title: title("Unrealized Profit"),
      bottom: [
        // USD
        ...list.map(({ color, name, tree }) =>
          line({
            metric: tree.unrealized.unrealizedProfit,
            name,
            color,
            unit: Unit.usd,
          }),
        ),
        // % of Own Market Cap
        ...list.map(({ color, name, tree }) =>
          line({
            metric: tree.relative.unrealizedProfitRelToOwnMarketCap,
            name,
            color,
            unit: Unit.pctOwnMcap,
          }),
        ),
        // % of Own P&L
        ...list.map(({ color, name, tree }) =>
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
        // USD
        ...list.map(({ color, name, tree }) =>
          line({
            metric: tree.unrealized.negUnrealizedLoss,
            name,
            color,
            unit: Unit.usd,
          }),
        ),
        // % of Market Cap
        ...list.map(({ color, name, tree }) =>
          line({
            metric: tree.relative.unrealizedLossRelToMarketCap,
            name,
            color,
            unit: Unit.pctMcap,
          }),
        ),
        // % of Own Market Cap
        ...list.map(({ color, name, tree }) =>
          line({
            metric: tree.relative.negUnrealizedLossRelToOwnMarketCap,
            name,
            color,
            unit: Unit.pctOwnMcap,
          }),
        ),
        // % of Own P&L
        ...list.map(({ color, name, tree }) =>
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
        // USD
        ...list.map(({ color, name, tree }) =>
          baseline({
            metric: tree.unrealized.netUnrealizedPnl,
            name,
            color,
            unit: Unit.usd,
          }),
        ),
        // % of Own Market Cap
        ...list.map(({ color, name, tree }) =>
          baseline({
            metric: tree.relative.netUnrealizedPnlRelToOwnMarketCap,
            name,
            color,
            unit: Unit.pctOwnMcap,
          }),
        ),
        // % of Own P&L
        ...list.map(({ color, name, tree }) =>
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
 * Create grouped profitability section for LongTerm cohorts (has own market cap + NUPL + peak regret + P/L ratio)
 * @param {{ list: readonly CohortLongTerm[], title: (metric: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createGroupedProfitabilitySectionLongTerm({ list, title }) {
  return {
    name: "Profitability",
    tree: [
      {
        name: "Unrealized",
        tree: [
          ...groupedPnlChartsLongTerm(list, title),
          {
            name: "NUPL",
            title: title("NUPL"),
            bottom: list.map(({ color, name, tree }) =>
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
              // USD
              ...list.map(({ color, name, tree }) =>
                line({
                  metric: tree.unrealized.peakRegret,
                  name,
                  color,
                  unit: Unit.usd,
                }),
              ),
              // % of Market Cap
              ...list.map(({ color, name, tree }) =>
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
      groupedRealizedSubfolderWithExtras(list, title),
      groupedVolumeSubfolder(list, title),
      {
        name: "Invested Capital",
        tree: groupedInvestedCapitalCharts(list, title),
      },
      groupedSentimentFolder(list, title),
    ],
  };
}

/**
 * Create grouped profitability section with Peak Regret + NUPL (for minAge cohorts)
 * @param {{ list: readonly CohortMinAge[], title: (metric: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createGroupedProfitabilitySectionWithPeakRegret({
  list,
  title,
}) {
  return {
    name: "Profitability",
    tree: [
      {
        name: "Unrealized",
        tree: [
          ...groupedPnlChartsWithMarketCap(list, title),
          {
            name: "NUPL",
            title: title("NUPL"),
            bottom: list.map(({ color, name, tree }) =>
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
              // USD
              ...list.map(({ color, name, tree }) =>
                line({
                  metric: tree.unrealized.peakRegret,
                  name,
                  color,
                  unit: Unit.usd,
                }),
              ),
              // % of Market Cap
              ...list.map(({ color, name, tree }) =>
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
      groupedRealizedSubfolder(list, title),
      groupedVolumeSubfolder(list, title),
      {
        name: "Invested Capital",
        tree: groupedInvestedCapitalCharts(list, title),
      },
      groupedSentimentFolder(list, title),
    ],
  };
}
