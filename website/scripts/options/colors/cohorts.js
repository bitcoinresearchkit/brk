/** Cohort color mappings */

/** @type {Readonly<Record<string, ColorName>>} */
export const termColors = {
  short: "yellow",
  long: "fuchsia",
};

/** @type {Readonly<Record<string, ColorName>>} */
export const maxAgeColors = {
  _1w: "red",
  _1m: "orange",
  _2m: "amber",
  _3m: "yellow",
  _4m: "lime",
  _5m: "green",
  _6m: "teal",
  _1y: "sky",
  _2y: "indigo",
  _3y: "violet",
  _4y: "purple",
  _5y: "fuchsia",
  _6y: "pink",
  _7y: "red",
  _8y: "orange",
  _10y: "amber",
  _12y: "yellow",
  _15y: "lime",
};

/** @type {Readonly<Record<string, ColorName>>} */
export const minAgeColors = {
  _1d: "red",
  _1w: "orange",
  _1m: "yellow",
  _2m: "lime",
  _3m: "green",
  _4m: "teal",
  _5m: "cyan",
  _6m: "blue",
  _1y: "indigo",
  _2y: "violet",
  _3y: "purple",
  _4y: "fuchsia",
  _5y: "pink",
  _6y: "rose",
  _7y: "red",
  _8y: "orange",
  _10y: "yellow",
  _12y: "lime",
};

/** @type {Readonly<Record<string, ColorName>>} */
export const ageRangeColors = {
  upTo1d: "pink",
  _1dTo1w: "red",
  _1wTo1m: "orange",
  _1mTo2m: "yellow",
  _2mTo3m: "yellow",
  _3mTo4m: "lime",
  _4mTo5m: "lime",
  _5mTo6m: "lime",
  _6mTo1y: "green",
  _1yTo2y: "cyan",
  _2yTo3y: "blue",
  _3yTo4y: "indigo",
  _4yTo5y: "violet",
  _5yTo6y: "purple",
  _6yTo7y: "purple",
  _7yTo8y: "fuchsia",
  _8yTo10y: "fuchsia",
  _10yTo12y: "pink",
  _12yTo15y: "red",
  from15y: "orange",
};

/** @type {Readonly<Record<string, ColorName>>} */
export const epochColors = {
  _0: "red",
  _1: "yellow",
  _2: "orange",
  _3: "lime",
  _4: "green",
};

/** @type {Readonly<Record<string, ColorName>>} */
export const geAmountColors = {
  _1sat: "orange",
  _10sats: "orange",
  _100sats: "yellow",
  _1kSats: "lime",
  _10kSats: "green",
  _100kSats: "cyan",
  _1mSats: "blue",
  _10mSats: "indigo",
  _1btc: "purple",
  _10btc: "violet",
  _100btc: "fuchsia",
  _1kBtc: "pink",
  _10kBtc: "red",
};

/** @type {Readonly<Record<string, ColorName>>} */
export const ltAmountColors = {
  _10sats: "orange",
  _100sats: "yellow",
  _1kSats: "lime",
  _10kSats: "green",
  _100kSats: "cyan",
  _1mSats: "blue",
  _10mSats: "indigo",
  _1btc: "purple",
  _10btc: "violet",
  _100btc: "fuchsia",
  _1kBtc: "pink",
  _10kBtc: "red",
  _100kBtc: "orange",
};

/** @type {Readonly<Record<string, ColorName>>} */
export const amountRangeColors = {
  _0sats: "red",
  _1satTo10sats: "orange",
  _10satsTo100sats: "yellow",
  _100satsTo1kSats: "lime",
  _1kSatsTo10kSats: "green",
  _10kSatsTo100kSats: "cyan",
  _100kSatsTo1mSats: "blue",
  _1mSatsTo10mSats: "indigo",
  _10mSatsTo1btc: "purple",
  _1btcTo10btc: "violet",
  _10btcTo100btc: "fuchsia",
  _100btcTo1kBtc: "pink",
  _1kBtcTo10kBtc: "red",
  _10kBtcTo100kBtc: "orange",
  _100kBtcOrMore: "yellow",
};

/** @type {Readonly<Record<string, ColorName>>} */
export const spendableTypeColors = {
  p2pk65: "red",
  p2pk33: "orange",
  p2pkh: "yellow",
  p2ms: "lime",
  p2sh: "green",
  p2wpkh: "teal",
  p2wsh: "blue",
  p2tr: "indigo",
  p2a: "purple",
  unknown: "violet",
  empty: "fuchsia",
};

/** @type {Readonly<Record<string, ColorName>>} */
export const yearColors = {
  _2009: "red",
  _2010: "orange",
  _2011: "amber",
  _2012: "yellow",
  _2013: "lime",
  _2014: "green",
  _2015: "teal",
  _2016: "cyan",
  _2017: "sky",
  _2018: "blue",
  _2019: "indigo",
  _2020: "violet",
  _2021: "purple",
  _2022: "fuchsia",
  _2023: "pink",
  _2024: "rose",
  _2025: "red",
  _2026: "orange",
};
