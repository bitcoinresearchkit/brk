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
import { ROLLING_WINDOWS, line, baseline, dots, dotsBaseline, percentRatio, percentRatioBaseline } from "../series.js";
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
 * @typedef {Object} PnlSeriesConfig
 * @property {AnySeriesPattern} profit
 * @property {AnySeriesPattern} loss
 * @property {AnySeriesPattern} negLoss
 * @property {AnySeriesPattern} [gross]
 */

/**
 * @param {PnlSeriesConfig} m
 * @param {Unit} unit
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function pnlLines(m, unit) {
  const series = [
    line({ series: m.profit, name: "Profit", color: colors.profit, unit }),
    line({ series: m.loss, name: "Loss", color: colors.loss, unit }),
  ];
  if (m.gross) {
    series.push(line({ series: m.gross, name: "Total", color: colors.default, unit }));
  }
  series.push(line({ series: m.negLoss, name: "Negative Loss", color: colors.loss, unit, defaultActive: false }));
  return series;
}

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
 * @param {{ profit: { base: { usd: AnySeriesPattern } }, loss: { base: { usd: AnySeriesPattern }, negative: AnySeriesPattern }, grossPnl: { usd: AnySeriesPattern } }} u
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function unrealizedUsdSeries(u) {
  return [
    ...pnlLines(
      { profit: u.profit.base.usd, loss: u.loss.base.usd, negLoss: u.loss.negative, gross: u.grossPnl.usd },
      Unit.usd,
    ),
    priceLine({ unit: Unit.usd, defaultActive: false }),
  ];
}

/**
 * @param {{ percent: AnySeriesPattern, ratio: AnySeriesPattern }} profit
 * @param {{ percent: AnySeriesPattern, ratio: AnySeriesPattern }} loss
 * @param {string} name
 * @param {(name: string) => string} title
 * @returns {AnyPartialOption}
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
 * Unrealized P&L tree for All cohort
 * @param {Brk.SeriesTree_Cohorts_Utxo_All_Unrealized} u
 * @param {(name: string) => string} title
 * @returns {PartialOptionsTree}
 */
function unrealizedPnlTreeAll(u, title) {
  return [
    { name: "USD", title: title("Unrealized P&L"), bottom: unrealizedUsdSeries(u) },
    relPnlChart(u.profit.relToMcap, u.loss.relToMcap, "% of Mcap", title),
    relPnlChart(u.profit.relToOwnGross, u.loss.relToOwnGross, "% of Own P&L", title),
    ...unrealizedCumulativeRollingTree(u.profit, u.loss, title),
  ];
}

/**
 * Unrealized P&L tree for Full cohorts (STH)
 * @param {Brk.GrossInvestedLossNetNuplProfitSentimentPattern2} u
 * @param {(name: string) => string} title
 * @returns {PartialOptionsTree}
 */
function unrealizedPnlTreeFull(u, title) {
  return [
    { name: "USD", title: title("Unrealized P&L"), bottom: unrealizedUsdSeries(u) },
    relPnlChart(u.profit.relToMcap, u.loss.relToMcap, "% of Mcap", title),
    relPnlChart(u.profit.relToOwnMcap, u.loss.relToOwnMcap, "% of Own Mcap", title),
    relPnlChart(u.profit.relToOwnGross, u.loss.relToOwnGross, "% of Own P&L", title),
    ...unrealizedCumulativeRollingTree(u.profit, u.loss, title),
  ];
}

/**
 * Unrealized P&L tree for LTH (loss relToMcap only)
 * @param {Brk.GrossInvestedLossNetNuplProfitSentimentPattern2} u
 * @param {(name: string) => string} title
 * @returns {PartialOptionsTree}
 */
function unrealizedPnlTreeLongTerm(u, title) {
  return [
    { name: "USD", title: title("Unrealized P&L"), bottom: unrealizedUsdSeries(u) },
    {
      name: "% of Mcap",
      title: title("Unrealized Loss (% of Mcap)"),
      bottom: percentRatio({ pattern: u.loss.relToMcap, name: "Loss", color: colors.loss }),
    },
    relPnlChart(u.profit.relToOwnMcap, u.loss.relToOwnMcap, "% of Own Mcap", title),
    relPnlChart(u.profit.relToOwnGross, u.loss.relToOwnGross, "% of Own P&L", title),
    ...unrealizedCumulativeRollingTree(u.profit, u.loss, title),
  ];
}

/**
 * Unrealized P&L tree for mid-tier cohorts (AgeRange/MaxAge)
 * @param {Brk.LossNetNuplProfitPattern} u
 * @param {(name: string) => string} title
 * @returns {PartialOptionsTree}
 */
function unrealizedPnlTreeMid(u, title) {
  return [
    {
      name: "USD",
      title: title("Unrealized P&L"),
      bottom: [
        ...pnlLines(
          { profit: u.profit.base.usd, loss: u.loss.base.usd, negLoss: u.loss.negative },
          Unit.usd,
        ),
        priceLine({ unit: Unit.usd, defaultActive: false }),
      ],
    },
    ...unrealizedCumulativeRollingTree(u.profit, u.loss, title),
  ];
}

