/** Market section - Main entry point */

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
  const { market, supply } = brk.tree.computed;
  const { movingAverage, ath, returns, volatility, range, dca, lookback, indicators } = market;

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
        bottom: [s({ metric: supply.marketCap.indexes, name: "Capitalization", unit: "usd" })],
      },

      // All Time High
      {
        name: "All Time High",
        title: "All Time High",
        top: [s({ metric: ath.priceAth, name: "ath", unit: "usd" })],
        bottom: [
          s({ metric: ath.priceDrawdown, name: "Drawdown", color: colors.red, unit: "percentage" }),
          s({ metric: ath.daysSincePriceAth, name: "since", unit: "days" }),
          s({ metric: ath.maxDaysBetweenPriceAths, name: "Max", color: colors.red, unit: "days" }),
          s({ metric: ath.maxYearsBetweenPriceAths, name: "Max", color: colors.red, unit: "years" }),
        ],
      },

      // Averages
      createAveragesSection(ctx, averages),

      // Performance
      createPerformanceSection(ctx, returns),

      // Indicators
      createIndicatorsSection(ctx, { volatility, range, movingAverage, indicators }),

      // Investing
      createInvestingSection(ctx, { dca, lookback, returns }),
    ],
  };
}
