/** Market section */

import { colors } from "../utils/colors.js";
import { brk } from "../client.js";
import { includes } from "../utils/array.js";
import { Unit } from "../utils/units.js";
import { priceLine, priceLines } from "./constants.js";
import { baseline, histogram, line, price } from "./series.js";
import { createPriceRatioCharts } from "./shared.js";
import { periodIdToName } from "./utils.js";

/**
 * @typedef {Object} Period
 * @property {string} id
 * @property {Color} color
 * @property {AnyMetricPattern} returns
 * @property {AnyPricePattern} lookback
 * @property {boolean} [defaultActive]
 */

/**
 * @typedef {Period & { cagr: AnyMetricPattern }} PeriodWithCagr
 */

/**
 * @typedef {Object} MaPeriod
 * @property {string} id
 * @property {Color} color
 * @property {ActivePriceRatioPattern} ratio
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
    tree: createPriceRatioCharts({
      context: `${periodIdToName(a.id, true)} ${label}`,
      legend: "average",
      pricePattern: a.ratio.price,
      ratio: a.ratio,
      color: a.color,
    }),
  });

  return {
    name: label,
    tree: [
      {
        name: "Compare",
        title: `Price ${label}s`,
        top: averages.map((a) =>
          price({ metric: a.ratio.price, name: a.id, color: a.color }),
        ),
      },
      ...common.map(toFolder),
      { name: "More...", tree: more.map(toFolder) },
    ],
  };
}

/**
 * @param {Colors} colors
 * @param {string} name
 * @param {string} title
 * @param {Unit} unit
 * @param {{ _1w: AnyMetricPattern, _1m: AnyMetricPattern, _1y: AnyMetricPattern }} metrics
 */
