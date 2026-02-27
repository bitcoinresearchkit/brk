import { colors } from "../utils/colors.js";
import { brk } from "../client.js";
import { Unit } from "../utils/units.js";
import { dots, line, price, rollingWindowsTree } from "./series.js";
import { satsBtcUsd, createPriceRatioCharts } from "./shared.js";

/**
 * Create Cointime section
 * @returns {PartialOptionsGroup}
 */
export function createCointimeSection() {
  const { cointime, distribution, supply } = brk.metrics;
  const {
    pricing,
    cap,
    activity,
    supply: cointimeSupply,
    adjusted,
    reserveRisk,
    value,
  } = cointime;
  const { all } = distribution.utxoCohorts;

  // Reference lines for cap comparisons
  const capReferenceLines = /** @type {const} */ ([
    { metric: supply.marketCap, name: "Market", color: colors.default },
    {
      metric: all.realized.realizedCap,
      name: "Realized",
      color: colors.realized,
    },
  ]);

  const prices = /** @type {const} */ ([
    {
      pricePattern: pricing.trueMarketMean,
      ratio: pricing.trueMarketMeanRatio,
      name: "True Market Mean",
      color: colors.trueMarketMean,
    },
    {
      pricePattern: pricing.vaultedPrice,
      ratio: pricing.vaultedPriceRatio,
      name: "Vaulted",
      color: colors.vaulted,
    },
    {
      pricePattern: pricing.activePrice,
      ratio: pricing.activePriceRatio,
      name: "Active",
      color: colors.active,
    },
    {
      pricePattern: pricing.cointimePrice,
      ratio: pricing.cointimePriceRatio,
      name: "Cointime",
      color: colors.cointime,
    },
  ]);

  const caps = /** @type {const} */ ([
    { metric: cap.vaultedCap, name: "Vaulted", color: colors.vaulted },
    { metric: cap.activeCap, name: "Active", color: colors.active },
    { metric: cap.cointimeCap, name: "Cointime", color: colors.cointime },
    { metric: cap.investorCap, name: "Investor", color: colors.investor },
    { metric: cap.thermoCap, name: "Thermo", color: colors.thermo },
  ]);

  const supplyBreakdown = /** @type {const} */ ([
    { pattern: all.supply.total, name: "Total", color: colors.bitcoin },
    {
      pattern: cointimeSupply.vaultedSupply,
      name: "Vaulted",
      color: colors.vaulted,
    },
    {
      pattern: cointimeSupply.activeSupply,
      name: "Active",
      color: colors.active,
    },
  ]);

  const coinblocks = /** @type {const} */ ([
    {
      pattern: all.activity.coinblocksDestroyed,
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
      pattern: value.cointimeValueCreated,
      name: "Created",
      title: "Cointime Value Created",
      color: colors.created,
    },
    {
      pattern: value.cointimeValueDestroyed,
      name: "Destroyed",
      title: "Cointime Value Destroyed",
      color: colors.destroyed,
    },
    {
      pattern: value.cointimeValueStored,
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
                metric: all.realized.realizedPrice,
                name: "Realized",
                color: colors.realized,
              }),
              price({
                metric: all.realized.investorPrice,
                name: "Investor",
                color: colors.investor,
              }),
              ...prices.map(({ pricePattern, name, color }) =>
                price({ metric: pricePattern, name, color }),
              ),
            ],
          },
          ...prices.map(({ pricePattern, ratio, name, color }) => ({
            name,
            tree: createPriceRatioCharts({
              context: `${name} Price`,
              legend: name,
              pricePattern,
              ratio,
              color,
              priceReferences: [
                price({
                  metric: all.realized.realizedPrice,
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
              ...capReferenceLines.map(({ metric, name, color }) =>
                line({ metric, name, color, unit: Unit.usd }),
              ),
              ...caps.map(({ metric, name, color }) =>
                line({ metric, name, color, unit: Unit.usd }),
              ),
            ],
          },
          ...caps.map(({ metric, name, color }) => ({
            name,
            title: `${name} Cap`,
            bottom: [
              line({ metric, name, color, unit: Unit.usd }),
              ...capReferenceLines.map((ref) =>
                line({
                  metric: ref.metric,
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
            metric: activity.liveliness,
            name: "Liveliness",
            color: colors.liveliness,
            unit: Unit.ratio,
          }),
          line({
            metric: activity.vaultedness,
            name: "Vaultedness",
            color: colors.vaulted,
            unit: Unit.ratio,
          }),
          line({
            metric: activity.activityToVaultednessRatio,
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
            tree: [
              {
                name: "Base",
                title: "Coinblocks",
                bottom: coinblocks.map(({ pattern, name, color }) =>
                  line({
                    metric: pattern.height,
                    name,
                    color,
                    unit: Unit.coinblocks,
                  }),
                ),
              },
              {
                name: "Cumulative",
                title: "Coinblocks (Total)",
                bottom: coinblocks.map(({ pattern, name, color }) =>
                  line({
                    metric: pattern.cumulative,
                    name,
                    color,
                    unit: Unit.coinblocks,
                  }),
                ),
              },
            ],
          },
          ...coinblocks.map(({ pattern, name, title, color }) => ({
            name,
            tree: [
              {
                name: "Base",
                title,
                bottom: [
                  line({
                    metric: pattern.height,
                    name,
                    color,
                    unit: Unit.coinblocks,
                  }),
                ],
              },
              rollingWindowsTree({ windows: pattern.sum, title, unit: Unit.coinblocks }),
              {
                name: "Cumulative",
                title: `${title} (Total)`,
                bottom: [
                  line({
                    metric: pattern.cumulative,
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
            tree: [
              {
                name: "Base",
                title: "Cointime Value",
                bottom: [
                  ...cointimeValues.map(({ pattern, name, color }) =>
                    line({ metric: pattern.height, name, color, unit: Unit.usd }),
                  ),
                  line({
                    metric: vocdd.pattern.height,
                    name: vocdd.name,
                    color: vocdd.color,
                    unit: Unit.usd,
                  }),
                ],
              },
              {
                name: "Cumulative",
                title: "Cointime Value (Total)",
                bottom: [
                  ...cointimeValues.map(({ pattern, name, color }) =>
                    line({
                      metric: pattern.cumulative,
                      name,
                      color,
                      unit: Unit.usd,
                    }),
                  ),
                  line({
                    metric: vocdd.pattern.cumulative,
                    name: vocdd.name,
                    color: vocdd.color,
                    unit: Unit.usd,
                  }),
                ],
              },
            ],
          },
          ...cointimeValues.map(({ pattern, name, title, color }) => ({
            name,
            tree: [
              {
                name: "Base",
                title,
                bottom: [
                  line({ metric: pattern.height, name, color, unit: Unit.usd }),
                ],
              },
              rollingWindowsTree({ windows: pattern.sum, title, unit: Unit.usd }),
              {
                name: "Cumulative",
                title: `${title} (Total)`,
                bottom: [
                  line({
                    metric: pattern.cumulative,
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
                    metric: vocdd.pattern.height,
                    name: vocdd.name,
                    color: vocdd.color,
                    unit: Unit.usd,
                  }),
                  line({
                    metric: reserveRisk.vocdd365dMedian,
                    name: "365d Median",
                    color: colors.time._1y,
                    unit: Unit.usd,
                  }),
                ],
              },
              rollingWindowsTree({ windows: vocdd.pattern.sum, title: vocdd.title, unit: Unit.usd }),
              {
                name: "Cumulative",
                title: `${vocdd.title} (Total)`,
                bottom: [
                  line({
                    metric: vocdd.pattern.cumulative,
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

      // Indicators - derived decision metrics
      {
        name: "Indicators",
        tree: [
          {
            name: "Reserve Risk",
            title: "Reserve Risk",
            bottom: [
              line({
                metric: reserveRisk.reserveRisk,
                name: "Ratio",
                color: colors.reserveRisk,
                unit: Unit.ratio,
              }),
            ],
          },
          {
            name: "HODL Bank",
            title: "HODL Bank",
            bottom: [
              line({
                metric: reserveRisk.hodlBank,
                name: "Value",
                color: colors.hodlBank,
                unit: Unit.usd,
              }),
            ],
          },
        ],
      },

      // Cointime-Adjusted - comparing base vs adjusted metrics
      {
        name: "Cointime-Adjusted",
        tree: [
          {
            name: "Inflation",
            title: "Cointime-Adjusted Inflation",
            bottom: [
              dots({
                metric: supply.inflation,
                name: "Base",
                color: colors.base,
                unit: Unit.percentage,
              }),
              dots({
                metric: adjusted.cointimeAdjInflationRate,
                name: "Cointime-Adjusted",
                color: colors.adjusted,
                unit: Unit.percentage,
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
                    metric: supply.velocity.btc,
                    name: "Base",
                    color: colors.base,
                    unit: Unit.ratio,
                  }),
                  line({
                    metric: adjusted.cointimeAdjTxBtcVelocity,
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
                    metric: supply.velocity.usd,
                    name: "Base",
                    color: colors.thermo,
                    unit: Unit.ratio,
                  }),
                  line({
                    metric: adjusted.cointimeAdjTxUsdVelocity,
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
