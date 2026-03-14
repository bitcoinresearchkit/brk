import { colors } from "../utils/colors.js";
import { brk } from "../client.js";
import { Unit } from "../utils/units.js";
import { dots, line, baseline, price, rollingWindowsTree } from "./series.js";
import { satsBtcUsd } from "./shared.js";

/**
 * Create Cointime section
 * @returns {PartialOptionsGroup}
 */
export function createCointimeSection() {
  const { cointime, cohorts, supply } = brk.metrics;
  const {
    prices: cointimePrices,
    cap,
    activity,
    supply: cointimeSupply,
    adjusted,
    reserveRisk,
    value,
    coinblocksDestroyed,
  } = cointime;
  const { all } = cohorts.utxo;

  // Reference lines for cap comparisons
  const capReferenceLines = /** @type {const} */ ([
    { metric: supply.marketCap.usd, name: "Market", color: colors.default },
    {
      metric: all.realized.cap.usd,
      name: "Realized",
      color: colors.realized,
    },
  ]);

  const prices = /** @type {const} */ ([
    {
      pattern: cointimePrices.trueMarketMean,
      name: "True Market Mean",
      color: colors.trueMarketMean,
    },
    {
      pattern: cointimePrices.vaulted,
      name: "Vaulted",
      color: colors.vaulted,
    },
    {
      pattern: cointimePrices.active,
      name: "Active",
      color: colors.active,
    },
    {
      pattern: cointimePrices.cointime,
      name: "Cointime",
      color: colors.cointime,
    },
  ]);

  const caps = /** @type {const} */ ([
    { metric: cap.vaulted.usd, name: "Vaulted", color: colors.vaulted },
    { metric: cap.active.usd, name: "Active", color: colors.active },
    { metric: cap.cointime.usd, name: "Cointime", color: colors.cointime },
    { metric: cap.investor.usd, name: "Investor", color: colors.investor },
    { metric: cap.thermo.usd, name: "Thermo", color: colors.thermo },
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
      pattern: coinblocksDestroyed,
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
                metric: all.realized.price,
                name: "Realized",
                color: colors.realized,
              }),
              price({
                metric: all.realized.investor.price,
                name: "Investor",
                color: colors.investor,
              }),
              ...prices.map(({ pattern, name, color }) =>
                price({ metric: pattern, name, color }),
              ),
            ],
          },
          ...prices.map(({ pattern, name, color }) => {
            const p = pattern.percentiles;
            const pctUsd = /** @type {const} */ ([
              { name: "pct95", prop: p.pct95.price, color: colors.ratioPct._95 },
              { name: "pct5", prop: p.pct5.price, color: colors.ratioPct._5 },
              { name: "pct99", prop: p.pct99.price, color: colors.ratioPct._99 },
              { name: "pct1", prop: p.pct1.price, color: colors.ratioPct._1 },
            ]);
            const pctRatio = /** @type {const} */ ([
              { name: "pct95", prop: p.pct95.ratio, color: colors.ratioPct._95 },
              { name: "pct5", prop: p.pct5.ratio, color: colors.ratioPct._5 },
              { name: "pct99", prop: p.pct99.ratio, color: colors.ratioPct._99 },
              { name: "pct1", prop: p.pct1.ratio, color: colors.ratioPct._1 },
            ]);
            return {
              name,
              tree: [
                {
                  name: "Price",
                  title: `${name} Price`,
                  top: [
                    price({ metric: pattern, name, color }),
                    price({
                      metric: all.realized.price,
                      name: "Realized",
                      color: colors.realized,
                      defaultActive: false,
                    }),
                    ...pctUsd.map(({ name: pName, prop, color: pColor }) =>
                      price({ metric: prop, name: pName, color: pColor, defaultActive: false, options: { lineStyle: 1 } }),
                    ),
                  ],
                },
                {
                  name: "Ratio",
                  title: `${name} Price Ratio`,
                  top: [price({ metric: pattern, name, color })],
                  bottom: [
                    baseline({ metric: pattern.ratio, name: "Ratio", unit: Unit.ratio, base: 1 }),
                    ...pctRatio.map(({ name: pName, prop, color: pColor }) =>
                      line({ metric: prop, name: pName, color: pColor, defaultActive: false, unit: Unit.ratio, options: { lineStyle: 1 } }),
                    ),
                  ],
                },
              ],
            };
          }),
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
            metric: activity.ratio,
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
                    metric: pattern.base,
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
                    metric: pattern.base,
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
                    line({ metric: pattern.base, name, color, unit: Unit.usd }),
                  ),
                  line({
                    metric: vocdd.pattern.base,
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
                  line({ metric: pattern.base, name, color, unit: Unit.usd }),
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
                    metric: vocdd.pattern.base,
                    name: vocdd.name,
                    color: vocdd.color,
                    unit: Unit.usd,
                  }),
                  line({
                    metric: reserveRisk.vocddMedian1y,
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
                metric: reserveRisk.value,
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
                metric: supply.inflationRate.percent,
                name: "Base",
                color: colors.base,
                unit: Unit.percentage,
              }),
              dots({
                metric: adjusted.inflationRate.percent,
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
                    metric: adjusted.txVelocityBtc,
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
                    metric: adjusted.txVelocityUsd,
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