/**
 * Unrealized cumulative + rolling P&L tree (profit and loss have cumulative.usd + sum[w].usd)
 * @param {{ cumulative: { usd: AnySeriesPattern }, sum: { _24h: { usd: AnySeriesPattern }, _1w: { usd: AnySeriesPattern }, _1m: { usd: AnySeriesPattern }, _1y: { usd: AnySeriesPattern } } }} profit
 * @param {{ cumulative: { usd: AnySeriesPattern }, sum: { _24h: { usd: AnySeriesPattern }, _1w: { usd: AnySeriesPattern }, _1m: { usd: AnySeriesPattern }, _1y: { usd: AnySeriesPattern } } }} loss
 * @param {(name: string) => string} title
 * @returns {PartialOptionsTree}
 */
function unrealizedCumulativeRollingTree(profit, loss, title) {
  return [
    {
      name: "Cumulative",
      title: title("Cumulative Unrealized P&L"),
      bottom: [
        line({ series: profit.cumulative.usd, name: "Profit", color: colors.profit, unit: Unit.usd }),
        line({ series: loss.cumulative.usd, name: "Loss", color: colors.loss, unit: Unit.usd }),
      ],
    },
    {
      name: "Rolling",
      tree: [
        {
          name: "Profit",
          tree: [
            {
              name: "Compare",
              title: title("Rolling Unrealized Profit"),
              bottom: ROLLING_WINDOWS.map((w) =>
                line({ series: profit.sum[w.key].usd, name: w.name, color: w.color, unit: Unit.usd }),
              ),
            },
            ...ROLLING_WINDOWS.map((w) => ({
              name: w.name,
              title: title(`Unrealized Profit (${w.name})`),
              bottom: [line({ series: profit.sum[w.key].usd, name: "Profit", color: colors.profit, unit: Unit.usd })],
            })),
          ],
        },
        {
          name: "Loss",
          tree: [
            {
              name: "Compare",
              title: title("Rolling Unrealized Loss"),
              bottom: ROLLING_WINDOWS.map((w) =>
                line({ series: loss.sum[w.key].usd, name: w.name, color: w.color, unit: Unit.usd }),
              ),
            },
            ...ROLLING_WINDOWS.map((w) => ({
              name: w.name,
              title: title(`Unrealized Loss (${w.name})`),
              bottom: [line({ series: loss.sum[w.key].usd, name: "Loss", color: colors.loss, unit: Unit.usd })],
            })),
          ],
        },
      ],
    },
  ];
}

// ============================================================================
// Net Unrealized P&L Builders
// ============================================================================

/**
 * @param {Brk.SeriesTree_Cohorts_Utxo_All_Unrealized} u
 * @param {(name: string) => string} title
 * @returns {PartialOptionsTree}
 */
function netUnrealizedTreeAll(u, title) {
  return [
    { name: "USD", title: title("Net Unrealized P&L"), bottom: [netBaseline(u.netPnl.usd, Unit.usd)] },
    {
      name: "% of Own P&L",
      title: title("Net Unrealized P&L (% of Own P&L)"),
      bottom: percentRatioBaseline({ pattern: u.netPnl.relToOwnGross, name: "Net P&L" }),
    },
  ];
}

/**
 * @param {Brk.GrossInvestedLossNetNuplProfitSentimentPattern2} u
 * @param {(name: string) => string} title
 * @returns {PartialOptionsTree}
 */
