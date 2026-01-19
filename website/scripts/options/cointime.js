/** Cointime section builder - typed tree-based patterns */

import { Unit } from "../utils/units.js";
import {
  satsBtcUsd,
  priceLines,
  percentileUsdMap,
  percentileMap,
  sdPatterns,
  sdBands,
} from "./shared.js";

/**
 * Create price with ratio options for cointime prices
 * @param {PartialContext} ctx
 * @param {Object} args
 * @param {string} args.title
 * @param {string} args.legend
 * @param {AnyMetricPattern} args.price
 * @param {ActivePriceRatioPattern} args.ratio
 * @param {Color} [args.color]
 * @returns {PartialOptionsTree}
 */
function createCointimePriceWithRatioOptions(
  ctx,
  { title, legend, price, ratio, color },
) {
  const { line, colors, createPriceLine } = ctx;

  const pctUsdMap = percentileUsdMap(colors, ratio);
  const pctMap = percentileMap(colors, ratio);
  const sdPats = sdPatterns(ratio);

  return [
    {
      name: "price",
      title,
      top: [line({ metric: price, name: legend, color, unit: Unit.usd })],
    },
    {
      name: "Ratio",
      title: `${title} Ratio`,
      top: [
        line({ metric: price, name: legend, color, unit: Unit.usd }),
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
        line({ metric: ratio.ratio, name: "Ratio", color, unit: Unit.ratio }),
        line({
          metric: ratio.ratio1wSma,
          name: "1w SMA",
          color: colors.lime,
          unit: Unit.ratio,
        }),
        line({
          metric: ratio.ratio1mSma,
          name: "1m SMA",
          color: colors.teal,
          unit: Unit.ratio,
        }),
        line({
          metric: ratio.ratio1ySd.sma,
          name: "1y SMA",
          color: colors.sky,
          unit: Unit.ratio,
        }),
        line({
          metric: ratio.ratio2ySd.sma,
          name: "2y SMA",
          color: colors.indigo,
          unit: Unit.ratio,
        }),
        line({
          metric: ratio.ratio4ySd.sma,
          name: "4y SMA",
          color: colors.purple,
          unit: Unit.ratio,
        }),
        line({
          metric: ratio.ratioSd.sma,
          name: "All SMA",
          color: colors.rose,
          unit: Unit.ratio,
        }),
        ...pctMap.map(({ name: pctName, prop, color: pctColor }) =>
          line({
            metric: prop,
            name: pctName,
            color: pctColor,
            defaultActive: false,
            unit: Unit.ratio,
            options: { lineStyle: 1 },
          }),
        ),
        createPriceLine({ unit: Unit.ratio, number: 1 }),
      ],
    },
    {
      name: "ZScores",
      tree: [
        // Compare all Z-Scores
        {
          name: "Compare",
          title: `Compare ${title} Z-Scores`,
          top: [
            line({ metric: price, name: legend, color, unit: Unit.usd }),
            line({
              metric: ratio.ratio1ySd._0sdUsd,
              name: "1y 0sd",
              color: colors.fuchsia,
              defaultActive: false,
              unit: Unit.usd,
            }),
            line({
              metric: ratio.ratio2ySd._0sdUsd,
              name: "2y 0sd",
              color: colors.purple,
              defaultActive: false,
              unit: Unit.usd,
            }),
            line({
              metric: ratio.ratio4ySd._0sdUsd,
              name: "4y 0sd",
              color: colors.violet,
              defaultActive: false,
              unit: Unit.usd,
            }),
            line({
              metric: ratio.ratioSd._0sdUsd,
              name: "0sd",
              color: colors.indigo,
              defaultActive: false,
              unit: Unit.usd,
            }),
          ],
          bottom: [
            line({
              metric: ratio.ratioSd.zscore,
              name: "All",
              color: colors.default,
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
              color: colors.avocado,
              unit: Unit.sd,
            }),
            line({
              metric: ratio.ratio1ySd.zscore,
              name: "1y",
              color: colors.yellow,
              unit: Unit.sd,
            }),
            ...priceLines(ctx, Unit.sd, [0, 1, -1, 2, -2, 3, -3, 4, -4]),
          ],
        },
        // Individual Z-Score charts
        ...sdPats.map(({ nameAddon, titleAddon, sd }) => ({
          name: nameAddon,
          title: `${title} ${titleAddon} Z-Score`,
          top: sdBands(colors, sd).map(({ name: bandName, prop, color: bandColor }) =>
            line({
              metric: prop,
              name: bandName,
              color: bandColor,
              unit: Unit.usd,
            }),
          ),
          bottom: [
            line({ metric: sd.zscore, name: "Z-Score", color, unit: Unit.sd }),
            ...priceLines(ctx, Unit.sd, [0, 1, -1, 2, -2, 3, -3]),
          ],
        })),
      ],
    },
  ];
}

