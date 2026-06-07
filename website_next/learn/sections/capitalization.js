import { capitalizationSeries } from "../capitalization.js";
import { units } from "../charts/units.js";
import { viewTypes } from "../charts/views.js";
import { marketCapSection } from "./capitalization/market.js";
import { realizedCapSection } from "./capitalization/realized.js";

export const capitalizationSection = {
  title: "Capitalization",
  description:
    "Shows ways to value Bitcoin in US dollars. Market cap uses today's price, while realized cap uses the price when coins last moved on-chain.",
  chart: {
    title: "Capitalization",
    unit: units.usd,
    defaultType: viewTypes.line,
    series: capitalizationSeries,
  },
  children: [
    marketCapSection,
    realizedCapSection,
  ],
};
