/**
 * Address cohort folder builder
 * Creates option trees for address-based cohorts (has addrCount)
 * Address cohorts use _0satsPattern which has CostBasisPattern (no percentiles)
 */

import { Unit } from "../../utils/units.js";
import {
  createSingleSupplySeries,
  createGroupedSupplyTotalSeries,
  createGroupedSupplyInProfitSeries,
  createGroupedSupplyInLossSeries,
  createUtxoCountSeries,
  createAddressCountSeries,
  createRealizedPriceSeries,
  createRealizedPriceRatioSeries,
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
                bottom: createGroupedSupplyTotalSeries(ctx, list),
              },
              {
                name: "in profit",
                title: `Supply In Profit ${title}`,
                bottom: createGroupedSupplyInProfitSeries(ctx, list),
              },
              {
                name: "in loss",
                title: `Supply In Loss ${title}`,
                bottom: createGroupedSupplyInLossSeries(ctx, list),
              },
            ],
          },

      // UTXO count
      {
        name: "utxo count",
        title: `UTXO Count ${title}`,
        bottom: createUtxoCountSeries(ctx, list, useGroupName),
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
                  top: createRealizedPriceSeries(ctx, list),
                },
                {
                  name: "Ratio",
                  title: `Realized Price Ratio ${title}`,
                  bottom: createRealizedPriceRatioSeries(ctx, list),
                },
              ]
            : createRealizedPriceOptions(
                ctx,
                /** @type {AddressCohortObject} */ (args),
                title,
              )),
          {
            name: "capitalization",
            title: `Realized Capitalization ${title}`,
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
      ...createCostBasisSection(ctx, list, useGroupName, title),

      // Activity section
      ...createActivitySection(ctx, list, useGroupName, title),
    ],
  };
}

/**
 * Create realized price options for single cohort
 * @param {PartialContext} ctx
 * @param {AddressCohortObject} args
 * @param {string} title
 * @returns {PartialOptionsTree}
 */
function createRealizedPriceOptions(ctx, args, title) {
  const { s } = ctx;
  const { tree, color } = args;

  return [
    {
      name: "price",
      title: `Realized Price ${title}`,
      top: [
        s({
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
  const { s, createPriceLine } = ctx;
  const isSingle = !("list" in args);

  return list.flatMap(({ color, name, tree }) => [
    s({
      metric: tree.realized.realizedCap,
      name: useGroupName ? name : "Capitalization",
      color,
      unit: Unit.usd,
    }),
    ...(isSingle
      ? [
          /** @type {AnyFetchedSeriesBlueprint} */ ({
            type: "Baseline",
            metric: tree.realized.realizedCap30dDelta,
            title: "30d Change",
            unit: Unit.usd,
            defaultActive: false,
          }),
          createPriceLine({ unit: Unit.usd, defaultActive: false }),
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
  const { colors, s } = ctx;
  const { realized } = args.tree;

  return [
    {
      name: "pnl",
      title: `Realized Profit And Loss ${title}`,
      bottom: [
        s({
          metric: mergeMetricPatterns(
            realized.realizedProfit.base,
            realized.realizedProfit.sum,
          ),
          name: "Profit",
          color: colors.green,
          unit: Unit.usd,
        }),
        s({
          metric: mergeMetricPatterns(
            realized.realizedLoss.base,
            realized.realizedLoss.sum,
          ),
          name: "Loss",
          color: colors.red,
          unit: Unit.usd,
          defaultActive: false,
        }),
        // RealizedPattern (address cohorts) doesn't have realizedProfitToLossRatio
        s({
          metric: realized.totalRealizedPnl,
          name: "Total",
          color: colors.default,
          defaultActive: false,
          unit: Unit.usd,
        }),
        s({
          metric: realized.negRealizedLoss.sum,
          name: "Negative Loss",
          color: colors.red,
          unit: Unit.usd,
        }),
        s({
          metric: realized.negRealizedLoss.cumulative,
          name: "Negative Loss",
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
  const { colors, s } = ctx;

  return [
    {
      name: "Unrealized",
      tree: [
        {
          name: "nupl",
          title: `Net Unrealized Profit/Loss ${title}`,
          bottom: list.flatMap(({ color, name, tree }) => [
            /** @type {AnyFetchedSeriesBlueprint} */ ({
              type: "Baseline",
              metric: tree.unrealized.netUnrealizedPnl,
              title: useGroupName ? name : "NUPL",
              color: useGroupName ? color : undefined,
              colors: useGroupName ? undefined : [colors.red, colors.green],
              unit: Unit.ratio,
              options: { baseValue: { price: 0 } },
            }),
          ]),
        },
        {
          name: "profit",
          title: `Unrealized Profit ${title}`,
          bottom: list.flatMap(({ color, name, tree }) => [
            s({
              metric: tree.unrealized.unrealizedProfit,
              name: useGroupName ? name : "Profit",
              color,
              unit: Unit.usd,
            }),
          ]),
        },
        {
          name: "loss",
          title: `Unrealized Loss ${title}`,
          bottom: list.flatMap(({ color, name, tree }) => [
            s({
              metric: tree.unrealized.unrealizedLoss,
              name: useGroupName ? name : "Loss",
              color,
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
 * @param {PartialContext} ctx
 * @param {readonly AddressCohortObject[]} list
 * @param {boolean} useGroupName
 * @param {string} title
 * @returns {PartialOptionsTree}
 */
function createCostBasisSection(ctx, list, useGroupName, title) {
  const { s } = ctx;

  return [
    {
      name: "Cost Basis",
      tree: [
        {
          name: "min",
          title: `Min Cost Basis ${title}`,
          top: list.map(({ color, name, tree }) =>
            s({
              metric: tree.costBasis.minCostBasis,
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
            s({
              metric: tree.costBasis.maxCostBasis,
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
 * @param {PartialContext} ctx
 * @param {readonly AddressCohortObject[]} list
 * @param {boolean} useGroupName
 * @param {string} title
 * @returns {PartialOptionsTree}
 */
function createActivitySection(ctx, list, useGroupName, title) {
  const { s, brk } = ctx;

  return [
    {
      name: "Activity",
      tree: [
        {
          name: "coinblocks destroyed",
          title: `Coinblocks Destroyed ${title}`,
          bottom: list.flatMap(({ color, name, tree }) => [
            s({
              metric: tree.activity.coinblocksDestroyed.sum,
              name: useGroupName ? name : "Coinblocks",
              color,
              unit: Unit.coinblocks,
            }),
            s({
              metric: tree.activity.coinblocksDestroyed.cumulative,
              name: useGroupName ? name : "Coinblocks",
              color,
              unit: Unit.coinblocks,
            }),
          ]),
        },
        {
          name: "coindays destroyed",
          title: `Coindays Destroyed ${title}`,
          bottom: list.flatMap(({ color, name, tree }) => [
            s({
              metric: tree.activity.coindaysDestroyed.sum,
              name: useGroupName ? name : "Coindays",
              color,
              unit: Unit.coindays,
            }),
            s({
              metric: tree.activity.coindaysDestroyed.cumulative,
              name: useGroupName ? name : "Coindays",
              color,
              unit: Unit.coindays,
            }),
          ]),
        },
      ],
    },
  ];
}
