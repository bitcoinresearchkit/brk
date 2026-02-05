import { oklchToRgba } from "../chart/oklch.js";
import { dark } from "./theme.js";

/** @type {Map<string, string>} */
const rgbaCache = new Map();

/**
 * Convert oklch to rgba with caching
 * @param {string} color - oklch color string
 */
function toRgba(color) {
  if (color === "transparent") return color;
  const cached = rgbaCache.get(color);
  if (cached) return cached;
  const rgba = oklchToRgba(color);
  rgbaCache.set(color, rgba);
  return rgba;
}

/**
 * Reduce color opacity to 50% for dimming effect
 * @param {string} color - oklch color string
 */
function tameColor(color) {
  if (color === "transparent") return color;
  return `${color.slice(0, -1)} / 25%)`;
}

/**
 * @typedef {Object} ColorMethods
 * @property {() => string} tame - Returns tamed (50% opacity) version
 * @property {(highlighted: boolean) => string} highlight - Returns normal if highlighted, tamed otherwise
 */

/**
 * @typedef {(() => string) & ColorMethods} Color
 */

/**
 * Creates a Color object that is callable and has utility methods
 * @param {() => string} getter
 * @returns {Color}
 */
function createColor(getter) {
  const color = /** @type {Color} */ (() => toRgba(getter()));
  color.tame = () => toRgba(tameColor(getter()));
  color.highlight = (highlighted) =>
    highlighted ? toRgba(getter()) : toRgba(tameColor(getter()));
  return color;
}

const globalComputedStyle = getComputedStyle(window.document.documentElement);

/**
 * @param {string} name
 */
function getColor(name) {
  return globalComputedStyle.getPropertyValue(`--${name}`);
}

/**
 * @param {string} property
 */
function getLightDarkValue(property) {
  const value = globalComputedStyle.getPropertyValue(property);
  const [light, _dark] = value.slice(11, -1).split(", ");
  return dark ? _dark : light;
}

const palette = {
  red: createColor(() => getColor("red")),
  orange: createColor(() => getColor("orange")),
  amber: createColor(() => getColor("amber")),
  yellow: createColor(() => getColor("yellow")),
  avocado: createColor(() => getColor("avocado")),
  lime: createColor(() => getColor("lime")),
  green: createColor(() => getColor("green")),
  emerald: createColor(() => getColor("emerald")),
  teal: createColor(() => getColor("teal")),
  cyan: createColor(() => getColor("cyan")),
  sky: createColor(() => getColor("sky")),
  blue: createColor(() => getColor("blue")),
  indigo: createColor(() => getColor("indigo")),
  violet: createColor(() => getColor("violet")),
  purple: createColor(() => getColor("purple")),
  fuchsia: createColor(() => getColor("fuchsia")),
  pink: createColor(() => getColor("pink")),
  rose: createColor(() => getColor("rose")),
};

