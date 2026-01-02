/**
 * Address cohort folder builder
 * Creates option trees for address-based cohorts (has addrCount)
 * Address cohorts use _0satsPattern which has PricePaidPattern (no percentiles)
 */

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
            bottom: createSingleSupplySeries(ctx, /** @type {AddressCohortObject} */ (args), title),
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
            : createRealizedPriceOptions(ctx, /** @type {AddressCohortObject} */ (args), title)),
          {
            name: "capitalization",
            title: `Realized Capitalization ${title}`,
            bottom: createRealizedCapWithExtras(ctx, list, args, useGroupName, title),
          },
          ...(!useGroupName ? createRealizedPnlSection(ctx, /** @type {AddressCohortObject} */ (args), title) : []),
        ],
      },

      // Unrealized section
      ...createUnrealizedSection(ctx, list, useGroupName, title),

      // Price paid section (no percentiles for address cohorts)
      ...createPricePaidSection(ctx, list, useGroupName, title),

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
      top: [s({ metric: tree.realized.realizedPrice, name: "realized", color })],
    },
  ];
}

/**
 * Create realized cap with extras
 * @param {PartialContext} ctx
 * @param {readonly AddressCohortObject[]} list
 * @param {AddressCohortObject | AddressCohortGroupObject} args
 * @param {boolean} useGroupName
 * @param {string} title
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function createRealizedCapWithExtras(ctx, list, args, useGroupName, title) {
  const { colors, s, createPriceLine } = ctx;
  const isSingle = !("list" in args);

  return list.flatMap(({ color, name, tree }) => [
    s({ metric: tree.realized.realizedCap, name: useGroupName ? name : "Capitalization", color }),
    ...(isSingle
      ? [
          /** @type {AnyFetchedSeriesBlueprint} */ ({
            type: "Baseline",
            metric: tree.realized.realizedCap30dDelta,
            title: "30d change",
            defaultActive: false,
          }),
          createPriceLine({ unit: "usd", defaultActive: false }),
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
  const { tree } = args;

  return [
    {
      name: "pnl",
      title: `Realized Profit And Loss ${title}`,
      bottom: [
        s({ metric: tree.realized.realizedProfit.base, name: "Profit", color: colors.green }),
        s({ metric: tree.realized.realizedLoss.base, name: "Loss", color: colors.red, defaultActive: false }),
        // RealizedPattern (address cohorts) doesn't have realizedProfitToLossRatio
        s({ metric: tree.realized.totalRealizedPnl.base, name: "Total", color: colors.default, defaultActive: false }),
        s({ metric: tree.realized.negRealizedLoss.base, name: "Negative Loss", color: colors.red }),
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
              colors: [colors.red, colors.green],
              options: { baseValue: { price: 0 } },
            }),
          ]),
        },
        {
          name: "profit",
          title: `Unrealized Profit ${title}`,
          bottom: list.flatMap(({ color, name, tree }) => [
            s({ metric: tree.unrealized.unrealizedProfit, name: useGroupName ? name : "Profit", color }),
          ]),
        },
        {
          name: "loss",
          title: `Unrealized Loss ${title}`,
          bottom: list.flatMap(({ color, name, tree }) => [
            s({ metric: tree.unrealized.unrealizedLoss, name: useGroupName ? name : "Loss", color }),
          ]),
        },
      ],
    },
  ];
}

/**
 * Create price paid section (no percentiles for address cohorts)
 * @param {PartialContext} ctx
 * @param {readonly AddressCohortObject[]} list
 * @param {boolean} useGroupName
 * @param {string} title
 * @returns {PartialOptionsTree}
 */
function createPricePaidSection(ctx, list, useGroupName, title) {
  const { s } = ctx;

  return [
    {
      name: "Price Paid",
      tree: [
        {
          name: "min",
          title: `Min Price Paid ${title}`,
          top: list.map(({ color, name, tree }) =>
            s({ metric: tree.pricePaid.minPricePaid, name: useGroupName ? name : "Min", color }),
          ),
        },
        {
          name: "max",
          title: `Max Price Paid ${title}`,
          top: list.map(({ color, name, tree }) =>
            s({ metric: tree.pricePaid.maxPricePaid, name: useGroupName ? name : "Max", color }),
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
  const { s } = ctx;

  return [
    {
      name: "Activity",
      tree: [
        {
          name: "coinblocks destroyed",
          title: `Coinblocks Destroyed ${title}`,
          bottom: list.flatMap(({ color, name, tree }) => [
            s({
              metric: tree.activity.coinblocksDestroyed.base,
              name: useGroupName ? name : "Coinblocks",
              color,
            }),
          ]),
        },
        {
          name: "coindays destroyed",
          title: `Coindays Destroyed ${title}`,
          bottom: list.flatMap(({ color, name, tree }) => [
            s({
              metric: tree.activity.coindaysDestroyed.base,
              name: useGroupName ? name : "Coindays",
              color,
            }),
          ]),
        },
      ],
    },
  ];
}
