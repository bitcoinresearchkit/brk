/** Indicators section - Main entry point */

import { createMomentumSection } from "./momentum.js";
import { createVolatilitySection } from "./volatility.js";
import { createBandsSection } from "./bands.js";
import { createOnchainSection } from "./onchain.js";

/**
 * Create Indicators section
 * @param {PartialContext} ctx
 * @param {Object} args
 * @param {Market["volatility"]} args.volatility
 * @param {Market["range"]} args.range
 * @param {Market["movingAverage"]} args.movingAverage
 * @param {Market["indicators"]} args.indicators
 */
export function createIndicatorsSection(ctx, { volatility, range, movingAverage, indicators }) {
  return {
    name: "Indicators",
    tree: [
      createMomentumSection(ctx, indicators),
      createVolatilitySection(ctx, { volatility, range }),
      createBandsSection(ctx, { range, movingAverage }),
      createOnchainSection(ctx, { indicators, movingAverage }),
    ],
  };
}
