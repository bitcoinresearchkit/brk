/** UTXO Profitability section — range bands, cumulative profit/loss thresholds */

import { colors } from "../../utils/colors.js";
import { entries } from "../../utils/array.js";
import { Unit } from "../../utils/units.js";
import { line, price } from "../series.js";
import { brk } from "../../client.js";
import { satsBtcUsd } from "../shared.js";

/**
 * @param {{ name: string, color: Color, pattern: RealizedSupplyPattern }[]} list
 * @param {string} titlePrefix
 * @returns {PartialOptionsTree}
 */
function bucketCharts(list, titlePrefix) {
  return [
    {
      name: "Supply",
      title: `${titlePrefix}: Supply`,
      bottom: list.flatMap(({ name, color, pattern }) =>
        satsBtcUsd({ pattern: pattern.supply, name, color }),
      ),
    },
    {
      name: "Realized Cap",
      title: `${titlePrefix}: Realized Cap`,
      bottom: list.map(({ name, color, pattern }) =>
        line({ metric: pattern.realizedCap, name, color, unit: Unit.usd }),
      ),
    },
    {
      name: "Realized Price",
      title: `${titlePrefix}: Realized Price`,
      top: list.map(({ name, color, pattern }) =>
        price({ metric: pattern.realizedPrice, name, color }),
      ),
    },
  ];
}

/**
 * @returns {PartialOptionsGroup}
 */
export function createUtxoProfitabilitySection() {
  const { range, profit, loss } = brk.metrics.cohorts.utxo.profitability;
  const {
    PROFITABILITY_RANGE_NAMES,
    PROFIT_NAMES,
    LOSS_NAMES,
  } = brk;

  const rangeList = entries(PROFITABILITY_RANGE_NAMES).map(
    ([key, names], i, arr) => ({
      name: names.short,
      color: colors.at(i, arr.length),
      pattern: range[key],
    }),
  );

  const profitList = entries(PROFIT_NAMES).map(([key, names], i, arr) => ({
    name: names.short,
    color: colors.at(i, arr.length),
    pattern: profit[key],
  }));

  const lossList = entries(LOSS_NAMES).map(([key, names], i, arr) => ({
    name: names.short,
    color: colors.at(i, arr.length),
    pattern: loss[key],
  }));

  return {
    name: "UTXO Profitability",
    tree: [
      {
        name: "Range",
        tree: bucketCharts(rangeList, "Profitability Range"),
      },
      {
        name: "In Profit",
        tree: bucketCharts(profitList, "In Profit"),
      },
      {
        name: "In Loss",
        tree: bucketCharts(lossList, "In Loss"),
      },
    ],
  };
}
