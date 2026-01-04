/** Bands indicators (MinMax, Mayer Multiple) */

/**
 * Create Bands section
 * @param {PartialContext} ctx
 * @param {Object} args
 * @param {Market["range"]} args.range
 * @param {Market["movingAverage"]} args.movingAverage
 */
export function createBandsSection(ctx, { range, movingAverage }) {
  const { s, colors } = ctx;

  return {
    name: "Bands",
    tree: [
      {
        name: "MinMax",
        tree: [
          { id: "1w", title: "1 Week", min: range.price1wMin, max: range.price1wMax },
          { id: "2w", title: "2 Week", min: range.price2wMin, max: range.price2wMax },
          { id: "1m", title: "1 Month", min: range.price1mMin, max: range.price1mMax },
          { id: "1y", title: "1 Year", min: range.price1yMin, max: range.price1yMax },
        ].map(({ id, title, min, max }) => ({
          name: id,
          title: `Bitcoin Price ${title} MinMax Bands`,
          top: [
            s({ metric: min, name: "min", color: colors.red, unit: "usd" }),
            s({ metric: max, name: "max", color: colors.green, unit: "usd" }),
          ],
        })),
      },
      {
        name: "Mayer Multiple",
        title: "Mayer Multiple",
        top: [
          s({ metric: movingAverage.price200dSma.price, name: "200d sma", color: colors.yellow, unit: "usd" }),
          s({ metric: movingAverage.price200dSmaX24, name: "200d sma x2.4", color: colors.green, unit: "usd" }),
          s({ metric: movingAverage.price200dSmaX08, name: "200d sma x0.8", color: colors.red, unit: "usd" }),
        ],
      },
    ],
  };
}
