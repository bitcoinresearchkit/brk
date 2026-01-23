/** Moving averages section */

import { Unit } from "../../utils/units.js";
import { priceLine, priceLines } from "../constants.js";
import { line, baseline } from "../series.js";
import {
  percentileUsdMap,
  percentileMap,
  sdPatterns,
  sdBands,
} from "../shared.js";
import { periodIdToName } from "./utils.js";

/**
 * Build averages data array from market patterns
 * @param {Colors} colors
 * @param {MarketMovingAverage} ma
 */
export function buildAverages(colors, ma) {
  return /** @type {const} */ ([
    ["1w", 7, "red", ma.price1wSma, ma.price1wEma],
    ["8d", 8, "orange", ma.price8dSma, ma.price8dEma],
    ["13d", 13, "amber", ma.price13dSma, ma.price13dEma],
    ["21d", 21, "yellow", ma.price21dSma, ma.price21dEma],
    ["1m", 30, "lime", ma.price1mSma, ma.price1mEma],
    ["34d", 34, "green", ma.price34dSma, ma.price34dEma],
    ["55d", 55, "emerald", ma.price55dSma, ma.price55dEma],
    ["89d", 89, "teal", ma.price89dSma, ma.price89dEma],
    ["144d", 144, "cyan", ma.price144dSma, ma.price144dEma],
    ["200d", 200, "sky", ma.price200dSma, ma.price200dEma],
    ["1y", 365, "blue", ma.price1ySma, ma.price1yEma],
    ["2y", 730, "indigo", ma.price2ySma, ma.price2yEma],
    ["200w", 1400, "violet", ma.price200wSma, ma.price200wEma],
    ["4y", 1460, "purple", ma.price4ySma, ma.price4yEma],
  ]).map(([id, days, colorKey, sma, ema]) => ({
    id,
    name: periodIdToName(id, true),
    days,
    color: colors[colorKey],
    sma,
    ema,
  }));
}

/**
 * Create price with ratio options (for moving averages)
 * @param {PartialContext} ctx
 * @param {Object} args
 * @param {string} args.title
 * @param {string} args.legend
 * @param {EmaRatioPattern} args.ratio
 * @param {Color} [args.color]
 * @returns {PartialOptionsTree}
 */
