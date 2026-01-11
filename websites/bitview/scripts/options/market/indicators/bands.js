/** Bands indicators (MinMax, Mayer Multiple) */

import { Unit } from "../../../utils/units.js";

/**
 * Create Bands section
 * @param {PartialContext} ctx
 * @param {Object} args
 * @param {Market["range"]} args.range
 * @param {Market["movingAverage"]} args.movingAverage
 */
export function createBandsSection(ctx, { range, movingAverage }) {
  const { line, colors } = ctx;

  return {
    name: "Bands",
    tree: [
      {
        name: "MinMax",
        tree: [
          {
            id: "1w",
            title: "1 Week",
            min: range.price1wMin,
            max: range.price1wMax,
          },
          {
            id: "2w",
            title: "2 Week",
            min: range.price2wMin,
            max: range.price2wMax,
          },
          {
            id: "1m",
            title: "1 Month",
            min: range.price1mMin,
            max: range.price1mMax,
          },
          {
            id: "1y",
            title: "1 Year",
            min: range.price1yMin,
            max: range.price1yMax,
          },
        ].map(({ id, title, min, max }) => ({
          name: id,
          title: `Bitcoin Price ${title} MinMax Bands`,
          top: [
            line({
              metric: min,
              name: "Min",
              color: colors.red,
              unit: Unit.usd,
            }),
            line({
              metric: max,
              name: "Max",
              color: colors.green,
              unit: Unit.usd,
            }),
          ],
        })),
      },
      {
        name: "Mayer Multiple",
        title: "Mayer Multiple",
        top: [
          line({
            metric: movingAverage.price200dSma.price,
            name: "200d SMA",
            color: colors.yellow,
            unit: Unit.usd,
          }),
          line({
            metric: movingAverage.price200dSmaX24,
            name: "200d SMA x2.4",
            color: colors.green,
            unit: Unit.usd,
          }),
          line({
            metric: movingAverage.price200dSmaX08,
            name: "200d SMA x0.8",
            color: colors.red,
            unit: Unit.usd,
          }),
        ],
      },
    ],
  };
}