function netUnrealizedTreeFull(u, title) {
  return [
    { name: "USD", title: title("Net Unrealized P&L"), bottom: [netBaseline(u.netPnl.usd, Unit.usd)] },
    {
      name: "% of Own Mcap",
      title: title("Net Unrealized P&L (% of Own Mcap)"),
      bottom: percentRatioBaseline({ pattern: u.netPnl.relToOwnMcap, name: "Net P&L" }),
    },
    {
      name: "% of Own P&L",
      title: title("Net Unrealized P&L (% of Own P&L)"),
      bottom: percentRatioBaseline({ pattern: u.netPnl.relToOwnGross, name: "Net P&L" }),
    },
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
 * @param {Brk.GrossInvestedLossNetNuplProfitSentimentPattern2 | Brk.SeriesTree_Cohorts_Utxo_All_Unrealized} u
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
 * @param {Brk.GrossInvestedLossNetNuplProfitSentimentPattern2 | Brk.SeriesTree_Cohorts_Utxo_All_Unrealized} u
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
 * @param {Brk.BpsRatioPattern} nupl
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function nuplSeries(nupl) {
  return [baseline({ series: nupl.ratio, name: "NUPL", unit: Unit.ratio })];
}

// ============================================================================
// Realized P&L Builders — Full (All/STH/LTH)
// ============================================================================

/**
 * @param {Brk.CapGrossInvestorLossMvrvNetPeakPriceProfitSellSoprPattern | Brk.SeriesTree_Cohorts_Utxo_Lth_Realized} r
 * @param {(name: string) => string} title
 * @returns {PartialOptionsTree}
 */
function realizedPnlSumTreeFull(r, title) {
  return [
    {
      name: "USD",
      title: title("Realized P&L"),
      bottom: [
        dots({ series: r.profit.base.usd, name: "Profit", color: colors.profit, unit: Unit.usd }),
        dots({ series: r.loss.negative, name: "Negative Loss", color: colors.loss, unit: Unit.usd, defaultActive: false }),
        dots({ series: r.loss.base.usd, name: "Loss", color: colors.loss, unit: Unit.usd, defaultActive: false }),
      ],
    },
    {
      name: "% of Rcap",
      title: title("Realized P&L (% of Realized Cap)"),
      bottom: [
        ...percentRatioBaseline({ pattern: r.profit.relToRcap, name: "Profit", color: colors.profit }),
        ...percentRatioBaseline({ pattern: r.loss.relToRcap, name: "Loss", color: colors.loss }),
      ],
    },
  ];
}

/**
 * @param {Brk.CapGrossInvestorLossMvrvNetPeakPriceProfitSellSoprPattern | Brk.SeriesTree_Cohorts_Utxo_Lth_Realized} r
 * @param {(name: string) => string} title
 * @returns {PartialOptionsTree}
 */
function realizedNetPnlSumTreeFull(r, title) {
  return [
    { name: "USD", title: title("Net Realized P&L"), bottom: [dotsBaseline({ series: r.netPnl.base.usd, name: "Net", unit: Unit.usd })] },
    {
      name: "% of Rcap",
      title: title("Net Realized P&L (% of Realized Cap)"),
      bottom: percentRatioBaseline({ pattern: r.netPnl.relToRcap, name: "Net" }),
    },
  ];
}

/**
 * @param {Brk.CapGrossInvestorLossMvrvNetPeakPriceProfitSellSoprPattern | Brk.SeriesTree_Cohorts_Utxo_Lth_Realized} r
 * @param {(name: string) => string} title
 * @returns {PartialOptionsTree}
 */
function realizedPnlCumulativeTreeFull(r, title) {
  return [
    {
      name: "USD",
      title: title("Cumulative Realized P&L"),
      bottom: [
        line({ series: r.profit.cumulative.usd, name: "Profit", color: colors.profit, unit: Unit.usd }),
        line({ series: r.loss.cumulative.usd, name: "Loss", color: colors.loss, unit: Unit.usd }),
        line({ series: r.loss.negative, name: "Negative Loss", color: colors.loss, unit: Unit.usd, defaultActive: false }),
      ],
    },
    {
      name: "% of Rcap",
      title: title("Cumulative Realized P&L (% of Realized Cap)"),
      bottom: [
        ...percentRatioBaseline({ pattern: r.profit.relToRcap, name: "Profit", color: colors.profit }),
        ...percentRatioBaseline({ pattern: r.loss.relToRcap, name: "Loss", color: colors.loss }),
      ],
    },
  ];
}

/**
 * Net realized P&L delta tree (absolute + rate across all rolling windows)
 * @param {Brk.BaseChangeCumulativeDeltaRelSumPattern | Brk.BaseCumulativeDeltaSumPattern} netPnl
 * @param {(name: string) => string} title
 * @returns {PartialOptionsGroup}
 */
function realizedNetPnlDeltaTree(netPnl, title) {
  return {
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
            title: title(`Net Realized P&L Change (${w.name})`),
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
            title: title(`Net Realized P&L Rate (${w.name})`),
            bottom: percentRatioBaseline({ pattern: netPnl.delta.rate[w.key], name: "Rate" }),
          })),
        ],
      },
    ],
  };
}

/**
 * Full realized delta tree (absolute + rate + rel to mcap/rcap)
 * @param {Brk.CapGrossInvestorLossMvrvNetPeakPriceProfitSellSoprPattern | Brk.SeriesTree_Cohorts_Utxo_Lth_Realized} r
 * @param {(name: string) => string} title
 * @returns {PartialOptionsGroup}
 */
function realizedNetPnlDeltaTreeFull(r, title) {
  const base = realizedNetPnlDeltaTree(r.netPnl, title);
  return {
    ...base,
    tree: [
      ...base.tree,
      {
        name: "% of Mcap",
        title: title("Net Realized P&L Change (% of Mcap)"),
        bottom: percentRatioBaseline({ pattern: r.netPnl.change1m.relToMcap, name: "30d Change" }),
      },
      {
        name: "% of Rcap",
        title: title("Net Realized P&L Change (% of Rcap)"),
        bottom: percentRatioBaseline({ pattern: r.netPnl.change1m.relToRcap, name: "30d Change" }),
      },
    ],
  };
}

