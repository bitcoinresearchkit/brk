/** Market section - Main entry point */

import { Unit } from "../../utils/units.js";
import { buildAverages, createAveragesSection } from "./averages.js";
import { createPerformanceSection } from "./performance.js";
import { createIndicatorsSection } from "./indicators/index.js";
import { createInvestingSection } from "./investing.js";

/**
 * Create Market section
 * @param {PartialContext} ctx
 * @returns {PartialOptionsGroup}
 */
export function createMarketSection(ctx) {
  const { colors, brk, s } = ctx;
  const { market, supply } = brk.tree;
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

  const averages = buildAverages(colors, movingAverage);

  return {
    name: "Market",
    tree: [
      // Price
      {
        name: "Price",
        title: "Bitcoin Price",
      },

      // Capitalization
      {
        name: "Capitalization",
        title: "Market Capitalization",
        bottom: [
          s({
            metric: brk.mergeMetricPatterns(
              supply.marketCap.height,
              supply.marketCap.indexes,
            ),
            name: "Capitalization",
            unit: Unit.usd,
          }),
        ],
      },

      // All Time High
      {
        name: "All Time High",
        title: "All Time High",
        top: [s({ metric: ath.priceAth, name: "ATH", unit: Unit.usd })],
        bottom: [
          s({
            metric: ath.priceDrawdown,
            name: "Drawdown",
            color: colors.red,
            unit: Unit.percentage,
          }),
          s({ metric: ath.daysSincePriceAth, name: "Since", unit: Unit.days }),
          s({
            metric: ath.yearsSincePriceAth,
            name: "Since",
            unit: Unit.years,
          }),
          s({
            metric: ath.maxDaysBetweenPriceAths,
            name: "Max",
            color: colors.red,
            unit: Unit.days,
          }),
          s({
            metric: ath.maxYearsBetweenPriceAths,
            name: "Max",
            color: colors.red,
            unit: Unit.years,
          }),
        ],
      },

      // Averages
      createAveragesSection(ctx, averages),

      // Performance
      createPerformanceSection(ctx, returns),

      // Indicators
      createIndicatorsSection(ctx, {
        volatility,
        range,
        movingAverage,
        indicators,
      }),

      // Investing
      createInvestingSection(ctx, { dca, lookback, returns }),
    ],
  };
}
