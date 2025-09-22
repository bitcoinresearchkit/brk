/**
 * @import { Accessor } from "../packages/solidjs-signals/wrapper";
 */

const globalComputedStyle = getComputedStyle(window.document.documentElement);

/**
 * @param {Accessor<boolean>} dark
 */
export function createColors(dark) {
  /**
   * @param {string} color
   */
  function getColor(color) {
    return globalComputedStyle.getPropertyValue(`--${color}`);
  }
  function red() {
    return getColor("red");
  }
  function orange() {
    return getColor("orange");
  }
  function amber() {
    return getColor("amber");
  }
  function yellow() {
    return getColor("yellow");
  }
  function avocado() {
    return getColor("avocado");
  }
  function lime() {
    return getColor("lime");
  }
  function green() {
    return getColor("green");
  }
  function emerald() {
    return getColor("emerald");
  }
  function teal() {
    return getColor("teal");
  }
  function cyan() {
    return getColor("cyan");
  }
  function sky() {
    return getColor("sky");
  }
  function blue() {
    return getColor("blue");
  }
  function indigo() {
    return getColor("indigo");
  }
  function violet() {
    return getColor("violet");
  }
  function purple() {
    return getColor("purple");
  }
  function fuchsia() {
    return getColor("fuchsia");
  }
  function pink() {
    return getColor("pink");
  }
  function rose() {
    return getColor("rose");
  }
  function gray() {
    return getColor("gray");
  }

  /**
   * @param {string} property
   */
  function getLightDarkValue(property) {
    const value = globalComputedStyle.getPropertyValue(property);
    const [light, _dark] = value.slice(11, -1).split(", ");
    return dark() ? _dark : light;
  }

  function textColor() {
    return getLightDarkValue("--color");
  }
  function borderColor() {
    return getLightDarkValue("--border-color");
  }

  return {
    default: textColor,
    gray,
    border: borderColor,

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
}
/**
 * @typedef {ReturnType<typeof createColors>} Colors
 * @typedef {Colors["orange"]} Color
 * @typedef {keyof Colors} ColorName
 */
