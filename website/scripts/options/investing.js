/** Investing section - Investment strategy tools and analysis */

import { Unit } from "../utils/units.js";
import { priceLine } from "./constants.js";
import { line, baseline, price, dotted } from "./series.js";
import { satsBtcUsd } from "./shared.js";
import { periodIdToName } from "./utils.js";

/**
 * Create Investing section
 * @param {PartialContext} ctx
 * @returns {PartialOptionsGroup}
 */
export function createInvestingSection(ctx) {
  const { brk } = ctx;
  const { market } = brk.metrics;
  const { dca, lookback, returns } = market;

  return {
    name: "Investing",
    tree: [
      createDcaVsLumpSumSection(ctx, { dca, lookback, returns }),
      createDcaByPeriodSection(ctx, { dca }),
      createLumpSumByPeriodSection(ctx, { dca, lookback }),
      createDcaByStartYearSection(ctx, { dca }),
    ],
  };
}

/** Period configuration by term group */
const PERIODS = {
  short: [
    { id: "1w", key: /** @type {const} */ ("_1w") },
    { id: "1m", key: /** @type {const} */ ("_1m") },
    { id: "3m", key: /** @type {const} */ ("_3m") },
    { id: "6m", key: /** @type {const} */ ("_6m") },
  ],
  medium: [
    { id: "1y", key: /** @type {const} */ ("_1y") },
    { id: "2y", key: /** @type {const} */ ("_2y") },
    { id: "3y", key: /** @type {const} */ ("_3y") },
  ],
  long: [
    { id: "4y", key: /** @type {const} */ ("_4y") },
    { id: "5y", key: /** @type {const} */ ("_5y") },
    { id: "6y", key: /** @type {const} */ ("_6y") },
    { id: "8y", key: /** @type {const} */ ("_8y") },
    { id: "10y", key: /** @type {const} */ ("_10y") },
  ],
};

const ALL_PERIODS = [...PERIODS.short, ...PERIODS.medium, ...PERIODS.long];

/** DCA year classes by decade */
const YEAR_GROUPS = {
  _2020s: /** @type {const} */ ([2026, 2025, 2024, 2023, 2022, 2021, 2020]),
  _2010s: /** @type {const} */ ([2019, 2018, 2017, 2016, 2015]),
};

const ALL_YEARS = [...YEAR_GROUPS._2020s, ...YEAR_GROUPS._2010s];

/** @typedef {ReturnType<typeof buildYearClass>} YearClass */

/**
 * Build DCA class data from year
 * @param {Colors} colors
 * @param {MarketDca} dca
 * @param {number} year
 */
function buildYearClass(colors, dca, year) {
  const key = /** @type {keyof Colors["dcaYears"]} */ (`_${year}`);
  return {
    year,
    color: colors.dcaYears[key],
    costBasis: dca.classAveragePrice[key],
    returns: dca.classReturns[key],
    stack: dca.classStack[key],
    daysInProfit: dca.classDaysInProfit[key],
    daysInLoss: dca.classDaysInLoss[key],
    minReturn: dca.classMinReturn[key],
    maxReturn: dca.classMaxReturn[key],
  };
}

/**
 * Pattern for creating a single entry (period or year)
 * @typedef {Object} SingleEntryPattern
 * @property {string} name - Display name
 * @property {string} [titlePrefix] - Prefix for chart titles (defaults to name)
 * @property {Color} color - Primary color
 * @property {AnyPricePattern} costBasis - Cost basis metric
 * @property {AnyMetricPattern} returns - Returns metric
 * @property {AnyMetricPattern} minReturn - Min return metric
 * @property {AnyMetricPattern} maxReturn - Max return metric
 * @property {AnyMetricPattern} daysInProfit - Days in profit metric
 * @property {AnyMetricPattern} daysInLoss - Days in loss metric
 * @property {AnyValuePattern} stack - Stack pattern
 */

/**
 * Item for compare charts
 * @typedef {Object} CompareItem
 * @property {string} name - Display name
 * @property {Color} color - Item color
 * @property {AnyPricePattern} costBasis - Cost basis metric
 * @property {AnyMetricPattern} returns - Returns metric
 * @property {AnyMetricPattern} daysInProfit - Days in profit metric
 * @property {AnyMetricPattern} daysInLoss - Days in loss metric
 * @property {AnyValuePattern} stack - Stack pattern
 */

