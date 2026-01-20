/**
 * Reduce color opacity to 50% for dimming effect
 * @param {string} color - oklch color string
 */
export function tameColor(color) {
  if (color === "transparent") return color;
  return `${color.slice(0, -1)} / 50%)`;
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
  const color = /** @type {Color} */ (() => getter());
  color.tame = () => tameColor(getter());
  color.highlight = (highlighted) => highlighted ? getter() : tameColor(getter());
  return color;
}

/**
 * @param {Accessor<boolean>} dark
 */
export function createColors(dark) {
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
    return dark() ? _dark : light;
  }

  return {
    default: createColor(() => getLightDarkValue("--color")),
    gray: createColor(() => getColor("gray")),
    border: createColor(() => getLightDarkValue("--border-color")),

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
}

/**
 * @typedef {ReturnType<typeof createColors>} Colors
 * @typedef {keyof Colors} ColorName
 */
