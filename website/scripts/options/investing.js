/** Investing section - Investment strategy tools and analysis */

import { colors } from "../utils/colors.js";
import { brk } from "../client.js";
import { percentRatioBaseline, price } from "./series.js";
import { satsBtcUsd } from "./shared.js";
import { periodIdToName } from "./utils.js";

const SHORT_PERIODS = /** @type {const} */ ([
  "_1w",
  "_1m",
  "_3m",
  "_6m",
  "_1y",
]);
const LONG_PERIODS = /** @type {const} */ ([
  "_2y",
  "_3y",
  "_4y",
  "_5y",
  "_6y",
  "_8y",
  "_10y",
]);

/** @typedef {typeof SHORT_PERIODS[number]} ShortPeriodKey */
/** @typedef {typeof LONG_PERIODS[number]} LongPeriodKey */
/** @typedef {ShortPeriodKey | LongPeriodKey} AllPeriodKey */

/**
 * Add CAGR to a base entry item
 * @param {BaseEntryItem} entry
 * @param {PercentRatioPattern} cagr
 * @returns {LongEntryItem}
 */
const withCagr = (entry, cagr) => ({ ...entry, cagr });

const YEARS_2020S = /** @type {const} */ ([
  2026, 2025, 2024, 2023, 2022, 2021, 2020,
]);
const YEARS_2010S = /** @type {const} */ ([2019, 2018, 2017, 2016, 2015]);

/** @typedef {typeof YEARS_2020S[number] | typeof YEARS_2010S[number]} DcaYear */
/** @typedef {`from${DcaYear}`} DcaYearKey */

/** @param {AllPeriodKey} key */
const periodName = (key) => periodIdToName(key.slice(1), true);

/**
 * @typedef {{ percent: AnySeriesPattern, ratio: AnySeriesPattern }} PercentRatioPattern
 */

/**
 * Base entry item for compare and single-entry charts
 * @typedef {Object} BaseEntryItem
 * @property {string} name - Display name
 * @property {Color} color - Item color
 * @property {AnyPricePattern} costBasis - Cost basis series
 * @property {PercentRatioPattern} returns - Returns series
 * @property {AnyValuePattern} stack - Stack pattern
 */

/**
 * Long-term entry item with CAGR
 * @typedef {BaseEntryItem & { cagr: PercentRatioPattern }} LongEntryItem
 */

const ALL_YEARS = /** @type {const} */ ([...YEARS_2020S, ...YEARS_2010S]);

/**
 * Build DCA class entry from year
 * @param {MarketDca} dca
 * @param {DcaYear} year
 * @param {number} i
 * @returns {BaseEntryItem}
 */
function buildYearEntry(dca, year, i) {
  const key = /** @type {DcaYearKey} */ (`from${year}`);
  return {
    name: `${year}`,
    color: colors.at(i, ALL_YEARS.length),
    costBasis: dca.class.costBasis[key],
    returns: dca.class.return[key],
    stack: dca.class.stack[key],
  };
}

/**
 * Create Investing section
 * @returns {PartialOptionsGroup}
 */
export function createInvestingSection() {
  const { market } = brk.series;
  const { dca, lookback, returns } = market;

  return {
    name: "Investing",
    tree: [
      createDcaVsLumpSumSection({ dca, lookback, returns }),
      createDcaByPeriodSection({ dca, returns }),
      createLumpSumByPeriodSection({ dca, lookback, returns }),
      createDcaByStartYearSection({ dca }),
    ],
  };
}

/**
 * Create compare folder from items
 * @param {string} context
 * @param {Pick<BaseEntryItem, 'name' | 'color' | 'costBasis' | 'returns' | 'stack'>[]} items
 */