/**
 * Create profitability folder for compare charts
 * @param {string} context
 * @param {CompareItem[]} items
 */
function createProfitabilityFolder(context, items) {
  const top = items.map(({ name, color, costBasis }) =>
    price({ metric: costBasis, name, color }),
  );
  return {
    name: "Profitability",
    tree: [
      {
        name: "Days in Profit",
        title: `Days in Profit: ${context}`,
        top,
        bottom: items.map(({ name, color, daysInProfit }) =>
          line({ metric: daysInProfit, name, color, unit: Unit.days }),
        ),
      },
      {
        name: "Days in Loss",
        title: `Days in Loss: ${context}`,
        top,
        bottom: items.map(({ name, color, daysInLoss }) =>
          line({ metric: daysInLoss, name, color, unit: Unit.days }),
        ),
      },
    ],
  };
}

/**
 * Create compare folder from items
 * @param {string} context
 * @param {CompareItem[]} items
 */
function createCompareFolder(context, items) {
  const topPane = items.map(({ name, color, costBasis }) =>
    price({ metric: costBasis, name, color }),
  );
  return {
    name: "Compare",
    tree: [
      {
        name: "Cost Basis",
        title: `Cost Basis: ${context}`,
        top: topPane,
      },
      {
        name: "Returns",
        title: `Returns: ${context}`,
        top: topPane,
        bottom: items.map(({ name, color, returns }) =>
          baseline({
            metric: returns,
            name,
            color: [color, color],
            unit: Unit.percentage,
          }),
        ),
      },
      createProfitabilityFolder(context, items),
      {
        name: "Accumulated",
        title: `Accumulated Value: ${context}`,
        top: topPane,
        bottom: items.flatMap(({ name, color, stack }) =>
          satsBtcUsd({ pattern: stack, name, color }),
        ),
      },
    ],
  };
}

/**
 * Create a single entry from a pattern
 * @param {Colors} colors
 * @param {SingleEntryPattern} pattern
 */
