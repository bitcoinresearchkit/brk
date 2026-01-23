/** On-chain indicators (Pi Cycle, Puell, NVT, Gini) */

import { Unit } from "../../../utils/units.js";
import { priceLine } from "../../constants.js";
import { baseline, line } from "../../series.js";

/**
 * Create On-chain section
 * @param {PartialContext} ctx
 * @param {Object} args
 * @param {Market["indicators"]} args.indicators
 * @param {Market["movingAverage"]} args.movingAverage
 */
export function createOnchainSection(ctx, { indicators, movingAverage }) {
  const { colors } = ctx;

  return {
    name: "On-chain",
    tree: [
      {
        name: "Pi Cycle",
        title: "Pi Cycle",
        top: [
          line({
            metric: movingAverage.price111dSma.price,
            name: "111d SMA",
            color: colors.green,
            unit: Unit.usd,
          }),
          line({
            metric: movingAverage.price350dSmaX2,
            name: "350d SMA x2",
            color: colors.red,
            unit: Unit.usd,
          }),
        ],
        bottom: [
          baseline({
            metric: indicators.piCycle,
            name: "Pi Cycle",
            unit: Unit.ratio,
            base: 1,
          }),
          priceLine({ ctx, unit: Unit.ratio, number: 1 }),
        ],
      },
      {
        name: "Puell Multiple",
        title: "Puell Multiple",
        bottom: [
          line({
            metric: indicators.puellMultiple,
            name: "Puell",
            color: colors.green,
            unit: Unit.ratio,
          }),
        ],
      },
      {
        name: "NVT",
        title: "NVT Ratio",
        bottom: [
          line({
            metric: indicators.nvt,
            name: "NVT",
            color: colors.orange,
            unit: Unit.ratio,
          }),
        ],
      },
      {
        name: "Gini",
        title: "Gini Coefficient",
        bottom: [
          line({
            metric: indicators.gini,
            name: "Gini",
            color: colors.red,
            unit: Unit.ratio,
          }),
        ],
      },
    ],
  };
}
