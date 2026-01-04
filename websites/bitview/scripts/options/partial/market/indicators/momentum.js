/** Momentum indicators (RSI, StochRSI, Stochastic, MACD) */

/**
 * Create Momentum section
 * @param {PartialContext} ctx
 * @param {Market["indicators"]} indicators
 */
export function createMomentumSection(ctx, indicators) {
  const { s, colors, createPriceLine } = ctx;

  return {
    name: "Momentum",
    tree: [
      {
        name: "RSI",
        title: "Relative Strength Index (14d)",
        bottom: [
          s({
            metric: indicators.rsi14d,
            name: "RSI",
            color: colors.indigo,
            unit: "index",
          }),
          s({
            metric: indicators.rsi14dMin,
            name: "Min",
            color: colors.red,
            defaultActive: false,
            unit: "index",
          }),
          s({
            metric: indicators.rsi14dMax,
            name: "Max",
            color: colors.green,
            defaultActive: false,
            unit: "index",
          }),
          createPriceLine({ unit: "index", number: 70 }),
          createPriceLine({ unit: "index", number: 50, defaultActive: false }),
          createPriceLine({ unit: "index", number: 30 }),
        ],
      },
      {
        name: "StochRSI",
        title: "Stochastic RSI",
        bottom: [
          // s({
          //   metric: indicators.stochRsi,
          //   name: "Stoch RSI",
          //   color: colors.purple,
          //   unit: "index",
          // }),
          s({
            metric: indicators.stochRsiK,
            name: "K",
            color: colors.blue,
            unit: "index",
          }),
          s({
            metric: indicators.stochRsiD,
            name: "D",
            color: colors.orange,
            unit: "index",
          }),
          createPriceLine({ unit: "index", number: 80 }),
          createPriceLine({ unit: "index", number: 20 }),
        ],
      },
      // {
      //   name: "Stochastic",
      //   title: "Stochastic Oscillator",
      //   bottom: [
      //     s({ metric: indicators.stochK, name: "K", color: colors.blue, unit: "index" }),
      //     s({ metric: indicators.stochD, name: "D", color: colors.orange, unit: "index" }),
      //     createPriceLine({ unit: "index", number: 80 }),
      //     createPriceLine({ unit: "index", number: 20 }),
      //   ],
      // },
      {
        name: "MACD",
        title: "Moving Average Convergence Divergence",
        bottom: [
          s({
            metric: indicators.macdLine,
            name: "MACD",
            color: colors.blue,
            unit: "usd",
          }),
          s({
            metric: indicators.macdSignal,
            name: "Signal",
            color: colors.orange,
            unit: "usd",
          }),
          /** @type {FetchedHistogramSeriesBlueprint} */ ({
            metric: indicators.macdHistogram,
            title: "Histogram",
            type: "Histogram",
            unit: "usd",
          }),
          createPriceLine({ unit: "usd" }),
        ],
      },
    ],
  };
}