function createSingleEntry(colors, pattern) {
  const {
    name,
    titlePrefix = name,
    color,
    costBasis,
    returns,
    minReturn,
    maxReturn,
    daysInProfit,
    daysInLoss,
    stack,
  } = pattern;
  const top = [price({ metric: costBasis, name: "Cost Basis", color })];
  return {
    name,
    tree: [
      { name: "Cost Basis", title: `Cost Basis: ${titlePrefix}`, top },
      {
        name: "Returns",
        title: `Returns: ${titlePrefix}`,
        top,
        bottom: [
          baseline({ metric: returns, name: "Current", unit: Unit.percentage }),
          dotted({
            metric: maxReturn,
            name: "Max",
            color: colors.green,
            unit: Unit.percentage,
            defaultActive: false,
          }),
          dotted({
            metric: minReturn,
            name: "Min",
            color: colors.red,
            unit: Unit.percentage,
            defaultActive: false,
          }),
        ],
      },
      {
        name: "Profitability",
        title: `Profitability: ${titlePrefix}`,
        top,
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
        ],
      },
      {
        name: "Accumulated",
        title: `Accumulated Value: ${titlePrefix}`,
        top,
        bottom: satsBtcUsd({ pattern: stack, name: "Value" }),
      },
    ],
  };
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

  // Chart builders
  /** @param {AllPeriodKey} key */
  const topPane = (key) => [
    price({
      metric: dca.periodAveragePrice[key],
      name: "DCA",
      color: colors.green,
    }),
    price({ metric: lookback[key], name: "Lump Sum", color: colors.orange }),
  ];

  /** @param {string} name @param {AllPeriodKey} key */
  const costBasisChart = (name, key) => ({
    name: "Cost Basis",
    title: `Cost Basis: ${name} DCA vs Lump Sum`,
    top: topPane(key),
  });

  /** @param {string} name @param {AllPeriodKey} key */
  const returnsFolder = (name, key) => ({
    name: "Returns",
    tree: [
      {
        name: "Current",
        title: `Returns: ${name} DCA vs Lump Sum`,
        top: topPane(key),
        bottom: [
          baseline({
            metric: dca.periodReturns[key],
            name: "DCA",
            unit: Unit.percentage,
          }),
          baseline({
            metric: dca.periodLumpSumReturns[key],
            name: "Lump Sum",
            color: [colors.cyan, colors.orange],
            unit: Unit.percentage,
          }),
        ],
      },
      {
        name: "Max",
        title: `Max Return: ${name} DCA vs Lump Sum`,
        top: topPane(key),
        bottom: [
          baseline({
            metric: dca.periodMaxReturn[key],
            name: "DCA",
            unit: Unit.percentage,
          }),
          baseline({
            metric: dca.periodLumpSumMaxReturn[key],
            name: "Lump Sum",
            color: [colors.cyan, colors.orange],
            unit: Unit.percentage,
          }),
        ],
      },
      {
        name: "Min",
        title: `Min Return: ${name} DCA vs Lump Sum`,
        top: topPane(key),
        bottom: [
          baseline({
            metric: dca.periodMinReturn[key],
            name: "DCA",
            unit: Unit.percentage,
          }),
          baseline({
            metric: dca.periodLumpSumMinReturn[key],
            name: "Lump Sum",
            color: [colors.cyan, colors.orange],
            unit: Unit.percentage,
          }),
        ],
      },
    ],
  });

  /** @param {string} name @param {LongPeriodKey} key */
  const returnsFolderWithCagr = (name, key) => ({
    name: "Returns",
    tree: [
      {
        name: "Current",
        title: `Returns: ${name} DCA vs Lump Sum`,
        top: topPane(key),
        bottom: [
          baseline({
            metric: dca.periodReturns[key],
            name: "DCA",
            unit: Unit.percentage,
          }),
          baseline({
            metric: dca.periodLumpSumReturns[key],
            name: "Lump Sum",
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
            name: "Lump Sum CAGR",
            color: colors.indigo,
            unit: Unit.percentage,
            defaultActive: false,
          }),
          priceLine({ ctx, unit: Unit.percentage }),
        ],
      },
      {
        name: "Max",
        title: `Max Return: ${name} DCA vs Lump Sum`,
        top: topPane(key),
        bottom: [
          line({
            metric: dca.periodMaxReturn[key],
            name: "DCA",
            color: colors.green,
            unit: Unit.percentage,
          }),
          line({
            metric: dca.periodLumpSumMaxReturn[key],
            name: "Lump Sum",
            color: colors.orange,
            unit: Unit.percentage,
          }),
        ],
      },
      {
        name: "Min",
        title: `Min Return: ${name} DCA vs Lump Sum`,
        top: topPane(key),
        bottom: [
          line({
            metric: dca.periodMinReturn[key],
            name: "DCA",
            color: colors.green,
            unit: Unit.percentage,
          }),
          line({
            metric: dca.periodLumpSumMinReturn[key],
            name: "Lump Sum",
            color: colors.orange,
            unit: Unit.percentage,
          }),
        ],
      },
    ],
  });

  /** @param {string} name @param {AllPeriodKey} key */
  const profitabilityFolder = (name, key) => ({
    name: "Profitability",
    tree: [
      {
        name: "Days in Profit",
        title: `Days in Profit: ${name} DCA vs Lump Sum`,
        top: topPane(key),
        bottom: [
          line({
            metric: dca.periodDaysInProfit[key],
            name: "DCA",
            color: colors.green,
            unit: Unit.days,
          }),
          line({
            metric: dca.periodLumpSumDaysInProfit[key],
            name: "Lump Sum",
            color: colors.orange,
            unit: Unit.days,
          }),
        ],
      },
      {
        name: "Days in Loss",
        title: `Days in Loss: ${name} DCA vs Lump Sum`,
        top: topPane(key),
        bottom: [
          line({
            metric: dca.periodDaysInLoss[key],
            name: "DCA",
            color: colors.green,
            unit: Unit.days,
          }),
          line({
            metric: dca.periodLumpSumDaysInLoss[key],
            name: "Lump Sum",
            color: colors.orange,
            unit: Unit.days,
          }),
        ],
      },
    ],
  });

  /** @param {string} name @param {AllPeriodKey} key */
  const stackChart = (name, key) => ({
    name: "Accumulated",
    title: `Accumulated Value: ${name} DCA vs Lump Sum`,
    top: topPane(key),
    bottom: [
      ...satsBtcUsd({
        pattern: dca.periodStack[key],
        name: "DCA",
        color: colors.green,
      }),
      ...satsBtcUsd({
        pattern: dca.periodLumpSumStack[key],
        name: "Lump Sum",
        color: colors.orange,
      }),
    ],
  });

  /**
   * Check if a period key has CAGR data
   * @param {AllPeriodKey} key
   * @returns {key is LongPeriodKey}
   */
  const hasCagr = (key) => key in dca.periodCagr;

  /**
   * Create individual period entry
   * @param {{ id: string, key: AllPeriodKey }} period
   */
  const createPeriodEntry = ({ id, key }) => {
    const name = periodIdToName(id, true);
    return {
      name,
      tree: [
        costBasisChart(name, key),
        hasCagr(key)
          ? returnsFolderWithCagr(name, key)
          : returnsFolder(name, key),
        profitabilityFolder(name, key),
        stackChart(name, key),
      ],
    };
  };

  /**
   * Create term group
   * @param {string} name
   * @param {string} title
   * @param {{ id: string, key: AllPeriodKey }[]} periods
   */
  const createTermGroup = (name, title, periods) => ({
    name,
    title,
    tree: periods.map(createPeriodEntry),
  });

  return {
    name: "DCA vs Lump Sum",
    title: "Compare Investment Strategies",
    tree: [
      createTermGroup("Short Term", "Under 1 Year", PERIODS.short),
      createTermGroup("Medium Term", "1-3 Years", PERIODS.medium),
      createTermGroup("Long Term", "4+ Years", PERIODS.long),
    ],
  };
}

