/** Moving averages section */

import { Unit } from "../../utils/units.js";
import { line } from "../series.js";
import { createRatioChart, createZScoresFolder, formatCohortTitle } from "../shared.js";
import { periodIdToName } from "./utils.js";

/**
 * @param {Colors} colors
 * @param {MarketMovingAverage} ma
 */
function buildSmaAverages(colors, ma) {
  return /** @type {const} */ ([
    ["1w", 7, "red", ma.price1wSma],
    ["8d", 8, "orange", ma.price8dSma],
    ["13d", 13, "amber", ma.price13dSma],
    ["21d", 21, "yellow", ma.price21dSma],
    ["1m", 30, "lime", ma.price1mSma],
    ["34d", 34, "green", ma.price34dSma],
    ["55d", 55, "emerald", ma.price55dSma],
    ["89d", 89, "teal", ma.price89dSma],
    ["111d", 111, "cyan", ma.price111dSma],
    ["144d", 144, "sky", ma.price144dSma],
    ["200d", 200, "blue", ma.price200dSma],
    ["350d", 350, "indigo", ma.price350dSma],
    ["1y", 365, "violet", ma.price1ySma],
    ["2y", 730, "purple", ma.price2ySma],
    ["200w", 1400, "fuchsia", ma.price200wSma],
    ["4y", 1460, "pink", ma.price4ySma],
  ]).map(([id, days, colorKey, ratio]) => ({
    id,
    name: periodIdToName(id, true),
    days,
    color: colors[colorKey],
    ratio,
  }));
}

/**
 * @param {Colors} colors
 * @param {MarketMovingAverage} ma
 */
function buildEmaAverages(colors, ma) {
  return /** @type {const} */ ([
    ["1w", 7, "red", ma.price1wEma],
    ["8d", 8, "orange", ma.price8dEma],
    ["12d", 12, "amber", ma.price12dEma],
    ["13d", 13, "yellow", ma.price13dEma],
    ["21d", 21, "lime", ma.price21dEma],
    ["26d", 26, "green", ma.price26dEma],
    ["1m", 30, "emerald", ma.price1mEma],
    ["34d", 34, "teal", ma.price34dEma],
    ["55d", 55, "cyan", ma.price55dEma],
    ["89d", 89, "sky", ma.price89dEma],
    ["144d", 144, "blue", ma.price144dEma],
    ["200d", 200, "indigo", ma.price200dEma],
    ["1y", 365, "violet", ma.price1yEma],
    ["2y", 730, "purple", ma.price2yEma],
    ["200w", 1400, "fuchsia", ma.price200wEma],
    ["4y", 1460, "pink", ma.price4yEma],
  ]).map(([id, days, colorKey, ratio]) => ({
    id,
    name: periodIdToName(id, true),
    days,
    color: colors[colorKey],
    ratio,
  }));
}

/**
 * Create price with ratio options (for moving averages)
 * @param {PartialContext} ctx
 * @param {Object} args
 * @param {string} args.title
 * @param {string} args.legend
 * @param {EmaRatioPattern} args.ratio
 * @param {Color} args.color
 * @returns {PartialOptionsTree}
 */
export function createPriceWithRatioOptions(
  ctx,
  { title, legend, ratio, color },
) {
  const priceMetric = ratio.price;

  return [
    {
      name: "Price",
      title,
      top: [line({ metric: priceMetric, name: legend, color, unit: Unit.usd })],
    },
    createRatioChart(ctx, { title: formatCohortTitle(title), price: priceMetric, ratio, color }),
    createZScoresFolder(ctx, {
      title,
      legend,
      price: priceMetric,
      ratio,
      color,
    }),
  ];
}

/** Common period IDs to show at top level */
const COMMON_PERIODS = ["1w", "1m", "200d", "1y", "200w", "4y"];

/**
 * @param {PartialContext} ctx
 * @param {MarketMovingAverage} movingAverage
 */
export function createAveragesSection(ctx, movingAverage) {
  const { colors } = ctx;
  const smaAverages = buildSmaAverages(colors, movingAverage);
  const emaAverages = buildEmaAverages(colors, movingAverage);

  /**
   * @param {string} label
   * @param {ReturnType<typeof buildSmaAverages> | ReturnType<typeof buildEmaAverages>} averages
   */
  const createSubSection = (label, averages) => {
    const commonAverages = averages.filter(({ id }) => COMMON_PERIODS.includes(id));
    const moreAverages = averages.filter(({ id }) => !COMMON_PERIODS.includes(id));

    return {
      name: label,
      tree: [
        {
          name: "Compare",
          title: `Price ${label}s`,
          top: averages.map(({ id, color, ratio }) =>
            line({
              metric: ratio.price,
              name: id,
              color,
              unit: Unit.usd,
            }),
          ),
        },
        // Common periods at top level
        ...commonAverages.map(({ name, color, ratio }) => ({
          name,
          tree: createPriceWithRatioOptions(ctx, {
            ratio,
            title: `${name} ${label}`,
            legend: "average",
            color,
          }),
        })),
        // Less common periods in "More..." folder
        {
          name: "More...",
          tree: moreAverages.map(({ name, color, ratio }) => ({
            name,
            tree: createPriceWithRatioOptions(ctx, {
              ratio,
              title: `${name} ${label}`,
              legend: "average",
              color,
            }),
          })),
        },
      ],
    };
  };

  return {
    name: "Moving Averages",
    tree: [
      createSubSection("SMA", smaAverages),
      createSubSection("EMA", emaAverages),
    ],
  };
}
