/** Market section */

import { colors } from "../utils/colors.js";
import { brk } from "../client.js";
import { includes } from "../utils/array.js";
import { Unit } from "../utils/units.js";
import { priceLine, priceLines } from "./constants.js";
import {
  baseline,
  histogram,
  line,
  price,
  percentRatio,
  percentRatioBaseline,
} from "./series.js";
import { periodIdToName } from "./utils.js";

/**
 * @typedef {Object} Period
 * @property {string} id
 * @property {Color} color
 * @property {{ percent: AnyMetricPattern, ratio: AnyMetricPattern }} returns
 * @property {AnyPricePattern} lookback
 * @property {boolean} [defaultActive]
 */

/**
 * @typedef {Period & { cagr: { percent: AnyMetricPattern, ratio: AnyMetricPattern } }} PeriodWithCagr
 */

/**
 * @typedef {Object} MaPeriod
 * @property {string} id
 * @property {Color} color
 * @property {Brk.BpsCentsRatioSatsUsdPattern} ratio
 */

const commonMaIds = /** @type {const} */ ([
  "1w",
  "1m",
  "200d",
  "1y",
  "200w",
  "4y",
]);

/**
 * @param {string} label
 * @param {MaPeriod[]} averages
 */
function createMaSubSection(label, averages) {
  const common = averages.filter((a) => includes(commonMaIds, a.id));
  const more = averages.filter((a) => !includes(commonMaIds, a.id));

  /** @param {MaPeriod} a */
  const toFolder = (a) => ({
    name: periodIdToName(a.id, true),
    tree: [
      {
        name: "Price",
        title: `${periodIdToName(a.id, true)} ${label}`,
        top: [price({ metric: a.ratio, name: "average", color: a.color })],
      },
      {
        name: "Ratio",
        title: `${periodIdToName(a.id, true)} ${label} Ratio`,
        top: [price({ metric: a.ratio, name: "average", color: a.color })],
        bottom: [
          baseline({
            metric: a.ratio.ratio,
            name: "Ratio",
            color: a.color,
            unit: Unit.ratio,
          }),
        ],
      },
    ],
  });

  return {
    name: label,
    tree: [
      {
        name: "Compare",
        title: `Price ${label}s`,
        top: averages.map((a) =>
          price({ metric: a.ratio, name: a.id, color: a.color }),
        ),
      },
      ...common.map(toFolder),
      { name: "More...", tree: more.map(toFolder) },
    ],
  };
}

/**
 * @param {string} name
 * @param {string} title
 * @param {Unit} unit
 * @param {{ _1w: AnyMetricPattern, _1m: AnyMetricPattern, _1y: AnyMetricPattern }} metrics
 */
function volatilityChart(name, title, unit, metrics) {
  return {
    name,
    title,
    bottom: [
      line({ metric: metrics._1w, name: "1w", color: colors.time._1w, unit }),
      line({ metric: metrics._1m, name: "1m", color: colors.time._1m, unit }),
      line({ metric: metrics._1y, name: "1y", color: colors.time._1y, unit }),
    ],
  };
}

/**
 * @param {string} name
 * @param {Period[]} periods
 */
function returnsSubSection(name, periods) {
  return {
    name,
    tree: [
      {
        name: "Compare",
        title: `${name} Returns`,
        bottom: periods.flatMap((p) =>
          percentRatioBaseline({
            pattern: p.returns,
            name: p.id,
            color: p.color,
          }),
        ),
      },
      ...periods.map((p) => ({
        name: periodIdToName(p.id, true),
        title: `${periodIdToName(p.id, true)} Returns`,
        bottom: percentRatioBaseline({ pattern: p.returns, name: "Total" }),
      })),
    ],
  };
}

/**
 * @param {string} name
 * @param {PeriodWithCagr[]} periods
 */
