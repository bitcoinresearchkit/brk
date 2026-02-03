/**
 * Address cohort folder builder
 * Creates option trees for address-based cohorts (has addrCount)
 * Address cohorts use _0satsPattern which has CostBasisPattern (no percentiles)
 */

import { colors } from "../../utils/colors.js";
import { Unit } from "../../utils/units.js";
import { priceLine } from "../constants.js";
import { line, baseline, price } from "../series.js";
import { formatCohortTitle, satsBtcUsd } from "../shared.js";
import {
  createSingleSupplySeries,
  createGroupedSupplySection,
  createUtxoCountSeries,
  createAddressCountSeries,
  createRealizedPriceSeries,
  createRealizedPriceRatioSeries,
  createSingleCoinsDestroyedSeries,
  createGroupedCoinblocksDestroyedSeries,
  createGroupedCoindaysDestroyedSeries,
  createSingleSentSeries,
  groupedSupplyRelativeGenerators,
  createSingleSupplyRelativeOptions,
  createSingleSellSideRiskSeries,
  createSingleValueCreatedDestroyedSeries,
  createSingleValueFlowBreakdownSeries,
  createSingleCapitulationProfitFlowSeries,
  createSingleSoprSeries,
  createSingleInvestorPriceSeries,
  createSingleInvestorPriceRatioSeries,
  createInvestorPriceSeries,
  createInvestorPriceRatioSeries,
} from "./shared.js";

/**
 * Create a cohort folder for address cohorts
 * Includes address count section (addrCount exists on AddressCohortObject)
 * @param {AddressCohortObject | AddressCohortGroupObject} args
 * @returns {PartialOptionsGroup}
 */
export function createAddressCohortFolder(args) {
  const list = "list" in args ? args.list : [args];
  const useGroupName = "list" in args;
  const isSingle = !("list" in args);

  const title = formatCohortTitle(args.name);

  return {
    name: args.name || "all",
    tree: [
      // Supply section
      isSingle
        ? {
            name: "Supply",
            title: title("Supply"),
            bottom: createSingleSupplySeries(
              /** @type {AddressCohortObject} */ (args),
              createSingleSupplyRelativeOptions(
                /** @type {AddressCohortObject} */ (args),
              ),
            ),
          }
        : createGroupedSupplySection(
            list,
            title,
            groupedSupplyRelativeGenerators,
          ),

      // UTXO count
      {
        name: "UTXO Count",
        title: title("UTXO Count"),
        bottom: createUtxoCountSeries(list, useGroupName),
      },

      // Address count (ADDRESS COHORTS ONLY - fully type safe!)
      {
        name: "Address Count",
        title: title("Address Count"),
        bottom: createAddressCountSeries(list, useGroupName),
      },

      // Realized section
      {
        name: "Realized",
        tree: [
          ...(useGroupName
            ? [
                {
                  name: "Price",
                  title: title("Realized Price"),
                  top: createRealizedPriceSeries(list),
                },
                {
                  name: "Ratio",
                  title: title("Realized Price Ratio"),
                  bottom: createRealizedPriceRatioSeries(list),
                },
                {
                  name: "Investor Price",
                  tree: [
                    {
                      name: "Price",
                      title: title("Investor Price"),
                      top: createInvestorPriceSeries(list),
                    },
                    {
                      name: "Ratio",
                      title: title("Investor Price Ratio"),
                      bottom: createInvestorPriceRatioSeries(list),
                    },
                  ],
                },
              ]
            : createRealizedPriceOptions(
                /** @type {AddressCohortObject} */ (args),
                title,
              )),
          {
            name: "Capitalization",
            title: title("Realized Cap"),
            bottom: createRealizedCapWithExtras(list, args, useGroupName),
          },
          {
            name: "Value",
            title: title("Realized Value"),
            bottom: list.map(({ color, name, tree }) =>
              line({
                metric: tree.realized.realizedValue,
                name: useGroupName ? name : "Realized Value",
                color,
                unit: Unit.usd,
              }),
            ),
          },
          ...(useGroupName
            ? createGroupedRealizedPnlSection(list, title)
            : createRealizedPnlSection(
                /** @type {AddressCohortObject} */ (args),
                title,
              )),
        ],
      },

      // Unrealized section
      ...createUnrealizedSection(list, useGroupName, title),

      // Cost basis section (no percentiles for address cohorts)
      ...createCostBasisSection(list, useGroupName, title),

      // Activity section
      ...createActivitySection(args, title),
    ],
  };
}

