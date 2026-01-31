/** Investing section (DCA) */

import { Unit } from "../../utils/units.js";
import { priceLine } from "../constants.js";
import { line, baseline, price } from "../series.js";
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
    daysInProfit: dca.classDaysInProfit[`_${year}`],
    daysInLoss: dca.classDaysInLoss[`_${year}`],
    maxDrawdown: dca.classMaxDrawdown[`_${year}`],
    maxReturn: dca.classMaxReturn[`_${year}`],
  }));
}

/**
 * Create DCA vs Lump Sum section
 * @param {PartialContext} ctx
 * @param {Object} args
 * @param {Market["dca"]} args.dca
 * @param {Market["lookback"]} args.lookback
 * @param {Market["returns"]} args.returns
 */
export function createDcaVsLumpSumSection(ctx, { dca, lookback, returns }) {
  const { colors } = ctx;

  /**
   * @param {string} name
   * @param {AllPeriodKey} key
   */
  const costBasisChart = (name, key) => ({
    name: "Cost Basis",
    title: `${name} Cost Basis`,
    top: [
      price({
        metric: dca.periodAveragePrice[key],
        name: "DCA",
        color: colors.green,
      }),
      price({
        metric: lookback[key],
        name: "Lump sum",
        color: colors.orange,
      }),
    ],
  });

  /**
   * @param {string} name
   * @param {AllPeriodKey} key
   */
  const daysInProfitChart = (name, key) => ({
    name: "Days in Profit",
    title: `${name} Days in Profit`,
    top: [
      price({ metric: dca.periodAveragePrice[key], name: "DCA", color: colors.green }),
      price({ metric: lookback[key], name: "Lump sum", color: colors.orange }),
    ],
    bottom: [
      line({ metric: dca.periodDaysInProfit[key], name: "DCA", color: colors.green, unit: Unit.days }),
      line({ metric: dca.periodLumpSumDaysInProfit[key], name: "Lump sum", color: colors.orange, unit: Unit.days }),
    ],
  });

  /**
   * @param {string} name
   * @param {AllPeriodKey} key
   */
  const daysInLossChart = (name, key) => ({
    name: "Days in Loss",
    title: `${name} Days in Loss`,
    top: [
      price({ metric: dca.periodAveragePrice[key], name: "DCA", color: colors.green }),
      price({ metric: lookback[key], name: "Lump sum", color: colors.orange }),
    ],
    bottom: [
      line({ metric: dca.periodDaysInLoss[key], name: "DCA", color: colors.red, unit: Unit.days }),
      line({ metric: dca.periodLumpSumDaysInLoss[key], name: "Lump sum", color: colors.orange, unit: Unit.days }),
    ],
  });

  /**
   * @param {string} name
   * @param {AllPeriodKey} key
   */
  const maxDrawdownChart = (name, key) => ({
    name: "Max Drawdown",
    title: `${name} Max Drawdown`,
    top: [
      price({ metric: dca.periodAveragePrice[key], name: "DCA", color: colors.green }),
      price({ metric: lookback[key], name: "Lump sum", color: colors.orange }),
    ],
    bottom: [
      line({ metric: dca.periodMaxDrawdown[key], name: "DCA", color: colors.green, unit: Unit.percentage }),
      line({ metric: dca.periodLumpSumMaxDrawdown[key], name: "Lump sum", color: colors.orange, unit: Unit.percentage }),
    ],
  });

  /**
   * @param {string} name
   * @param {AllPeriodKey} key
   */
  const maxReturnChart = (name, key) => ({
    name: "Max Return",
    title: `${name} Max Return`,
    top: [
      price({ metric: dca.periodAveragePrice[key], name: "DCA", color: colors.green }),
      price({ metric: lookback[key], name: "Lump sum", color: colors.orange }),
    ],
    bottom: [
      line({ metric: dca.periodMaxReturn[key], name: "DCA", color: colors.green, unit: Unit.percentage }),
      line({ metric: dca.periodLumpSumMaxReturn[key], name: "Lump sum", color: colors.orange, unit: Unit.percentage }),
    ],
  });

  /**
   * @param {string} name
   * @param {AllPeriodKey} key
   */
  const stackChart = (name, key) => ({
    name: "Stack",
    title: `${name} Stack`,
    bottom: [
      ...satsBtcUsd({ pattern: dca.periodStack[key], name: "DCA", color: colors.green }),
      ...satsBtcUsd({ pattern: dca.periodLumpSumStack[key], name: "Lump sum", color: colors.orange }),
    ],
  });

  /**
   * @param {string} id
   * @param {ShortPeriodKey} key
   */
  const createPeriodTree = (id, key) => {
    const name = periodIdToName(id, true);
    return {
      name,
      tree: [
        costBasisChart(name, key),
        {
          name: "Returns",
          title: `${name} Returns`,
          bottom: [
            baseline({
              metric: dca.periodReturns[key],
              name: "DCA",
              unit: Unit.percentage,
            }),
            baseline({
              metric: dca.periodLumpSumReturns[key],
              name: "Lump sum",
              color: [colors.cyan, colors.orange],
              unit: Unit.percentage,
            }),
          ],
        },
        {
          name: "Profitability",
          tree: [
            daysInProfitChart(name, key),
            daysInLossChart(name, key),
            maxDrawdownChart(name, key),
            maxReturnChart(name, key),
          ],
        },
        stackChart(name, key),
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
        costBasisChart(name, key),
        {
          name: "Returns",
          title: `${name} Returns`,
          bottom: [
            baseline({
              metric: dca.periodReturns[key],
              name: "DCA",
              unit: Unit.percentage,
            }),
            baseline({
              metric: dca.periodLumpSumReturns[key],
              name: "Lump sum",
              color: [colors.cyan, colors.orange],
              unit: Unit.percentage,
            }),
            line({
              metric: dca.periodCagr[key],
              name: "DCA CAGR",
              color: colors.purple,
              unit: Unit.percentage,
              defaultActive: false,
            }),
            line({
              metric: returns.cagr[key],
              name: "Lump sum CAGR",
              color: colors.indigo,
              unit: Unit.percentage,
              defaultActive: false,
            }),
            priceLine({ ctx, unit: Unit.percentage }),
          ],
        },
        {
          name: "Profitability",
          tree: [
            daysInProfitChart(name, key),
            daysInLossChart(name, key),
            maxDrawdownChart(name, key),
            maxReturnChart(name, key),
          ],
        },
        stackChart(name, key),
      ],
    };
  };

  return {
    name: "DCA vs Lump Sum",
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
  };
}