function returnsSubSectionWithCagr(name, periods) {
  return {
    name,
    tree: [
      {
        name: "Compare",
        title: `${name} Returns`,
        bottom: periods.flatMap((p) =>
          percentRatioBaseline({
            pattern: p.returns,
            name: p.id,
            color: p.color,
          }),
        ),
      },
      ...periods.map((p) => ({
        name: periodIdToName(p.id, true),
        title: `${periodIdToName(p.id, true)} Returns`,
        bottom: [
          ...percentRatioBaseline({ pattern: p.returns, name: "Total" }),
          ...percentRatioBaseline({ pattern: p.cagr, name: "annual" }),
        ],
      })),
    ],
  };
}

/**
 * @param {string} name
 * @param {Period[]} periods
 */
function historicalSubSection(name, periods) {
  return {
    name,
    tree: [
      {
        name: "Compare",
        title: `${name} Historical`,
        top: periods.map((p) =>
          price({ metric: p.lookback, name: p.id, color: p.color }),
        ),
      },
      ...periods.map((p) => ({
        name: periodIdToName(p.id, true),
        title: `${periodIdToName(p.id, true)} Ago`,
        top: [price({ metric: p.lookback, name: "Price" })],
      })),
    ],
  };
}

/**
 * Create Market section
 * @returns {PartialOptionsGroup}
 */
