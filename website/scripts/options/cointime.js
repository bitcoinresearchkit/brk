import { colors } from "../utils/colors.js";
import { brk } from "../utils/client.js";
import { Unit } from "../utils/units.js";
import {
  dots,
  line,
  price,
  multiSeriesTree,
  percentRatioDots,
  sumsAndAveragesCumulative,
} from "./series.js";
import { satsBtcUsd, priceRatioPercentilesTree } from "./shared.js";

/**
 * Create Cointime section
 * @returns {PartialOptionsGroup}
 */
export function createCointimeSection() {
  const { cointime, cohorts, supply } = brk.series;
  const {
    prices: cointimePrices,
    cap,
    activity,
    supply: cointimeSupply,
    adjusted,
    reserveRisk,
    value,
  } = cointime;
  const { all } = cohorts.utxo;

  // Reference lines for cap comparisons
  const capReferenceLines = /** @type {const} */ ([
    { series: supply.marketCap.usd, name: "Market", color: colors.default },
    {
      series: all.realized.cap.usd,
      name: "Realized",
      color: colors.realized,
    },
  ]);

  /** @type {readonly { pattern: PriceRatioPercentilesPattern, name: string, title: (name: string) => string, color: Color, defaultActive: boolean }[]} */
  const prices = [
    {
      pattern: cointimePrices.trueMarketMean,
      name: "True Market Mean",
      title: (name) => name,
      color: colors.trueMarketMean,
      defaultActive: true,
    },
    {
      pattern: cointimePrices.vaulted,
      name: "Vaulted",
      title: (name) => `${name} Price`,
      color: colors.vaulted,
      defaultActive: true,
    },
    {
      pattern: cointimePrices.active,
      name: "Active",
      title: (name) => `${name} Price`,
      color: colors.active,
      defaultActive: true,
    },
    {
      pattern: cointimePrices.cointime,
      name: "Cointime",
      title: (name) => `${name} Price`,
      color: colors.cointime,
      defaultActive: true,
    },
  ];

  const caps = /** @type {const} */ ([
    {
      series: cap.vaulted.usd,
      name: "Vaulted",
      color: colors.vaulted,
      defaultActive: true,
    },
    {
      series: cap.active.usd,
      name: "Active",
      color: colors.active,
      defaultActive: true,
    },
    {
      series: cap.cointime.usd,
      name: "Cointime",
      color: colors.cointime,
      defaultActive: true,
    },
    {
      series: cap.investor.usd,
      name: "Investor",
      color: colors.investor,
      defaultActive: false,
    },
    {
      series: cap.thermo.usd,
      name: "Thermo",
      color: colors.thermo,
      defaultActive: false,
    },
  ]);

  const supplyBreakdown = /** @type {const} */ ([
    { pattern: all.supply.total, name: "Total", color: colors.bitcoin },
    {
      pattern: cointimeSupply.vaulted,
      name: "Vaulted",
      color: colors.vaulted,
    },
    {
      pattern: cointimeSupply.active,
      name: "Active",
      color: colors.active,
    },
  ]);

  const coinblocks = /** @type {const} */ ([
    {
      pattern: activity.coinblocksDestroyed,
      name: "Destroyed",
      title: "Coinblocks Destroyed",
      color: colors.destroyed,
    },
    {
      pattern: activity.coinblocksCreated,
      name: "Created",
      title: "Coinblocks Created",
      color: colors.created,
    },
    {
      pattern: activity.coinblocksStored,
      name: "Stored",
      title: "Coinblocks Stored",
      color: colors.stored,
    },
  ]);

  // Colors aligned with coinblocks: Destroyed=red, Created=orange, Stored=green
  const cointimeValues = /** @type {const} */ ([
    {
      pattern: value.created,
      name: "Created",
      title: "Cointime Value Created",
      color: colors.created,
    },
    {
      pattern: value.destroyed,
      name: "Destroyed",
      title: "Cointime Value Destroyed",
      color: colors.destroyed,
    },
    {
      pattern: value.stored,
      name: "Stored",
      title: "Cointime Value Stored",
      color: colors.stored,
    },
  ]);

  const vocdd = /** @type {const} */ ({
    pattern: value.vocdd,
    name: "VOCDD",
    title: "Value of Coin Days Destroyed",
    color: colors.vocdd,
  });

  return {
    name: "Cointime",
    tree: [
      {
        name: "Prices",
        tree: [
          {
            name: "Compare",
            title: "Cointime Prices",
            top: [
              price({
                series: all.realized.price,
                name: "Realized",
                color: colors.realized,
              }),
              price({
                series: all.realized.capitalized.price,
                name: "Capitalized",
                color: colors.capitalized,
              }),
              ...prices.map(({ pattern, name, color, defaultActive }) =>
                price({ series: pattern, name, color, defaultActive }),
              ),
            ],
          },
          ...prices.map(({ pattern, name, title, color }) => ({
            name,
            tree: priceRatioPercentilesTree({
              pattern,
              title: title(name),
              legend: name,
              color,
              priceReferences: [
                price({
                  series: all.realized.price,
                  name: "Realized",
                  color: colors.realized,
                  defaultActive: false,
                }),
              ],
            }),
          })),
        ],
      },

      {
        name: "Caps",
        tree: [
          {
            name: "Compare",
            title: "Cointime Caps",
            bottom: [
              ...capReferenceLines.map(({ series, name, color }) =>
                line({ series, name, color, unit: Unit.usd }),
              ),
              ...caps.map(({ series, name, color, defaultActive }) =>
                line({ series, name, color, defaultActive, unit: Unit.usd }),
              ),
            ],
          },
          ...caps.map(({ series, name, color }) => ({
            name,
            title: `${name} Cap`,
            bottom: [
              line({ series, name, color, unit: Unit.usd }),
              ...capReferenceLines.map((ref) =>
                line({
                  series: ref.series,
                  name: ref.name,
                  color: ref.color,
                  unit: Unit.usd,
                }),
              ),
            ],
          })),
        ],
      },

      {
        name: "Supply",
        title: "Active vs Vaulted Supply",
        bottom: supplyBreakdown.flatMap(({ pattern, name, color }) =>
          satsBtcUsd({ pattern, name, color }),
        ),
      },

      {
        name: "Activity",
        title: "Liveliness & Vaultedness",
        bottom: [
          line({
            series: activity.liveliness,
            name: "Liveliness",
            color: colors.liveliness,
            unit: Unit.ratio,
          }),
          line({
            series: activity.vaultedness,
            name: "Vaultedness",
            color: colors.vaulted,
            unit: Unit.ratio,
          }),
          line({
            series: activity.ratio,
            name: "Liveliness / Vaultedness",
            color: colors.activity,
            unit: Unit.ratio,
            defaultActive: false,
          }),
        ],
      },

      {
        name: "Coinblocks",
        tree: [
          ...multiSeriesTree({
            entries: coinblocks.map(({ pattern, name, color }) => ({
              name,
              color,
              average: pattern.average,
              sum: pattern.sum,
              cumulative: pattern.cumulative,
            })),
            metric: "Coinblocks",
            unit: Unit.coinblocks,
          }),
          ...coinblocks.map(({ pattern, name, title: metric, color }) => ({
            name,
            tree: sumsAndAveragesCumulative({
              sum: pattern.sum,
              average: pattern.average,
              cumulative: pattern.cumulative,
              metric,
              unit: Unit.coinblocks,
              color,
            }),
          })),
        ],
      },

      {
        name: "Value",
        tree: [
          ...multiSeriesTree({
            entries: [
              ...cointimeValues.map(({ pattern, name, color }) => ({
                name,
                color,
                average: pattern.average,
                sum: pattern.sum,
                cumulative: pattern.cumulative,
              })),
              {
                name: vocdd.name,
                color: vocdd.color,
                average: vocdd.pattern.average,
                sum: vocdd.pattern.sum,
                cumulative: vocdd.pattern.cumulative,
              },
            ],
            metric: "Cointime Value",
            unit: Unit.usd,
          }),
          ...cointimeValues.map(({ pattern, name, title: metric, color }) => ({
            name,
            tree: sumsAndAveragesCumulative({
              sum: pattern.sum,
              average: pattern.average,
              cumulative: pattern.cumulative,
              metric,
              unit: Unit.usd,
              color,
            }),
          })),
          {
            name: vocdd.name,
            tree: sumsAndAveragesCumulative({
              sum: vocdd.pattern.sum,
              average: vocdd.pattern.average,
              cumulative: vocdd.pattern.cumulative,
              metric: vocdd.title,
              unit: Unit.usd,
              color: vocdd.color,
            }),
          },
        ],
      },

      {
        name: "Indicators",
        tree: [
          {
            name: "AVIV",
            title: "AVIV Ratio",
            bottom: [
              line({
                series: cap.aviv.ratio,
                name: "AVIV",
                unit: Unit.ratio,
              }),
            ],
          },
          {
            name: "Reserve Risk",
            title: "Reserve Risk",
            bottom: [
              line({
                series: reserveRisk.value,
                name: "Ratio",
                color: colors.reserveRisk,
                unit: Unit.ratio,
              }),
            ],
          },
        ],
      },

      {
        name: "Adjusted",
        tree: [
          {
            name: "Inflation",
            title: "Cointime-Adjusted Inflation",
            bottom: [
              dots({
                series: supply.inflationRate.percent,
                name: "Base",
                color: colors.base,
                unit: Unit.percentage,
              }),
              ...percentRatioDots({
                pattern: adjusted.inflationRate,
                name: "Cointime-Adjusted",
                color: colors.adjusted,
              }),
            ],
          },
          {
            name: "BTC Velocity",
            title: "Cointime-Adjusted BTC Velocity",
            bottom: [
              line({
                series: supply.velocity.native,
                name: "Base",
                color: colors.base,
                unit: Unit.ratio,
              }),
              line({
                series: adjusted.txVelocityNative,
                name: "Cointime-Adjusted",
                color: colors.adjusted,
                unit: Unit.ratio,
              }),
            ],
          },
          {
            name: "USD Velocity",
            title: "Cointime-Adjusted USD Velocity",
            bottom: [
              line({
                series: supply.velocity.fiat,
                name: "Base",
                color: colors.thermo,
                unit: Unit.ratio,
              }),
              line({
                series: adjusted.txVelocityFiat,
                name: "Cointime-Adjusted",
                color: colors.vaulted,
                unit: Unit.ratio,
              }),
            ],
          },
        ],
      },
    ],
  };
}