/**
 * Create DCA by Period section (DCA only, no Lump Sum comparison)
 * @param {PartialContext} ctx
 * @param {Object} args
 * @param {Market["dca"]} args.dca
 */
export function createDcaByPeriodSection(ctx, { dca }) {
  const { colors } = ctx;

  /**
   * Create compare charts for a set of periods
   * @param {string} context
   * @param {{ id: string, key: AllPeriodKey }[]} periods
   */
  const createCompare = (context, periods) =>
    createCompareFolder(
      context,
      periods.map(({ id, key }) => ({
        name: id,
        color: colors.dcaPeriods[key],
        costBasis: dca.periodAveragePrice[key],
        returns: dca.periodReturns[key],
        daysInProfit: dca.periodDaysInProfit[key],
        daysInLoss: dca.periodDaysInLoss[key],
        stack: dca.periodStack[key],
      })),
    );

  /**
   * Create individual period entry (DCA only)
   * @param {{ id: string, key: AllPeriodKey }} period
   */
  const createPeriodEntry = ({ id, key }) => {
    const name = periodIdToName(id, true);
    return createSingleEntry(colors, {
      name,
      titlePrefix: `${name} DCA`,
      color: colors.dcaPeriods[key],
      costBasis: dca.periodAveragePrice[key],
      returns: dca.periodReturns[key],
      maxReturn: dca.periodMaxReturn[key],
      minReturn: dca.periodMinReturn[key],
      daysInProfit: dca.periodDaysInProfit[key],
      daysInLoss: dca.periodDaysInLoss[key],
      stack: dca.periodStack[key],
    });
  };

  /** @param {string} name @param {string} title @param {{ id: string, key: AllPeriodKey }[]} periods */
  const createTermGroup = (name, title, periods) => ({
    name,
    title,
    tree: [
      createCompare(`${name} DCA`, periods),
      ...periods.map(createPeriodEntry),
    ],
  });

  return {
    name: "DCA by Period",
    title: "DCA Performance by Investment Period",
    tree: [
      createCompare("All Periods DCA", ALL_PERIODS),
      createTermGroup("Short Term", "Under 1 Year", PERIODS.short),
      createTermGroup("Medium Term", "1-3 Years", PERIODS.medium),
      createTermGroup("Long Term", "4+ Years", PERIODS.long),
    ],
  };
}

/**
 * Create Lump Sum by Period section
 * @param {PartialContext} ctx
 * @param {Object} args
 * @param {Market["dca"]} args.dca
 * @param {Market["lookback"]} args.lookback
 */
