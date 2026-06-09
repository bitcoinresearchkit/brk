import {
  createCohortSeries,
  createCohortSeriesFromKeys,
} from "./cohort-series.js";
import {
  ageRanges,
  amountRanges,
  classes,
  epochs,
  spendableTypes,
} from "./groups.js";
import { createRollingWindowSeries } from "./rolling-windows.js";
import { colors } from "../../utils/colors.js";

export const totalSeries = createCohortSeries([
  {
    label: "UTXOs",
    color: colors.orange,
    metric: (client) => client.series.cohorts.utxo.all.outputs.unspentCount.base,
  },
]);

export const changeSeries = createRollingWindowSeries(
  (key) => (client) =>
    client.series.cohorts.utxo.all.outputs.unspentCount.delta.absolute[key],
);

export const growthRateSeries = createRollingWindowSeries(
  (key) => (client) =>
    client.series.cohorts.utxo.all.outputs.unspentCount.delta.rate[key].percent,
);

export const spentSeries = createRollingWindowSeries(
  (key) => (client) =>
    client.series.cohorts.utxo.all.outputs.spentCount.sum[key],
);

export const spendingRateSeries = createCohortSeries([
  {
    label: "Spending rate",
    color: colors.orange,
    metric: (client) => client.series.cohorts.utxo.all.outputs.spendingRate,
  },
]);

export const spendingRateTermSeries = createCohortSeries([
  {
    label: "STH",
    color: colors.yellow,
    metric: (client) => client.series.cohorts.utxo.sth.outputs.spendingRate,
  },
  {
    label: "LTH",
    color: colors.fuchsia,
    metric: (client) => client.series.cohorts.utxo.lth.outputs.spendingRate,
  },
]);

export const spendingRateAgeSeries = createCohortSeriesFromKeys(
  ageRanges,
  (key) => (client) =>
    client.series.cohorts.utxo.ageRange[key].outputs.spendingRate,
);

export const spendingRateBalanceSeries = createCohortSeriesFromKeys(
  amountRanges,
  (key) => (client) =>
    client.series.cohorts.utxo.amountRange[key].outputs.spendingRate,
);

export const spendingRateTypeSeries = createCohortSeriesFromKeys(
  spendableTypes,
  (key) => (client) =>
    client.series.cohorts.utxo.type[key].outputs.spendingRate,
);

export const spendingRateEpochSeries = createCohortSeriesFromKeys(
  epochs,
  (key) => (client) =>
    client.series.cohorts.utxo.epoch[key].outputs.spendingRate,
);

export const spendingRateClassSeries = createCohortSeriesFromKeys(
  classes,
  (key) => (client) =>
    client.series.cohorts.utxo.class[key].outputs.spendingRate,
);

export const termSeries = createCohortSeries([
  {
    label: "STH",
    color: colors.yellow,
    metric: (client) => client.series.cohorts.utxo.sth.outputs.unspentCount.base,
  },
  {
    label: "LTH",
    color: colors.fuchsia,
    metric: (client) => client.series.cohorts.utxo.lth.outputs.unspentCount.base,
  },
]);

export const ageSeries = createCohortSeriesFromKeys(
  ageRanges,
  (key) => (client) =>
    client.series.cohorts.utxo.ageRange[key].outputs.unspentCount.base,
);

export const balanceSeries = createCohortSeriesFromKeys(
  amountRanges,
  (key) => (client) =>
    client.series.cohorts.utxo.amountRange[key].outputs.unspentCount.base,
);

export const typeSeries = createCohortSeriesFromKeys(
  spendableTypes,
  (key) => (client) =>
    client.series.cohorts.utxo.type[key].outputs.unspentCount.base,
);

export const epochSeries = createCohortSeriesFromKeys(
  epochs,
  (key) => (client) =>
    client.series.cohorts.utxo.epoch[key].outputs.unspentCount.base,
);

export const classSeries = createCohortSeriesFromKeys(
  classes,
  (key) => (client) =>
    client.series.cohorts.utxo.class[key].outputs.unspentCount.base,
);
