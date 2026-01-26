/** Market section - Main entry point */

import { localhost } from "../../utils/env.js";
import { Unit } from "../../utils/units.js";
import { candlestick, line } from "../series.js";
import { createAveragesSection } from "./averages.js";
import { createReturnsSection } from "./performance.js";
import { createMomentumSection } from "./momentum.js";
import { createVolatilitySection } from "./volatility.js";
import { createBandsSection } from "./bands.js";
import { createValuationSection } from "./onchain.js";
import { createDcaVsLumpSumSection, createDcaByYearSection } from "./investing.js";

/**
 * Create Market section
 * @param {PartialContext} ctx
 * @returns {PartialOptionsGroup}
 */
export function createMarketSection(ctx) {
  const { colors, brk } = ctx;
  const { market, supply, price } = brk.metrics;
  const {
    movingAverage,
    ath,
    returns,
    volatility,
    range,
    dca,
    lookback,
    indicators,
  } = market;


  return {
    name: "Market",
    tree: [
      // Price
      {
        name: "Price",
        title: "Bitcoin Price",
      },
      ...(localhost
        ? [
            {
              name: "Oracle",
              title: "Oracle Price",
              top: [
                candlestick({
                  metric: price.oracle.closeOhlcDollars,
                  name: "close",
                  unit: Unit.usd,
                }),
                candlestick({
                  metric: price.oracle.midOhlcDollars,
                  name: "mid",
                  unit: Unit.usd,
                }),
                line({
                  metric: price.oracle.phaseDailyDollars.median,
                  name: "o. p50",
                  unit: Unit.usd,
                  color: colors.yellow,
                }),
                line({
                  metric: price.oracle.phaseV2DailyDollars.median,
                  name: "o2. p50",
                  unit: Unit.usd,
                  color: colors.orange,
                }),
                line({
                  metric: price.oracle.phaseV2PeakDailyDollars.median,
                  name: "o2.2 p50",
                  unit: Unit.usd,
                  color: colors.orange,
                }),
                line({
                  metric: price.oracle.phaseV3DailyDollars.median,
                  name: "o3. p50",
                  unit: Unit.usd,
                  color: colors.red,
                }),
                line({
                  metric: price.oracle.phaseV3PeakDailyDollars.median,
                  name: "o3.2 p50",
                  unit: Unit.usd,
                  color: colors.red,
                }),
                line({
                  metric: price.oracle.phaseDailyDollars.max,
                  name: "o. max",
                  unit: Unit.usd,
                  color: colors.lime,
                }),
                line({
                  metric: price.oracle.phaseV2DailyDollars.max,
                  name: "o.2 max",
                  unit: Unit.usd,
                  color: colors.emerald,
                }),
                line({
                  metric: price.oracle.phaseDailyDollars.min,
                  name: "o. min",
                  unit: Unit.usd,
                  color: colors.rose,
                }),
                line({
                  metric: price.oracle.phaseV2DailyDollars.min,
                  name: "o.2 min",
                  unit: Unit.usd,
                  color: colors.purple,
                }),
              ],
            },
          ]
        : []),

      // Capitalization
      {
        name: "Capitalization",
        title: "Market Cap",
        bottom: [
          line({
            metric: supply.marketCap,
            name: "Capitalization",
            unit: Unit.usd,
          }),
        ],
      },

      // All Time High
      {
        name: "All Time High",
        title: "All Time High",
        top: [line({ metric: ath.priceAth, name: "ATH", unit: Unit.usd })],
        bottom: [
          line({
            metric: ath.priceDrawdown,
            name: "Drawdown",
            color: colors.red,
            unit: Unit.percentage,
          }),
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

      // Moving Averages
      createAveragesSection(ctx, movingAverage),

      // Returns
      createReturnsSection(ctx, returns),

      // Volatility
      createVolatilitySection(ctx, { volatility, range }),

      // Momentum
      createMomentumSection(ctx, indicators),

      // Bands
      createBandsSection(ctx, { range, movingAverage }),

      // Valuation
      createValuationSection(ctx, { indicators, movingAverage }),

      // DCA vs Lump Sum
      createDcaVsLumpSumSection(ctx, { dca, lookback, returns }),

      // DCA by Year
      createDcaByYearSection(ctx, { dca }),
    ],
  };
}