function createCompareFolder(context, items) {
  const topPane = items.map(({ name, color, costBasis }) =>
    price({ series: costBasis, name, color }),
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
        bottom: items.flatMap(({ name, color, returns }) =>
          percentRatioBaseline({
            pattern: returns,
            name,
            color: [color, color],
          }),
        ),
      },
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
 * Create single entry tree structure
 * @param {BaseEntryItem & { titlePrefix?: string }} item
 * @param {object[]} returnsBottom - Bottom pane items for returns chart
 */
function createSingleEntryTree(item, returnsBottom) {
  const { name, titlePrefix = name, color, costBasis, stack } = item;
  const top = [price({ series: costBasis, name: "Cost Basis", color })];
  return {
    name,
    tree: [
      { name: "Cost Basis", title: `Cost Basis: ${titlePrefix}`, top },
      {
        name: "Returns",
        title: `Returns: ${titlePrefix}`,
        top,
        bottom: returnsBottom,
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
 * Create a single entry from a base item (no CAGR)
 * @param {BaseEntryItem & { titlePrefix?: string }} item
 */
function createShortSingleEntry(item) {
  return createSingleEntryTree(item, percentRatioBaseline({ pattern: item.returns, name: "Current" }));
}

/**
 * Create a single entry from a long item (with CAGR)
 * @param {LongEntryItem & { titlePrefix?: string }} item
 */
function createLongSingleEntry(item) {
  return createSingleEntryTree(item, [
    ...percentRatioBaseline({ pattern: item.returns, name: "Current" }),
    ...percentRatioBaseline({ pattern: item.cagr, name: "CAGR" }),
  ]);
}

/**
 * Create DCA vs Lump Sum section
 * @param {Object} args
 * @param {Market["dca"]} args.dca
 * @param {Market["lookback"]} args.lookback
 * @param {Market["returns"]} args.returns
 */
export function createDcaVsLumpSumSection({ dca, lookback, returns }) {
  /** @param {AllPeriodKey} key */
  const topPane = (key) => [
    price({
      series: dca.period.costBasis[key],
      name: "DCA",
      color: colors.profit,
    }),
    price({ series: lookback[key], name: "Lump Sum", color: colors.bitcoin }),
  ];

  /** @param {string} name @param {AllPeriodKey} key */
  const costBasisChart = (name, key) => ({
    name: "Cost Basis",
    title: `Cost Basis: ${name} DCA vs Lump Sum`,
    top: topPane(key),
  });

  /** @param {string} name @param {ShortPeriodKey} key */
  const shortReturnsChart = (name, key) => ({
    name: "Returns",
    title: `Returns: ${name} DCA vs Lump Sum`,
    top: topPane(key),
    bottom: [
      ...percentRatioBaseline({
        pattern: dca.period.return[key],
        name: "DCA",
      }),
      ...percentRatioBaseline({
        pattern: dca.period.lumpSumReturn[key],
        name: "Lump Sum",
        color: colors.bi.p2,
      }),
    ],
  });

  /** @param {string} name @param {LongPeriodKey} key */
  const longReturnsChart = (name, key) => ({
    name: "Returns",
    title: `Returns: ${name} DCA vs Lump Sum`,
    top: topPane(key),
    bottom: [
      ...percentRatioBaseline({
        pattern: dca.period.return[key],
        name: "DCA",
      }),
      ...percentRatioBaseline({
        pattern: dca.period.lumpSumReturn[key],
        name: "Lump Sum",
        color: colors.bi.p2,
      }),
      ...percentRatioBaseline({
        pattern: dca.period.cagr[key],
        name: "DCA CAGR",
      }),
      ...percentRatioBaseline({
        pattern: returns.cagr[key],
        name: "Lump Sum CAGR",
        color: colors.bi.p2,
      }),
    ],
  });

  /** @param {string} name @param {AllPeriodKey} key */
  const stackChart = (name, key) => ({
    name: "Accumulated",
    title: `Accumulated Value ($100/day): ${name} DCA vs Lump Sum`,
    top: topPane(key),
    bottom: [
      ...satsBtcUsd({
        pattern: dca.period.stack[key],
        name: "DCA",
        color: colors.profit,
      }),
      ...satsBtcUsd({
        pattern: dca.period.lumpSumStack[key],
        name: "Lump Sum",
        color: colors.bitcoin,
      }),
    ],
  });

  /** @param {ShortPeriodKey} key */
  const createShortPeriodEntry = (key) => {
    const name = periodName(key);
    return {
      name,
      tree: [
        costBasisChart(name, key),
        shortReturnsChart(name, key),
        stackChart(name, key),
      ],
    };
  };

  /** @param {LongPeriodKey} key */
  const createLongPeriodEntry = (key) => {
    const name = periodName(key);
    return {
      name,
      tree: [
        costBasisChart(name, key),
        longReturnsChart(name, key),
        stackChart(name, key),
      ],
    };
  };

  return {
    name: "DCA vs Lump Sum",
    title: "Compare Investment Strategies",
    tree: [
      {
        name: "Short Term",
        title: "Up to 1 Year",
        tree: SHORT_PERIODS.map(createShortPeriodEntry),
      },
      {
        name: "Long Term",
        title: "2+ Years",
        tree: LONG_PERIODS.map(createLongPeriodEntry),
      },
    ],
  };
}

/**
 * Create period-based section (DCA or Lump Sum)
 * @param {Object} args
 * @param {Market["dca"]} args.dca
 * @param {Market["lookback"]} [args.lookback]
 * @param {Market["returns"]} args.returns
 */
function createPeriodSection({ dca, lookback, returns }) {
  const isLumpSum = !!lookback;
  const suffix = isLumpSum ? "Lump Sum" : "DCA";

  const allPeriods = /** @type {const} */ ([...SHORT_PERIODS, ...LONG_PERIODS]);

  /** @param {AllPeriodKey} key @param {number} i @returns {BaseEntryItem} */
  const buildBaseEntry = (key, i) => ({
    name: periodName(key),
    color: colors.at(i, allPeriods.length),
    costBasis: isLumpSum ? lookback[key] : dca.period.costBasis[key],
    returns: isLumpSum
      ? dca.period.lumpSumReturn[key]
      : dca.period.return[key],
    stack: isLumpSum
      ? dca.period.lumpSumStack[key]
      : dca.period.stack[key],
  });

  /** @param {LongPeriodKey} key @param {number} i @returns {LongEntryItem} */
  const buildLongEntry = (key, i) =>
    withCagr(
      buildBaseEntry(key, i),
      isLumpSum ? returns.cagr[key] : dca.period.cagr[key],
    );

  /** @param {BaseEntryItem} entry */
  const createShortEntry = (entry) =>
    createShortSingleEntry({
      ...entry,
      titlePrefix: `${entry.name} ${suffix}`,
    });

  /** @param {LongEntryItem} entry */
  const createLongEntry = (entry) =>
    createLongSingleEntry({
      ...entry,
      titlePrefix: `${entry.name} ${suffix}`,
    });

  const shortEntries = SHORT_PERIODS.map((key, i) => buildBaseEntry(key, i));
  const longEntries = LONG_PERIODS.map((key, i) =>
    buildLongEntry(key, SHORT_PERIODS.length + i),
  );

  return {
    name: `${suffix} by Period`,
    title: `${suffix} Performance by Investment Period`,
    tree: [
      createCompareFolder(`All Periods ${suffix}`, [
        ...shortEntries,
        ...longEntries,
      ]),
      {
        name: "Short Term",
        title: "Up to 1 Year",
        tree: [
          createCompareFolder(`Short Term ${suffix}`, shortEntries),
          ...shortEntries.map(createShortEntry),
        ],
      },
      {
        name: "Long Term",
        title: "2+ Years",
        tree: [
          createCompareFolder(`Long Term ${suffix}`, longEntries),
          ...longEntries.map(createLongEntry),
        ],
      },
    ],
  };
}

/**
 * Create DCA by Period section
 * @param {Object} args
 * @param {Market["dca"]} args.dca
 * @param {Market["returns"]} args.returns
 */
export function createDcaByPeriodSection({ dca, returns }) {
  return createPeriodSection({ dca, returns });
}

/**
 * Create Lump Sum by Period section
 * @param {Object} args
 * @param {Market["dca"]} args.dca
 * @param {Market["lookback"]} args.lookback
 * @param {Market["returns"]} args.returns
 */
export function createLumpSumByPeriodSection({ dca, lookback, returns }) {
  return createPeriodSection({ dca, lookback, returns });
}

/**
 * Create DCA by Start Year section
 * @param {Object} args
 * @param {Market["dca"]} args.dca
 */
export function createDcaByStartYearSection({ dca }) {
  /** @param {string} name @param {string} title @param {BaseEntryItem[]} entries */
  const createDecadeGroup = (name, title, entries) => ({
    name,
    title,
    tree: [
      createCompareFolder(`${name} DCA`, entries),
      ...entries.map((entry) =>
        createShortSingleEntry({
          ...entry,
          titlePrefix: `${entry.name} DCA`,
        }),
      ),
    ],
  });

  const entries2020s = YEARS_2020S.map((year, i) =>
    buildYearEntry(dca, year, i),
  );
  const entries2010s = YEARS_2010S.map((year, i) =>
    buildYearEntry(dca, year, YEARS_2020S.length + i),
  );

  return {
    name: "DCA by Start Year",
    title: "DCA Performance by When You Started",
    tree: [
      createCompareFolder("All Years DCA", [...entries2020s, ...entries2010s]),
      createDecadeGroup("2020s", "2020-2026", entries2020s),
      createDecadeGroup("2010s", "2015-2019", entries2010s),
    ],
  };
}
