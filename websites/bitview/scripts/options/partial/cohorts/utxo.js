/**
 * UTXO cohort folder builders
 * Creates option trees for UTXO-based cohorts (no addrCount)
 *
 * Two main builders:
 * - createAgeCohortFolder: For term, maxAge, minAge, ageRange, epoch (has cost basis percentiles)
 * - createAmountCohortFolder: For geAmount, ltAmount, amountRange, type (no cost basis percentiles)
 */

import {
  createSingleSupplySeries,
  createGroupedSupplyTotalSeries,
  createGroupedSupplyInProfitSeries,
  createGroupedSupplyInLossSeries,
  createUtxoCountSeries,
  createRealizedPriceSeries,
  createRealizedPriceRatioSeries,
  createCostBasisPercentilesSeries,
} from "./shared.js";

/**
 * Create a cohort folder for age-based UTXO cohorts (term, maxAge, minAge, ageRange, epoch)
 * These cohorts have cost basis percentiles via CostBasisPattern2
 * @param {PartialContext} ctx
 * @param {AgeCohortObject | AgeCohortGroupObject} args
 * @returns {PartialOptionsGroup}
 */
export function createAgeCohortFolder(ctx, args) {
  const list = "list" in args ? args.list : [args];
  const useGroupName = "list" in args;
  const isSingle = !("list" in args);
  const title = args.title ? `${useGroupName ? "by" : "of"} ${args.title}` : "";

  return {
    name: args.name || "all",
    tree: [
      ...createSupplySection(ctx, list, args, useGroupName, isSingle, title),
      createUtxoCountSection(ctx, list, useGroupName, title),
      createRealizedSection(ctx, list, args, useGroupName, isSingle, title),
      ...createUnrealizedSection(ctx, list, useGroupName, title),
      ...createCostBasisSectionWithPercentiles(ctx, list, useGroupName, title),
      ...createActivitySection(ctx, list, useGroupName, title),
    ],
  };
}

/**
 * Create a cohort folder for amount-based UTXO cohorts (geAmount, ltAmount, amountRange, type)
 * These cohorts have only min/max cost basis via CostBasisPattern
 * @param {PartialContext} ctx
 * @param {AmountCohortObject | AmountCohortGroupObject} args
 * @returns {PartialOptionsGroup}
 */
export function createAmountCohortFolder(ctx, args) {
  const list = "list" in args ? args.list : [args];
  const useGroupName = "list" in args;
  const isSingle = !("list" in args);
  const title = args.title ? `${useGroupName ? "by" : "of"} ${args.title}` : "";

  return {
    name: args.name || "all",
    tree: [
      ...createSupplySection(ctx, list, args, useGroupName, isSingle, title),
      createUtxoCountSection(ctx, list, useGroupName, title),
      createRealizedSection(ctx, list, args, useGroupName, isSingle, title),
      ...createUnrealizedSection(ctx, list, useGroupName, title),
      ...createCostBasisSectionBasic(ctx, list, useGroupName, title),
      ...createActivitySection(ctx, list, useGroupName, title),
    ],
  };
}

// Keep the generic version for backwards compatibility
/**
 * Create a cohort folder for UTXO cohorts (generic, uses runtime check for percentiles)
 * @deprecated Use createAgeCohortFolder or createAmountCohortFolder for type safety
 * @param {PartialContext} ctx
 * @param {UtxoCohortObject | UtxoCohortGroupObject} args
 * @returns {PartialOptionsGroup}
 */
export function createUtxoCohortFolder(ctx, args) {
  const list = "list" in args ? args.list : [args];
  const useGroupName = "list" in args;
  const isSingle = !("list" in args);
  const title = args.title ? `${useGroupName ? "by" : "of"} ${args.title}` : "";

  // Runtime check for percentiles
  const hasPercentiles = "percentiles" in list[0].tree.costBasis;

  return {
    name: args.name || "all",
    tree: [
      ...createSupplySection(ctx, list, args, useGroupName, isSingle, title),
      createUtxoCountSection(ctx, list, useGroupName, title),
      createRealizedSection(ctx, list, args, useGroupName, isSingle, title),
      ...createUnrealizedSection(ctx, list, useGroupName, title),
      ...(hasPercentiles
        ? createCostBasisSectionWithPercentiles(
            ctx,
            /** @type {readonly AgeCohortObject[]} */ (list),
            useGroupName,
            title,
          )
        : createCostBasisSectionBasic(ctx, list, useGroupName, title)),
      ...createActivitySection(ctx, list, useGroupName, title),
    ],
  };
}

/**
 * Create supply section
 * @param {PartialContext} ctx
 * @param {readonly UtxoCohortObject[]} list
 * @param {UtxoCohortObject | UtxoCohortGroupObject} args
 * @param {boolean} useGroupName
 * @param {boolean} isSingle
 * @param {string} title
 * @returns {PartialOptionsTree}
 */