export function createMarketSection() {
  const { market, supply, cohorts, prices, indicators } = brk.metrics;
  const {
    movingAverage: ma,
    ath,
    returns,
    volatility,
    range,
    technical,
    lookback,
    dca,
  } = market;

  const shortPeriodsBase = [
    { id: "24h", returns: returns.periods._24h, lookback: lookback._24h },
    { id: "1w", returns: returns.periods._1w, lookback: lookback._1w },
    { id: "1m", returns: returns.periods._1m, lookback: lookback._1m },
    {
      id: "3m",
      returns: returns.periods._3m,
      lookback: lookback._3m,
      defaultActive: false,
    },
    {
      id: "6m",
      returns: returns.periods._6m,
      lookback: lookback._6m,
      defaultActive: false,
    },
    { id: "1y", returns: returns.periods._1y, lookback: lookback._1y },
  ];

  const longPeriodsBase = [
    {
      id: "2y",
      returns: returns.periods._2y,
      cagr: returns.cagr._2y,
      lookback: lookback._2y,
      defaultActive: false,
    },
    {
      id: "3y",
      returns: returns.periods._3y,
      cagr: returns.cagr._3y,
      lookback: lookback._3y,
      defaultActive: false,
    },
    {
      id: "4y",
      returns: returns.periods._4y,
      cagr: returns.cagr._4y,
      lookback: lookback._4y,
    },
    {
      id: "5y",
      returns: returns.periods._5y,
      cagr: returns.cagr._5y,
      lookback: lookback._5y,
      defaultActive: false,
    },
    {
      id: "6y",
      returns: returns.periods._6y,
      cagr: returns.cagr._6y,
      lookback: lookback._6y,
      defaultActive: false,
    },
    {
      id: "8y",
      returns: returns.periods._8y,
      cagr: returns.cagr._8y,
      lookback: lookback._8y,
      defaultActive: false,
    },
    {
      id: "10y",
      returns: returns.periods._10y,
      cagr: returns.cagr._10y,
      lookback: lookback._10y,
      defaultActive: false,
    },
  ];

  const totalReturnPeriods = shortPeriodsBase.length + longPeriodsBase.length;

  /** @type {Period[]} */
  const shortPeriods = shortPeriodsBase.map((p, i) => ({
    ...p,
    color: colors.at(i, totalReturnPeriods),
  }));

  /** @type {PeriodWithCagr[]} */
  const longPeriods = longPeriodsBase.map((p, i) => ({
    ...p,
    color: colors.at(shortPeriodsBase.length + i, totalReturnPeriods),
  }));

  /** @type {MaPeriod[]} */
  const sma = [
    { id: "1w", ratio: ma.sma._1w },
    { id: "8d", ratio: ma.sma._8d },
    { id: "13d", ratio: ma.sma._13d },
    { id: "21d", ratio: ma.sma._21d },
    { id: "1m", ratio: ma.sma._1m },
    { id: "34d", ratio: ma.sma._34d },
    { id: "55d", ratio: ma.sma._55d },
    { id: "89d", ratio: ma.sma._89d },
    { id: "111d", ratio: ma.sma._111d },
    { id: "144d", ratio: ma.sma._144d },
    { id: "200d", ratio: ma.sma._200d },
    { id: "350d", ratio: ma.sma._350d },
    { id: "1y", ratio: ma.sma._1y },
    { id: "2y", ratio: ma.sma._2y },
    { id: "200w", ratio: ma.sma._200w },
    { id: "4y", ratio: ma.sma._4y },
  ].map((p, i, arr) => ({ ...p, color: colors.at(i, arr.length) }));

  /** @type {MaPeriod[]} */
  const ema = [
    { id: "1w", ratio: ma.ema._1w },
    { id: "8d", ratio: ma.ema._8d },
    { id: "12d", ratio: ma.ema._12d },
    { id: "13d", ratio: ma.ema._13d },
    { id: "21d", ratio: ma.ema._21d },
    { id: "26d", ratio: ma.ema._26d },
    { id: "1m", ratio: ma.ema._1m },
    { id: "34d", ratio: ma.ema._34d },
    { id: "55d", ratio: ma.ema._55d },
    { id: "89d", ratio: ma.ema._89d },
    { id: "144d", ratio: ma.ema._144d },
    { id: "200d", ratio: ma.ema._200d },
    { id: "1y", ratio: ma.ema._1y },
    { id: "2y", ratio: ma.ema._2y },
    { id: "200w", ratio: ma.ema._200w },
    { id: "4y", ratio: ma.ema._4y },
  ].map((p, i, arr) => ({ ...p, color: colors.at(i, arr.length) }));

  // SMA vs EMA comparison periods (common periods only)
  const smaVsEma = [
    {
      id: "1w",
      name: "1 Week",
      sma: ma.sma._1w,
      ema: ma.ema._1w,
    },
    {
      id: "1m",
      name: "1 Month",
      sma: ma.sma._1m,
      ema: ma.ema._1m,
    },
    {
      id: "200d",
      name: "200 Day",
      sma: ma.sma._200d,
      ema: ma.ema._200d,
    },
    {
      id: "1y",
      name: "1 Year",
      sma: ma.sma._1y,
      ema: ma.ema._1y,
    },
    {
      id: "200w",
      name: "200 Week",
      sma: ma.sma._200w,
      ema: ma.ema._200w,
    },
    {
      id: "4y",
      name: "4 Year",
      sma: ma.sma._4y,
      ema: ma.ema._4y,
    },
  ].map((p, i, arr) => ({ ...p, color: colors.at(i, arr.length) }));

  return {
    name: "Market",
    tree: [
      { name: "Price", title: "Bitcoin Price" },

      {
        name: "Sats/$",
        title: "Sats per Dollar",
        bottom: [
          line({
            metric: prices.spot.sats,
            name: "Sats/$",
            unit: Unit.sats,
          }),
        ],
      },

      {
        name: "Capitalization",
        tree: [
          {
            name: "Market Cap",
            title: "Market Capitalization",
            bottom: [
              line({
                metric: supply.marketCap.usd,
                name: "Market Cap",
                unit: Unit.usd,
              }),
            ],
          },
          {
            name: "Realized Cap",
            title: "Realized Capitalization",
            bottom: [
              line({
                metric: cohorts.utxo.all.realized.cap.usd,
                name: "Realized Cap",
                color: colors.realized,
                unit: Unit.usd,
              }),
            ],
          },
          {
            name: "Growth Rate",
            title: "Capitalization Growth Rate",
            bottom: [
              ...percentRatio({
                pattern: supply.marketCap.delta.rate._24h,
                name: "Market Cap (24h)",
                color: colors.bitcoin,
              }),
              baseline({
                metric: supply.marketMinusRealizedCapGrowthRate._24h,
                name: "Market - Realized",
                unit: Unit.percentage,
              }),
            ],
          },
        ],
      },

      {
        name: "All Time High",
        tree: [
          {
            name: "Drawdown",
            title: "ATH Drawdown",
            top: [price({ metric: ath.high, name: "ATH" })],
            bottom: percentRatio({
              pattern: ath.drawdown,
              name: "Drawdown",
              color: colors.loss,
            }),
          },
          {
            name: "Time Since",
            title: "Time Since ATH",
            top: [price({ metric: ath.high, name: "ATH" })],
            bottom: [
              line({
                metric: ath.daysSince,
                name: "Since",
                unit: Unit.days,
              }),
              line({
                metric: ath.yearsSince,
                name: "Since",
                unit: Unit.years,
              }),
              line({
                metric: ath.maxDaysBetween,
                name: "Max",
                color: colors.loss,
                unit: Unit.days,
              }),
              line({
                metric: ath.maxYearsBetween,
                name: "Max",
                color: colors.loss,
                unit: Unit.years,
              }),
            ],
          },
        ],
      },

      {
        name: "Returns",
        tree: [
          {
            name: "Compare",
            title: "Returns Comparison",
            bottom: [...shortPeriods, ...longPeriods].flatMap((p) =>
              percentRatioBaseline({
                pattern: p.returns,
                name: p.id,
                color: p.color,
                defaultActive: p.defaultActive,
              }),
            ),
          },
          returnsSubSection("Short-term", shortPeriods),
          returnsSubSectionWithCagr("Long-term", longPeriods),
        ],
      },

      {
        name: "Volatility",
        tree: [
          volatilityChart("Index", "Volatility Index", Unit.percentage, {
            _1w: volatility._1w,
            _1m: volatility._1m,
            _1y: volatility._1y,
          }),
          {
            name: "True Range",
            title: "True Range",
            bottom: [
              line({
                metric: range.trueRange,
                name: "Daily",
                color: colors.time._24h,
                unit: Unit.usd,
              }),
              line({
                metric: range.trueRangeSum2w,
                name: "2w Sum",
                color: colors.time._1w,
                unit: Unit.usd,
                defaultActive: false,
              }),
            ],
          },
          {
            name: "Choppiness",
            title: "Choppiness Index",
            bottom: [
              ...percentRatio({
                pattern: range.choppinessIndex2w,
                name: "2w",
                color: colors.indicator.main,
              }),
              ...priceLines({ unit: Unit.index, numbers: [61.8, 38.2] }),
            ],
          },
        ],
      },

      {
        name: "Moving Averages",
        tree: [
          {
            name: "SMA vs EMA",
            tree: [
              {
                name: "All Periods",
                title: "SMA vs EMA Comparison",
                top: smaVsEma.flatMap((p) => [
                  price({
                    metric: p.sma,
                    name: `${p.id} SMA`,
                    color: p.color,
                  }),
                  price({
                    metric: p.ema,
                    name: `${p.id} EMA`,
                    color: p.color,
                    style: 1,
                  }),
                ]),
              },
              ...smaVsEma.map((p) => ({
                name: p.name,
                title: `${p.name} SMA vs EMA`,
                top: [
                  price({ metric: p.sma, name: "SMA", color: p.color }),
                  price({
                    metric: p.ema,
                    name: "EMA",
                    color: p.color,
                    style: 1,
                  }),
                ],
              })),
            ],
          },
          createMaSubSection("SMA", sma),
          createMaSubSection("EMA", ema),
        ],
      },

      {
        name: "Bands",
        tree: [
          {
            name: "MinMax",
            tree: [
              {
                id: "1w",
                name: "1 Week",
                min: range.min._1w,
                max: range.max._1w,
              },
              {
                id: "2w",
                name: "2 Week",
                min: range.min._2w,
                max: range.max._2w,
              },
              {
                id: "1m",
                name: "1 Month",
                min: range.min._1m,
                max: range.max._1m,
              },
              {
                id: "1y",
                name: "1 Year",
                min: range.min._1y,
                max: range.max._1y,
              },
            ].map((p) => ({
              name: p.id,
              title: `${p.name} MinMax`,
              top: [
                price({
                  metric: p.max,
                  name: "Max",
                  key: "price-max",
                  color: colors.stat.max,
                }),
                price({
                  metric: p.min,
                  name: "Min",
                  key: "price-min",
                  color: colors.stat.min,
                }),
              ],
            })),
          },
          {
            name: "Mayer Multiple",
            title: "Mayer Multiple",
            top: [
              price({
                metric: ma.sma._200d,
                name: "200d SMA",
                color: colors.indicator.main,
              }),
              price({
                metric: ma.sma._200d.x24,
                name: "200d SMA x2.4",
                color: colors.indicator.upper,
              }),
              price({
                metric: ma.sma._200d.x08,
                name: "200d SMA x0.8",
                color: colors.indicator.lower,
              }),
            ],
          },
        ],
      },

      {
        name: "Momentum",
        tree: [
          {
            name: "RSI",
            tree: [
              {
                name: "Compare",
                title: "RSI Comparison",
                bottom: [
                  line({
                    metric: technical.rsi._24h.rsi.percent,
                    name: "1d",
                    color: colors.time._24h,
                    unit: Unit.index,
                  }),
                  line({
                    metric: technical.rsi._1w.rsi.percent,
                    name: "1w",
                    color: colors.time._1w,
                    unit: Unit.index,
                  }),
                  line({
                    metric: technical.rsi._1m.rsi.percent,
                    name: "1m",
                    color: colors.time._1m,
                    unit: Unit.index,
                  }),
                  line({
                    metric: technical.rsi._1y.rsi.percent,
                    name: "1y",
                    color: colors.time._1y,
                    unit: Unit.index,
                  }),
                  priceLine({ unit: Unit.index, number: 70 }),
                  priceLine({ unit: Unit.index, number: 30 }),
                ],
              },
              {
                name: "1 Day",
                title: "RSI (1d)",
                bottom: [
                  line({
                    metric: technical.rsi._24h.rsi.percent,
                    name: "RSI",
                    color: colors.indicator.main,
                    unit: Unit.index,
                  }),
                  line({
                    metric: technical.rsi._24h.rsiMax.percent,
                    name: "Max",
                    color: colors.stat.max,
                    defaultActive: false,
                    unit: Unit.index,
                  }),
                  line({
                    metric: technical.rsi._24h.rsiMin.percent,
                    name: "Min",
                    color: colors.stat.min,
                    defaultActive: false,
                    unit: Unit.index,
                  }),
                  priceLine({ unit: Unit.index, number: 70 }),
                  priceLine({
                    unit: Unit.index,
                    number: 50,
                    defaultActive: false,
                  }),
                  priceLine({ unit: Unit.index, number: 30 }),
                ],
              },
              {
                name: "1 Week",
                title: "RSI (1w)",
                bottom: [
                  line({
                    metric: technical.rsi._1w.rsi.percent,
                    name: "RSI",
                    color: colors.indicator.main,
                    unit: Unit.index,
                  }),
                  line({
                    metric: technical.rsi._1w.rsiMax.percent,
                    name: "Max",
                    color: colors.stat.max,
                    defaultActive: false,
                    unit: Unit.index,
                  }),
                  line({
                    metric: technical.rsi._1w.rsiMin.percent,
                    name: "Min",
                    color: colors.stat.min,
                    defaultActive: false,
                    unit: Unit.index,
                  }),
                  priceLine({ unit: Unit.index, number: 70 }),
                  priceLine({
                    unit: Unit.index,
                    number: 50,
                    defaultActive: false,
                  }),
                  priceLine({ unit: Unit.index, number: 30 }),
                ],
              },
              {
                name: "1 Month",
                title: "RSI (1m)",
                bottom: [
                  line({
                    metric: technical.rsi._1m.rsi.percent,
                    name: "RSI",
                    color: colors.indicator.main,
                    unit: Unit.index,
                  }),
                  line({
                    metric: technical.rsi._1m.rsiMax.percent,
                    name: "Max",
                    color: colors.stat.max,
                    defaultActive: false,
                    unit: Unit.index,
                  }),
                  line({
                    metric: technical.rsi._1m.rsiMin.percent,
                    name: "Min",
                    color: colors.stat.min,
                    defaultActive: false,
                    unit: Unit.index,
                  }),
                  priceLine({ unit: Unit.index, number: 70 }),
                  priceLine({
                    unit: Unit.index,
                    number: 50,
                    defaultActive: false,
                  }),
                  priceLine({ unit: Unit.index, number: 30 }),
                ],
              },
              {
                name: "1 Year",
                title: "RSI (1y)",
                bottom: [
                  line({
                    metric: technical.rsi._1y.rsi.percent,
                    name: "RSI",
                    color: colors.indicator.main,
                    unit: Unit.index,
                  }),
                  line({
                    metric: technical.rsi._1y.rsiMax.percent,
                    name: "Max",
                    color: colors.stat.max,
                    defaultActive: false,
                    unit: Unit.index,
                  }),
                  line({
                    metric: technical.rsi._1y.rsiMin.percent,
                    name: "Min",
                    color: colors.stat.min,
                    defaultActive: false,
                    unit: Unit.index,
                  }),
                  priceLine({ unit: Unit.index, number: 70 }),
                  priceLine({
                    unit: Unit.index,
                    number: 50,
                    defaultActive: false,
                  }),
                  priceLine({ unit: Unit.index, number: 30 }),
                ],
              },
            ],
          },
          {
            name: "StochRSI",
            tree: [
              {
                name: "Compare",
                title: "Stochastic RSI Comparison",
                bottom: [
                  line({
                    metric: technical.rsi._24h.stochRsiK.percent,
                    name: "1d K",
                    color: colors.time._24h,
                    unit: Unit.index,
                  }),
                  line({
                    metric: technical.rsi._1w.stochRsiK.percent,
                    name: "1w K",
                    color: colors.time._1w,
                    unit: Unit.index,
                  }),
                  line({
                    metric: technical.rsi._1m.stochRsiK.percent,
                    name: "1m K",
                    color: colors.time._1m,
                    unit: Unit.index,
                  }),
                  line({
                    metric: technical.rsi._1y.stochRsiK.percent,
                    name: "1y K",
                    color: colors.time._1y,
                    unit: Unit.index,
                  }),
                  ...priceLines({ unit: Unit.index, numbers: [80, 20] }),
                ],
              },
              {
                name: "1 Day",
                title: "Stochastic RSI (1d)",
                bottom: [
                  line({
                    metric: technical.rsi._24h.stochRsiK.percent,
                    name: "K",
                    color: colors.indicator.fast,
                    unit: Unit.index,
                  }),
                  line({
                    metric: technical.rsi._24h.stochRsiD.percent,
                    name: "D",
                    color: colors.indicator.slow,
                    unit: Unit.index,
                  }),
                  ...priceLines({ unit: Unit.index, numbers: [80, 20] }),
                ],
              },
              {
                name: "1 Week",
                title: "Stochastic RSI (1w)",
                bottom: [
                  line({
                    metric: technical.rsi._1w.stochRsiK.percent,
                    name: "K",
                    color: colors.indicator.fast,
                    unit: Unit.index,
                  }),
                  line({
                    metric: technical.rsi._1w.stochRsiD.percent,
                    name: "D",
                    color: colors.indicator.slow,
                    unit: Unit.index,
                  }),
                  ...priceLines({ unit: Unit.index, numbers: [80, 20] }),
                ],
              },
              {
                name: "1 Month",
                title: "Stochastic RSI (1m)",
                bottom: [
                  line({
                    metric: technical.rsi._1m.stochRsiK.percent,
                    name: "K",
                    color: colors.indicator.fast,
                    unit: Unit.index,
                  }),
                  line({
                    metric: technical.rsi._1m.stochRsiD.percent,
                    name: "D",
                    color: colors.indicator.slow,
                    unit: Unit.index,
                  }),
                  ...priceLines({ unit: Unit.index, numbers: [80, 20] }),
                ],
              },
              {
                name: "1 Year",
                title: "Stochastic RSI (1y)",
                bottom: [
                  line({
                    metric: technical.rsi._1y.stochRsiK.percent,
                    name: "K",
                    color: colors.indicator.fast,
                    unit: Unit.index,
                  }),
                  line({
                    metric: technical.rsi._1y.stochRsiD.percent,
                    name: "D",
                    color: colors.indicator.slow,
                    unit: Unit.index,
                  }),
                  ...priceLines({ unit: Unit.index, numbers: [80, 20] }),
                ],
              },
            ],
          },
          {
            name: "Stochastic",
            title: "Stochastic Oscillator",
            bottom: [
              line({
                metric: technical.stochK.percent,
                name: "K",
                color: colors.indicator.fast,
                unit: Unit.index,
              }),
              line({
                metric: technical.stochD.percent,
                name: "D",
                color: colors.indicator.slow,
                unit: Unit.index,
              }),
              ...priceLines({ unit: Unit.index, numbers: [80, 20] }),
            ],
          },
          {
            name: "MACD",
            tree: [
              {
                name: "Compare",
                title: "MACD Comparison",
                bottom: [
                  line({
                    metric: technical.macd._24h.line,
                    name: "1d",
                    color: colors.time._24h,
                    unit: Unit.usd,
                  }),
                  line({
                    metric: technical.macd._1w.line,
                    name: "1w",
                    color: colors.time._1w,
                    unit: Unit.usd,
                  }),
                  line({
                    metric: technical.macd._1m.line,
                    name: "1m",
                    color: colors.time._1m,
                    unit: Unit.usd,
                  }),
                  line({
                    metric: technical.macd._1y.line,
                    name: "1y",
                    color: colors.time._1y,
                    unit: Unit.usd,
                  }),
                ],
              },
              {
                name: "1 Day",
                title: "MACD (1d)",
                bottom: [
                  line({
                    metric: technical.macd._24h.line,
                    name: "MACD",
                    color: colors.indicator.fast,
                    unit: Unit.usd,
                  }),
                  line({
                    metric: technical.macd._24h.signal,
                    name: "Signal",
                    color: colors.indicator.slow,
                    unit: Unit.usd,
                  }),
                  histogram({
                    metric: technical.macd._24h.histogram,
                    name: "Histogram",
                    unit: Unit.usd,
                  }),
                ],
              },
              {
                name: "1 Week",
                title: "MACD (1w)",
                bottom: [
                  line({
                    metric: technical.macd._1w.line,
                    name: "MACD",
                    color: colors.indicator.fast,
                    unit: Unit.usd,
                  }),
                  line({
                    metric: technical.macd._1w.signal,
                    name: "Signal",
                    color: colors.indicator.slow,
                    unit: Unit.usd,
                  }),
                  histogram({
                    metric: technical.macd._1w.histogram,
                    name: "Histogram",
                    unit: Unit.usd,
                  }),
                ],
              },
              {
                name: "1 Month",
                title: "MACD (1m)",
                bottom: [
                  line({
                    metric: technical.macd._1m.line,
                    name: "MACD",
                    color: colors.indicator.fast,
                    unit: Unit.usd,
                  }),
                  line({
                    metric: technical.macd._1m.signal,
                    name: "Signal",
                    color: colors.indicator.slow,
                    unit: Unit.usd,
                  }),
                  histogram({
                    metric: technical.macd._1m.histogram,
                    name: "Histogram",
                    unit: Unit.usd,
                  }),
                ],
              },
              {
                name: "1 Year",
                title: "MACD (1y)",
                bottom: [
                  line({
                    metric: technical.macd._1y.line,
                    name: "MACD",
                    color: colors.indicator.fast,
                    unit: Unit.usd,
                  }),
                  line({
                    metric: technical.macd._1y.signal,
                    name: "Signal",
                    color: colors.indicator.slow,
                    unit: Unit.usd,
                  }),
                  histogram({
                    metric: technical.macd._1y.histogram,
                    name: "Histogram",
                    unit: Unit.usd,
                  }),
                ],
              },
            ],
          },
        ],
      },

      {
        name: "Historical",
        tree: [
          {
            name: "Compare",
            title: "Historical Comparison",
            top: [...shortPeriods, ...longPeriods].map((p) =>
              price({
                metric: p.lookback,
                name: p.id,
                color: p.color,
                defaultActive: p.defaultActive,
              }),
            ),
          },
          historicalSubSection("Short-term", shortPeriods),
          historicalSubSection("Long-term", longPeriods),
        ],
      },

      {
        name: "DCA",
        title: "Dollar Cost Average Sats/Day",
        bottom: [
          line({
            metric: dca.satsPerDay,
            name: "Sats/Day",
            unit: Unit.sats,
          }),
        ],
      },

      {
        name: "Indicators",
        tree: [
          {
            name: "Pi Cycle",
            title: "Pi Cycle",
            top: [
              price({
                metric: ma.sma._111d,
                name: "111d SMA",
                color: colors.indicator.upper,
              }),
              price({
                metric: ma.sma._350d.x2,
                name: "350d SMA x2",
                color: colors.indicator.lower,
              }),
            ],
            bottom: [
              baseline({
                metric: technical.piCycle.ratio,
                name: "Pi Cycle",
                unit: Unit.ratio,
                base: 1,
              }),
            ],
          },
          {
            name: "Puell Multiple",
            title: "Puell Multiple",
            bottom: [
              line({
                metric: indicators.puellMultiple.ratio,
                name: "Puell",
                color: colors.usd,
                unit: Unit.ratio,
              }),
            ],
          },
          {
            name: "NVT",
            title: "NVT Ratio",
            bottom: [
              line({
                metric: indicators.nvt.ratio,
                name: "NVT",
                color: colors.bitcoin,
                unit: Unit.ratio,
              }),
            ],
          },
          {
            name: "Gini",
            title: "Gini Coefficient",
            bottom: percentRatio({
              pattern: indicators.gini,
              name: "Gini",
              color: colors.loss,
            }),
          },
          {
            name: "RHODL Ratio",
            title: "RHODL Ratio",
            bottom: [
              line({
                metric: indicators.rhodlRatio.ratio,
                name: "RHODL",
                color: colors.bitcoin,
                unit: Unit.ratio,
              }),
            ],
          },
          {
            name: "Thermocap Multiple",
            title: "Thermocap Multiple",
            bottom: [
              line({
                metric: indicators.thermocapMultiple.ratio,
                name: "Thermocap",
                color: colors.bitcoin,
                unit: Unit.ratio,
              }),
            ],
          },
          {
            name: "Stock-to-Flow",
            title: "Stock-to-Flow",
            bottom: [
              line({
                metric: indicators.stockToFlow,
                name: "S2F",
                color: colors.bitcoin,
                unit: Unit.ratio,
              }),
            ],
          },
          {
            name: "Dormancy",
            title: "Dormancy",
            bottom: [
              line({
                metric: indicators.dormancy.supplyAdjusted,
                name: "Supply Adjusted",
                color: colors.bitcoin,
                unit: Unit.ratio,
              }),
              line({
                metric: indicators.dormancy.flow,
                name: "Flow",
                color: colors.usd,
                unit: Unit.ratio,
                defaultActive: false,
              }),
            ],
          },
          {
            name: "Seller Exhaustion",
            title: "Seller Exhaustion Constant",
            bottom: [
              line({
                metric: indicators.sellerExhaustionConstant,
                name: "SEC",
                color: colors.bitcoin,
                unit: Unit.ratio,
              }),
            ],
          },
          {
            name: "CDD Supply Adjusted",
            title: "Coindays Destroyed (Supply Adjusted)",
            bottom: [
              line({
                metric: indicators.coindaysDestroyedSupplyAdjusted,
                name: "CDD SA",
                color: colors.bitcoin,
                unit: Unit.ratio,
              }),
            ],
          },
          {
            name: "CYD Supply Adjusted",
            title: "Coinyears Destroyed (Supply Adjusted)",
            bottom: [
              line({
                metric: indicators.coinyearsDestroyedSupplyAdjusted,
                name: "CYD SA",
                color: colors.bitcoin,
                unit: Unit.ratio,
              }),
            ],
          },
        ],
      },
    ],
  };
}