/**
 * Create DCA by Year section
 * @param {PartialContext} ctx
 * @param {Object} args
 * @param {Market["dca"]} args.dca
 */
export function createDcaByYearSection(ctx, { dca }) {
  const { colors } = ctx;
  const dcaClasses = buildDcaClasses(colors, dca);

  return {
    name: "DCA by Year",
    tree: [
      // Comparison charts (all years overlaid)
      {
        name: "Compare",
        tree: [
          {
            name: "Cost basis",
            title: "DCA Cost Basis",
            top: dcaClasses.map(({ year, color, defaultActive, costBasis }) =>
              price({
                metric: costBasis,
                name: `${year}`,
                color,
                defaultActive,
              }),
            ),
          },
          {
            name: "Returns",
            title: "DCA Returns",
            bottom: dcaClasses.map(({ year, defaultActive, returns }) =>
              baseline({
                metric: returns,
                name: `${year}`,
                defaultActive,
                unit: Unit.percentage,
              }),
            ),
          },
          {
            name: "Profitability",
            title: "DCA Profitability",
            bottom: [
              ...dcaClasses.map(({ year, color, defaultActive, daysInProfit }) =>
                line({
                  metric: daysInProfit,
                  name: `${year} Days in Profit`,
                  color,
                  defaultActive,
                  unit: Unit.days,
                }),
              ),
              ...dcaClasses.map(({ year, color, daysInLoss }) =>
                line({
                  metric: daysInLoss,
                  name: `${year} Days in Loss`,
                  color,
                  defaultActive: false,
                  unit: Unit.days,
                }),
              ),
            ],
          },
          {
            name: "Stack",
            title: "DCA Stack",
            bottom: dcaClasses.flatMap(
              ({ year, color, defaultActive, stack }) =>
                satsBtcUsd({ pattern: stack, name: `${year}`, color, defaultActive }),
            ),
          },
        ],
      },
      // Individual year charts
      ...dcaClasses.map(
        ({
          year,
          color,
          costBasis,
          returns,
          stack,
          daysInProfit,
          daysInLoss,
          maxDrawdown,
          maxReturn,
        }) => ({
          name: `${year}`,
          tree: [
            {
              name: "Cost Basis",
              title: `${year} Cost Basis`,
              top: [
                price({
                  metric: costBasis,
                  name: "Cost Basis",
                  color,
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
                  unit: Unit.percentage,
                }),
              ],
            },
            {
              name: "Profitability",
              title: `${year} Profitability`,
              bottom: [
                line({
                  metric: daysInProfit,
                  name: "Days in Profit",
                  color: colors.green,
                  unit: Unit.days,
                }),
                line({
                  metric: daysInLoss,
                  name: "Days in Loss",
                  color: colors.red,
                  unit: Unit.days,
                }),
                line({
                  metric: maxDrawdown,
                  name: "Max Drawdown",
                  color: colors.purple,
                  unit: Unit.percentage,
                  defaultActive: false,
                }),
                line({
                  metric: maxReturn,
                  name: "Max Return",
                  color: colors.cyan,
                  unit: Unit.percentage,
                  defaultActive: false,
                }),
              ],
            },
            {
              name: "Stack",
              title: `${year} Stack`,
              bottom: satsBtcUsd({ pattern: stack, name: "Stack", color }),
            },
          ],
        }),
      ),
    ],
  };
}
