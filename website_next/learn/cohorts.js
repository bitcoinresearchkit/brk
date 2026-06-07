import {
  createCohortSeries,
  createCohortSeriesFromKeys,
} from "./cohort-series.js";
import {
  ageRanges,
  amountRanges,
  classes,
  epochs,
  outputTypes,
} from "./groups.js";
import { colors } from "../utils/colors.js";

export const termSeries = createCohortSeries([
  {
    label: "STH",
    color: colors.yellow,
    metric: (client) => client.series.cohorts.utxo.sth.supply.total.btc,
  },
  {
    label: "LTH",
    color: colors.fuchsia,
    metric: (client) => client.series.cohorts.utxo.lth.supply.total.btc,
  },
]);

export const ageSeries = createCohortSeriesFromKeys(
  ageRanges,
  (key) => (client) =>
    client.series.cohorts.utxo.ageRange[key].supply.total.btc,
);

export const utxoBalanceSeries = createCohortSeriesFromKeys(
  amountRanges,
  (key) => (client) =>
    client.series.cohorts.utxo.amountRange[key].supply.total.btc,
);

export const addressBalanceSeries = createCohortSeriesFromKeys(
  amountRanges,
  (key) => (client) =>
    client.series.cohorts.addr.amountRange[key].supply.total.btc,
);

export const typeSeries = createCohortSeriesFromKeys(
  outputTypes,
  (key) => (client) =>
    key === "opReturn"
      ? client.series.outputs.value.opReturn.cumulative.btc
      : client.series.cohorts.utxo.type[key].supply.total.btc,
);

export const epochSeries = createCohortSeriesFromKeys(
  epochs,
  (key) => (client) => client.series.cohorts.utxo.epoch[key].supply.total.btc,
);

export const classSeries = createCohortSeriesFromKeys(
  classes,
  (key) => (client) => client.series.cohorts.utxo.class[key].supply.total.btc,
);