export function createPriceWithRatioOptions(
  ctx,
  { title, legend, ratio, color },
) {
  const { colors } = ctx;
  const priceMetric = ratio.price;

  const pctUsdMap = percentileUsdMap(colors, ratio);
  const pctMap = percentileMap(colors, ratio);
  const sdPats = sdPatterns(ratio);

  return [
    {
      name: "price",
      title,
      top: [line({ metric: priceMetric, name: legend, color, unit: Unit.usd })],
    },
    {
      name: "Ratio",
      title: `${title} Ratio`,
      top: [
        line({ metric: priceMetric, name: legend, color, unit: Unit.usd }),
        ...pctUsdMap.map(({ name: pctName, prop, color: pctColor }) =>
          line({
            metric: prop,
            name: pctName,
            color: pctColor,
            defaultActive: false,
            unit: Unit.usd,
            options: { lineStyle: 1 },
          }),
        ),
      ],
      bottom: [
        baseline({
          metric: ratio.ratio,
          name: "Ratio",
          base: 1,
          color,
          unit: Unit.ratio,
        }),
        .../** @type {const} */ ([
          {
            metric: ratio.ratio1wSma,
            name: "1w SMA",
            color: colors.lime,
          },
          {
            metric: ratio.ratio1mSma,
            name: "1m SMA",
            color: colors.teal,
          },
          {
            metric: ratio.ratio1ySd.sma,
            name: "1y SMA",
            color: colors.sky,
          },
          {
            metric: ratio.ratio2ySd.sma,
            name: "2y SMA",
            color: colors.indigo,
          },
          {
            metric: ratio.ratio4ySd.sma,
            name: "4y SMA",
            color: colors.purple,
          },
          {
            metric: ratio.ratioSd.sma,
            name: "All SMA",
            color: colors.rose,
          },
        ]).map(({ metric, name, color }) =>
          line({
            metric,
            name,
            color,
            unit: Unit.ratio,
            defaultActive: false,
            style: 1,
          }),
        ),
        ...pctMap.map(({ name: pctName, prop, color: pctColor }) =>
          line({
            metric: prop,
            name: pctName,
            color: pctColor,
            defaultActive: false,
            unit: Unit.ratio,
            style: 1,
          }),
        ),
        priceLine({ ctx, unit: Unit.ratio, number: 1 }),
      ],
    },
    {
      name: "ZScores",
      tree: [
        {
          name: "Compare",
          title: `${title} Z-Scores`,
          top: [
            line({ metric: priceMetric, name: legend, color, unit: Unit.usd }),
            line({
              metric: ratio.ratio1ySd._0sdUsd,
              name: "1y 0σ",
              color: colors.orange,
              defaultActive: false,
              unit: Unit.usd,
            }),
            line({
              metric: ratio.ratio2ySd._0sdUsd,
              name: "2y 0σ",
              color: colors.yellow,
              defaultActive: false,
              unit: Unit.usd,
            }),
            line({
              metric: ratio.ratio4ySd._0sdUsd,
              name: "4y 0σ",
              color: colors.lime,
              defaultActive: false,
              unit: Unit.usd,
            }),
            line({
              metric: ratio.ratioSd._0sdUsd,
              name: "all 0σ",
              color: colors.blue,
              defaultActive: false,
              unit: Unit.usd,
            }),
          ],
          bottom: [
            line({
              metric: ratio.ratioSd.zscore,
              name: "all",
              color: colors.blue,
              unit: Unit.sd,
            }),
            line({
              metric: ratio.ratio4ySd.zscore,
              name: "4y",
              color: colors.lime,
              unit: Unit.sd,
            }),
            line({
              metric: ratio.ratio2ySd.zscore,
              name: "2y",
              color: colors.yellow,
              unit: Unit.sd,
            }),
            line({
              metric: ratio.ratio1ySd.zscore,
              name: "1y",
              color: colors.orange,
              unit: Unit.sd,
            }),
            ...priceLines({
              ctx,
              unit: Unit.sd,
              numbers: [0, 1, -1, 2, -2, 3, -3],
              defaultActive: false,
            }),
          ],
        },
        ...sdPats.map(({ nameAddon, titleAddon, sd }) => ({
          name: nameAddon,
          title: `${title} ${titleAddon} Z-Score`,
          top: [
            line({ metric: priceMetric, name: legend, color, unit: Unit.usd }),
            ...sdBands(colors, sd).map(
              ({ name: bandName, prop, color: bandColor }) =>
                line({
                  metric: prop,
                  name: bandName,
                  color: bandColor,
                  unit: Unit.usd,
                  defaultActive: false,
                }),
            ),
          ],
          bottom: [
            baseline({
              metric: sd.zscore,
              name: "Z-Score",
              color,
              unit: Unit.sd,
            }),
            priceLine({
              ctx,
              unit: Unit.sd,
            }),
            ...priceLines({
              ctx,
              unit: Unit.sd,
              numbers: [1, -1, 2, -2, 3, -3],
              defaultActive: false,
            }),
          ],
        })),
      ],
    },
  ];
}

/**
 * Create Averages section
 * @param {PartialContext} ctx
 * @param {ReturnType<typeof buildAverages>} averages
 */
export function createAveragesSection(ctx, averages) {
  return {
    name: "Averages",
    tree: [
      { nameAddon: "Simple", metricAddon: /** @type {const} */ ("sma") },
      { nameAddon: "Exponential", metricAddon: /** @type {const} */ ("ema") },
    ].map(({ nameAddon, metricAddon }) => ({
      name: nameAddon,
      tree: [
        {
          name: "Compare",
          title: `Price ${metricAddon.toUpperCase()}s`,
          top: averages.map(({ id, color, sma, ema }) =>
            line({
              metric: (metricAddon === "sma" ? sma : ema).price,
              name: id,
              color,
              unit: Unit.usd,
            }),
          ),
        },
        ...averages.map(({ name, color, sma, ema }) => ({
          name,
          tree: createPriceWithRatioOptions(ctx, {
            ratio: metricAddon === "sma" ? sma : ema,
            title: `${name} ${metricAddon.toUpperCase()}`,
            legend: "average",
            color,
          }),
        })),
      ],
    })),
  };
}
