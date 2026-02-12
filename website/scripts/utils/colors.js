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
 * Resolve a light-dark() value based on current theme
 * @param {string} value
 */
function resolveLightDark(value) {
  if (value.startsWith("light-dark(")) {
    const [light, _dark] = value.slice(11, -1).split(", ");
    return dark ? _dark : light;
  }
  return value;
}

/**
 * @param {string} name
 */
function getColor(name) {
  return globalComputedStyle.getPropertyValue(`--${name}`).trim();
}

/**
 * @param {string} property
 */
function getLightDarkValue(property) {
  return resolveLightDark(
    globalComputedStyle.getPropertyValue(property).trim(),
  );
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

const paletteArr = Object.values(palette);

/**
 * Get a palette color by index, spreading small groups for better separation
 * @param {number} index
 * @param {number} [length]
 */
function at(index, length) {
  const n = paletteArr.length;
  if (length && length <= n / 2) {
    return paletteArr[Math.round((index * n) / length) % n];
  }
  return paletteArr[index % n];
}

/**
 * Build a named color map from keys, using position-based palette assignment
 * @param {readonly string[]} keys
 */
function seq(keys) {
  return Object.fromEntries(keys.map((key, i) => [key, at(i, keys.length)]));
}

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
  mining: seq(["coinbase", "subsidy", "fee"]),

  // Network
  segwit: palette.cyan,

  // Entity (transactions, inputs, outputs)
  entity: seq(["tx", "input", "output"]),

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
    _5: palette.teal,
    _2: palette.sky,
    _1: palette.indigo,
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

  scriptType: seq([
    "p2pk65",
    "p2pk33",
    "p2pkh",
    "p2ms",
    "p2sh",
    "p2wpkh",
    "p2wsh",
    "p2tr",
    "p2a",
    "opreturn",
    "unknown",
    "empty",
  ]),

  arr: paletteArr,

  at,
};
