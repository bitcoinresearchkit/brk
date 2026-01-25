import { Unit } from "../utils/units.js";
import { line } from "./series.js";
import {
  satsBtcUsd,
  createRatioChart,
  createZScoresFolder,
} from "./shared.js";

/**
 * Create price with ratio options for cointime prices
 * @param {PartialContext} ctx
 * @param {Object} args
 * @param {string} args.title
 * @param {string} args.legend
 * @param {AnyMetricPattern} args.price
 * @param {ActivePriceRatioPattern} args.ratio
 * @param {Color} args.color
 * @returns {PartialOptionsTree}
 */
function createCointimePriceWithRatioOptions(
  ctx,
  { title, legend, price, ratio, color },
) {
  return [
    {
      name: "price",
      title,
      top: [line({ metric: price, name: legend, color, unit: Unit.usd })],
    },
    createRatioChart(ctx, { title, price, ratio, color }),
    createZScoresFolder(ctx, { title, legend, price, ratio, color }),
  ];
}

/**
 * Create Cointime section
 * @param {PartialContext} ctx
 * @returns {PartialOptionsGroup}
 */
export function createCointimeSection(ctx) {
  const { colors, brk } = ctx;
  const { cointime, distribution, supply } = brk.metrics;
  const { pricing, cap, activity, supply: cointimeSupply, adjusted, reserveRisk, value } = cointime;
  const { all } = distribution.utxoCohorts;

  // Cointime prices data
  const cointimePrices = [
    {
      price: pricing.trueMarketMean,
      ratio: pricing.trueMarketMeanRatio,
      name: "True market mean",
      title: "True Market Mean",
      color: colors.blue,
    },
    {
      price: pricing.vaultedPrice,
      ratio: pricing.vaultedPriceRatio,
      name: "Vaulted",
      title: "Vaulted Price",
      color: colors.lime,
    },
    {
      price: pricing.activePrice,
      ratio: pricing.activePriceRatio,
      name: "Active",
      title: "Active Price",
      color: colors.rose,
    },
    {
      price: pricing.cointimePrice,
      ratio: pricing.cointimePriceRatio,
      name: "cointime",
      title: "Cointime Price",
      color: colors.yellow,
    },
  ];

  // Cointime capitalizations data
  const cointimeCapitalizations = [
    {
      metric: cap.vaultedCap,
      name: "vaulted",
      title: "Vaulted Cap",
      color: colors.lime,
    },
    {
      metric: cap.activeCap,
      name: "active",
      title: "Active Cap",
      color: colors.rose,
    },
    {
      metric: cap.cointimeCap,
      name: "cointime",
      title: "Cointime Cap",
      color: colors.yellow,
    },
    {
      metric: cap.investorCap,
      name: "investor",
      title: "Investor Cap",
      color: colors.fuchsia,
    },
    {
      metric: cap.thermoCap,
      name: "thermo",
      title: "Thermo Cap",
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
            title: "Cointime Prices",
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
            title: "Cointime Caps",
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
          ...satsBtcUsd(all.supply.total, "All", colors.orange),
          ...satsBtcUsd(cointimeSupply.vaultedSupply, "Vaulted", colors.lime),
          ...satsBtcUsd(cointimeSupply.activeSupply, "Active", colors.rose),
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

      // Reserve Risk
      {
        name: "Reserve Risk",
        tree: [
          {
            name: "reserve risk",
            title: "Reserve Risk",
            bottom: [
              line({
                metric: reserveRisk.reserveRisk,
                name: "Reserve Risk",
                color: colors.orange,
                unit: Unit.ratio,
              }),
            ],
          },
          {
            name: "hodl bank",
            title: "HODL Bank",
            bottom: [
              line({
                metric: reserveRisk.hodlBank,
                name: "HODL Bank",
                color: colors.blue,
                unit: Unit.ratio,
              }),
            ],
          },
          {
            name: "vocdd 365d sma",
            title: "VOCDD 365d SMA",
            bottom: [
              line({
                metric: reserveRisk.vocdd365dSma,
                name: "VOCDD 365d SMA",
                color: colors.purple,
                unit: Unit.ratio,
              }),
            ],
          },
        ],
      },

      // Cointime Value
      {
        name: "Value",
        tree: [
          {
            name: "created",
            title: "Cointime Value Created",
            bottom: [
              line({
                metric: value.cointimeValueCreated.sum,
                name: "Created",
                color: colors.green,
                unit: Unit.usd,
              }),
              line({
                metric: value.cointimeValueCreated.cumulative,
                name: "Cumulative",
                color: colors.green,
                unit: Unit.usd,
                defaultActive: false,
              }),
            ],
          },
          {
            name: "destroyed",
            title: "Cointime Value Destroyed",
            bottom: [
              line({
                metric: value.cointimeValueDestroyed.sum,
                name: "Destroyed",
                color: colors.red,
                unit: Unit.usd,
              }),
              line({
                metric: value.cointimeValueDestroyed.cumulative,
                name: "Cumulative",
                color: colors.red,
                unit: Unit.usd,
                defaultActive: false,
              }),
            ],
          },
          {
            name: "stored",
            title: "Cointime Value Stored",
            bottom: [
              line({
                metric: value.cointimeValueStored.sum,
                name: "Stored",
                color: colors.blue,
                unit: Unit.usd,
              }),
              line({
                metric: value.cointimeValueStored.cumulative,
                name: "Cumulative",
                color: colors.blue,
                unit: Unit.usd,
                defaultActive: false,
              }),
            ],
          },
          {
            name: "vocdd",
            title: "VOCDD (Value of Coin Days Destroyed)",
            bottom: [
              line({
                metric: value.vocdd.sum,
                name: "VOCDD",
                color: colors.orange,
                unit: Unit.usd,
              }),
              line({
                metric: value.vocdd.cumulative,
                name: "Cumulative",
                color: colors.orange,
                unit: Unit.usd,
                defaultActive: false,
              }),
            ],
          },
        ],
      },

      // Adjusted metrics
      {
        name: "Adjusted",
        tree: [
          // Inflation
          {
            name: "Inflation",
            title: "Adjusted Inflation",
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
            title: "Adjusted Velocity",
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