/**
 * Create realized price options for single cohort
 * @param {AddressCohortObject} args
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsTree}
 */
function createRealizedPriceOptions(args, title) {
  const { tree, color } = args;

  return [
    {
      name: "Price",
      title: title("Realized Price"),
      top: [
        price({
          metric: tree.realized.realizedPrice,
          name: "Realized",
          color,
        }),
      ],
    },
    {
      name: "Investor Price",
      tree: [
        {
          name: "Price",
          title: title("Investor Price"),
          top: createSingleInvestorPriceSeries(tree, color),
        },
        {
          name: "Ratio",
          title: title("Investor Price Ratio"),
          bottom: createSingleInvestorPriceRatioSeries(tree, color),
        },
      ],
    },
  ];
}

/**
 * Create realized cap with extras
 * @param {readonly AddressCohortObject[]} list
 * @param {AddressCohortObject | AddressCohortGroupObject} args
 * @param {boolean} useGroupName
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function createRealizedCapWithExtras(list, args, useGroupName) {
  const isSingle = !("list" in args);

  return list.flatMap(({ color, name, tree }) => [
    line({
      metric: tree.realized.realizedCap,
      name: useGroupName ? name : "Capitalization",
      color,
      unit: Unit.usd,
    }),
    ...(isSingle
      ? [
          baseline({
            metric: tree.realized.realizedCap30dDelta,
            name: "30d Change",
            unit: Unit.usd,
            defaultActive: false,
          }),
        ]
      : []),
    // RealizedPattern (address cohorts) doesn't have realizedCapRelToOwnMarketCap
  ]);
}

/**
 * Create realized PnL section for single cohort
 * @param {AddressCohortObject} args
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsTree}
 */
