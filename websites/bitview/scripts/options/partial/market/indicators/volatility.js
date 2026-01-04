/** Volatility indicators (Index, True Range, Choppiness, Sharpe, Sortino) */

/**
 * Create Volatility section
 * @param {PartialContext} ctx
 * @param {Object} args
 * @param {Market["volatility"]} args.volatility
 * @param {Market["range"]} args.range
 */
export function createVolatilitySection(ctx, { volatility, range }) {
  const { s, colors, createPriceLine } = ctx;

  return {
    name: "Volatility",
    tree: [
      {
        name: "Index",
        title: "Bitcoin Price Volatility Index",
        bottom: [
          s({ metric: volatility.price1wVolatility, name: "1w", color: colors.red, unit: "percentage" }),
          s({ metric: volatility.price1mVolatility, name: "1m", color: colors.orange, unit: "percentage" }),
          s({ metric: volatility.price1yVolatility, name: "1y", color: colors.lime, unit: "percentage" }),
        ],
      },
      {
        name: "True Range",
        title: "Bitcoin Price True Range",
        bottom: [s({ metric: range.priceTrueRange, name: "value", color: colors.yellow, unit: "usd" })],
      },
      {
        name: "Choppiness",
        title: "Bitcoin Price Choppiness Index",
        bottom: [
          s({ metric: range.price2wChoppinessIndex, name: "2w", color: colors.red, unit: "index" }),
          createPriceLine({ unit: "index", number: 61.8 }),
          createPriceLine({ unit: "index", number: 38.2 }),
        ],
      },
      {
        name: "Sharpe Ratio",
        title: "Sharpe Ratio",
        bottom: [
          s({ metric: volatility.sharpe1w, name: "1w", color: colors.red, unit: "ratio" }),
          s({ metric: volatility.sharpe1m, name: "1m", color: colors.orange, unit: "ratio" }),
          s({ metric: volatility.sharpe1y, name: "1y", color: colors.lime, unit: "ratio" }),
          createPriceLine({ unit: "ratio" }),
        ],
      },
      {
        name: "Sortino Ratio",
        title: "Sortino Ratio",
        bottom: [
          s({ metric: volatility.sortino1w, name: "1w", color: colors.red, unit: "ratio" }),
          s({ metric: volatility.sortino1m, name: "1m", color: colors.orange, unit: "ratio" }),
          s({ metric: volatility.sortino1y, name: "1y", color: colors.lime, unit: "ratio" }),
          createPriceLine({ unit: "ratio" }),
        ],
      },
    ],
  };
}
