/** On-chain indicators (Pi Cycle, Puell, NVT, Gini) */

/**
 * Create On-chain section
 * @param {PartialContext} ctx
 * @param {Object} args
 * @param {Market["indicators"]} args.indicators
 * @param {Market["movingAverage"]} args.movingAverage
 */
export function createOnchainSection(ctx, { indicators, movingAverage }) {
  const { s, colors, createPriceLine } = ctx;

  return {
    name: "On-chain",
    tree: [
      {
        name: "Pi Cycle",
        title: "Pi Cycle Top Indicator",
        top: [
          s({
            metric: movingAverage.price111dSma.price,
            name: "111d SMA",
            color: colors.green,
            unit: "usd",
          }),
          s({
            metric: movingAverage.price350dSmaX2,
            name: "350d SMA x2",
            color: colors.red,
            unit: "usd",
          }),
        ],
        bottom: [
          s({
            metric: indicators.piCycle,
            name: "Pi Cycle",
            color: colors.purple,
            unit: "ratio",
          }),
          createPriceLine({ unit: "ratio", number: 1 }),
        ],
      },
      {
        name: "Puell Multiple",
        title: "Puell Multiple",
        bottom: [
          s({
            metric: indicators.puellMultiple,
            name: "Puell",
            color: colors.green,
            unit: "ratio",
          }),
        ],
      },
      {
        name: "NVT",
        title: "Network Value to Transactions Ratio",
        bottom: [
          s({
            metric: indicators.nvt,
            name: "NVT",
            color: colors.orange,
            unit: "ratio",
          }),
        ],
      },
      {
        name: "Gini",
        title: "Gini Coefficient",
        bottom: [
          s({
            metric: indicators.gini,
            name: "Gini",
            color: colors.red,
            unit: "ratio",
          }),
        ],
      },
    ],
  };
}