function createSupplySection(ctx, list, args, useGroupName, isSingle, title) {
  return [
    isSingle
      ? {
          name: "supply",
          title: `Supply ${title}`,
          bottom: createSingleSupplySeries(
            ctx,
            /** @type {UtxoCohortObject} */ (args),
            title,
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
  ];
}

/**
 * Create UTXO count section
 * @param {PartialContext} ctx
 * @param {readonly UtxoCohortObject[]} list
 * @param {boolean} useGroupName
 * @param {string} title
 * @returns {PartialChartOption}
 */
function createUtxoCountSection(ctx, list, useGroupName, title) {
  return {
    name: "utxo count",
    title: `UTXO Count ${title}`,
    bottom: createUtxoCountSeries(ctx, list, useGroupName),
  };
}

/**
 * Create realized section
 * @param {PartialContext} ctx
 * @param {readonly UtxoCohortObject[]} list
 * @param {UtxoCohortObject | UtxoCohortGroupObject} args
 * @param {boolean} useGroupName
 * @param {boolean} isSingle
 * @param {string} title
 * @returns {PartialOptionsGroup}
 */
function createRealizedSection(ctx, list, args, useGroupName, isSingle, title) {
  return {
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
            /** @type {UtxoCohortObject} */ (args),
            title,
          )),
      {
        name: "capitalization",
        title: `Realized Capitalization ${title}`,
        bottom: createRealizedCapWithExtras(
          ctx,
          list,
          args,
          useGroupName,
          title,
        ),
      },
      ...(!useGroupName
        ? createRealizedPnlSection(
            ctx,
            /** @type {UtxoCohortObject} */ (args),
            title,
          )
        : []),
    ],
  };
}

/**
 * Create realized price options for single cohort
 * @param {PartialContext} ctx
 * @param {UtxoCohortObject} args
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
        s({ metric: tree.realized.realizedPrice, name: "realized", color }),
      ],
    },
  ];
}

/**
 * Create realized cap with extras
 * @param {PartialContext} ctx
 * @param {readonly UtxoCohortObject[]} list
 * @param {UtxoCohortObject | UtxoCohortGroupObject} args
 * @param {boolean} useGroupName
 * @param {string} title
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function createRealizedCapWithExtras(ctx, list, args, useGroupName, title) {
  const { colors, s, createPriceLine } = ctx;
  const isSingle = !("list" in args);

  return list.flatMap(({ color, name, tree }) => [
    s({
      metric: tree.realized.realizedCap,
      name: useGroupName ? name : "Capitalization",
      color,
    }),
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
    ...(isSingle && "realizedCapRelToOwnMarketCap" in tree.realized
      ? [
          /** @type {AnyFetchedSeriesBlueprint} */ ({
            type: "Baseline",
            metric: tree.realized.realizedCapRelToOwnMarketCap,
            title: "ratio",
            options: { baseValue: { price: 100 } },
            colors: [colors.red, colors.green],
          }),
          createPriceLine({ unit: "%cmcap", defaultActive: true, number: 100 }),
        ]
      : []),
  ]);
}

/**
 * Create realized PnL section for single cohort
 * @param {PartialContext} ctx
 * @param {UtxoCohortObject} args
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
        s({
          metric: tree.realized.realizedProfit.base,
          name: "Profit",
          color: colors.green,
        }),
        s({
          metric: tree.realized.realizedLoss.base,
          name: "Loss",
          color: colors.red,
          defaultActive: false,
        }),
        ...("realizedProfitToLossRatio" in tree.realized
          ? [
              s({
                metric: tree.realized.realizedProfitToLossRatio,
                name: "profit / loss",
                color: colors.yellow,
              }),
            ]
          : []),
        s({
          metric: tree.realized.totalRealizedPnl.base,
          name: "Total",
          color: colors.default,
          defaultActive: false,
        }),
        s({
          metric: tree.realized.negRealizedLoss.base,
          name: "Negative Loss",
          color: colors.red,
        }),
      ],
    },
  ];
}

/**
 * Create unrealized section
 * @param {PartialContext} ctx
 * @param {readonly UtxoCohortObject[]} list
 * @param {boolean} useGroupName
 * @param {string} title
 * @returns {PartialOptionsTree}
 */
function createUnrealizedSection(ctx, list, useGroupName, title) {
  const { colors, s, createPriceLine } = ctx;

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
            s({
              metric: tree.unrealized.unrealizedProfit,
              name: useGroupName ? name : "Profit",
              color,
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
            }),
          ]),
        },
      ],
    },
  ];
}

/**
 * Create cost basis section for cohorts WITH percentiles (age cohorts)
 * @param {PartialContext} ctx
 * @param {readonly AgeCohortObject[]} list
 * @param {boolean} useGroupName
 * @param {string} title
 * @returns {PartialOptionsTree}
 */
function createCostBasisSectionWithPercentiles(ctx, list, useGroupName, title) {
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
            }),
          ),
        },
        {
          name: "percentiles",
          title: `Cost Basis Percentiles ${title}`,
          top: createCostBasisPercentilesSeries(ctx, list, useGroupName),
        },
      ],
    },
  ];
}

/**
 * Create cost basis section for cohorts WITHOUT percentiles (amount cohorts)
 * @param {PartialContext} ctx
 * @param {readonly UtxoCohortObject[]} list
 * @param {boolean} useGroupName
 * @param {string} title
 * @returns {PartialOptionsTree}
 */
function createCostBasisSectionBasic(ctx, list, useGroupName, title) {
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
 * @param {readonly UtxoCohortObject[]} list
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
