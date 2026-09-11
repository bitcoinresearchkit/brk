import { profitabilityRanges } from "./groups.js";
import { createCohortSeries, createCohortSeriesFromKeys } from "./cohort-series.js";
import { colors } from "../../utils/colors.js";

export const circulatingSupplySeries = createCohortSeries([
  {
    label: "Circulating",
    color: colors.orange,
    metric: (client) => client.series.supply.circulating.btc,
  },
]);

export const supplyProfitabilitySeries = createCohortSeriesFromKeys(
  profitabilityRanges,
  (key) => (client) => client.series.cohorts.profitability.supply[key].all.btc,
);
