/** Build cohort data arrays from brk.metrics */

import { colors } from "../../utils/colors.js";
import { entries } from "../../utils/array.js";
import { brk } from "../../client.js";

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
  ADDRESSABLE_TYPES.includes(/** @type {any} */ (key));

/**
 * Build all cohort data from brk tree
 */
export function buildCohortData() {
  const utxoCohorts = brk.metrics.distribution.utxoCohorts;
  const addressCohorts = brk.metrics.distribution.addressCohorts;
  const { addrCount } = brk.metrics.distribution;
  const {
    TERM_NAMES,
    EPOCH_NAMES,
    MAX_AGE_NAMES,
    MIN_AGE_NAMES,
    AGE_RANGE_NAMES,
    GE_AMOUNT_NAMES,
    LT_AMOUNT_NAMES,
    AMOUNT_RANGE_NAMES,
    SPENDABLE_TYPE_NAMES,
    YEAR_NAMES,
  } = brk;

  // Base cohort representing "all"
  const cohortAll = {
    name: "",
    title: "",
    color: colors.bitcoin,
    tree: utxoCohorts.all,
    addrCount: addrCount.all,
  };

  // Term cohorts
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

  // Max age cohorts (up to X time)
  const upToDate = entries(utxoCohorts.maxAge).map(([key, tree], i, arr) => {
    const names = MAX_AGE_NAMES[key];
    return {
      name: names.short,
      title: `UTXOs ${names.long}`,
      color: colors.at(i, arr.length),
      tree,
    };
  });

  // Min age cohorts (from X time)
  const fromDate = entries(utxoCohorts.minAge).map(([key, tree], i, arr) => {
    const names = MIN_AGE_NAMES[key];
    return {
      name: names.short,
      title: `UTXOs ${names.long}`,
      color: colors.at(i, arr.length),
      tree,
    };
  });

  // Age range cohorts
  const dateRange = entries(utxoCohorts.ageRange).map(([key, tree], i, arr) => {
    const names = AGE_RANGE_NAMES[key];
    return {
      name: names.short,
      title: `UTXOs ${names.long}`,
      color: colors.at(i, arr.length),
      tree,
    };
  });

  // Epoch cohorts
  const epoch = entries(utxoCohorts.epoch).map(([key, tree], i, arr) => {
    const names = EPOCH_NAMES[key];
    return {
      name: names.short,
      title: names.long,
      color: colors.at(i, arr.length),
      tree,
    };
  });

  // UTXOs above amount
  const utxosAboveAmount = entries(utxoCohorts.geAmount).map(
    ([key, tree], i, arr) => {
      const names = GE_AMOUNT_NAMES[key];
      return {
        name: names.short,
        title: `UTXOs ${names.long}`,
        color: colors.at(i, arr.length),
        tree,
      };
    },
  );

  // Addresses above amount
  const addressesAboveAmount = entries(addressCohorts.geAmount).map(
    ([key, cohort], i, arr) => {
      const names = GE_AMOUNT_NAMES[key];
      return {
        name: names.short,
        title: `Addresses ${names.long}`,
        color: colors.at(i, arr.length),
        tree: cohort,
        addrCount: {
          count: cohort.addrCount,
          _30dChange: cohort.addrCount30dChange,
        },
      };
    },
  );

  // UTXOs under amount
  const utxosUnderAmount = entries(utxoCohorts.ltAmount).map(
    ([key, tree], i, arr) => {
      const names = LT_AMOUNT_NAMES[key];
      return {
        name: names.short,
        title: `UTXOs ${names.long}`,
        color: colors.at(i, arr.length),
        tree,
      };
    },
  );

  // Addresses under amount
  const addressesUnderAmount = entries(addressCohorts.ltAmount).map(
    ([key, cohort], i, arr) => {
      const names = LT_AMOUNT_NAMES[key];
      return {
        name: names.short,
        title: `Addresses ${names.long}`,
        color: colors.at(i, arr.length),
        tree: cohort,
        addrCount: {
          count: cohort.addrCount,
          _30dChange: cohort.addrCount30dChange,
        },
      };
    },
  );

  // UTXOs amount ranges
  const utxosAmountRanges = entries(utxoCohorts.amountRange).map(
    ([key, tree], i, arr) => {
      const names = AMOUNT_RANGE_NAMES[key];
      return {
        name: names.short,
        title: `UTXOs ${names.long}`,
        color: colors.at(i, arr.length),
        tree,
      };
    },
  );

  // Addresses amount ranges
  const addressesAmountRanges = entries(addressCohorts.amountRange).map(
    ([key, cohort], i, arr) => {
      const names = AMOUNT_RANGE_NAMES[key];
      return {
        name: names.short,
        title: `Addresses ${names.long}`,
        color: colors.at(i, arr.length),
        tree: cohort,
        addrCount: {
          count: cohort.addrCount,
          _30dChange: cohort.addrCount30dChange,
        },
      };
    },
  );

  // Spendable type cohorts - split by addressability
  const typeAddressable = ADDRESSABLE_TYPES.map((key, i, arr) => {
    const names = SPENDABLE_TYPE_NAMES[key];
    return {
      name: names.short,
      title: names.short,
      color: colors.at(i, arr.length),
      tree: utxoCohorts.type[key],
      addrCount: addrCount[key],
    };
  });

  const typeOther = entries(utxoCohorts.type)
    .filter(([key]) => !isAddressable(key))
    .map(([key, tree], i, arr) => {
      const names = SPENDABLE_TYPE_NAMES[key];
      return {
        name: names.short,
        title: names.short,
        color: colors.at(i, arr.length),
        tree,
      };
    });

  // Year cohorts
  const year = entries(utxoCohorts.year)
    .reverse()
    .map(([key, tree], i, arr) => {
      const names = YEAR_NAMES[key];
      return {
        name: names.short,
        title: names.long,
        color: colors.at(i, arr.length),
        tree,
      };
    });

  return {
    cohortAll,
    termShort,
    termLong,
    upToDate,
    fromDate,
    dateRange,
    epoch,
    utxosAboveAmount,
    addressesAboveAmount,
    utxosUnderAmount,
    addressesUnderAmount,
    utxosAmountRanges,
    addressesAmountRanges,
    typeAddressable,
    typeOther,
    year,
  };
}
