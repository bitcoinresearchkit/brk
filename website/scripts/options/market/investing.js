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

  /**
   * @param {string} id
   * @param {ShortPeriodKey} key
   */
  const createPeriodTree = (id, key) => {
    const name = periodIdToName(id, true);
    return {
      name,
      tree: [
        {
          name: "Cost basis",
          title: `${name} Cost Basis`,
          top: [
            line({ metric: dca.periodAveragePrice[key], name: "DCA", color: colors.green, unit: Unit.usd }),
            line({ metric: lookback[key], name: "Lump sum", color: colors.orange, unit: Unit.usd }),
          ],
        },
        {
          name: "Returns",
          title: `${name} Returns`,
          bottom: [
            baseline({ metric: dca.periodReturns[key], name: "DCA", unit: Unit.percentage }),
            baseline({ metric: returns.priceReturns[key], name: "Lump sum", color: [colors.cyan, colors.orange], unit: Unit.percentage }),
            priceLine({ ctx, unit: Unit.percentage }),
          ],
        },
        {
          name: "Stack",
          title: `${name} Stack`,
          bottom: [
            ...satsBtcUsd(dca.periodStack[key], "DCA", colors.green),
            ...satsBtcUsd(dca.periodLumpSumStack[key], "Lump sum", colors.orange),
          ],
        },
      ],
    };
  };

  /**
   * @param {string} id
   * @param {LongPeriodKey} key
   */
  const createPeriodTreeWithCagr = (id, key) => {
    const name = periodIdToName(id, true);
    return {
      name,
      tree: [
        {
          name: "Cost basis",
          title: `${name} Cost Basis`,
          top: [
            line({ metric: dca.periodAveragePrice[key], name: "DCA", color: colors.green, unit: Unit.usd }),
            line({ metric: lookback[key], name: "Lump sum", color: colors.orange, unit: Unit.usd }),
          ],
        },
        {
          name: "Returns",
          title: `${name} Returns`,
          bottom: [
            baseline({ metric: dca.periodReturns[key], name: "DCA", unit: Unit.percentage }),
            baseline({ metric: returns.priceReturns[key], name: "Lump sum", color: [colors.cyan, colors.orange], unit: Unit.percentage }),
            line({ metric: dca.periodCagr[key], name: "DCA CAGR", color: colors.purple, unit: Unit.percentage, defaultActive: false }),
            line({ metric: returns.cagr[key], name: "Lump sum CAGR", color: colors.indigo, unit: Unit.percentage, defaultActive: false }),
            priceLine({ ctx, unit: Unit.percentage }),
          ],
        },
        {
          name: "Stack",
          title: `${name} Stack`,
          bottom: [
            ...satsBtcUsd(dca.periodStack[key], "DCA", colors.green),
            ...satsBtcUsd(dca.periodLumpSumStack[key], "Lump sum", colors.orange),
          ],
        },
      ],
    };
  };

  return {
    name: "Investing",
    tree: [
      // DCA vs Lump sum
      {
        name: "DCA vs Lump sum",
        tree: [
          createPeriodTree("1w", "_1w"),
          createPeriodTree("1m", "_1m"),
          createPeriodTree("3m", "_3m"),
          createPeriodTree("6m", "_6m"),
          createPeriodTree("1y", "_1y"),
          createPeriodTreeWithCagr("2y", "_2y"),
          createPeriodTreeWithCagr("3y", "_3y"),
          createPeriodTreeWithCagr("4y", "_4y"),
          createPeriodTreeWithCagr("5y", "_5y"),
          createPeriodTreeWithCagr("6y", "_6y"),
          createPeriodTreeWithCagr("8y", "_8y"),
          createPeriodTreeWithCagr("10y", "_10y"),
        ],
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