function createRealizedPnlSection(args, title) {
  const { realized } = args.tree;

  return [
    {
      name: "P&L",
      title: title("Realized P&L"),
      bottom: [
        line({
          metric: realized.realizedProfit.sum,
          name: "Profit",
          color: colors.profit,
          unit: Unit.usd,
        }),
        line({
          metric: realized.realizedProfit7dEma,
          name: "Profit 7d EMA",
          color: colors.profit,
          unit: Unit.usd,
        }),
        line({
          metric: realized.realizedProfit.cumulative,
          name: "Profit Cumulative",
          color: colors.profit,
          unit: Unit.usd,
          defaultActive: false,
        }),
        line({
          metric: realized.realizedLoss.sum,
          name: "Loss",
          color: colors.loss,
          unit: Unit.usd,
        }),
        line({
          metric: realized.realizedLoss7dEma,
          name: "Loss 7d EMA",
          color: colors.loss,
          unit: Unit.usd,
        }),
        line({
          metric: realized.realizedLoss.cumulative,
          name: "Loss Cumulative",
          color: colors.loss,
          unit: Unit.usd,
          defaultActive: false,
        }),
        line({
          metric: realized.negRealizedLoss.sum,
          name: "Negative Loss",
          color: colors.loss,
          unit: Unit.usd,
          defaultActive: false,
        }),
        line({
          metric: realized.negRealizedLoss.cumulative,
          name: "Negative Loss Cumulative",
          color: colors.loss,
          unit: Unit.usd,
          defaultActive: false,
        }),
        line({
          metric: realized.totalRealizedPnl,
          name: "Total",
          color: colors.default,
          unit: Unit.usd,
          defaultActive: false,
        }),
        baseline({
          metric: realized.realizedProfitRelToRealizedCap.sum,
          name: "Profit",
          color: colors.profit,
          unit: Unit.pctRcap,
        }),
        baseline({
          metric: realized.realizedProfitRelToRealizedCap.cumulative,
          name: "Profit Cumulative",
          color: colors.profit,
          unit: Unit.pctRcap,
          defaultActive: false,
        }),
        baseline({
          metric: realized.realizedLossRelToRealizedCap.sum,
          name: "Loss",
          color: colors.loss,
          unit: Unit.pctRcap,
        }),
        baseline({
          metric: realized.realizedLossRelToRealizedCap.cumulative,
          name: "Loss Cumulative",
          color: colors.loss,
          unit: Unit.pctRcap,
          defaultActive: false,
        }),
      ],
    },
    {
      name: "Net pnl",
      title: title("Net Realized P&L"),
      bottom: [
        baseline({
          metric: realized.netRealizedPnl.sum,
          name: "Net",
          unit: Unit.usd,
        }),
        baseline({
          metric: realized.netRealizedPnl7dEma,
          name: "Net 7d EMA",
          unit: Unit.usd,
        }),
        baseline({
          metric: realized.netRealizedPnl.cumulative,
          name: "Net Cumulative",
          unit: Unit.usd,
          defaultActive: false,
        }),
        baseline({
          metric: realized.netRealizedPnlCumulative30dDelta,
          name: "Cumulative 30d Change",
          unit: Unit.usd,
          defaultActive: false,
        }),
        baseline({
          metric: realized.netRealizedPnlRelToRealizedCap.sum,
          name: "Net",
          unit: Unit.pctRcap,
        }),
        baseline({
          metric: realized.netRealizedPnlRelToRealizedCap.cumulative,
          name: "Net Cumulative",
          unit: Unit.pctRcap,
          defaultActive: false,
        }),
        baseline({
          metric: realized.netRealizedPnlCumulative30dDeltaRelToRealizedCap,
          name: "Cumulative 30d Change",
          unit: Unit.pctRcap,
          defaultActive: false,
        }),
        baseline({
          metric: realized.netRealizedPnlCumulative30dDeltaRelToMarketCap,
          name: "Cumulative 30d Change",
          unit: Unit.pctMcap,
        }),
        priceLine({
          unit: Unit.usd,
          number: 1,
        }),
        priceLine({
          unit: Unit.pctMcap,
        }),
        priceLine({
          unit: Unit.pctRcap,
        }),
      ],
    },
    {
      name: "SOPR",
      title: title("SOPR"),
      bottom: [
        ...createSingleSoprSeries(args.tree),
        priceLine({
          unit: Unit.ratio,
          number: 1,
        }),
      ],
    },
    {
      name: "Sell Side Risk",
      title: title("Sell Side Risk Ratio"),
      bottom: createSingleSellSideRiskSeries(args.tree),
    },
    {
      name: "Value",
      tree: [
        {
          name: "Created & Destroyed",
          title: title("Value Created & Destroyed"),
          bottom: createSingleValueCreatedDestroyedSeries(args.tree),
        },
        {
          name: "Breakdown",
          title: title("Value Flow Breakdown"),
          bottom: createSingleValueFlowBreakdownSeries(args.tree),
        },
        {
          name: "Flow",
          title: title("Capitulation & Profit Flow"),
          bottom: createSingleCapitulationProfitFlowSeries(args.tree),
        },
      ],
    },
    {
      name: "Peak Regret",
      title: title("Peak Regret"),
      bottom: [
        line({
          metric: realized.peakRegret.sum,
          name: "Sum",
          color: colors.loss,
          unit: Unit.usd,
        }),
        line({
          metric: realized.peakRegret.cumulative,
          name: "Cumulative",
          color: colors.loss,
          unit: Unit.usd,
          defaultActive: false,
        }),
        baseline({
          metric: realized.peakRegretRelToRealizedCap,
          name: "Rel. to Realized Cap",
          color: colors.realized,
          unit: Unit.pctRcap,
        }),
      ],
    },
    {
      name: "Sent In P/L",
      tree: [
        {
          name: "In Profit",
          title: title("Sent In Profit"),
          bottom: [
            line({
              metric: realized.sentInProfit.bitcoin.sum,
              name: "Sum",
              color: colors.profit,
              unit: Unit.btc,
            }),
            line({
              metric: realized.sentInProfit.bitcoin.cumulative,
              name: "Cumulative",
              color: colors.profit,
              unit: Unit.btc,
              defaultActive: false,
            }),
            line({
              metric: realized.sentInProfit.sats.sum,
              name: "Sum",
              color: colors.profit,
              unit: Unit.sats,
            }),
            line({
              metric: realized.sentInProfit.sats.cumulative,
              name: "Cumulative",
              color: colors.profit,
              unit: Unit.sats,
              defaultActive: false,
            }),
            line({
              metric: realized.sentInProfit.dollars.sum,
              name: "Sum",
              color: colors.profit,
              unit: Unit.usd,
            }),
            line({
              metric: realized.sentInProfit.dollars.cumulative,
              name: "Cumulative",
              color: colors.profit,
              unit: Unit.usd,
              defaultActive: false,
            }),
          ],
        },
        {
          name: "In Loss",
          title: title("Sent In Loss"),
          bottom: [
            line({
              metric: realized.sentInLoss.bitcoin.sum,
              name: "Sum",
              color: colors.loss,
              unit: Unit.btc,
            }),
            line({
              metric: realized.sentInLoss.bitcoin.cumulative,
              name: "Cumulative",
              color: colors.loss,
              unit: Unit.btc,
              defaultActive: false,
            }),
            line({
              metric: realized.sentInLoss.sats.sum,
              name: "Sum",
              color: colors.loss,
              unit: Unit.sats,
            }),
            line({
              metric: realized.sentInLoss.sats.cumulative,
              name: "Cumulative",
              color: colors.loss,
              unit: Unit.sats,
              defaultActive: false,
            }),
            line({
              metric: realized.sentInLoss.dollars.sum,
              name: "Sum",
              color: colors.loss,
              unit: Unit.usd,
            }),
            line({
              metric: realized.sentInLoss.dollars.cumulative,
              name: "Cumulative",
              color: colors.loss,
              unit: Unit.usd,
              defaultActive: false,
            }),
          ],
        },
        {
          name: "In Profit 14d EMA",
          title: title("Sent In Profit 14d EMA"),
          bottom: satsBtcUsd({
            pattern: realized.sentInProfit14dEma,
            name: "14d EMA",
            color: colors.profit,
          }),
        },
        {
          name: "In Loss 14d EMA",
          title: title("Sent In Loss 14d EMA"),
          bottom: satsBtcUsd({
            pattern: realized.sentInLoss14dEma,
            name: "14d EMA",
            color: colors.loss,
          }),
        },
      ],
    },
  ];
}

