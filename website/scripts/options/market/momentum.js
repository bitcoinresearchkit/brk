/** Momentum indicators (RSI, StochRSI, Stochastic, MACD) */

import { Unit } from "../../utils/units.js";
import { priceLine, priceLines } from "../constants.js";
import { line, histogram } from "../series.js";

/**
 * Create Momentum section
 * @param {PartialContext} ctx
 * @param {Market["indicators"]} indicators
 */
export function createMomentumSection(ctx, indicators) {
  const { colors } = ctx;

  return {
    name: "Momentum",
    tree: [
      {
        name: "RSI",
        title: "RSI (14d)",
        bottom: [
          line({
            metric: indicators.rsi14d,
            name: "RSI",
            color: colors.indigo,
            unit: Unit.index,
          }),
          line({
            metric: indicators.rsi14dMin,
            name: "Min",
            color: colors.red,
            defaultActive: false,
            unit: Unit.index,
          }),
          line({
            metric: indicators.rsi14dMax,
            name: "Max",
            color: colors.green,
            defaultActive: false,
            unit: Unit.index,
          }),
          priceLine({ ctx, unit: Unit.index, number: 70 }),
          priceLine({
            ctx,
            unit: Unit.index,
            number: 50,
            defaultActive: false,
          }),
          priceLine({ ctx, unit: Unit.index, number: 30 }),
        ],
      },
      {
        name: "StochRSI",
        title: "Stochastic RSI",
        bottom: [
          // line({
          //   metric: indicators.stochRsi,
          //   name: "Stoch RSI",
          //   color: colors.purple,
          //   unit: Unit.index,
          // }),
          line({
            metric: indicators.stochRsiK,
            name: "K",
            color: colors.blue,
            unit: Unit.index,
          }),
          line({
            metric: indicators.stochRsiD,
            name: "D",
            color: colors.orange,
            unit: Unit.index,
          }),
          ...priceLines({ ctx, unit: Unit.index, numbers: [80, 20] }),
        ],
      },
      // {
      //   name: "Stochastic",
      //   title: "Stochastic Oscillator",
      //   bottom: [
      //     line({ metric: indicators.stochK, name: "K", color: colors.blue, unit: Unit.index }),
      //     line({ metric: indicators.stochD, name: "D", color: colors.orange, unit: Unit.index }),
      //     priceLines({ ctx, unit: Unit.index, numbers: [80, 20] }),
      //   ],
      // },
      {
        name: "MACD",
        title: "MACD",
        bottom: [
          line({
            metric: indicators.macdLine,
            name: "MACD",
            color: colors.blue,
            unit: Unit.usd,
          }),
          line({
            metric: indicators.macdSignal,
            name: "Signal",
            color: colors.orange,
            unit: Unit.usd,
          }),
          histogram({
            metric: indicators.macdHistogram,
            name: "Histogram",
            unit: Unit.usd,
          }),
        ],
      },
    ],
  };
}
