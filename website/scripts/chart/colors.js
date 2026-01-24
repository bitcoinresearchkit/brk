import { oklchToRgba } from "./oklch.js";
import { dark } from "../utils/theme.js";

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

export const colors = {
  default: createColor(() => getLightDarkValue("--color")),
  gray: createColor(() => getColor("gray")),
  border: createColor(() => getLightDarkValue("--border-color")),

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

  /** Semantic stat colors for pattern helpers */
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
};

/**
 * @typedef {typeof colors} Colors
 * @typedef {Exclude<keyof Colors, "stat">} ColorName
 */
