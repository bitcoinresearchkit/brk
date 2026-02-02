/** Performance section */

import { Unit } from "../../utils/units.js";
import { priceLine } from "../constants.js";
import { baseline } from "../series.js";
import { periodIdToName } from "../utils.js";

/**
 * Create Returns section
 * @param {PartialContext} ctx
 * @param {Market["returns"]} returns
 */
export function createReturnsSection(ctx, returns) {
  const { colors } = ctx;

  const shortTermPeriods = /** @type {const} */ ([
    ["1d", "_1d", undefined],
    ["1w", "_1w", undefined],
    ["1m", "_1m", undefined],
  ]);

  const mediumTermPeriods = /** @type {const} */ ([
    ["3m", "_3m", undefined],
    ["6m", "_6m", undefined],
    ["1y", "_1y", undefined],
  ]);

  const longTermPeriods = /** @type {const} */ ([
    ["2y", "_2y", "_2y"],
    ["3y", "_3y", "_3y"],
    ["4y", "_4y", "_4y"],
    ["5y", "_5y", "_5y"],
    ["6y", "_6y", "_6y"],
    ["8y", "_8y", "_8y"],
    ["10y", "_10y", "_10y"],
  ]);

  /**
   * @template {keyof typeof returns.priceReturns} K
   * @param {readonly [string, K, K | undefined]} period
   */
  const createPeriodChart = ([id, returnKey, cagrKey]) => {
    const priceReturns = returns.priceReturns[/** @type {K} */ (returnKey)];
    const cagr = cagrKey
      ? returns.cagr[/** @type {keyof typeof returns.cagr} */ (cagrKey)]
      : undefined;
    const name = periodIdToName(id, true);
    return {
      name,
      title: `${name} Returns`,
      bottom: [
        baseline({
          metric: priceReturns,
          name: "Total",
          unit: Unit.percentage,
        }),
        ...(cagr
          ? [
              baseline({
                metric: cagr,
                name: "CAGR",
                color: [colors.cyan, colors.orange],
                unit: Unit.percentage,
              }),
            ]
          : []),
        priceLine({ ctx, unit: Unit.percentage }),
      ],
    };
  };

  return {
    name: "Returns",
    tree: [
      // Compare all periods
      {
        name: "Compare",
        title: "Returns Comparison",
        bottom: [
          baseline({
            metric: returns.priceReturns._1d,
            name: "1d",
            color: colors.red,
            unit: Unit.percentage,
          }),
          baseline({
            metric: returns.priceReturns._1w,
            name: "1w",
            color: colors.orange,
            unit: Unit.percentage,
          }),
          baseline({
            metric: returns.priceReturns._1m,
            name: "1m",
            color: colors.yellow,
            unit: Unit.percentage,
          }),
          baseline({
            metric: returns.priceReturns._3m,
            name: "3m",
            color: colors.lime,
            unit: Unit.percentage,
            defaultActive: false,
          }),
          baseline({
            metric: returns.priceReturns._6m,
            name: "6m",
            color: colors.green,
            unit: Unit.percentage,
            defaultActive: false,
          }),
          baseline({
            metric: returns.priceReturns._1y,
            name: "1y",
            color: colors.teal,
            unit: Unit.percentage,
          }),
          baseline({
            metric: returns.priceReturns._4y,
            name: "4y",
            color: colors.blue,
            unit: Unit.percentage,
          }),
          priceLine({ ctx, unit: Unit.percentage }),
        ],
      },
      // Short-term (1d, 1w, 1m)
      {
        name: "Short-term",
        tree: shortTermPeriods.map(createPeriodChart),
      },
      // Medium-term (3m, 6m, 1y)
      {
        name: "Medium-term",
        tree: mediumTermPeriods.map(createPeriodChart),
      },
      // Long-term (2y+)
      {
        name: "Long-term",
        tree: longTermPeriods.map(createPeriodChart),
      },
    ],
  };
}
