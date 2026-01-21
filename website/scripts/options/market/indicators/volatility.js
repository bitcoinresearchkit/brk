/** Volatility indicators (Index, True Range, Choppiness, Sharpe, Sortino) */

import { Unit } from "../../../utils/units.js";
import { line } from "../../series.js";

/**
 * Create Volatility section
 * @param {PartialContext} ctx
 * @param {Object} args
 * @param {Market["volatility"]} args.volatility
 * @param {Market["range"]} args.range
 */
export function createVolatilitySection(ctx, { volatility, range }) {
  const { colors, createPriceLine } = ctx;

  return {
    name: "Volatility",
    tree: [
      {
        name: "Index",
        title: "Bitcoin Price Volatility Index",
        bottom: [
          line({
            metric: volatility.price1wVolatility,
            name: "1w",
            color: colors.red,
            unit: Unit.percentage,
          }),
          line({
            metric: volatility.price1mVolatility,
            name: "1m",
            color: colors.orange,
            unit: Unit.percentage,
          }),
          line({
            metric: volatility.price1yVolatility,
            name: "1y",
            color: colors.lime,
            unit: Unit.percentage,
          }),
        ],
      },
      {
        name: "True Range",
        title: "Bitcoin Price True Range",
        bottom: [
          line({
            metric: range.priceTrueRange,
            name: "Value",
            color: colors.yellow,
            unit: Unit.usd,
          }),
        ],
      },
      {
        name: "Choppiness",
        title: "Bitcoin Price Choppiness Index",
        bottom: [
          line({
            metric: range.price2wChoppinessIndex,
            name: "2w",
            color: colors.red,
            unit: Unit.index,
          }),
          createPriceLine({ unit: Unit.index, number: 61.8 }),
          createPriceLine({ unit: Unit.index, number: 38.2 }),
        ],
      },
      {
        name: "Sharpe Ratio",
        title: "Sharpe Ratio",
        bottom: [
          line({
            metric: volatility.sharpe1w,
            name: "1w",
            color: colors.red,
            unit: Unit.ratio,
          }),
          line({
            metric: volatility.sharpe1m,
            name: "1m",
            color: colors.orange,
            unit: Unit.ratio,
          }),
          line({
            metric: volatility.sharpe1y,
            name: "1y",
            color: colors.lime,
            unit: Unit.ratio,
          }),
          createPriceLine({ unit: Unit.ratio }),
        ],
      },
      {
        name: "Sortino Ratio",
        title: "Sortino Ratio",
        bottom: [
          line({
            metric: volatility.sortino1w,
            name: "1w",
            color: colors.red,
            unit: Unit.ratio,
          }),
          line({
            metric: volatility.sortino1m,
            name: "1m",
            color: colors.orange,
            unit: Unit.ratio,
          }),
          line({
            metric: volatility.sortino1y,
            name: "1y",
            color: colors.lime,
            unit: Unit.ratio,
          }),
          createPriceLine({ unit: Unit.ratio }),
        ],
      },
    ],
  };
}