/**
 * Rolling net realized P&L tree (reusable by full and mid realized)
 * @param {{ sum: { _24h: { usd: AnySeriesPattern }, _1w: { usd: AnySeriesPattern }, _1m: { usd: AnySeriesPattern }, _1y: { usd: AnySeriesPattern } } }} netPnl
 * @param {(name: string) => string} title
 * @returns {PartialOptionsGroup}
 */
function rollingNetRealizedTree(netPnl, title) {
  return {
    name: "Net",
    tree: [
      {
        name: "Compare",
        title: title("Rolling Net Realized P&L"),
        bottom: ROLLING_WINDOWS.map((w) =>
          baseline({ series: netPnl.sum[w.key].usd, name: w.name, color: w.color, unit: Unit.usd }),
        ),
      },
      ...ROLLING_WINDOWS.map((w) => ({
        name: w.name,
        title: title(`Net Realized P&L (${w.name})`),
        bottom: [baseline({ series: netPnl.sum[w.key].usd, name: "Net", unit: Unit.usd })],
      })),
    ],
  };
}

/**
 * Rolling realized with P/L and ratio (full realized only)
 * @param {Brk.CapGrossInvestorLossMvrvNetPeakPriceProfitSellSoprPattern | Brk.SeriesTree_Cohorts_Utxo_Lth_Realized} r
 * @param {(name: string) => string} title
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
          bottom: ROLLING_WINDOWS.map((w) =>
            line({ series: r.profit.sum[w.key].usd, name: w.name, color: w.color, unit: Unit.usd }),
          ),
        },
        ...ROLLING_WINDOWS.map((w) => ({
          name: w.name,
          title: title(`Realized Profit (${w.name})`),
          bottom: [line({ series: r.profit.sum[w.key].usd, name: "Profit", color: colors.profit, unit: Unit.usd })],
        })),
      ],
    },
    {
      name: "Loss",
      tree: [
        {
          name: "Compare",
          title: title("Rolling Realized Loss"),
          bottom: ROLLING_WINDOWS.map((w) =>
            line({ series: r.loss.sum[w.key].usd, name: w.name, color: w.color, unit: Unit.usd }),
          ),
        },
        ...ROLLING_WINDOWS.map((w) => ({
          name: w.name,
          title: title(`Realized Loss (${w.name})`),
          bottom: [line({ series: r.loss.sum[w.key].usd, name: "Loss", color: colors.loss, unit: Unit.usd })],
        })),
      ],
    },
    rollingNetRealizedTree(r.netPnl, title),
    {
      name: "P/L Ratio",
      tree: [
        {
          name: "Compare",
          title: title("Rolling Realized P/L Ratio"),
          bottom: ROLLING_WINDOWS.map((w) =>
            baseline({ series: r.profitToLossRatio[w.key], name: w.name, color: w.color, unit: Unit.ratio, base: 1 }),
          ),
        },
        ...ROLLING_WINDOWS.map((w) => ({
          name: w.name,
          title: title(`Realized P/L Ratio (${w.name})`),
          bottom: [baseline({ series: r.profitToLossRatio[w.key], name: "P/L Ratio", unit: Unit.ratio, base: 1 })],
        })),
      ],
    },
  ];
}

/**
 * Rolling realized profit/loss sums (basic — no P/L ratio)
 * @param {Brk.BaseCumulativeSumPattern3} profit
 * @param {Brk.BaseCumulativeSumPattern3} loss
 * @param {(name: string) => string} title
 * @returns {PartialOptionsTree}
 */
function singleRollingRealizedTreeBasic(profit, loss, title) {
  return [
    {
      name: "Profit",
      tree: ROLLING_WINDOWS.map((w) => ({
        name: w.name,
        title: title(`Realized Profit (${w.name})`),
        bottom: [line({ series: profit.sum[w.key].usd, name: "Profit", color: colors.profit, unit: Unit.usd })],
      })),
    },
    {
      name: "Loss",
      tree: ROLLING_WINDOWS.map((w) => ({
        name: w.name,
        title: title(`Realized Loss (${w.name})`),
        bottom: [line({ series: loss.sum[w.key].usd, name: "Loss", color: colors.loss, unit: Unit.usd })],
      })),
    },
  ];
}

// ============================================================================
// Realized Subfolder Builders
// ============================================================================

/**
 * Value Created/Destroyed tree for a single P&L side (profit or loss)
 * @param {CountPattern<number>} valueCreated
 * @param {CountPattern<number>} valueDestroyed
 * @param {string} label - "Profit" or "Loss"
 * @param {(name: string) => string} title
 * @returns {PartialOptionsGroup}
 */