/**
 * Create Cointime section
 * @param {PartialContext} ctx
 * @returns {PartialOptionsGroup}
 */
export function createCointimeSection(ctx) {
  const { colors, brk, line } = ctx;
  const { cointime, distribution, supply } = brk.metrics;
  const { pricing, cap, activity, supply: cointimeSupply, adjusted } = cointime;
  const { all } = distribution.utxoCohorts;

  // Cointime prices data
  const cointimePrices = [
    {
      price: pricing.trueMarketMean,
      ratio: pricing.trueMarketMeanRatio,
      name: "True market mean",
      title: "true market mean",
      color: colors.blue,
    },
    {
      price: pricing.vaultedPrice,
      ratio: pricing.vaultedPriceRatio,
      name: "Vaulted",
      title: "vaulted price",
      color: colors.lime,
    },
    {
      price: pricing.activePrice,
      ratio: pricing.activePriceRatio,
      name: "Active",
      title: "active price",
      color: colors.rose,
    },
    {
      price: pricing.cointimePrice,
      ratio: pricing.cointimePriceRatio,
      name: "cointime",
      title: "cointime price",
      color: colors.yellow,
    },
  ];

  // Cointime capitalizations data
  const cointimeCapitalizations = [
    {
      metric: cap.vaultedCap,
      name: "vaulted",
      title: "vaulted Capitalization",
      color: colors.lime,
    },
    {
      metric: cap.activeCap,
      name: "active",
      title: "active Capitalization",
      color: colors.rose,
    },
    {
      metric: cap.cointimeCap,
      name: "cointime",
      title: "cointime Capitalization",
      color: colors.yellow,
    },
    {
      metric: cap.investorCap,
      name: "investor",
      title: "investor Capitalization",
      color: colors.fuchsia,
    },
    {
      metric: cap.thermoCap,
      name: "thermo",
      title: "thermo Capitalization",
      color: colors.emerald,
    },
  ];

  return {
    name: "Cointime",
    tree: [
      // Prices
      {
        name: "Prices",
        tree: [
          {
            name: "Compare",
            title: "Compare Cointime Prices",
            top: cointimePrices.map(({ price, name, color }) =>
              line({ metric: price, name, color, unit: Unit.usd }),
            ),
          },
          ...cointimePrices.map(({ price, ratio, name, color, title }) => ({
            name,
            tree: createCointimePriceWithRatioOptions(ctx, {
              price,
              ratio,
              legend: name,
              color,
              title,
            }),
          })),
        ],
      },

      // Capitalization
      {
        name: "Capitalization",
        tree: [
          {
            name: "Compare",
            title: "Compare Cointime Capitalizations",
            bottom: [
              line({
                metric: supply.marketCap,
                name: "Market",
                color: colors.default,
                unit: Unit.usd,
              }),
              line({
                metric: all.realized.realizedCap,
                name: "Realized",
                color: colors.orange,
                unit: Unit.usd,
              }),
              ...cointimeCapitalizations.map(({ metric, name, color }) =>
                line({ metric, name, color, unit: Unit.usd }),
              ),
            ],
          },
          ...cointimeCapitalizations.map(({ metric, name, color, title }) => ({
            name,
            title,
            bottom: [
              line({ metric, name, color, unit: Unit.usd }),
              line({
                metric: supply.marketCap,
                name: "Market",
                color: colors.default,
                unit: Unit.usd,
              }),
              line({
                metric: all.realized.realizedCap,
                name: "Realized",
                color: colors.orange,
                unit: Unit.usd,
              }),
            ],
          })),
        ],
      },

      // Supply
      {
        name: "Supply",
        title: "Cointime Supply",
        bottom: [
          ...satsBtcUsd(ctx, all.supply.total, "All", colors.orange),
          ...satsBtcUsd(ctx, cointimeSupply.vaultedSupply, "Vaulted", colors.lime),
          ...satsBtcUsd(ctx, cointimeSupply.activeSupply, "Active", colors.rose),
        ],
      },

      // Liveliness & Vaultedness
      {
        name: "Liveliness & Vaultedness",
        title: "Liveliness & Vaultedness",
        bottom: [
          line({
            metric: activity.liveliness,
            name: "Liveliness",
            color: colors.rose,
            unit: Unit.ratio,
          }),
          line({
            metric: activity.vaultedness,
            name: "Vaultedness",
            color: colors.lime,
            unit: Unit.ratio,
          }),
          line({
            metric: activity.activityToVaultednessRatio,
            name: "Liveliness / Vaultedness",
            color: colors.purple,
            unit: Unit.ratio,
          }),
        ],
      },

      // Coinblocks
      {
        name: "Coinblocks",
        title: "Coinblocks",
        bottom: [
          // Destroyed comes from the all cohort's activity
          line({
            metric: all.activity.coinblocksDestroyed.sum,
            name: "Destroyed",
            color: colors.red,
            unit: Unit.coinblocks,
          }),
          line({
            metric: all.activity.coinblocksDestroyed.cumulative,
            name: "Cumulative Destroyed",
            color: colors.red,
            defaultActive: false,
            unit: Unit.coinblocks,
          }),
          // Created and stored from cointime
          line({
            metric: activity.coinblocksCreated.sum,
            name: "Created",
            color: colors.orange,
            unit: Unit.coinblocks,
          }),
          line({
            metric: activity.coinblocksCreated.cumulative,
            name: "Cumulative Created",
            color: colors.orange,
            defaultActive: false,
            unit: Unit.coinblocks,
          }),
          line({
            metric: activity.coinblocksStored.sum,
            name: "Stored",
            color: colors.green,
            unit: Unit.coinblocks,
          }),
          line({
            metric: activity.coinblocksStored.cumulative,
            name: "Cumulative Stored",
            color: colors.green,
            defaultActive: false,
            unit: Unit.coinblocks,
          }),
        ],
      },

      // Adjusted metrics
      {
        name: "Adjusted",
        tree: [
          // Inflation
          {
            name: "Inflation",
            title: "Cointime-Adjusted Inflation Rate",
            bottom: [
              line({
                metric: supply.inflation,
                name: "Base",
                color: colors.orange,
                unit: Unit.percentage,
              }),
              line({
                metric: adjusted.cointimeAdjInflationRate,
                name: "Adjusted",
                color: colors.purple,
                unit: Unit.percentage,
              }),
            ],
          },
          // Velocity
          {
            name: "Velocity",
            title: "Cointime-Adjusted Transactions Velocity",
            bottom: [
              line({
                metric: supply.velocity.btc,
                name: "BTC",
                color: colors.orange,
                unit: Unit.ratio,
              }),
              line({
                metric: adjusted.cointimeAdjTxBtcVelocity,
                name: "Adj. BTC",
                color: colors.red,
                unit: Unit.ratio,
              }),
              line({
                metric: supply.velocity.usd,
                name: "USD",
                color: colors.emerald,
                unit: Unit.ratio,
              }),
              line({
                metric: adjusted.cointimeAdjTxUsdVelocity,
                name: "Adj. USD",
                color: colors.lime,
                unit: Unit.ratio,
              }),
            ],
          },
        ],
      },
    ],
  };
}
