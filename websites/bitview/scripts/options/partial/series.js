/** Series helpers for creating chart series blueprints */

/**
 * Create a single series from a tree accessor
 * @param {Object} args
 * @param {MetricAccessor<any>} args.metric - Tree accessor with .by property
 * @param {string} args.name - Display name for the series
 * @param {Color} [args.color]
 * @param {Unit} [args.unit]
 * @param {boolean} [args.defaultActive]
 * @param {LineSeriesPartialOptions} [args.options]
 * @returns {AnyFetchedSeriesBlueprint}
 */
export function s({ metric, name, color, defaultActive, unit, options }) {
  return {
    metric,
    title: name,
    color,
    unit,
    defaultActive,
    options,
  };
}

/**
 * Create series from a BlockCountPattern ({ base, sum, cumulative })
 * @param {Colors} colors
 * @param {BlockCountPattern<any>} pattern
 * @param {string} title
 * @param {Color} [color]
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function fromBlockCount(colors, pattern, title, color) {
  return [
    { metric: pattern.base, title, color: color ?? colors.default },
    {
      metric: pattern.sum,
      title: `${title} (sum)`,
      color: colors.red,
      defaultActive: false,
    },
    {
      metric: pattern.cumulative,
      title: `${title} (cum.)`,
      color: colors.cyan,
      defaultActive: false,
    },
  ];
}

/**
 * Create series from a BitcoinPattern ({ base, sum, cumulative, average, min, max, median, pct* })
 * @param {Colors} colors
 * @param {BitcoinPattern<any>} pattern
 * @param {string} title
 * @param {Color} [color]
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function fromBitcoin(colors, pattern, title, color) {
  return [
    { metric: pattern.base, title, color: color ?? colors.default },
    { metric: pattern.average, title: "Average", defaultActive: false },
    {
      metric: pattern.sum,
      title: `${title} (sum)`,
      color: colors.red,
      defaultActive: false,
    },
    {
      metric: pattern.cumulative,
      title: `${title} (cum.)`,
      color: colors.cyan,
      defaultActive: false,
    },
    {
      metric: pattern.max,
      title: "Max",
      color: colors.pink,
      defaultActive: false,
    },
    {
      metric: pattern.min,
      title: "Min",
      color: colors.green,
      defaultActive: false,
    },
    {
      metric: pattern.median,
      title: "Median",
      color: colors.amber,
      defaultActive: false,
    },
    {
      metric: pattern.pct75,
      title: "pct75",
      color: colors.red,
      defaultActive: false,
    },
    {
      metric: pattern.pct25,
      title: "pct25",
      color: colors.yellow,
      defaultActive: false,
    },
    {
      metric: pattern.pct90,
      title: "pct90",
      color: colors.rose,
      defaultActive: false,
    },
    {
      metric: pattern.pct10,
      title: "pct10",
      color: colors.lime,
      defaultActive: false,
    },
  ];
}

/**
 * Create series from a BlockSizePattern ({ sum, cumulative, average, min, max, median, pct* })
 * @param {Colors} colors
 * @param {BlockSizePattern<any>} pattern
 * @param {string} title
 * @param {Color} [color]
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function fromBlockSize(colors, pattern, title, color) {
  return [
    { metric: pattern.sum, title, color: color ?? colors.default },
    { metric: pattern.average, title: "Average", defaultActive: false },
    {
      metric: pattern.cumulative,
      title: `${title} (cum.)`,
      color: colors.cyan,
      defaultActive: false,
    },
    {
      metric: pattern.max,
      title: "Max",
      color: colors.pink,
      defaultActive: false,
    },
    {
      metric: pattern.min,
      title: "Min",
      color: colors.green,
      defaultActive: false,
    },
    {
      metric: pattern.median,
      title: "Median",
      color: colors.amber,
      defaultActive: false,
    },
    {
      metric: pattern.pct75,
      title: "pct75",
      color: colors.red,
      defaultActive: false,
    },
    {
      metric: pattern.pct25,
      title: "pct25",
      color: colors.yellow,
      defaultActive: false,
    },
    {
      metric: pattern.pct90,
      title: "pct90",
      color: colors.rose,
      defaultActive: false,
    },
    {
      metric: pattern.pct10,
      title: "pct10",
      color: colors.lime,
      defaultActive: false,
    },
  ];
}