function realizedValueTree(valueCreated, valueDestroyed, label, title) {
  return {
    name: label,
    tree: [
      {
        name: "Rolling",
        tree: [
          {
            name: "Compare",
            title: title(`${label} Value Created vs Destroyed`),
            bottom: ROLLING_WINDOWS.flatMap((w) => [
              line({ series: valueCreated.sum[w.key], name: `Created (${w.name})`, color: w.color, unit: Unit.usd }),
              line({ series: valueDestroyed.sum[w.key], name: `Destroyed (${w.name})`, color: w.color, unit: Unit.usd, style: 2 }),
            ]),
          },
          ...ROLLING_WINDOWS.map((w) => ({
            name: w.name,
            title: title(`${label} Value (${w.name})`),
            bottom: [
              line({ series: valueCreated.sum[w.key], name: "Created", color: colors.profit, unit: Unit.usd }),
              line({ series: valueDestroyed.sum[w.key], name: "Destroyed", color: colors.loss, unit: Unit.usd }),
            ],
          })),
        ],
      },
      {
        name: "Cumulative",
        title: title(`Cumulative ${label} Value`),
        bottom: [
          line({ series: valueCreated.cumulative, name: "Created", color: colors.profit, unit: Unit.usd }),
          line({ series: valueDestroyed.cumulative, name: "Destroyed", color: colors.loss, unit: Unit.usd }),
        ],
      },
    ],
  };
}

/**
 * Investor price percentiles tree (pct1/2/5/95/98/99)
 * @param {InvestorPercentilesPattern} percentiles
 * @param {(name: string) => string} title
 * @returns {PartialOptionsGroup}
 */
function investorPricePercentilesTree(percentiles, title) {
  /** @type {readonly [InvestorPercentileEntry, string, Color][]} */
  const pcts = [
    [percentiles.pct99, "p99", colors.stat.max],
    [percentiles.pct98, "p98", colors.stat.pct90],
    [percentiles.pct95, "p95", colors.stat.pct75],
    [percentiles.pct5, "p5", colors.stat.pct25],
    [percentiles.pct2, "p2", colors.stat.pct10],
    [percentiles.pct1, "p1", colors.stat.min],
  ];

  return {
    name: "Percentiles",
    tree: [
      {
        name: "USD",
        title: title("Investor Price Percentiles"),
        bottom: pcts.map(([p, name, color]) =>
          line({ series: p.price.usd, name, color, unit: Unit.usd }),
        ),
      },
      {
        name: "Sats",
        title: title("Investor Price Percentiles (Sats)"),
        bottom: pcts.map(([p, name, color]) =>
          line({ series: p.price.sats, name, color, unit: Unit.sats }),
        ),
      },
      {
        name: "Ratio",
        title: title("Investor Price Percentile Ratios"),
        bottom: pcts.map(([p, name, color]) =>
          baseline({ series: p.ratio, name, color, unit: Unit.ratio }),
        ),
      },
    ],
  };
}

/**
 * Full realized subfolder (All/STH/LTH)
 * @param {Brk.CapGrossInvestorLossMvrvNetPeakPriceProfitSellSoprPattern | Brk.SeriesTree_Cohorts_Utxo_Lth_Realized} r
 * @param {(name: string) => string} title
 * @returns {PartialOptionsGroup}
 */
