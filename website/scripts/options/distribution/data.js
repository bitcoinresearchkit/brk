import { colors } from "../../utils/colors.js";
import { entries } from "../../utils/array.js";
import { brk } from "../../utils/client.js";

/** @type {readonly AddressableType[]} */
const ADDRESSABLE_TYPES = [
  "p2pk65",
  "p2pk33",
  "p2pkh",
  "p2sh",
  "p2wpkh",
  "p2wsh",
  "p2tr",
  "p2a",
];

/** @type {(key: SpendableType) => key is AddressableType} */
const isAddressable = (key) =>
  /** @type {readonly string[]} */ (ADDRESSABLE_TYPES).includes(key);

export function buildCohortData() {
  const utxoCohorts = brk.series.cohorts.utxo;
  const addressCohorts = brk.series.cohorts.addr;
  const { addrs } = brk.series;
  const {
    TERM_NAMES,
    EPOCH_NAMES,
    UNDER_AGE_NAMES,
    OVER_AGE_NAMES,
    AGE_RANGE_NAMES,
    OVER_AMOUNT_NAMES,
    UNDER_AMOUNT_NAMES,
    AMOUNT_RANGE_NAMES,
    SPENDABLE_TYPE_NAMES,
    CLASS_NAMES,
    PROFITABILITY_RANGE_NAMES,
    PROFIT_NAMES,
    LOSS_NAMES,
  } = brk;

  const cohortAll = {
    name: "",
    title: "",
    color: colors.bitcoin,
    tree: utxoCohorts.all,
    addressCount: {
      base: addrs.funded.all,
      delta: addrs.delta.all,
    },
  };

  const shortNames = TERM_NAMES.short;
  const termShort = {
    name: shortNames.short,
    title: shortNames.long,
    color: colors.term.short,
    tree: utxoCohorts.sth,
  };

  const longNames = TERM_NAMES.long;
  const termLong = {
    name: longNames.short,
    title: longNames.long,
    color: colors.term.long,
    tree: utxoCohorts.lth,
  };

  // Under age cohorts
  const underAge = entries(UNDER_AGE_NAMES).map(([key, names], i, arr) => ({
    name: names.short,
    title: `UTXOs ${names.long}`,
    color: colors.at(i, arr.length),
    tree: utxoCohorts.underAge[key],
  }));

  // Over age cohorts
  const overAge = entries(OVER_AGE_NAMES).map(([key, names], i, arr) => ({
    name: names.short,
    title: `UTXOs ${names.long}`,
    color: colors.at(i, arr.length),
    tree: utxoCohorts.overAge[key],
  }));

  const ageRange = entries(AGE_RANGE_NAMES).map(([key, names], i, arr) => ({
    name: names.short,
    title: `UTXOs ${names.long}`,
    color: colors.at(i, arr.length),
    tree: utxoCohorts.ageRange[key],
    matured: utxoCohorts.matured[key],
  }));

  const epoch = entries(EPOCH_NAMES).map(([key, names], i, arr) => ({
    name: names.short,
    title: names.long,
    color: colors.at(i, arr.length),
    tree: utxoCohorts.epoch[key],
  }));

  const utxosOverAmount = entries(OVER_AMOUNT_NAMES).map(
    ([key, names], i, arr) => ({
      name: names.short,
      title: `UTXOs ${names.long}`,
      color: colors.at(i, arr.length),
      tree: utxoCohorts.overAmount[key],
    }),
  );

  const addressesOverAmount = entries(OVER_AMOUNT_NAMES).map(
    ([key, names], i, arr) => {
      const cohort = addressCohorts.overAmount[key];
      return {
        name: names.short,
        title: `Addresses ${names.long}`,
        color: colors.at(i, arr.length),
        tree: cohort,
        addressCount: cohort.addrCount,
      };
    },
  );

  const utxosUnderAmount = entries(UNDER_AMOUNT_NAMES).map(
    ([key, names], i, arr) => ({
      name: names.short,
      title: `UTXOs ${names.long}`,
      color: colors.at(i, arr.length),
      tree: utxoCohorts.underAmount[key],
    }),
  );

  const addressesUnderAmount = entries(UNDER_AMOUNT_NAMES).map(
    ([key, names], i, arr) => {
      const cohort = addressCohorts.underAmount[key];
      return {
        name: names.short,
        title: `Addresses ${names.long}`,
        color: colors.at(i, arr.length),
        tree: cohort,
        addressCount: cohort.addrCount,
      };
    },
  );

  const utxosAmountRange = entries(AMOUNT_RANGE_NAMES).map(
    ([key, names], i, arr) => ({
      name: names.short,
      title: `UTXOs ${names.long}`,
      color: colors.at(i, arr.length),
      tree: utxoCohorts.amountRange[key],
    }),
  );

  const addressesAmountRange = entries(AMOUNT_RANGE_NAMES).map(
    ([key, names], i, arr) => {
      const cohort = addressCohorts.amountRange[key];
      return {
        name: names.short,
        title: `Addresses ${names.long}`,
        color: colors.at(i, arr.length),
        tree: cohort,
        addressCount: cohort.addrCount,
      };
    },
  );

  const typeAddressable = ADDRESSABLE_TYPES.map((key, i, arr) => {
    const names = SPENDABLE_TYPE_NAMES[key];
    return {
      name: names.short,
      title: names.short,
      color: colors.at(i, arr.length),
      tree: utxoCohorts.type[key],
      addressCount: {
        base: addrs.funded[key],
        delta: addrs.delta[key],
      },
    };
  });

  const typeOther = entries(SPENDABLE_TYPE_NAMES)
    .filter(([key]) => !isAddressable(key))
    .map(([key, names], i, arr) => ({
      name: names.short,
      title: names.short,
      color: colors.at(i, arr.length),
      tree: utxoCohorts.type[key],
    }));

  const class_ = entries(CLASS_NAMES)
    .reverse()
    .map(([key, names], i, arr) => ({
      name: names.short,
      title: names.long,
      color: colors.at(i, arr.length),
      tree: utxoCohorts.class[key],
    }));

  const { range, profit, loss } = utxoCohorts.profitability;

  const profitabilityRange = entries(PROFITABILITY_RANGE_NAMES).map(
    ([key, names], i, arr) => ({
      name: names.short,
      color: colors.at(i, arr.length),
      pattern: range[key],
    }),
  );

  const profitabilityProfit = entries(PROFIT_NAMES).map(
    ([key, names], i, arr) => ({
      name: names.short,
      color: colors.at(i, arr.length),
      pattern: profit[key],
    }),
  );

  const profitabilityLoss = entries(LOSS_NAMES).map(([key, names], i, arr) => ({
    name: names.short,
    color: colors.at(i, arr.length),
    pattern: loss[key],
  }));

  return {
    cohortAll,
    termShort,
    termLong,
    underAge,
    overAge,
    ageRange,
    epoch,
    utxosOverAmount,
    addressesOverAmount,
    utxosUnderAmount,
    addressesUnderAmount,
    utxosAmountRange,
    addressesAmountRange,
    typeAddressable,
    typeOther,
    class: class_,
    profitabilityRange,
    profitabilityProfit,
    profitabilityLoss,
  };
}