export function createLumpSumByPeriodSection(ctx, { dca, lookback }) {
  const { colors } = ctx;

  /**
   * Create compare charts for a set of periods
   * @param {string} context
   * @param {{ id: string, key: AllPeriodKey }[]} periods
   */
  const createCompare = (context, periods) =>
    createCompareFolder(
      context,
      periods.map(({ id, key }) => ({
        name: id,
        color: colors.dcaPeriods[key],
        costBasis: lookback[key],
        returns: dca.periodLumpSumReturns[key],
        daysInProfit: dca.periodLumpSumDaysInProfit[key],
        daysInLoss: dca.periodLumpSumDaysInLoss[key],
        stack: dca.periodLumpSumStack[key],
      })),
    );

  /**
   * Create individual period entry (Lump Sum only)
   * @param {{ id: string, key: AllPeriodKey }} period
   */
  const createPeriodEntry = ({ id, key }) => {
    const name = periodIdToName(id, true);
    return createSingleEntry(colors, {
      name,
      titlePrefix: `${name} Lump Sum`,
      color: colors.dcaPeriods[key],
      costBasis: lookback[key],
      returns: dca.periodLumpSumReturns[key],
      maxReturn: dca.periodLumpSumMaxReturn[key],
      minReturn: dca.periodLumpSumMinReturn[key],
      daysInProfit: dca.periodLumpSumDaysInProfit[key],
      daysInLoss: dca.periodLumpSumDaysInLoss[key],
      stack: dca.periodLumpSumStack[key],
    });
  };

  /** @param {string} name @param {string} title @param {{ id: string, key: AllPeriodKey }[]} periods */
  const createTermGroup = (name, title, periods) => ({
    name,
    title,
    tree: [
      createCompare(`${name} Lump Sum`, periods),
      ...periods.map(createPeriodEntry),
    ],
  });

  return {
    name: "Lump Sum by Period",
    title: "Lump Sum Performance by Investment Period",
    tree: [
      createCompare("All Periods Lump Sum", ALL_PERIODS),
      createTermGroup("Short Term", "Under 1 Year", PERIODS.short),
      createTermGroup("Medium Term", "1-3 Years", PERIODS.medium),
      createTermGroup("Long Term", "4+ Years", PERIODS.long),
    ],
  };
}

/**
 * Create DCA by Start Year section
 * @param {PartialContext} ctx
 * @param {Object} args
 * @param {Market["dca"]} args.dca
 */
export function createDcaByStartYearSection(ctx, { dca }) {
  const { colors } = ctx;

  /**
   * Convert YearClass to CompareItem
   * @param {YearClass} c
   * @returns {CompareItem}
   */
  const toCompareItem = (c) => ({
    name: `${c.year}`,
    color: c.color,
    costBasis: c.costBasis,
    returns: c.returns,
    daysInProfit: c.daysInProfit,
    daysInLoss: c.daysInLoss,
    stack: c.stack,
  });

  /**
   * Create individual year entry
   * @param {YearClass} yearClass
   */
  const createYearEntry = (yearClass) =>
    createSingleEntry(colors, {
      name: `${yearClass.year}`,
      titlePrefix: `${yearClass.year} DCA`,
      color: yearClass.color,
      costBasis: yearClass.costBasis,
      returns: yearClass.returns,
      maxReturn: yearClass.maxReturn,
      minReturn: yearClass.minReturn,
      daysInProfit: yearClass.daysInProfit,
      daysInLoss: yearClass.daysInLoss,
      stack: yearClass.stack,
    });

  /** @param {string} name @param {string} title @param {YearClass[]} classes */
  const createDecadeGroup = (name, title, classes) => ({
    name,
    title,
    tree: [
      createCompareFolder(`${name} DCA`, classes.map(toCompareItem)),
      ...classes.map(createYearEntry),
    ],
  });

  // Build all classes once, then filter by decade
  const allClasses = ALL_YEARS.map((year) => buildYearClass(colors, dca, year));
  const classes2020s = allClasses.filter((c) => c.year >= 2020);
  const classes2010s = allClasses.filter((c) => c.year < 2020);

  return {
    name: "DCA by Start Year",
    title: "DCA Performance by When You Started",
    tree: [
      createCompareFolder("All Years DCA", allClasses.map(toCompareItem)),
      createDecadeGroup("2020s", "2020-2026", classes2020s),
      createDecadeGroup("2010s", "2015-2019", classes2010s),
    ],
  };
}