function realizedSubfolderFull(r, title) {
  return {
    name: "Realized",
    tree: [
      { name: "P&L", tree: realizedPnlSumTreeFull(r, title) },
      { name: "Net", tree: realizedNetPnlSumTreeFull(r, title) },
      realizedNetPnlDeltaTreeFull(r, title),
      {
        name: "Gross P&L",
        tree: [
          { name: "Base", title: title("Gross Realized P&L"), bottom: [dots({ series: r.grossPnl.base.usd, name: "Gross P&L", color: colors.bitcoin, unit: Unit.usd })] },
          {
            name: "Rolling",
            tree: [
              {
                name: "Compare",
                title: title("Rolling Gross Realized P&L"),
                bottom: ROLLING_WINDOWS.map((w) =>
                  line({ series: r.grossPnl.sum[w.key].usd, name: w.name, color: w.color, unit: Unit.usd }),
                ),
              },
              ...ROLLING_WINDOWS.map((w) => ({
                name: w.name,
                title: title(`Gross Realized P&L (${w.name})`),
                bottom: [line({ series: r.grossPnl.sum[w.key].usd, name: "Gross P&L", color: colors.bitcoin, unit: Unit.usd })],
              })),
            ],
          },
          { name: "Cumulative", title: title("Total Realized P&L"), bottom: [line({ series: r.grossPnl.cumulative.usd, name: "Total", unit: Unit.usd, color: colors.bitcoin })] },
        ],
      },
      {
        name: "Value",
        tree: [
          realizedValueTree(r.profit.valueCreated, r.profit.valueDestroyed, "Profit", title),
          realizedValueTree(r.loss.valueCreated, r.loss.valueDestroyed, "Loss", title),
        ],
      },
      {
        name: "P/L Ratio",
        title: title("Realized Profit/Loss Ratio"),
        bottom: [baseline({ series: r.profitToLossRatio._1y, name: "P/L Ratio", unit: Unit.ratio, base: 1 })],
      },
      {
        name: "Peak Regret",
        title: title("Realized Peak Regret"),
        bottom: [line({ series: r.peakRegret.base, name: "Peak Regret", unit: Unit.usd })],
      },
      {
        name: "Investor Price",
        tree: [
          investorPricePercentilesTree(r.investor.price.percentiles, title),
        ],
      },
      { name: "Rolling", tree: singleRollingRealizedTreeFull(r, title) },
      {
        name: "Cumulative",
        tree: [
          { name: "P&L", tree: realizedPnlCumulativeTreeFull(r, title) },
          {
            name: "Net",
            tree: [
              { name: "USD", title: title("Cumulative Net Realized P&L"), bottom: [baseline({ series: r.netPnl.cumulative.usd, name: "Net", unit: Unit.usd })] },
              {
                name: "% of Rcap",
                title: title("Cumulative Net P&L (% of Realized Cap)"),
                bottom: percentRatioBaseline({ pattern: r.netPnl.relToRcap, name: "Net" }),
              },
            ],
          },
          {
            name: "Peak Regret",
            tree: [
              { name: "USD", title: title("Cumulative Peak Regret"), bottom: [line({ series: r.peakRegret.cumulative, name: "Peak Regret", unit: Unit.usd })] },
              {
                name: "% of Rcap",
                title: title("Cumulative Peak Regret (% of Realized Cap)"),
                bottom: percentRatioBaseline({ pattern: r.peakRegret.relToRcap, name: "Peak Regret" }),
              },
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
 * @param {(name: string) => string} title
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
          dots({ series: r.profit.base.usd, name: "Profit", color: colors.profit, unit: Unit.usd }),
          dots({ series: r.loss.negative, name: "Negative Loss", color: colors.loss, unit: Unit.usd, defaultActive: false }),
          dots({ series: r.loss.base.usd, name: "Loss", color: colors.loss, unit: Unit.usd, defaultActive: false }),
        ],
      },
      {
        name: "Net",
        title: title("Net Realized P&L"),
        bottom: [dotsBaseline({ series: r.netPnl.base.usd, name: "Net", unit: Unit.usd })],
      },
      realizedNetPnlDeltaTree(r.netPnl, title),
      {
        name: "Rolling",
        tree: [
          ...singleRollingRealizedTreeBasic(r.profit, r.loss, title),
          rollingNetRealizedTree(r.netPnl, title),
        ],
      },
      {
        name: "Cumulative",
        tree: [
          {
            name: "P&L",
            title: title("Cumulative Realized P&L"),
            bottom: [
              line({ series: r.profit.cumulative.usd, name: "Profit", color: colors.profit, unit: Unit.usd }),
              line({ series: r.loss.cumulative.usd, name: "Loss", color: colors.loss, unit: Unit.usd }),
            ],
          },
          {
            name: "Net",
            title: title("Cumulative Net Realized P&L"),
            bottom: [baseline({ series: r.netPnl.cumulative.usd, name: "Net", unit: Unit.usd })],
          },
        ],
      },
    ],
  };
}

/**
 * Basic realized subfolder (no netPnl, no relToRcap)
 * @param {Brk.CapLossMvrvPriceProfitSoprPattern} r
 * @param {(name: string) => string} title
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
          dots({ series: r.profit.base.usd, name: "Profit", color: colors.profit, unit: Unit.usd }),
          dots({ series: r.loss.base.usd, name: "Loss", color: colors.loss, unit: Unit.usd, defaultActive: false }),
        ],
      },
      { name: "Rolling", tree: singleRollingRealizedTreeBasic(r.profit, r.loss, title) },
      {
        name: "Cumulative",
        title: title("Cumulative Realized P&L"),
        bottom: [
          line({ series: r.profit.cumulative.usd, name: "Profit", color: colors.profit, unit: Unit.usd }),
          line({ series: r.loss.cumulative.usd, name: "Loss", color: colors.loss, unit: Unit.usd }),
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
          {
            name: "P&L",
            tree: [
              {
                name: "USD",
                title: title("Unrealized P&L"),
                bottom: [
                  ...pnlLines({ profit: u.profit.base.usd, loss: u.loss.base.usd, negLoss: u.loss.negative }, Unit.usd),
                  priceLine({ unit: Unit.usd, defaultActive: false }),
                ],
              },
              ...unrealizedCumulativeRollingTree(u.profit, u.loss, title),
            ],
          },
          { name: "NUPL", title: title("NUPL"), bottom: nuplSeries(u.nupl) },
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
      {
        name: "Unrealized",
        tree: [
          { name: "P&L", tree: unrealizedPnlTreeAll(u, title) },
          { name: "Net P&L", tree: netUnrealizedTreeAll(u, title) },
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
 * @param {{ cohort: CohortFull, title: (name: string) => string }} args
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
          { name: "P&L", tree: unrealizedPnlTreeFull(u, title) },
          { name: "Net P&L", tree: netUnrealizedTreeFull(u, title) },
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
      {
        name: "Unrealized",
        tree: [
          { name: "P&L", tree: unrealizedPnlTreeLongTerm(u, title) },
          { name: "Net P&L", tree: netUnrealizedTreeFull(u, title) },
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
 * @param {{ cohort: CohortAgeRange, title: (name: string) => string }} args
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
          { name: "P&L", tree: unrealizedPnlTreeMid(u, title) },
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
 * Section for CohortAddress (has unrealized profit/loss + NUPL, basic realized)
 * @param {{ cohort: CohortAddress, title: (name: string) => string }} args
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
          {
            name: "P&L",
            tree: [
              {
                name: "USD",
                title: title("Unrealized P&L"),
                bottom: [
                  ...pnlLines({ profit: u.profit.base.usd, loss: u.loss.base.usd, negLoss: u.loss.negative }, Unit.usd),
                  priceLine({ unit: Unit.usd, defaultActive: false }),
                ],
              },
              ...unrealizedCumulativeRollingTree(u.profit, u.loss, title),
            ],
          },
          { name: "NUPL", title: title("NUPL"), bottom: nuplSeries(u.nupl) },
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
 * @template {{ name: string, color: Color, tree: { realized: { profit: Brk.BaseCumulativeSumPattern3, loss: Brk.BaseCumulativeSumPattern3 } } }} T
 * @template {{ name: string, color: Color, tree: { realized: { profit: Brk.BaseCumulativeSumPattern3, loss: Brk.BaseCumulativeSumPattern3 } } }} A
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
 * @template {{ name: string, color: Color, tree: { realized: { profit: Brk.BaseCumulativeSumPattern3, loss: Brk.BaseCumulativeSumPattern3 } } }} T
 * @template {{ name: string, color: Color, tree: { realized: { profit: Brk.BaseCumulativeSumPattern3, loss: Brk.BaseCumulativeSumPattern3 } } }} A
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
        title: title(`Realized Profit (${w.name})`),
        bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) => line({ series: tree.realized.profit.sum[w.key].usd, name, color, unit: Unit.usd })),
      })),
    },
    {
      name: "Loss",
      tree: ROLLING_WINDOWS.map((w) => ({
        name: w.name,
        title: title(`Realized Loss (${w.name})`),
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
        title: title(`Realized P/L Ratio (${w.name})`),
        bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
          baseline({ series: tree.realized.profitToLossRatio[w.key], name, color, unit: Unit.ratio, base: 1 }),
        ),
      })),
    },
  ];
}

/**
 * Grouped realized subfolder (basic)
 * @template {{ name: string, color: Color, tree: { realized: { profit: Brk.BaseCumulativeSumPattern3, loss: Brk.BaseCumulativeSumPattern3 } } }} T
 * @template {{ name: string, color: Color, tree: { realized: { profit: Brk.BaseCumulativeSumPattern3, loss: Brk.BaseCumulativeSumPattern3 } } }} A
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
            title: title(`Net Realized P&L Change (${w.name})`),
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
            title: title(`Net Realized P&L Rate (${w.name})`),
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
 * Grouped unrealized P&L (USD only — for all cohorts that at least have nupl)
 * @template {{ name: string, color: Color, tree: { unrealized: { nupl: Brk.BpsRatioPattern } } }} T
 * @template {{ name: string, color: Color, tree: { unrealized: { nupl: Brk.BpsRatioPattern } } }} A
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
 * Grouped unrealized for full cohorts with relToMcap
 * @param {readonly (CohortFull | CohortLongTerm)[]} list
 * @param {CohortAll} all
 * @param {(name: string) => string} title
 * @returns {PartialOptionsTree}
 */
function groupedPnlChartsWithMarketCap(list, all, title) {
  return [
    {
      name: "Profit",
      tree: [
        {
          name: "USD",
          title: title("Unrealized Profit"),
          bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
            line({ series: tree.unrealized.profit.base.usd, name, color, unit: Unit.usd }),
          ),
        },
        {
          name: "% of Mcap",
          title: title("Unrealized Profit (% of Mcap)"),
          bottom: flatMapCohortsWithAll(list, all, ({ name, color, tree }) =>
            percentRatio({ pattern: tree.unrealized.profit.relToMcap, name, color }),
          ),
        },
      ],
    },
    {
      name: "Loss",
      tree: [
        {
          name: "USD",
          title: title("Unrealized Loss"),
          bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
            line({ series: tree.unrealized.loss.base.usd, name, color, unit: Unit.usd }),
          ),
        },
        {
          name: "% of Mcap",
          title: title("Unrealized Loss (% of Mcap)"),
          bottom: flatMapCohortsWithAll(list, all, ({ name, color, tree }) =>
            percentRatio({ pattern: tree.unrealized.loss.relToMcap, name, color }),
          ),
        },
      ],
    },
    {
      name: "Net P&L",
      title: title("Net Unrealized P&L"),
      bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
        baseline({ series: tree.unrealized.netPnl.usd, name, color, unit: Unit.usd }),
      ),
    },
  ];
}

