/** Build cohort data arrays from brk.tree */

import {
  termColors,
  maxAgeColors,
  minAgeColors,
  ageRangeColors,
  epochColors,
  geAmountColors,
  ltAmountColors,
  amountRangeColors,
  spendableTypeColors,
} from "../colors/index.js";

/**
 * @template {Record<string, any>} T
 * @param {T} obj
 * @returns {[keyof T & string, T[keyof T & string]][]}
 */
const entries = (obj) =>
  /** @type {[keyof T & string, T[keyof T & string]][]} */ (
    Object.entries(obj)
  );

/**
 * Build all cohort data from brk tree
 * @param {Colors} colors
 * @param {BrkClient} brk
 */
export function buildCohortData(colors, brk) {
  const utxoCohorts = brk.tree.distribution.utxoCohorts;
  const addressCohorts = brk.tree.distribution.addressCohorts;
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
  } = brk;

  // Base cohort representing "all" - CohortAll (adjustedSopr + percentiles but no RelToMarketCap)
  /** @type {CohortAll} */
  const cohortAll = {
    name: "",
    title: "",
    color: colors.orange,
    tree: utxoCohorts.all,
  };

  // Term cohorts - split because short is CohortFull, long is CohortWithPercentiles
  const shortNames = TERM_NAMES.short;
  /** @type {CohortFull} */
  const termShort = {
    name: shortNames.short,
    title: shortNames.long,
    color: colors[termColors.short],
    tree: utxoCohorts.term.short,
  };

  const longNames = TERM_NAMES.long;
  /** @type {CohortWithPercentiles} */
  const termLong = {
    name: longNames.short,
    title: longNames.long,
    color: colors[termColors.long],
    tree: utxoCohorts.term.long,
  };

  // Max age cohorts (up to X time) - CohortWithAdjusted (adjustedSopr only)
  /** @type {readonly CohortWithAdjusted[]} */
  const upToDate = entries(utxoCohorts.maxAge).map(([key, tree]) => {
    const names = MAX_AGE_NAMES[key];
    return {
      name: names.short,
      title: names.long,
      color: colors[maxAgeColors[key]],
      tree,
    };
  });

  // Min age cohorts (from X time) - CohortBasic (neither adjustedSopr nor percentiles)
  /** @type {readonly CohortBasic[]} */
  const fromDate = entries(utxoCohorts.minAge).map(([key, tree]) => {
    const names = MIN_AGE_NAMES[key];
    return {
      name: names.short,
      title: names.long,
      color: colors[minAgeColors[key]],
      tree,
    };
  });

  // Age range cohorts - CohortWithPercentiles (percentiles only)
  /** @type {readonly CohortWithPercentiles[]} */
  const dateRange = entries(utxoCohorts.ageRange).map(([key, tree]) => {
    const names = AGE_RANGE_NAMES[key];
    return {
      name: names.short,
      title: names.long,
      color: colors[ageRangeColors[key]],
      tree,
    };
  });

  // Epoch cohorts - CohortBasic (neither adjustedSopr nor percentiles)
  /** @type {readonly CohortBasic[]} */
  const epoch = entries(utxoCohorts.epoch).map(([key, tree]) => {
    const names = EPOCH_NAMES[key];
    return {
      name: names.short,
      title: names.long,
      color: colors[epochColors[key]],
      tree,
    };
  });

  // UTXOs above amount - CohortBasic (neither adjustedSopr nor percentiles)
  /** @type {readonly CohortBasic[]} */
  const utxosAboveAmount = entries(utxoCohorts.geAmount).map(([key, tree]) => {
    const names = GE_AMOUNT_NAMES[key];
    return {
      name: names.short,
      title: names.long,
      color: colors[geAmountColors[key]],
      tree,
    };
  });

  // Addresses above amount
  /** @type {readonly AddressCohortObject[]} */
  const addressesAboveAmount = entries(addressCohorts.geAmount).map(
    ([key, tree]) => {
      const names = GE_AMOUNT_NAMES[key];
      return {
        name: names.short,
        title: names.long,
        color: colors[geAmountColors[key]],
        tree,
      };
    },
  );

  // UTXOs under amount - CohortBasic (neither adjustedSopr nor percentiles)
  /** @type {readonly CohortBasic[]} */
  const utxosUnderAmount = entries(utxoCohorts.ltAmount).map(([key, tree]) => {
    const names = LT_AMOUNT_NAMES[key];
    return {
      name: names.short,
      title: names.long,
      color: colors[ltAmountColors[key]],
      tree,
    };
  });

  // Addresses under amount
  /** @type {readonly AddressCohortObject[]} */
  const addressesUnderAmount = entries(addressCohorts.ltAmount).map(
    ([key, tree]) => {
      const names = LT_AMOUNT_NAMES[key];
      return {
        name: names.short,
        title: names.long,
        color: colors[ltAmountColors[key]],
        tree,
      };
    },
  );

  // UTXOs amount ranges - CohortBasic (neither adjustedSopr nor percentiles)
  /** @type {readonly CohortBasic[]} */
  const utxosAmountRanges = entries(utxoCohorts.amountRange).map(
    ([key, tree]) => {
      const names = AMOUNT_RANGE_NAMES[key];
      return {
        name: names.short,
        title: names.long,
        color: colors[amountRangeColors[key]],
        tree,
      };
    },
  );

  // Addresses amount ranges
  /** @type {readonly AddressCohortObject[]} */
  const addressesAmountRanges = entries(addressCohorts.amountRange).map(
    ([key, tree]) => {
      const names = AMOUNT_RANGE_NAMES[key];
      return {
        name: names.short,
        title: names.long,
        color: colors[amountRangeColors[key]],
        tree,
      };
    },
  );

  // Spendable type cohorts - CohortBasic (neither adjustedSopr nor percentiles)
  /** @type {readonly CohortBasic[]} */
  const type = entries(utxoCohorts.type).map(([key, tree]) => {
    const names = SPENDABLE_TYPE_NAMES[key];
    return {
      name: names.short,
      title: names.long,
      color: colors[spendableTypeColors[key]],
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
    type,
  };
}
