/** Constant helpers for creating price lines and reference lines */

/**
 * Get constant pattern by number dynamically from tree
 * Examples: 0 → constant0, 38.2 → constant382, -1 → constantMinus1
 * @param {BrkClient["tree"]["computed"]["constants"]} constants
 * @param {number} num
 * @returns {Constant0Pattern<any>}
 */
export function getConstant(constants, num) {
  const key =
    num >= 0
      ? `constant${String(num).replace(".", "")}`
      : `constantMinus${Math.abs(num)}`;
  const constant = /** @type {Constant0Pattern<any> | undefined} */ (
    /** @type {Record<string, Constant0Pattern<any>>} */ (constants)[key]
  );
  if (!constant) throw new Error(`Unknown constant: ${num} (key: ${key})`);
  return constant;
}

/**
 * Flatten a Constant0Pattern into a simple MetricAccessor
 * Constant0Pattern has { dateindex: { by: {...} }, height: { by: {...} }, ... }
 * This flattens it to { by: { dateindex: MetricNode, height: MetricNode, ... } }
 * @param {Constant0Pattern<any>} pattern
 * @returns {MetricAccessor<any>}
 */
export function flattenConstant(pattern) {
  return {
    by: {
      dateindex: pattern.dateindex.by.dateindex,
      decadeindex: pattern.decadeindex.by.decadeindex,
      height: pattern.height.by.height,
      monthindex: pattern.monthindex.by.monthindex,
      quarterindex: pattern.quarterindex.by.quarterindex,
      semesterindex: pattern.semesterindex.by.semesterindex,
      weekindex: pattern.weekindex.by.weekindex,
      yearindex: pattern.yearindex.by.yearindex,
    },
    indexes() {
      return /** @type {Index[]} */ (Object.keys(this.by));
    },
  };
}

/**
 * Create a price line series (horizontal reference line)
 * @param {Object} args
 * @param {BrkClient["tree"]["computed"]["constants"]} args.constants
 * @param {Colors} args.colors
 * @param {number} [args.number]
 * @param {string} [args.name]
 * @param {boolean} [args.defaultActive]
 * @param {number} [args.lineStyle]
 * @param {Color} [args.color]
 * @param {Unit} args.unit
 * @returns {FetchedLineSeriesBlueprint}
 */
export function createPriceLine({
  constants,
  colors,
  number = 0,
  unit,
  defaultActive,
  color,
  name,
  lineStyle,
}) {
  return {
    metric: flattenConstant(getConstant(constants, number)),
    title: name ?? `${number}`,
    unit,
    defaultActive,
    color: color ?? colors.gray,
    options: {
      lineStyle: lineStyle ?? 4,
      lastValueVisible: false,
      crosshairMarkerVisible: false,
    },
  };
}

/**
 * Create multiple price lines from an array of numbers
 * @param {Object} args
 * @param {BrkClient["tree"]["computed"]["constants"]} args.constants
 * @param {Colors} args.colors
 * @param {number[]} args.numbers
 * @param {Unit} args.unit
 * @returns {FetchedLineSeriesBlueprint[]}
 */
export function createPriceLines({ constants, colors, numbers, unit }) {
  return numbers.map((number) => ({
    metric: flattenConstant(getConstant(constants, number)),
    title: `${number}`,
    unit,
    defaultActive: !number,
    color: colors.gray,
    options: {
      lineStyle: 4,
      lastValueVisible: false,
      crosshairMarkerVisible: false,
    },
  }));
}

/**
 * Create a constant line series
 * @param {Object} args
 * @param {Colors} args.colors
 * @param {Constant0Pattern<any>} args.constant
 * @param {string} args.name
 * @param {Unit} args.unit
 * @param {Color} [args.color]
 * @param {number} [args.lineStyle]
 * @param {boolean} [args.defaultActive]
 * @returns {FetchedLineSeriesBlueprint}
 */
export function line({
  colors,
  constant,
  name,
  unit,
  color,
  lineStyle,
  defaultActive,
}) {
  return {
    metric: flattenConstant(constant),
    title: name,
    unit,
    defaultActive,
    color: color ?? colors.gray,
    options: {
      lineStyle: lineStyle ?? 4,
      lastValueVisible: false,
      crosshairMarkerVisible: false,
    },
  };
}
