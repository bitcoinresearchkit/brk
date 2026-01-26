/**
 * Address cohort folder builder
 * Creates option trees for address-based cohorts (has addrCount)
 * Address cohorts use _0satsPattern which has CostBasisPattern (no percentiles)
 */

import { Unit } from "../../utils/units.js";
import { priceLine } from "../constants.js";
import { line, baseline, price } from "../series.js";
import { formatCohortTitle } from "../shared.js";
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
  createGroupedSentSatsSeries,
  createGroupedSentBitcoinSeries,
  createGroupedSentDollarsSeries,
  groupedSupplyRelativeGenerators,
  createSingleSupplyRelativeOptions,
  createSingleSellSideRiskSeries,
  createSingleValueCreatedDestroyedSeries,
  createSingleSoprSeries,
} from "./shared.js";

/**
 * Create a cohort folder for address cohorts
 * Includes address count section (addrCount exists on AddressCohortObject)
 * @param {PartialContext} ctx
 * @param {AddressCohortObject | AddressCohortGroupObject} args
 * @returns {PartialOptionsGroup}
 */
export function createAddressCohortFolder(ctx, args) {
  const list = "list" in args ? args.list : [args];
  const useGroupName = "list" in args;
  const isSingle = !("list" in args);

  const title = formatCohortTitle(args.title);

  return {
    name: args.name || "all",
    tree: [
      // Supply section
      isSingle
        ? {
            name: "Supply",
            title: title("Supply"),
            bottom: createSingleSupplySeries(
              ctx,
              /** @type {AddressCohortObject} */ (args),
              createSingleSupplyRelativeOptions(
                ctx,
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
        bottom: createAddressCountSeries(ctx, list, useGroupName),
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
              ]
            : createRealizedPriceOptions(
                /** @type {AddressCohortObject} */ (args),
                title,
              )),
          {
            name: "Capitalization",
            title: title("Realized Cap"),
            bottom: createRealizedCapWithExtras(ctx, list, args, useGroupName),
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
          ...(!useGroupName
            ? createRealizedPnlSection(
                ctx,
                /** @type {AddressCohortObject} */ (args),
                title,
              )
            : []),
        ],
      },

      // Unrealized section
      ...createUnrealizedSection(ctx, list, useGroupName, title),

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
  ];
}

/**
 * Create realized cap with extras
 * @param {PartialContext} ctx
 * @param {readonly AddressCohortObject[]} list
 * @param {AddressCohortObject | AddressCohortGroupObject} args
 * @param {boolean} useGroupName
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function createRealizedCapWithExtras(ctx, list, args, useGroupName) {
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
          priceLine({ ctx, unit: Unit.usd, defaultActive: false }),
        ]
      : []),
    // RealizedPattern (address cohorts) doesn't have realizedCapRelToOwnMarketCap
  ]);
}

/**
 * Create realized PnL section for single cohort
 * @param {PartialContext} ctx
 * @param {AddressCohortObject} args
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsTree}
 */
function createRealizedPnlSection(ctx, args, title) {
  const { colors } = ctx;
  const { realized } = args.tree;

  return [
    {
      name: "P&L",
      title: title("Realized P&L"),
      bottom: [
        line({
          metric: realized.realizedProfit.sum,
          name: "Profit",
          color: colors.green,
          unit: Unit.usd,
        }),
        line({
          metric: realized.realizedProfit.cumulative,
          name: "Profit Cumulative",
          color: colors.green,
          unit: Unit.usd,
          defaultActive: false,
        }),
        line({
          metric: realized.realizedLoss.sum,
          name: "Loss",
          color: colors.red,
          unit: Unit.usd,
        }),
        line({
          metric: realized.realizedLoss.cumulative,
          name: "Loss Cumulative",
          color: colors.red,
          unit: Unit.usd,
          defaultActive: false,
        }),
        line({
          metric: realized.negRealizedLoss.sum,
          name: "Negative Loss",
          color: colors.red,
          unit: Unit.usd,
          defaultActive: false,
        }),
        line({
          metric: realized.negRealizedLoss.cumulative,
          name: "Negative Loss Cumulative",
          color: colors.red,
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
          color: colors.green,
          unit: Unit.pctRcap,
        }),
        baseline({
          metric: realized.realizedProfitRelToRealizedCap.cumulative,
          name: "Profit Cumulative",
          color: colors.green,
          unit: Unit.pctRcap,
          defaultActive: false,
        }),
        baseline({
          metric: realized.realizedLossRelToRealizedCap.sum,
          name: "Loss",
          color: colors.red,
          unit: Unit.pctRcap,
        }),
        baseline({
          metric: realized.realizedLossRelToRealizedCap.cumulative,
          name: "Loss Cumulative",
          color: colors.red,
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
          ctx,
          unit: Unit.usd,
          number: 1,
        }),
        priceLine({
          ctx,
          unit: Unit.pctMcap,
        }),
        priceLine({
          ctx,
          unit: Unit.pctRcap,
        }),
      ],
    },
    {
      name: "SOPR",
      title: title("SOPR"),
      bottom: [
        ...createSingleSoprSeries(colors, args.tree),
        priceLine({
          ctx,
          unit: Unit.ratio,
          number: 1,
        }),
      ],
    },
    {
      name: "Sell Side Risk",
      title: title("Sell Side Risk Ratio"),
      bottom: createSingleSellSideRiskSeries(colors, args.tree),
    },
    {
      name: "Value",
      title: title("Value Created & Destroyed"),
      bottom: createSingleValueCreatedDestroyedSeries(colors, args.tree),
    },
  ];
}

/**
 * Create unrealized section
 * @param {PartialContext} ctx
 * @param {readonly AddressCohortObject[]} list
 * @param {boolean} useGroupName
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsTree}
 */
function createUnrealizedSection(ctx, list, useGroupName, title) {
  const { colors } = ctx;

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
              color: useGroupName ? color : colors.green,
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
              color: useGroupName ? color : colors.red,
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
              color: useGroupName ? color : colors.red,
              unit: Unit.usd,
            }),
          ]),
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
                  color: useGroupName ? color : colors.green,
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
                  color: useGroupName ? color : colors.red,
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
                  color: useGroupName ? color : colors.red,
                  unit: Unit.pctMcap,
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
              ctx,
              unit: Unit.ratio,
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
              name: "Sats",
              title: title("Sent (Sats)"),
              bottom: createGroupedSentSatsSeries(list),
            },
            {
              name: "Bitcoin",
              title: title("Sent (BTC)"),
              bottom: createGroupedSentBitcoinSeries(list),
            },
            {
              name: "Dollars",
              title: title("Sent ($)"),
              bottom: createGroupedSentDollarsSeries(list),
            },
          ],
        },
      ],
    },
  ];
}