/**
 * Create grouped realized P&L section for address cohorts (for compare view)
 * @param {readonly AddressCohortObject[]} list
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsTree}
 */
function createGroupedRealizedPnlSection(list, title) {
  const pnlConfigs = /** @type {const} */ ([
    {
      name: "Profit",
      sum: "realizedProfit",
      ema: "realizedProfit7dEma",
      rel: "realizedProfitRelToRealizedCap",
      isNet: false,
    },
    {
      name: "Loss",
      sum: "realizedLoss",
      ema: "realizedLoss7dEma",
      rel: "realizedLossRelToRealizedCap",
      isNet: false,
    },
    {
      name: "Net P&L",
      sum: "netRealizedPnl",
      ema: "netRealizedPnl7dEma",
      rel: "netRealizedPnlRelToRealizedCap",
      isNet: true,
    },
  ]);

  return [
    ...pnlConfigs.map(({ name, sum, ema, rel, isNet }) => ({
      name,
      tree: [
        {
          name: "Sum",
          title: title(`Realized ${name}`),
          bottom: [
            ...list.flatMap(({ color, name, tree }) => [
              (isNet ? baseline : line)({
                metric: tree.realized[sum].sum,
                name,
                color,
                unit: Unit.usd,
              }),
              baseline({
                metric: tree.realized[rel].sum,
                name,
                color,
                unit: Unit.pctRcap,
              }),
            ]),
          ],
        },
        {
          name: "7d EMA",
          title: title(`Realized ${name} 7d EMA`),
          bottom: [
            ...list.map(({ color, name, tree }) =>
              (isNet ? baseline : line)({
                metric: tree.realized[ema],
                name,
                color,
                unit: Unit.usd,
              }),
            ),
          ],
        },
      ],
    })),
    {
      name: "Peak Regret",
      tree: [
        {
          name: "Sum",
          title: title("Peak Regret"),
          bottom: list.flatMap(({ color, name, tree }) => [
            line({
              metric: tree.realized.peakRegret.sum,
              name,
              color,
              unit: Unit.usd,
            }),
          ]),
        },
        {
          name: "Cumulative",
          title: title("Peak Regret Cumulative"),
          bottom: list.flatMap(({ color, name, tree }) => [
            line({
              metric: tree.realized.peakRegret.cumulative,
              name,
              color,
              unit: Unit.usd,
            }),
          ]),
        },
        {
          name: "Rel. to Realized Cap",
          title: title("Peak Regret Rel. to Realized Cap"),
          bottom: list.flatMap(({ color, name, tree }) => [
            baseline({
              metric: tree.realized.peakRegretRelToRealizedCap,
              name,
              color,
              unit: Unit.pctRcap,
            }),
          ]),
        },
      ],
    },
    {
      name: "Sent In P/L",
      tree: [
        {
          name: "In Profit",
          title: title("Sent In Profit"),
          bottom: list.flatMap(({ color, name, tree }) => [
            line({
              metric: tree.realized.sentInProfit.bitcoin.sum,
              name,
              color,
              unit: Unit.btc,
            }),
            line({
              metric: tree.realized.sentInProfit.sats.sum,
              name,
              color,
              unit: Unit.sats,
            }),
            line({
              metric: tree.realized.sentInProfit.dollars.sum,
              name,
              color,
              unit: Unit.usd,
            }),
          ]),
        },
        {
          name: "In Profit Cumulative",
          title: title("Sent In Profit Cumulative"),
          bottom: list.flatMap(({ color, name, tree }) => [
            line({
              metric: tree.realized.sentInProfit.bitcoin.cumulative,
              name,
              color,
              unit: Unit.btc,
            }),
            line({
              metric: tree.realized.sentInProfit.sats.cumulative,
              name,
              color,
              unit: Unit.sats,
            }),
            line({
              metric: tree.realized.sentInProfit.dollars.cumulative,
              name,
              color,
              unit: Unit.usd,
            }),
          ]),
        },
        {
          name: "In Loss",
          title: title("Sent In Loss"),
          bottom: list.flatMap(({ color, name, tree }) => [
            line({
              metric: tree.realized.sentInLoss.bitcoin.sum,
              name,
              color,
              unit: Unit.btc,
            }),
            line({
              metric: tree.realized.sentInLoss.sats.sum,
              name,
              color,
              unit: Unit.sats,
            }),
            line({
              metric: tree.realized.sentInLoss.dollars.sum,
              name,
              color,
              unit: Unit.usd,
            }),
          ]),
        },
        {
          name: "In Loss Cumulative",
          title: title("Sent In Loss Cumulative"),
          bottom: list.flatMap(({ color, name, tree }) => [
            line({
              metric: tree.realized.sentInLoss.bitcoin.cumulative,
              name,
              color,
              unit: Unit.btc,
            }),
            line({
              metric: tree.realized.sentInLoss.sats.cumulative,
              name,
              color,
              unit: Unit.sats,
            }),
            line({
              metric: tree.realized.sentInLoss.dollars.cumulative,
              name,
              color,
              unit: Unit.usd,
            }),
          ]),
        },
        {
          name: "In Profit 14d EMA",
          title: title("Sent In Profit 14d EMA"),
          bottom: list.flatMap(({ color, name, tree }) =>
            satsBtcUsd({
              pattern: tree.realized.sentInProfit14dEma,
              name,
              color,
            }),
          ),
        },
        {
          name: "In Loss 14d EMA",
          title: title("Sent In Loss 14d EMA"),
          bottom: list.flatMap(({ color, name, tree }) =>
            satsBtcUsd({
              pattern: tree.realized.sentInLoss14dEma,
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
 * Create unrealized section
 * @param {readonly AddressCohortObject[]} list
 * @param {boolean} useGroupName
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsTree}
 */
function createUnrealizedSection(list, useGroupName, title) {
  return [
    {
      name: "Unrealized",
      tree: [
        {
          name: "Profit",
          title: title("Unrealized Profit"),
          bottom: list.flatMap(({ color, name, tree }) => [
            line({
              metric: tree.unrealized.unrealizedProfit,
              name: useGroupName ? name : "Profit",
              color: useGroupName ? color : colors.profit,
              unit: Unit.usd,
            }),
          ]),
        },
        {
          name: "Loss",
          title: title("Unrealized Loss"),
          bottom: list.flatMap(({ color, name, tree }) => [
            line({
              metric: tree.unrealized.unrealizedLoss,
              name: useGroupName ? name : "Loss",
              color: useGroupName ? color : colors.loss,
              unit: Unit.usd,
            }),
          ]),
        },
        {
          name: "Total P&L",
          title: title("Total Unrealized P&L"),
          bottom: list.flatMap(({ color, name, tree }) => [
            baseline({
              metric: tree.unrealized.totalUnrealizedPnl,
              name: useGroupName ? name : "Total",
              color: useGroupName ? color : undefined,
              unit: Unit.usd,
            }),
          ]),
        },
        {
          name: "Negative Loss",
          title: title("Negative Unrealized Loss"),
          bottom: list.flatMap(({ color, name, tree }) => [
            line({
              metric: tree.unrealized.negUnrealizedLoss,
              name: useGroupName ? name : "Negative Loss",
              color: useGroupName ? color : colors.loss,
              unit: Unit.usd,
            }),
          ]),
        },
        {
          name: "Invested Capital",
          tree: [
            {
              name: "In Profit",
              title: title("Invested Capital In Profit"),
              bottom: list.flatMap(({ color, name, tree }) => [
                line({
                  metric: tree.unrealized.investedCapitalInProfit,
                  name: useGroupName ? name : "In Profit",
                  color: useGroupName ? color : colors.profit,
                  unit: Unit.usd,
                }),
              ]),
            },
            {
              name: "In Loss",
              title: title("Invested Capital In Loss"),
              bottom: list.flatMap(({ color, name, tree }) => [
                line({
                  metric: tree.unrealized.investedCapitalInLoss,
                  name: useGroupName ? name : "In Loss",
                  color: useGroupName ? color : colors.loss,
                  unit: Unit.usd,
                }),
              ]),
            },
          ],
        },
        {
          name: "Relative",
          tree: [
            {
              name: "NUPL",
              title: title("NUPL (Rel to Market Cap)"),
              bottom: list.flatMap(({ color, name, tree }) => [
                baseline({
                  metric: tree.relative.nupl,
                  name: useGroupName ? name : "NUPL",
                  color: useGroupName ? color : undefined,
                  unit: Unit.ratio,
                  options: { baseValue: { price: 0 } },
                }),
              ]),
            },
            {
              name: "Profit",
              title: title("Unrealized Profit (% of Market Cap)"),
              bottom: list.flatMap(({ color, name, tree }) => [
                line({
                  metric: tree.relative.unrealizedProfitRelToMarketCap,
                  name: useGroupName ? name : "Profit",
                  color: useGroupName ? color : colors.profit,
                  unit: Unit.pctMcap,
                }),
              ]),
            },
            {
              name: "Loss",
              title: title("Unrealized Loss (% of Market Cap)"),
              bottom: list.flatMap(({ color, name, tree }) => [
                line({
                  metric: tree.relative.unrealizedLossRelToMarketCap,
                  name: useGroupName ? name : "Loss",
                  color: useGroupName ? color : colors.loss,
                  unit: Unit.pctMcap,
                }),
              ]),
            },
            {
              name: "Net P&L",
              title: title("Net Unrealized P&L (% of Market Cap)"),
              bottom: list.flatMap(({ color, name, tree }) => [
                baseline({
                  metric: tree.relative.netUnrealizedPnlRelToMarketCap,
                  name: useGroupName ? name : "Net",
                  color: useGroupName ? color : undefined,
                  unit: Unit.pctMcap,
                }),
              ]),
            },
            {
              name: "Negative Loss",
              title: title("Negative Unrealized Loss (% of Market Cap)"),
              bottom: list.flatMap(({ color, name, tree }) => [
                line({
                  metric: tree.relative.negUnrealizedLossRelToMarketCap,
                  name: useGroupName ? name : "Negative Loss",
                  color: useGroupName ? color : colors.loss,
                  unit: Unit.pctMcap,
                }),
              ]),
            },
            {
              name: "Invested Capital In Profit",
              title: title("Invested Capital In Profit"),
              bottom: list.flatMap(({ color, name, tree }) => [
                baseline({
                  metric: tree.relative.investedCapitalInProfitPct,
                  name: useGroupName ? name : "In Profit",
                  color: useGroupName ? color : colors.profit,
                  unit: Unit.pctRcap,
                }),
              ]),
            },
            {
              name: "Invested Capital In Loss",
              title: title("Invested Capital In Loss"),
              bottom: list.flatMap(({ color, name, tree }) => [
                baseline({
                  metric: tree.relative.investedCapitalInLossPct,
                  name: useGroupName ? name : "In Loss",
                  color: useGroupName ? color : colors.loss,
                  unit: Unit.pctRcap,
                }),
              ]),
            },
          ],
        },
        {
          name: "NUPL",
          title: title("Net Unrealized P&L"),
          bottom: list.flatMap(({ color, name, tree }) => [
            baseline({
              metric: tree.unrealized.netUnrealizedPnl,
              name: useGroupName ? name : "NUPL",
              color: useGroupName ? color : undefined,
              unit: Unit.ratio,
            }),
            priceLine({
              unit: Unit.ratio,
            }),
          ]),
        },
        {
          name: "Net Sentiment",
          title: title("Net Sentiment"),
          bottom: list.flatMap(({ color, name, tree }) => [
            baseline({
              metric: tree.unrealized.netSentiment,
              name: useGroupName ? name : "Net Sentiment",
              color: useGroupName ? color : undefined,
              unit: Unit.usd,
            }),
            priceLine({
              unit: Unit.usd,
            }),
          ]),
        },
      ],
    },
  ];
}

/**
 * Create cost basis section (no percentiles for address cohorts)
 * @param {readonly AddressCohortObject[]} list
 * @param {boolean} useGroupName
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsTree}
 */
function createCostBasisSection(list, useGroupName, title) {
  return [
    {
      name: "Cost Basis",
      tree: [
        {
          name: "Min",
          title: title("Min Cost Basis"),
          top: list.map(({ color, name, tree }) =>
            price({
              metric: tree.costBasis.min,
              name: useGroupName ? name : "Min",
              color,
            }),
          ),
        },
        {
          name: "Max",
          title: title("Max Cost Basis"),
          top: list.map(({ color, name, tree }) =>
            price({
              metric: tree.costBasis.max,
              name: useGroupName ? name : "Max",
              color,
            }),
          ),
        },
      ],
    },
  ];
}

/**
 * Create activity section
 * @param {AddressCohortObject | AddressCohortGroupObject} args
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsTree}
 */
function createActivitySection(args, title) {
  const list = "list" in args ? args.list : [args];
  const isSingle = !("list" in args);

  // Single cohort: all metrics on one chart
  if (isSingle) {
    const cohort = /** @type {AddressCohortObject} */ (args);
    return [
      {
        name: "Activity",
        tree: [
          {
            name: "Coins Destroyed",
            title: title("Coins Destroyed"),
            bottom: createSingleCoinsDestroyedSeries(cohort),
          },
          {
            name: "Sent",
            title: title("Sent"),
            bottom: createSingleSentSeries(cohort),
          },
        ],
      },
    ];
  }

  // Grouped cohorts: split charts for comparison
  return [
    {
      name: "Activity",
      tree: [
        {
          name: "Coinblocks Destroyed",
          title: title("Coinblocks Destroyed"),
          bottom: createGroupedCoinblocksDestroyedSeries(list),
        },
        {
          name: "Coindays Destroyed",
          title: title("Coindays Destroyed"),
          bottom: createGroupedCoindaysDestroyedSeries(list),
        },
        {
          name: "Sent",
          tree: [
            {
              name: "Sum",
              title: title("Sent"),
              bottom: list.flatMap(({ color, name, tree }) =>
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
            {
              name: "14d EMA",
              title: title("Sent 14d EMA"),
              bottom: list.flatMap(({ color, name, tree }) =>
                satsBtcUsd({ pattern: tree.activity.sent14dEma, name, color }),
              ),
            },
          ],
        },
      ],
    },
  ];
}