export const colors = {
  default: createColor(() => getLightDarkValue("--color")),
  gray: createColor(() => getColor("gray")),
  border: createColor(() => getLightDarkValue("--border-color")),

  // Directional
  profit: palette.green,
  loss: palette.red,
  bitcoin: palette.orange,
  usd: palette.green,

  // Bi-color pairs for baselines (spaced by 2 in palette)
  bi: {
    /** @type {[Color, Color]} */
    p1: [palette.green, palette.red],
    /** @type {[Color, Color]} */
    p2: [palette.teal, palette.amber],
    /** @type {[Color, Color]} */
    p3: [palette.sky, palette.avocado],
  },

  // Cointime economics
  liveliness: palette.pink,
  vaulted: palette.lime,
  active: palette.rose,
  activity: palette.purple,
  cointime: palette.yellow,
  destroyed: palette.red,
  created: palette.orange,
  stored: palette.green,

  // Valuations
  realized: palette.orange,
  investor: palette.fuchsia,
  thermo: palette.emerald,
  trueMarketMean: palette.blue,
  vocdd: palette.purple,
  hodlBank: palette.blue,
  reserveRisk: palette.orange,

  // Comparisons (base vs adjusted)
  base: palette.orange,
  adjusted: palette.purple,
  adjustedCreated: palette.lime,
  adjustedDestroyed: palette.pink,

  // Ratios
  plRatio: palette.yellow,

  // Mining
  mining: {
    coinbase: palette.red,
    subsidy: palette.orange,
    fee: palette.yellow,
  },

  // Network
  segwit: palette.cyan,

  // Entity (transactions, inputs, outputs)
  entity: {
    tx: palette.red,
    input: palette.orange,
    output: palette.yellow,
  },

  // Technical indicators
  indicator: {
    main: palette.indigo,
    fast: palette.blue,
    slow: palette.orange,
    upper: palette.green,
    lower: palette.red,
    mid: palette.yellow,
  },

  stat: {
    sum: palette.blue,
    cumulative: palette.indigo,
    avg: palette.orange,
    max: palette.green,
    pct90: palette.cyan,
    pct75: palette.blue,
    median: palette.yellow,
    pct25: palette.violet,
    pct10: palette.fuchsia,
    min: palette.red,
  },

  // Ratio percentile bands (extreme values)
  ratioPct: {
    _99: palette.rose,
    _98: palette.pink,
    _95: palette.fuchsia,
    _5: palette.cyan,
    _2: palette.sky,
    _1: palette.blue,
  },

  // Standard deviation bands (warm = positive, cool = negative)
  sd: {
    _0: palette.lime,
    p05: palette.yellow,
    m05: palette.teal,
    p1: palette.amber,
    m1: palette.cyan,
    p15: palette.orange,
    m15: palette.sky,
    p2: palette.red,
    m2: palette.blue,
    p25: palette.rose,
    m25: palette.indigo,
    p3: palette.pink,
    m3: palette.violet,
  },

  // Transaction versions
  txVersion: {
    v1: palette.red,
    v2: palette.orange,
    v3: palette.yellow,
  },

  pct: {
    _100: palette.red,
    _95: palette.orange,
    _90: palette.amber,
    _85: palette.yellow,
    _80: palette.avocado,
    _75: palette.lime,
    _70: palette.green,
    _65: palette.emerald,
    _60: palette.teal,
    _55: palette.cyan,
    _50: palette.sky,
    _45: palette.blue,
    _40: palette.indigo,
    _35: palette.violet,
    _30: palette.purple,
    _25: palette.fuchsia,
    _20: palette.pink,
    _15: palette.rose,
    _10: palette.red,
    _05: palette.orange,
    _0: palette.amber,
  },

  time: {
    _24h: palette.red,
    _1w: palette.yellow,
    _1m: palette.green,
    _1y: palette.blue,
    all: palette.purple,
  },

  term: {
    short: palette.yellow,
    long: palette.fuchsia,
  },

  age: {
    _1d: palette.red,
    _1w: palette.orange,
    _1m: palette.yellow,
    _2m: palette.lime,
    _3m: palette.green,
    _4m: palette.teal,
    _5m: palette.cyan,
    _6m: palette.blue,
    _1y: palette.indigo,
    _2y: palette.violet,
    _3y: palette.purple,
    _4y: palette.fuchsia,
    _5y: palette.pink,
    _6y: palette.rose,
    _7y: palette.red,
    _8y: palette.orange,
    _10y: palette.yellow,
    _12y: palette.lime,
    _15y: palette.green,
  },

  ageRange: {
    upTo1h: palette.red,
    _1hTo1d: palette.orange,
    _1dTo1w: palette.amber,
    _1wTo1m: palette.yellow,
    _1mTo2m: palette.avocado,
    _2mTo3m: palette.lime,
    _3mTo4m: palette.green,
    _4mTo5m: palette.emerald,
    _5mTo6m: palette.teal,
    _6mTo1y: palette.cyan,
    _1yTo2y: palette.sky,
    _2yTo3y: palette.blue,
    _3yTo4y: palette.indigo,
    _4yTo5y: palette.violet,
    _5yTo6y: palette.purple,
    _6yTo7y: palette.fuchsia,
    _7yTo8y: palette.pink,
    _8yTo10y: palette.rose,
    _10yTo12y: palette.red,
    _12yTo15y: palette.orange,
    from15y: palette.amber,
  },

  amount: {
    _1sat: palette.red,
    _10sats: palette.orange,
    _100sats: palette.yellow,
    _1kSats: palette.lime,
    _10kSats: palette.green,
    _100kSats: palette.teal,
    _1mSats: palette.cyan,
    _10mSats: palette.blue,
    _1btc: palette.indigo,
    _10btc: palette.violet,
    _100btc: palette.purple,
    _1kBtc: palette.fuchsia,
    _10kBtc: palette.pink,
    _100kBtc: palette.rose,
  },

  amountRange: {
    _0sats: palette.red,
    _1satTo10sats: palette.orange,
    _10satsTo100sats: palette.yellow,
    _100satsTo1kSats: palette.lime,
    _1kSatsTo10kSats: palette.green,
    _10kSatsTo100kSats: palette.teal,
    _100kSatsTo1mSats: palette.cyan,
    _1mSatsTo10mSats: palette.blue,
    _10mSatsTo1btc: palette.indigo,
    _1btcTo10btc: palette.violet,
    _10btcTo100btc: palette.purple,
    _100btcTo1kBtc: palette.fuchsia,
    _1kBtcTo10kBtc: palette.pink,
    _10kBtcTo100kBtc: palette.rose,
    _100kBtcOrMore: palette.red,
  },

  epoch: {
    _0: palette.red,
    _1: palette.orange,
    _2: palette.yellow,
    _3: palette.lime,
    _4: palette.green,
  },

  year: {
    _2009: palette.red,
    _2010: palette.orange,
    _2011: palette.amber,
    _2012: palette.yellow,
    _2013: palette.lime,
    _2014: palette.green,
    _2015: palette.teal,
    _2016: palette.cyan,
    _2017: palette.sky,
    _2018: palette.blue,
    _2019: palette.indigo,
    _2020: palette.violet,
    _2021: palette.purple,
    _2022: palette.fuchsia,
    _2023: palette.pink,
    _2024: palette.rose,
    _2025: palette.red,
    _2026: palette.orange,
  },

  returns: {
    _1d: palette.red,
    _1w: palette.orange,
    _1m: palette.yellow,
    _3m: palette.lime,
    _6m: palette.green,
    _1y: palette.teal,
    _2y: palette.cyan,
    _3y: palette.sky,
    _4y: palette.blue,
    _5y: palette.indigo,
    _6y: palette.violet,
    _8y: palette.purple,
    _10y: palette.fuchsia,
  },

  ma: {
    _1w: palette.red,
    _8d: palette.orange,
    _12d: palette.amber,
    _13d: palette.yellow,
    _14d: palette.avocado,
    _21d: palette.avocado,
    _26d: palette.lime,
    _1m: palette.green,
    _34d: palette.emerald,
    _55d: palette.teal,
    _2m: palette.cyan,
    _89d: palette.sky,
    _111d: palette.blue,
    _144d: palette.indigo,
    _200d: palette.violet,
    _350d: palette.purple,
    _1y: palette.fuchsia,
    _2y: palette.pink,
    _200w: palette.rose,
    _4y: palette.red,
  },

  dca: {
    _1w: palette.red,
    _1m: palette.orange,
    _3m: palette.yellow,
    _6m: palette.lime,
    _1y: palette.green,
    _2y: palette.teal,
    _3y: palette.cyan,
    _4y: palette.sky,
    _5y: palette.blue,
    _6y: palette.indigo,
    _8y: palette.violet,
    _10y: palette.purple,
  },

  scriptType: {
    p2pk65: palette.red,
    p2pk33: palette.orange,
    p2pkh: palette.yellow,
    p2ms: palette.lime,
    p2sh: palette.green,
    p2wpkh: palette.teal,
    p2wsh: palette.blue,
    p2tr: palette.indigo,
    p2a: palette.violet,
    opreturn: palette.purple,
    unknown: palette.fuchsia,
    empty: palette.pink,
  },

  arr: Object.values(palette),

  /**
   * Get a color by index (cycles through palette)
   * @param {number} index
   */
  at(index) {
    return this.arr[index % this.arr.length];
  },
};
