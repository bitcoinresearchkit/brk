import { colors } from "../utils/colors.js";
import { brk } from "../client.js";
import { Unit } from "../utils/units.js";
import {
  dots,
  line,
  price,
  sumsTree,
  multiSeriesTree,
  percentRatioDots,
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

  const prices = /** @type {const} */ ([
    {
      pattern: cointimePrices.trueMarketMean,
      name: "True Market Mean",
      color: colors.trueMarketMean,
      defaultActive: true,
    },
    {
      pattern: cointimePrices.vaulted,
      name: "Vaulted",
      color: colors.vaulted,
      defaultActive: true,
    },
    {
      pattern: cointimePrices.active,
      name: "Active",
      color: colors.active,
      defaultActive: true,
    },
    {
      pattern: cointimePrices.cointime,
      name: "Cointime",
      color: colors.cointime,
      defaultActive: true,
    },
  ]);

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
      // Prices - the core pricing models
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
                series: all.realized.investor.price,
                name: "Investor",
                color: colors.investor,
              }),
              ...prices.map(({ pattern, name, color, defaultActive }) =>
                price({ series: pattern, name, color, defaultActive }),
              ),
            ],
          },
          ...prices.map(({ pattern, name, color }) => ({
            name,
            tree: priceRatioPercentilesTree({
              pattern,
              title: `${name} Price`,
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

      // Caps - market capitalizations from different models
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

      // Supply - active vs vaulted breakdown
      {
        name: "Supply",
        title: "Active vs Vaulted Supply",
        bottom: supplyBreakdown.flatMap(({ pattern, name, color }) =>
          satsBtcUsd({ pattern, name, color }),
        ),
      },

      // Liveliness - the foundational cointime ratios
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
            name: "L/V Ratio",
            color: colors.activity,
            unit: Unit.ratio,
            defaultActive: false,
          }),
        ],
      },

      // Coinblocks - created, destroyed, stored
      {
        name: "Coinblocks",
        tree: [
          {
            name: "Compare",
            tree: multiSeriesTree({
              entries: coinblocks.map(({ pattern, name, color }) => ({
                name,
                color,
                base: pattern.base,
                rolling: pattern.sum,
                cumulative: pattern.cumulative,
              })),
              title: "Coinblocks",
              unit: Unit.coinblocks,
            }),
          },
          ...coinblocks.map(({ pattern, name, title, color }) => ({
            name,
            tree: [
              {
                name: "Base",
                title,
                bottom: [
                  line({
                    series: pattern.base,
                    name,
                    color,
                    unit: Unit.coinblocks,
                  }),
                ],
              },
              sumsTree({ windows: pattern.sum, title, unit: Unit.coinblocks }),
              {
                name: "Cumulative",
                title: `${title} (Total)`,
                bottom: [
                  line({
                    series: pattern.cumulative,
                    name,
                    color,
                    unit: Unit.coinblocks,
                  }),
                ],
              },
            ],
          })),
        ],
      },

      // Value - cointime value flows
      {
        name: "Value",
        tree: [
          {
            name: "Compare",
            tree: multiSeriesTree({
              entries: [
                ...cointimeValues.map(({ pattern, name, color }) => ({
                  name,
                  color,
                  base: pattern.base,
                  rolling: pattern.sum,
                  cumulative: pattern.cumulative,
                })),
                {
                  name: vocdd.name,
                  color: vocdd.color,
                  base: vocdd.pattern.base,
                  rolling: vocdd.pattern.sum,
                  cumulative: vocdd.pattern.cumulative,
                },
              ],
              title: "Cointime Value",
              unit: Unit.usd,
            }),
          },
          ...cointimeValues.map(({ pattern, name, title, color }) => ({
            name,
            tree: [
              {
                name: "Base",
                title,
                bottom: [
                  line({ series: pattern.base, name, color, unit: Unit.usd }),
                ],
              },
              sumsTree({ windows: pattern.sum, title, unit: Unit.usd }),
              {
                name: "Cumulative",
                title: `${title} (Total)`,
                bottom: [
                  line({
                    series: pattern.cumulative,
                    name,
                    color,
                    unit: Unit.usd,
                  }),
                ],
              },
            ],
          })),
          {
            name: vocdd.name,
            tree: [
              {
                name: "Base",
                title: vocdd.title,
                bottom: [
                  line({
                    series: vocdd.pattern.base,
                    name: vocdd.name,
                    color: vocdd.color,
                    unit: Unit.usd,
                  }),
                  line({
                    series: reserveRisk.vocddMedian1y,
                    name: "365d Median",
                    color: colors.time._1y,
                    unit: Unit.usd,
                  }),
                ],
              },
              sumsTree({
                windows: vocdd.pattern.sum,
                title: vocdd.title,
                unit: Unit.usd,
              }),
              {
                name: "Cumulative",
                title: `${vocdd.title} (Total)`,
                bottom: [
                  line({
                    series: vocdd.pattern.cumulative,
                    name: vocdd.name,
                    color: vocdd.color,
                    unit: Unit.usd,
                  }),
                ],
              },
            ],
          },
        ],
      },

      // Indicators - derived decision series
      {
        name: "Indicators",
        tree: [
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
          {
            name: "AVIV",
            title: "AVIV Ratio",
            bottom: [
              line({
                series: cap.aviv.ratio,
                name: "aviv",
                unit: Unit.ratio,
              }),
            ],
          },
        ],
      },

      // Cointime-Adjusted - comparing base vs adjusted series
      {
        name: "Cointime-Adjusted",
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
            name: "Velocity",
            tree: [
              {
                name: "BTC",
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
                name: "USD",
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
      },
    ],
  };
}