function volatilityChart(colors, name, title, unit, metrics) {
  return {
    name,
    title,
    bottom: [
      line({ metric: metrics._1w, name: "1w", color: colors.red, unit }),
      line({ metric: metrics._1m, name: "1m", color: colors.orange, unit }),
      line({ metric: metrics._1y, name: "1y", color: colors.lime, unit }),
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
        bottom: periods.map((p) =>
          baseline({
            metric: p.returns,
            name: p.id,
            color: p.color,
            unit: Unit.percentage,
          }),
        ),
      },
      ...periods.map((p) => ({
        name: periodIdToName(p.id, true),
        title: `${periodIdToName(p.id, true)} Returns`,
        bottom: [
          baseline({ metric: p.returns, name: "Total", unit: Unit.percentage }),
        ],
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
        bottom: periods.map((p) =>
          baseline({
            metric: p.returns,
            name: p.id,
            color: p.color,
            unit: Unit.percentage,
          }),
        ),
      },
      ...periods.map((p) => ({
        name: periodIdToName(p.id, true),
        title: `${periodIdToName(p.id, true)} Returns`,
        bottom: [
          baseline({ metric: p.returns, name: "Total", unit: Unit.percentage }),
          baseline({ metric: p.cagr, name: "annual", unit: Unit.cagr }),
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
  const { market, supply, price: priceMetrics } = brk.metrics;
  const {
    movingAverage: ma,
    ath,
    returns,
    volatility,
    range,
    indicators,
    lookback,
  } = market;

  /** @type {Period[]} */
  const shortPeriods = [
    {
      id: "1d",
      color: colors.returns._1d,
      returns: returns.priceReturns._1d,
      lookback: lookback._1d,
    },
    {
      id: "1w",
      color: colors.returns._1w,
      returns: returns.priceReturns._1w,
      lookback: lookback._1w,
    },
    {
      id: "1m",
      color: colors.returns._1m,
      returns: returns.priceReturns._1m,
      lookback: lookback._1m,
    },
    {
      id: "3m",
      color: colors.returns._3m,
      returns: returns.priceReturns._3m,
      lookback: lookback._3m,
      defaultActive: false,
    },
    {
      id: "6m",
      color: colors.returns._6m,
      returns: returns.priceReturns._6m,
      lookback: lookback._6m,
      defaultActive: false,
    },
    {
      id: "1y",
      color: colors.returns._1y,
      returns: returns.priceReturns._1y,
      lookback: lookback._1y,
    },
  ];

  /** @type {PeriodWithCagr[]} */
  const longPeriods = [
    {
      id: "2y",
      color: colors.returns._2y,
      returns: returns.priceReturns._2y,
      cagr: returns.cagr._2y,
      lookback: lookback._2y,
      defaultActive: false,
    },
    {
      id: "3y",
      color: colors.returns._3y,
      returns: returns.priceReturns._3y,
      cagr: returns.cagr._3y,
      lookback: lookback._3y,
      defaultActive: false,
    },
    {
      id: "4y",
      color: colors.returns._4y,
      returns: returns.priceReturns._4y,
      cagr: returns.cagr._4y,
      lookback: lookback._4y,
    },
    {
      id: "5y",
      color: colors.returns._5y,
      returns: returns.priceReturns._5y,
      cagr: returns.cagr._5y,
      lookback: lookback._5y,
      defaultActive: false,
    },
    {
      id: "6y",
      color: colors.returns._6y,
      returns: returns.priceReturns._6y,
      cagr: returns.cagr._6y,
      lookback: lookback._6y,
      defaultActive: false,
    },
    {
      id: "8y",
      color: colors.returns._8y,
      returns: returns.priceReturns._8y,
      cagr: returns.cagr._8y,
      lookback: lookback._8y,
      defaultActive: false,
    },
    {
      id: "10y",
      color: colors.returns._10y,
      returns: returns.priceReturns._10y,
      cagr: returns.cagr._10y,
      lookback: lookback._10y,
      defaultActive: false,
    },
  ];

  /** @type {MaPeriod[]} */
  const sma = [
    { id: "1w", color: colors.ma._1w, ratio: ma.price1wSma },
    { id: "8d", color: colors.ma._8d, ratio: ma.price8dSma },
    { id: "13d", color: colors.ma._13d, ratio: ma.price13dSma },
    { id: "21d", color: colors.ma._21d, ratio: ma.price21dSma },
    { id: "1m", color: colors.ma._1m, ratio: ma.price1mSma },
    { id: "34d", color: colors.ma._34d, ratio: ma.price34dSma },
    { id: "55d", color: colors.ma._55d, ratio: ma.price55dSma },
    { id: "89d", color: colors.ma._89d, ratio: ma.price89dSma },
    { id: "111d", color: colors.ma._111d, ratio: ma.price111dSma },
    { id: "144d", color: colors.ma._144d, ratio: ma.price144dSma },
    { id: "200d", color: colors.ma._200d, ratio: ma.price200dSma },
    { id: "350d", color: colors.ma._350d, ratio: ma.price350dSma },
    { id: "1y", color: colors.ma._1y, ratio: ma.price1ySma },
    { id: "2y", color: colors.ma._2y, ratio: ma.price2ySma },
    { id: "200w", color: colors.ma._200w, ratio: ma.price200wSma },
    { id: "4y", color: colors.ma._4y, ratio: ma.price4ySma },
  ];

  /** @type {MaPeriod[]} */
  const ema = [
    { id: "1w", color: colors.ma._1w, ratio: ma.price1wEma },
    { id: "8d", color: colors.ma._8d, ratio: ma.price8dEma },
    { id: "12d", color: colors.ma._12d, ratio: ma.price12dEma },
    { id: "13d", color: colors.ma._13d, ratio: ma.price13dEma },
    { id: "21d", color: colors.ma._21d, ratio: ma.price21dEma },
    { id: "26d", color: colors.ma._26d, ratio: ma.price26dEma },
    { id: "1m", color: colors.ma._1m, ratio: ma.price1mEma },
    { id: "34d", color: colors.ma._34d, ratio: ma.price34dEma },
    { id: "55d", color: colors.ma._55d, ratio: ma.price55dEma },
    { id: "89d", color: colors.ma._89d, ratio: ma.price89dEma },
    { id: "144d", color: colors.ma._144d, ratio: ma.price144dEma },
    { id: "200d", color: colors.ma._200d, ratio: ma.price200dEma },
    { id: "1y", color: colors.ma._1y, ratio: ma.price1yEma },
    { id: "2y", color: colors.ma._2y, ratio: ma.price2yEma },
    { id: "200w", color: colors.ma._200w, ratio: ma.price200wEma },
    { id: "4y", color: colors.ma._4y, ratio: ma.price4yEma },
  ];

  // SMA vs EMA comparison periods (common periods only)
  const smaVsEma = [
    {
      id: "1w",
      name: "1 Week",
      color: colors.ma._1w,
      sma: ma.price1wSma,
      ema: ma.price1wEma,
    },
    {
      id: "1m",
      name: "1 Month",
      color: colors.ma._1m,
      sma: ma.price1mSma,
      ema: ma.price1mEma,
    },
    {
      id: "200d",
      name: "200 Day",
      color: colors.ma._200d,
      sma: ma.price200dSma,
      ema: ma.price200dEma,
    },
    {
      id: "1y",
      name: "1 Year",
      color: colors.ma._1y,
      sma: ma.price1ySma,
      ema: ma.price1yEma,
    },
    {
      id: "200w",
      name: "200 Week",
      color: colors.ma._200w,
      sma: ma.price200wSma,
      ema: ma.price200wEma,
    },
    {
      id: "4y",
      name: "4 Year",
      color: colors.ma._4y,
      sma: ma.price4ySma,
      ema: ma.price4yEma,
    },
  ];

  return {
    name: "Market",
    tree: [
      { name: "Price", title: "Bitcoin Price" },

      {
        name: "Sats/$",
        title: "Sats per Dollar",
        bottom: [
          line({
            metric: priceMetrics.sats.split.close,
            name: "Sats/$",
            unit: Unit.sats,
          }),
        ],
      },

      {
        name: "Capitalization",
        title: "Market Capitalization",
        bottom: [
          line({
            metric: supply.marketCap,
            name: "Capitalization",
            unit: Unit.usd,
          }),
        ],
      },

      {
        name: "All Time High",
        tree: [
          {
            name: "Drawdown",
            title: "ATH Drawdown",
            top: [price({ metric: ath.priceAth, name: "ATH" })],
            bottom: [
              line({
                metric: ath.priceDrawdown,
                name: "Drawdown",
                color: colors.red,
                unit: Unit.percentage,
              }),
            ],
          },
          {
            name: "Time Since",
            title: "Time Since ATH",
            top: [price({ metric: ath.priceAth, name: "ATH" })],
            bottom: [
              line({
                metric: ath.daysSincePriceAth,
                name: "Since",
                unit: Unit.days,
              }),
              line({
                metric: ath.yearsSincePriceAth,
                name: "Since",
                unit: Unit.years,
              }),
              line({
                metric: ath.maxDaysBetweenPriceAths,
                name: "Max",
                color: colors.red,
                unit: Unit.days,
              }),
              line({
                metric: ath.maxYearsBetweenPriceAths,
                name: "Max",
                color: colors.red,
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
            bottom: [...shortPeriods, ...longPeriods].map((p) =>
              baseline({
                metric: p.returns,
                name: p.id,
                color: p.color,
                unit: Unit.percentage,
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
          volatilityChart(
            colors,
            "Index",
            "Volatility Index",
            Unit.percentage,
            {
              _1w: volatility.price1wVolatility,
              _1m: volatility.price1mVolatility,
              _1y: volatility.price1yVolatility,
            },
          ),
          {
            name: "True Range",
            title: "True Range",
            bottom: [
              line({
                metric: range.priceTrueRange,
                name: "Daily",
                color: colors.yellow,
                unit: Unit.usd,
              }),
              line({
                metric: range.priceTrueRange2wSum,
                name: "2w Sum",
                color: colors.orange,
                unit: Unit.usd,
                defaultActive: false,
              }),
            ],
          },
          {
            name: "Choppiness",
            title: "Choppiness Index",
            bottom: [
              line({
                metric: range.price2wChoppinessIndex,
                name: "2w",
                color: colors.red,
                unit: Unit.index,
              }),
              ...priceLines({ unit: Unit.index, numbers: [61.8, 38.2] }),
            ],
          },
          volatilityChart(colors, "Sharpe Ratio", "Sharpe Ratio", Unit.ratio, {
            _1w: volatility.sharpe1w,
            _1m: volatility.sharpe1m,
            _1y: volatility.sharpe1y,
          }),
          volatilityChart(
            colors,
            "Sortino Ratio",
            "Sortino Ratio",
            Unit.ratio,
            {
              _1w: volatility.sortino1w,
              _1m: volatility.sortino1m,
              _1y: volatility.sortino1y,
            },
          ),
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
                    metric: p.sma.price,
                    name: `${p.id} SMA`,
                    color: p.color,
                  }),
                  price({
                    metric: p.ema.price,
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
                  price({ metric: p.sma.price, name: "SMA", color: p.color }),
                  price({
                    metric: p.ema.price,
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
                min: range.price1wMin,
                max: range.price1wMax,
              },
              {
                id: "2w",
                name: "2 Week",
                min: range.price2wMin,
                max: range.price2wMax,
              },
              {
                id: "1m",
                name: "1 Month",
                min: range.price1mMin,
                max: range.price1mMax,
              },
              {
                id: "1y",
                name: "1 Year",
                min: range.price1yMin,
                max: range.price1yMax,
              },
            ].map((p) => ({
              name: p.id,
              title: `${p.name} MinMax`,
              top: [
                price({
                  metric: p.min,
                  name: "Min",
                  key: "price-min",
                  color: colors.red,
                }),
                price({
                  metric: p.max,
                  name: "Max",
                  key: "price-max",
                  color: colors.green,
                }),
              ],
            })),
          },
          {
            name: "Mayer Multiple",
            title: "Mayer Multiple",
            top: [
              price({
                metric: ma.price200dSma.price,
                name: "200d SMA",
                color: colors.yellow,
              }),
              price({
                metric: ma.price200dSmaX24,
                name: "200d SMA x2.4",
                color: colors.green,
              }),
              price({
                metric: ma.price200dSmaX08,
                name: "200d SMA x0.8",
                color: colors.red,
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
            title: "RSI (14d)",
            bottom: [
              line({
                metric: indicators.rsi14d,
                name: "RSI",
                color: colors.indigo,
                unit: Unit.index,
              }),
              line({
                metric: indicators.rsi14dMin,
                name: "Min",
                color: colors.red,
                defaultActive: false,
                unit: Unit.index,
              }),
              line({
                metric: indicators.rsi14dMax,
                name: "Max",
                color: colors.green,
                defaultActive: false,
                unit: Unit.index,
              }),
              priceLine({ unit: Unit.index, number: 70 }),
              priceLine({ unit: Unit.index, number: 50, defaultActive: false }),
              priceLine({ unit: Unit.index, number: 30 }),
            ],
          },
          {
            name: "StochRSI",
            title: "Stochastic RSI",
            bottom: [
              line({
                metric: indicators.stochRsiK,
                name: "K",
                color: colors.blue,
                unit: Unit.index,
              }),
              line({
                metric: indicators.stochRsiD,
                name: "D",
                color: colors.orange,
                unit: Unit.index,
              }),
              ...priceLines({ unit: Unit.index, numbers: [80, 20] }),
            ],
          },
          {
            name: "MACD",
            title: "MACD",
            bottom: [
              line({
                metric: indicators.macdLine,
                name: "MACD",
                color: colors.blue,
                unit: Unit.usd,
              }),
              line({
                metric: indicators.macdSignal,
                name: "Signal",
                color: colors.orange,
                unit: Unit.usd,
              }),
              histogram({
                metric: indicators.macdHistogram,
                name: "Histogram",
                unit: Unit.usd,
              }),
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
        name: "Indicators",
        tree: [
          {
            name: "Pi Cycle",
            title: "Pi Cycle",
            top: [
              price({
                metric: ma.price111dSma.price,
                name: "111d SMA",
                color: colors.green,
              }),
              price({
                metric: ma.price350dSmaX2,
                name: "350d SMA x2",
                color: colors.red,
              }),
            ],
            bottom: [
              baseline({
                metric: indicators.piCycle,
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
      },
    ],
  };
}
