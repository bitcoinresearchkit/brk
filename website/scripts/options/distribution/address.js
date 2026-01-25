/**
 * Address cohort folder builder
 * Creates option trees for address-based cohorts (has addrCount)
 * Address cohorts use _0satsPattern which has CostBasisPattern (no percentiles)
 */

import { Unit } from "../../utils/units.js";
import { priceLine } from "../constants.js";
import { line, baseline } from "../series.js";
import {
  createSingleSupplySeries,
  createGroupedSupplyTotalSeries,
  createGroupedSupplyInProfitSeries,
  createGroupedSupplyInLossSeries,
  createUtxoCountSeries,
  createAddressCountSeries,
  createRealizedPriceSeries,
  createRealizedPriceRatioSeries,
  createSingleCoinsDestroyedSeries,
  createGroupedCoinblocksDestroyedSeries,
  createGroupedCoindaysDestroyedSeries,
  createGroupedSatblocksDestroyedSeries,
  createGroupedSatdaysDestroyedSeries,
  createSingleSentSeries,
  createGroupedSentSatsSeries,
  createGroupedSentBitcoinSeries,
  createGroupedSentDollarsSeries,
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

  const title = args.title ? `${useGroupName ? "by" : "of"} ${args.title}` : "";

  return {
    name: args.name || "all",
    tree: [
      // Supply section
      isSingle
        ? {
            name: "supply",
            title: `Supply ${title}`,
            bottom: createSingleSupplySeries(
              ctx,
              /** @type {AddressCohortObject} */ (args),
            ),
          }
        : {
            name: "supply",
            tree: [
              {
                name: "total",
                title: `Supply ${title}`,
                bottom: createGroupedSupplyTotalSeries(list),
              },
              {
                name: "in profit",
                title: `Supply In Profit ${title}`,
                bottom: createGroupedSupplyInProfitSeries(list),
              },
              {
                name: "in loss",
                title: `Supply In Loss ${title}`,
                bottom: createGroupedSupplyInLossSeries(list),
              },
            ],
          },

      // UTXO count
      {
        name: "utxo count",
        title: `UTXO Count ${title}`,
        bottom: createUtxoCountSeries(list, useGroupName),
      },

      // Address count (ADDRESS COHORTS ONLY - fully type safe!)
      {
        name: "address count",
        title: `Address Count ${title}`,
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
                  title: `Realized Price ${title}`,
                  top: createRealizedPriceSeries(list),
                },
                {
                  name: "Ratio",
                  title: `Realized Price Ratio ${title}`,
                  bottom: createRealizedPriceRatioSeries(ctx, list),
                },
              ]
            : createRealizedPriceOptions(
                /** @type {AddressCohortObject} */ (args),
                title,
              )),
          {
            name: "capitalization",
            title: `Realized Cap ${title}`,
            bottom: createRealizedCapWithExtras(ctx, list, args, useGroupName),
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
 * @param {string} title
 * @returns {PartialOptionsTree}
 */
function createRealizedPriceOptions(args, title) {
  const { tree, color } = args;

  return [
    {
      name: "price",
      title: `Realized Price ${title}`,
      top: [
        line({
          metric: tree.realized.realizedPrice,
          name: "Realized",
          color,
          unit: Unit.usd,
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
 * @param {string} title
 * @returns {PartialOptionsTree}
 */
function createRealizedPnlSection(ctx, args, title) {
  const { colors } = ctx;
  const { realized } = args.tree;

  return [
    {
      name: "pnl",
      title: `Realized P&L ${title}`,
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
      title: `Net Realized P&L ${title}`,
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
          name: "Cumulative 30d change",
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
          name: "Cumulative 30d change",
          unit: Unit.pctRcap,
          defaultActive: false,
        }),
        baseline({
          metric: realized.netRealizedPnlCumulative30dDeltaRelToMarketCap,
          name: "Cumulative 30d change",
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
      name: "sopr",
      title: `SOPR ${title}`,
      bottom: [
        baseline({
          metric: realized.sopr,
          name: "SOPR",
          unit: Unit.ratio,
          base: 1,
        }),
        baseline({
          metric: realized.sopr7dEma,
          name: "7d EMA",
          color: [colors.lime, colors.rose],
          unit: Unit.ratio,
          defaultActive: false,
          base: 1,
        }),
        baseline({
          metric: realized.sopr30dEma,
          name: "30d EMA",
          color: [colors.avocado, colors.pink],
          unit: Unit.ratio,
          defaultActive: false,
          base: 1,
        }),
        priceLine({
          ctx,
          unit: Unit.ratio,
          number: 1,
        }),
      ],
    },
    {
      name: "Sell Side Risk",
      title: `Sell Side Risk Ratio ${title}`,
      bottom: [
        line({
          metric: realized.sellSideRiskRatio,
          name: "Raw",
          color: colors.orange,
          unit: Unit.ratio,
        }),
        line({
          metric: realized.sellSideRiskRatio7dEma,
          name: "7d EMA",
          color: colors.red,
          unit: Unit.ratio,
          defaultActive: false,
        }),
        line({
          metric: realized.sellSideRiskRatio30dEma,
          name: "30d EMA",
          color: colors.rose,
          unit: Unit.ratio,
          defaultActive: false,
        }),
      ],
    },
    {
      name: "value",
      title: `Value Created & Destroyed ${title}`,
      bottom: [
        line({
          metric: realized.valueCreated,
          name: "Created",
          color: colors.emerald,
          unit: Unit.usd,
        }),
        line({
          metric: realized.valueDestroyed,
          name: "Destroyed",
          color: colors.red,
          unit: Unit.usd,
        }),
      ],
    },
  ];
}

/**
 * Create unrealized section
 * @param {PartialContext} ctx
 * @param {readonly AddressCohortObject[]} list
 * @param {boolean} useGroupName
 * @param {string} title
 * @returns {PartialOptionsTree}
 */
function createUnrealizedSection(ctx, list, useGroupName, title) {
  const { colors } = ctx;

  return [
    {
      name: "Unrealized",
      tree: [
        {
          name: "profit",
          title: `Unrealized Profit ${title}`,
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
          name: "loss",
          title: `Unrealized Loss ${title}`,
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
          name: "total pnl",
          title: `Total Unrealized P&L ${title}`,
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
          name: "negative loss",
          title: `Negative Unrealized Loss ${title}`,
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
              name: "nupl",
              title: `NUPL (Rel to Market Cap) ${title}`,
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
              name: "profit",
              title: `Unrealized Profit (% of Market Cap) ${title}`,
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
              name: "loss",
              title: `Unrealized Loss (% of Market Cap) ${title}`,
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
              name: "net pnl",
              title: `Net Unrealized P&L (% of Market Cap) ${title}`,
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
              name: "negative loss",
              title: `Negative Unrealized Loss (% of Market Cap) ${title}`,
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
          name: "nupl",
          title: `Net Unrealized P&L ${title}`,
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
 * @param {string} title
 * @returns {PartialOptionsTree}
 */
function createCostBasisSection(list, useGroupName, title) {
  return [
    {
      name: "Cost Basis",
      tree: [
        {
          name: "min",
          title: `Min Cost Basis ${title}`,
          top: list.map(({ color, name, tree }) =>
            line({
              metric: tree.costBasis.min,
              name: useGroupName ? name : "Min",
              color,
              unit: Unit.usd,
            }),
          ),
        },
        {
          name: "max",
          title: `Max Cost Basis ${title}`,
          top: list.map(({ color, name, tree }) =>
            line({
              metric: tree.costBasis.max,
              name: useGroupName ? name : "Max",
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
 * Create activity section
 * @param {AddressCohortObject | AddressCohortGroupObject} args
 * @param {string} title
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
            title: `Coins Destroyed ${title}`,
            bottom: createSingleCoinsDestroyedSeries(cohort),
          },
          {
            name: "Sent",
            title: `Sent ${title}`,
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
          name: "coinblocks destroyed",
          title: `Coinblocks Destroyed ${title}`,
          bottom: createGroupedCoinblocksDestroyedSeries(list),
        },
        {
          name: "coindays destroyed",
          title: `Coindays Destroyed ${title}`,
          bottom: createGroupedCoindaysDestroyedSeries(list),
        },
        {
          name: "satblocks destroyed",
          title: `Satblocks Destroyed ${title}`,
          bottom: createGroupedSatblocksDestroyedSeries(list),
        },
        {
          name: "satdays destroyed",
          title: `Satdays Destroyed ${title}`,
          bottom: createGroupedSatdaysDestroyedSeries(list),
        },
        {
          name: "Sent",
          tree: [
            {
              name: "sats",
              title: `Sent (Sats) ${title}`,
              bottom: createGroupedSentSatsSeries(list),
            },
            {
              name: "bitcoin",
              title: `Sent (BTC) ${title}`,
              bottom: createGroupedSentBitcoinSeries(list),
            },
            {
              name: "dollars",
              title: `Sent ($) ${title}`,
              bottom: createGroupedSentDollarsSeries(list),
            },
          ],
        },
      ],
    },
  ];
}
