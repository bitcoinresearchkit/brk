/** Performance section */

import { Unit } from "../../utils/units.js";
import { periodIdToName } from "./utils.js";

/**
 * Create Performance section
 * @param {PartialContext} ctx
 * @param {Market["returns"]} returns
 */
export function createPerformanceSection(ctx, returns) {
  const { colors, baseline, createPriceLine } = ctx;

  return {
    name: "Performance",
    tree: /** @type {const} */ ([
      ["1d", "_1d", undefined],
      ["1w", "_1w", undefined],
      ["1m", "_1m", undefined],
      ["3m", "_3m", undefined],
      ["6m", "_6m", undefined],
      ["1y", "_1y", undefined],
      ["2y", "_2y", "_2y"],
      ["3y", "_3y", "_3y"],
      ["4y", "_4y", "_4y"],
      ["5y", "_5y", "_5y"],
      ["6y", "_6y", "_6y"],
      ["8y", "_8y", "_8y"],
      ["10y", "_10y", "_10y"],
    ]).map(([id, returnKey, cagrKey]) => {
      const priceReturns = returns.priceReturns[returnKey];
      const cagr = cagrKey ? returns.cagr[cagrKey] : undefined;
      const name = periodIdToName(id, true);
      return {
        name,
        title: `${name} Performance`,
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
                  color: [colors.lime, colors.pink],
                  unit: Unit.percentage,
                }),
              ]
            : []),
          createPriceLine({ unit: Unit.percentage }),
        ],
      };
    }),
  };
}
