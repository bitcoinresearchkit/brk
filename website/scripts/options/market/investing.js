/** Investing section (DCA) */

import { Unit } from "../../utils/units.js";
import { priceLine } from "../constants.js";
import { line, baseline } from "../series.js";
import { satsBtcUsd } from "../shared.js";
import { periodIdToName } from "./utils.js";

/**
 * Build DCA classes data array
 * @param {Colors} colors
 * @param {MarketDca} dca
 */
export function buildDcaClasses(colors, dca) {
  return /** @type {const} */ ([
    [2026, "rose", true],
    [2025, "pink", true],
    [2024, "fuchsia", true],
    [2023, "purple", true],
    [2022, "blue", true],
    [2021, "sky", true],
    [2020, "teal", true],
    [2019, "green", true],
    [2018, "yellow", true],
    [2017, "orange", true],
    [2016, "red", false],
    [2015, "pink", false],
  ]).map(([year, colorKey, defaultActive]) => ({
    year,
    color: colors[colorKey],
    defaultActive,
    costBasis: dca.classAveragePrice[`_${year}`],
    returns: dca.classReturns[`_${year}`],
    stack: dca.classStack[`_${year}`],
  }));
}

/**
 * Create Investing section
 * @param {PartialContext} ctx
 * @param {Object} args
 * @param {Market["dca"]} args.dca
 * @param {Market["lookback"]} args.lookback
 * @param {Market["returns"]} args.returns
 */
export function createInvestingSection(ctx, { dca, lookback, returns }) {
  const { colors } = ctx;
  const dcaClasses = buildDcaClasses(colors, dca);

  return {
    name: "Investing",
    tree: [
      // DCA vs Lump sum
      {
        name: "DCA vs Lump sum",
        tree: /** @type {const} */ ([
          ["1w", "_1w"],
          ["1m", "_1m"],
          ["3m", "_3m"],
          ["6m", "_6m"],
          ["1y", "_1y"],
          ["2y", "_2y"],
          ["3y", "_3y"],
          ["4y", "_4y"],
          ["5y", "_5y"],
          ["6y", "_6y"],
          ["8y", "_8y"],
          ["10y", "_10y"],
        ]).map(([id, key]) => {
          const name = periodIdToName(id, true);
          const priceAgo = lookback[key];
          const priceReturns = returns.priceReturns[key];
          const dcaCostBasis = dca.periodAveragePrice[key];
          const dcaReturns = dca.periodReturns[key];
          const dcaStack = dca.periodStack[key];
          const lumpSumStack = dca.periodLumpSumStack[key];
          return {
            name,
            tree: [
              {
                name: "Cost basis",
                title: `${name} Cost Basis`,
                top: [
                  line({
                    metric: dcaCostBasis,
                    name: "DCA",
                    color: colors.green,
                    unit: Unit.usd,
                  }),
                  line({
                    metric: priceAgo,
                    name: "Lump sum",
                    color: colors.orange,
                    unit: Unit.usd,
                  }),
                ],
              },
              {
                name: "Returns",
                title: `${name} Returns`,
                bottom: [
                  baseline({
                    metric: dcaReturns,
                    name: "DCA",
                    unit: Unit.percentage,
                  }),
                  baseline({
                    metric: priceReturns,
                    name: "Lump sum",
                    color: [colors.cyan, colors.orange],
                    unit: Unit.percentage,
                  }),
                  priceLine({ ctx, unit: Unit.percentage }),
                ],
              },
              {
                name: "Stack",
                title: `${name} Stack`,
                bottom: [
                  ...satsBtcUsd(dcaStack, "DCA", colors.green),
                  ...satsBtcUsd(lumpSumStack, "Lump sum", colors.orange),
                ],
              },
            ],
          };
        }),
      },

      // DCA classes
      {
        name: "DCA classes",
        tree: [
          // Comparison charts (all years overlaid)
          {
            name: "Compare",
            tree: [
              {
                name: "Cost basis",
                title: "DCA Cost Basis",
                top: dcaClasses.map(
                  ({ year, color, defaultActive, costBasis }) =>
                    line({
                      metric: costBasis,
                      name: `${year}`,
                      color,
                      defaultActive,
                      unit: Unit.usd,
                    }),
                ),
              },
              {
                name: "Returns",
                title: "DCA Returns",
                bottom: dcaClasses.map(
                  ({ year, color, defaultActive, returns }) =>
                    baseline({
                      metric: returns,
                      name: `${year}`,
                      color,
                      defaultActive,
                      unit: Unit.percentage,
                    }),
                ),
              },
              {
                name: "Stack",
                title: "DCA Stack",
                bottom: dcaClasses.flatMap(
                  ({ year, color, defaultActive, stack }) =>
                    satsBtcUsd(stack, `${year}`, color, { defaultActive }),
                ),
              },
            ],
          },
          // Individual year charts
          ...dcaClasses.map(({ year, color, costBasis, returns, stack }) => ({
            name: `${year}`,
            tree: [
              {
                name: "Cost basis",
                title: `${year} Cost Basis`,
                top: [
                  line({
                    metric: costBasis,
                    name: "Cost basis",
                    color,
                    unit: Unit.usd,
                  }),
                ],
              },
              {
                name: "Returns",
                title: `${year} Returns`,
                bottom: [
                  baseline({
                    metric: returns,
                    name: "Returns",
                    color,
                    unit: Unit.percentage,
                  }),
                ],
              },
              {
                name: "Stack",
                title: `${year} Stack`,
                bottom: satsBtcUsd(stack, "Stack", color),
              },
            ],
          })),
        ],
      },
    ],
  };
}