/**
 * Grouped unrealized for AgeRange/MaxAge (profit/loss without relToMcap)
 * @param {readonly (CohortAgeRange | CohortWithAdjusted)[]} list
 * @param {CohortAll} all
 * @param {(name: string) => string} title
 * @returns {PartialOptionsTree}
 */
function groupedPnlChartsWithOwnMarketCap(list, all, title) {
  return [
    {
      name: "Profit",
      title: title("Unrealized Profit"),
      bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
        line({ series: tree.unrealized.profit.base.usd, name, color, unit: Unit.usd }),
      ),
    },
    {
      name: "Loss",
      title: title("Unrealized Loss"),
      bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
        line({ series: tree.unrealized.loss.base.usd, name, color, unit: Unit.usd }),
      ),
    },
    {
      name: "Net P&L",
      title: title("Net Unrealized P&L"),
      bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
        baseline({ series: tree.unrealized.netPnl.usd, name, color, unit: Unit.usd }),
      ),
    },
  ];
}

/**
 * Grouped unrealized for LongTerm (profit/loss with relToOwnMcap + relToOwnGross)
 * @param {readonly CohortLongTerm[]} list
 * @param {CohortAll} all
 * @param {(name: string) => string} title
 * @returns {PartialOptionsTree}
 */
function groupedPnlChartsLongTerm(list, all, title) {
  return [
    {
      name: "Profit",
      tree: [
        {
          name: "USD",
          title: title("Unrealized Profit"),
          bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
            line({ series: tree.unrealized.profit.base.usd, name, color, unit: Unit.usd }),
          ),
        },
        {
          name: "% of Own Mcap",
          title: title("Unrealized Profit (% of Own Mcap)"),
          bottom: flatMapCohorts(list, ({ name, color, tree }) =>
            percentRatio({ pattern: tree.unrealized.profit.relToOwnMcap, name, color }),
          ),
        },
        {
          name: "% of Own P&L",
          title: title("Unrealized Profit (% of Own P&L)"),
          bottom: flatMapCohortsWithAll(list, all, ({ name, color, tree }) =>
            percentRatio({ pattern: tree.unrealized.profit.relToOwnGross, name, color }),
          ),
        },
      ],
    },
    {
      name: "Loss",
      tree: [
        {
          name: "USD",
          title: title("Unrealized Loss"),
          bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
            line({ series: tree.unrealized.loss.base.usd, name, color, unit: Unit.usd }),
          ),
        },
        {
          name: "% of Mcap",
          title: title("Unrealized Loss (% of Mcap)"),
          bottom: flatMapCohortsWithAll(list, all, ({ name, color, tree }) =>
            percentRatio({ pattern: tree.unrealized.loss.relToMcap, name, color }),
          ),
        },
        {
          name: "% of Own Mcap",
          title: title("Unrealized Loss (% of Own Mcap)"),
          bottom: flatMapCohorts(list, ({ name, color, tree }) =>
            percentRatio({ pattern: tree.unrealized.loss.relToOwnMcap, name, color }),
          ),
        },
        {
          name: "% of Own P&L",
          title: title("Unrealized Loss (% of Own P&L)"),
          bottom: flatMapCohortsWithAll(list, all, ({ name, color, tree }) =>
            percentRatio({ pattern: tree.unrealized.loss.relToOwnGross, name, color }),
          ),
        },
      ],
    },
    {
      name: "Net P&L",
      tree: [
        {
          name: "USD",
          title: title("Net Unrealized P&L"),
          bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
            baseline({ series: tree.unrealized.netPnl.usd, name, color, unit: Unit.usd }),
          ),
        },
        {
          name: "% of Own Mcap",
          title: title("Net Unrealized P&L (% of Own Mcap)"),
          bottom: flatMapCohorts(list, ({ name, color, tree }) =>
            percentRatioBaseline({ pattern: tree.unrealized.netPnl.relToOwnMcap, name, color }),
          ),
        },
        {
          name: "% of Own P&L",
          title: title("Net Unrealized P&L (% of Own P&L)"),
          bottom: flatMapCohortsWithAll(list, all, ({ name, color, tree }) =>
            percentRatioBaseline({ pattern: tree.unrealized.netPnl.relToOwnGross, name, color }),
          ),
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
          {
            name: "Profit",
            title: title("Unrealized Profit"),
            bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
              line({ series: tree.unrealized.profit.base.usd, name, color, unit: Unit.usd }),
            ),
          },
          {
            name: "Loss",
            title: title("Unrealized Loss"),
            bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
              line({ series: tree.unrealized.loss.base.usd, name, color, unit: Unit.usd }),
            ),
          },
          ...groupedNuplCharts(list, all, title),
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
 * @param {{ list: readonly (CohortFull | CohortLongTerm)[], all: CohortAll, title: (name: string) => string }} args
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
 * @param {{ list: readonly CohortLongTerm[], all: CohortAll, title: (name: string) => string }} args
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
