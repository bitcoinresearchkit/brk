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

const red = createColor(() => getColor("red"));
const orange = createColor(() => getColor("orange"));
const amber = createColor(() => getColor("amber"));
const yellow = createColor(() => getColor("yellow"));
const avocado = createColor(() => getColor("avocado"));
const lime = createColor(() => getColor("lime"));
const green = createColor(() => getColor("green"));
const emerald = createColor(() => getColor("emerald"));
const teal = createColor(() => getColor("teal"));
const cyan = createColor(() => getColor("cyan"));
const sky = createColor(() => getColor("sky"));
const blue = createColor(() => getColor("blue"));
const indigo = createColor(() => getColor("indigo"));
const violet = createColor(() => getColor("violet"));
const purple = createColor(() => getColor("purple"));
const fuchsia = createColor(() => getColor("fuchsia"));
const pink = createColor(() => getColor("pink"));
const rose = createColor(() => getColor("rose"));

const spectrumColors = {
  red,
  orange,
  amber,
  yellow,
  avocado,
  lime,
  green,
  emerald,
  teal,
  cyan,
  sky,
  blue,
  indigo,
  violet,
  purple,
  fuchsia,
  pink,
  rose,
};

const baseColors = {
  default: createColor(() => getLightDarkValue("--color")),
  gray: createColor(() => getColor("gray")),
  border: createColor(() => getLightDarkValue("--border-color")),
  ...spectrumColors,
};

export const colors = {
  ...baseColors,

  stat: {
    sum: blue,
    cumulative: indigo,
    avg: orange,
    max: green,
    pct90: cyan,
    pct75: blue,
    median: yellow,
    pct25: violet,
    pct10: fuchsia,
    min: red,
  },

  time: {
    _24h: pink,
    _1w: red,
    _1m: yellow,
    _1y: lime,
    all: teal,
  },

  term: {
    short: yellow,
    long: fuchsia,
  },

  age: {
    _1d: red,
    _1w: orange,
    _1m: yellow,
    _2m: lime,
    _3m: green,
    _4m: teal,
    _5m: cyan,
    _6m: blue,
    _1y: indigo,
    _2y: violet,
    _3y: purple,
    _4y: fuchsia,
    _5y: pink,
    _6y: rose,
    _7y: red,
    _8y: orange,
    _10y: yellow,
    _12y: lime,
    _15y: green,
  },

  ageRange: {
    upTo1h: rose,
    _1hTo1d: pink,
    _1dTo1w: red,
    _1wTo1m: orange,
    _1mTo2m: yellow,
    _2mTo3m: yellow,
    _3mTo4m: lime,
    _4mTo5m: lime,
    _5mTo6m: lime,
    _6mTo1y: green,
    _1yTo2y: cyan,
    _2yTo3y: blue,
    _3yTo4y: indigo,
    _4yTo5y: violet,
    _5yTo6y: purple,
    _6yTo7y: purple,
    _7yTo8y: fuchsia,
    _8yTo10y: fuchsia,
    _10yTo12y: pink,
    _12yTo15y: red,
    from15y: orange,
  },

  amount: {
    _1sat: orange,
    _10sats: orange,
    _100sats: yellow,
    _1kSats: lime,
    _10kSats: green,
    _100kSats: cyan,
    _1mSats: blue,
    _10mSats: indigo,
    _1btc: purple,
    _10btc: violet,
    _100btc: fuchsia,
    _1kBtc: pink,
    _10kBtc: red,
    _100kBtc: orange,
  },

  amountRange: {
    _0sats: red,
    _1satTo10sats: orange,
    _10satsTo100sats: yellow,
    _100satsTo1kSats: lime,
    _1kSatsTo10kSats: green,
    _10kSatsTo100kSats: cyan,
    _100kSatsTo1mSats: blue,
    _1mSatsTo10mSats: indigo,
    _10mSatsTo1btc: purple,
    _1btcTo10btc: violet,
    _10btcTo100btc: fuchsia,
    _100btcTo1kBtc: pink,
    _1kBtcTo10kBtc: red,
    _10kBtcTo100kBtc: orange,
    _100kBtcOrMore: yellow,
  },

  epoch: {
    _0: red,
    _1: yellow,
    _2: orange,
    _3: lime,
    _4: green,
  },

  year: {
    _2009: red,
    _2010: orange,
    _2011: amber,
    _2012: yellow,
    _2013: lime,
    _2014: green,
    _2015: teal,
    _2016: cyan,
    _2017: sky,
    _2018: blue,
    _2019: indigo,
    _2020: violet,
    _2021: purple,
    _2022: fuchsia,
    _2023: pink,
    _2024: rose,
    _2025: red,
    _2026: orange,
  },

  returns: {
    _1d: red,
    _1w: orange,
    _1m: yellow,
    _3m: lime,
    _6m: green,
    _1y: teal,
    _2y: cyan,
    _3y: sky,
    _4y: blue,
    _5y: indigo,
    _6y: violet,
    _8y: purple,
    _10y: fuchsia,
  },

  ma: {
    _1w: red,
    _8d: orange,
    _12d: amber,
    _13d: yellow,
    _21d: avocado,
    _26d: lime,
    _1m: green,
    _34d: emerald,
    _55d: teal,
    _89d: cyan,
    _111d: sky,
    _144d: blue,
    _200d: indigo,
    _350d: violet,
    _1y: purple,
    _2y: fuchsia,
    _200w: pink,
    _4y: rose,
  },

  dca: {
    _1w: red,
    _1m: orange,
    _3m: yellow,
    _6m: lime,
    _1y: green,
    _2y: teal,
    _3y: cyan,
    _4y: sky,
    _5y: blue,
    _6y: indigo,
    _8y: violet,
    _10y: purple,
  },

  scriptType: {
    p2pk65: red,
    p2pk33: orange,
    p2pkh: yellow,
    p2ms: lime,
    p2sh: green,
    p2wpkh: teal,
    p2wsh: blue,
    p2tr: indigo,
    p2a: purple,
    opreturn: pink,
    unknown: violet,
    empty: fuchsia,
  },
};

/**
 * @typedef {typeof colors} Colors
 * @typedef {keyof typeof baseColors} ColorName
 */

/** Palette for indexed series */
const palette = Object.values(spectrumColors);

/**
 * Get a color by index (cycles through palette)
 * @param {number} index
 */
export const colorAt = (index) => palette[index % palette.length];
