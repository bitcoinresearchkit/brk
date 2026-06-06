import {
  capitalizationSeries,
  marketCapAddressBalanceSeries,
  marketCapAgeSeries,
  marketCapClassSeries,
  marketCapEpochSeries,
  marketCapSeries,
  marketCapTermSeries,
  marketCapTypeSeries,
  marketCapUtxoBalanceSeries,
  realizedCapAddressBalanceSeries,
  realizedCapAgeSeries,
  realizedCapClassSeries,
  realizedCapEpochSeries,
  realizedCapSeries,
  realizedCapTermSeries,
  realizedCapTypeSeries,
  realizedCapUtxoBalanceSeries,
} from "./capitalization.js";
import {
  addressBalanceSeries,
  ageSeries,
  classSeries,
  epochSeries,
  termSeries,
  typeSeries,
  utxoBalanceSeries,
} from "./cohorts.js";
import { colors } from "../utils/colors.js";

/** @param {typeof import("../utils/client.js").brk} client */
function metricCirculatingSupply(client) {
  return client.series.supply.circulating.btc;
}

/** @param {typeof import("../utils/client.js").brk} client */
function metricSupplyInProfit(client) {
  return client.series.cohorts.utxo.profitability.profit.all.supply.all.btc;
}

/** @param {typeof import("../utils/client.js").brk} client */
function metricSupplyInLoss(client) {
  return client.series.cohorts.utxo.profitability.loss.all.supply.all.btc;
}

export const sections = [
  {
    title: "Supply",
    description:
      "How bitcoin moves from issuance into long-term ownership, profit, loss, and distribution.",
    chart: {
      title: "Circulating supply",
      series: [
        {
          label: "Circulating",
          color: colors.orange,
          metric: metricCirculatingSupply,
        },
      ],
    },
    children: [
      {
        title: "Profitability",
        description:
          "Which coins sit in profit or loss, and how that balance changes through cycles.",
        chart: {
          title: "Profitability",
          series: [
            {
              label: "In profit",
              color: colors.green,
              metric: metricSupplyInProfit,
            },
            {
              label: "In loss",
              color: colors.red,
              metric: metricSupplyInLoss,
            },
          ],
        },
      },
      {
        title: "Term",
        description:
          "Supply split between recently moved coins and long-term holder coins.",
        chart: {
          title: "Supply by term",
          series: termSeries,
        },
      },
      {
        title: "Age",
        description:
          "How long coins have remained still, from fresh movement to deep dormancy.",
        chart: {
          title: "Supply by age",
          series: ageSeries,
        },
      },
      {
        title: "UTXO Balance",
        description: "Supply grouped by the amount held in each unspent output.",
        chart: {
          title: "Supply by UTXO balance",
          series: utxoBalanceSeries,
        },
      },
      {
        title: "Address Balance",
        description: "Supply grouped by the balance held at each address.",
        chart: {
          title: "Supply by address balance",
          series: addressBalanceSeries,
        },
      },
      {
        title: "Type",
        description: "Supply grouped by output script type.",
        chart: {
          title: "Supply by type",
          series: typeSeries,
        },
      },
      {
        title: "Epoch",
        description:
          "Supply grouped by the halving epoch in which coins were created.",
        chart: {
          title: "Supply by epoch",
          series: epochSeries,
        },
      },
      {
        title: "Class",
        description:
          "Supply grouped by the calendar year in which coins were created.",
        chart: {
          title: "Supply by class",
          series: classSeries,
        },
      },
    ],
  },
  {
    title: "Capitalization",
    description:
      "Different ways to value the network by market price, realized cost, and accumulated flows.",
    chart: {
      title: "Capitalization",
      series: capitalizationSeries,
    },
    children: [
      {
        title: "Market Cap",
        description:
          "The current market value of circulating bitcoin at spot price.",
        chart: {
          title: "Market cap",
          series: marketCapSeries,
        },
        children: [
          {
            title: "Term",
            description:
              "Market value split between recently moved and long-term holder coins.",
            chart: {
              title: "Market cap by term",
              series: marketCapTermSeries,
            },
          },
          {
            title: "Age",
            description:
              "Market value grouped by how long coins have remained still.",
            chart: {
              title: "Market cap by age",
              series: marketCapAgeSeries,
            },
          },
          {
            title: "UTXO Balance",
            description:
              "Market value grouped by the amount held in each unspent output.",
            chart: {
              title: "Market cap by UTXO balance",
              series: marketCapUtxoBalanceSeries,
            },
          },
          {
            title: "Address Balance",
            description:
              "Market value grouped by the balance held at each address.",
            chart: {
              title: "Market cap by address balance",
              series: marketCapAddressBalanceSeries,
            },
          },
          {
            title: "Type",
            description: "Market value grouped by spendable output script type.",
            chart: {
              title: "Market cap by type",
              series: marketCapTypeSeries,
            },
          },
          {
            title: "Epoch",
            description:
              "Market value grouped by the halving epoch in which coins were created.",
            chart: {
              title: "Market cap by epoch",
              series: marketCapEpochSeries,
            },
          },
          {
            title: "Class",
            description:
              "Market value grouped by the calendar year in which coins were created.",
            chart: {
              title: "Market cap by class",
              series: marketCapClassSeries,
            },
          },
        ],
      },
      {
        title: "Realized Cap",
        description:
          "The aggregate value of coins priced where they last moved on-chain.",
        chart: {
          title: "Realized cap",
          series: realizedCapSeries,
        },
        children: [
          {
            title: "Term",
            description:
              "Realized value split between recently moved and long-term holder coins.",
            chart: {
              title: "Realized cap by term",
              series: realizedCapTermSeries,
            },
          },
          {
            title: "Age",
            description:
              "Realized value grouped by how long coins have remained still.",
            chart: {
              title: "Realized cap by age",
              series: realizedCapAgeSeries,
            },
          },
          {
            title: "UTXO Balance",
            description:
              "Realized value grouped by the amount held in each unspent output.",
            chart: {
              title: "Realized cap by UTXO balance",
              series: realizedCapUtxoBalanceSeries,
            },
          },
          {
            title: "Address Balance",
            description:
              "Realized value grouped by the balance held at each address.",
            chart: {
              title: "Realized cap by address balance",
              series: realizedCapAddressBalanceSeries,
            },
          },
          {
            title: "Type",
            description:
              "Realized value grouped by spendable output script type.",
            chart: {
              title: "Realized cap by type",
              series: realizedCapTypeSeries,
            },
          },
          {
            title: "Epoch",
            description:
              "Realized value grouped by the halving epoch in which coins were created.",
            chart: {
              title: "Realized cap by epoch",
              series: realizedCapEpochSeries,
            },
          },
          {
            title: "Class",
            description:
              "Realized value grouped by the calendar year in which coins were created.",
            chart: {
              title: "Realized cap by class",
              series: realizedCapClassSeries,
            },
          },
        ],
      },
    ],
  },
  {
    title: "Activity",
    description:
      "How often the chain is used, how value moves, and how demand appears " +
      "in fees and transactions.",
    chart: "Network activity",
    children: [
      {
        title: "Transactions",
        description:
          "Confirmed transaction count, throughput, and block-level settlement patterns.",
        chart: "Transaction count",
      },
      {
        title: "Fees",
        description:
          "The cost users pay for block space and what that reveals about demand.",
        chart: "Fee rate",
      },
      {
        title: "Addresses",
        description:
          "Address creation, reuse, activity, and balance changes across the network.",
        chart: "Active addresses",
      },
    ],
  },
  {
    title: "Mining",
    description:
      "The security budget, difficulty adjustments, pool behavior, and miner revenue.",
    chart: "Mining overview",
    children: [
      {
        title: "Hashrate",
        description:
          "Estimated computational power securing the network over time.",
        chart: "Hashrate",
      },
      {
        title: "Difficulty",
        description:
          "How Bitcoin adjusts mining difficulty to keep block production steady.",
        chart: "Difficulty",
      },
      {
        title: "Rewards",
        description:
          "Subsidy, fees, and the changing economics of block production.",
        chart: "Miner rewards",
      },
    ],
  },
  {
    title: "Market",
    description:
      "Price behavior, returns, volatility, and the market context around on-chain patterns.",
    chart: "Market overview",
    children: [
      {
        title: "Price",
        description:
          "Bitcoin price across time, cycles, drawdowns, and all-time highs.",
        chart: "Price",
      },
      {
        title: "Returns",
        description:
          "How returns vary by holding period, entry point, and cycle phase.",
        chart: "Returns",
      },
      {
        title: "Volatility",
        description:
          "The scale and rhythm of price movement across different windows.",
        chart: "Volatility",
      },
    ],
  },
  {
    title: "Ownership",
    description:
      "How coins are held across balances, entities, custody patterns, and long-term cohorts.",
    chart: "Ownership overview",
    children: [
      {
        title: "Balances",
        description:
          "Address and entity balances grouped by size, concentration, and historical change.",
        chart: "Balance cohorts",
      },
      {
        title: "Entities",
        description:
          "Estimated ownership clusters and how their behavior changes through market regimes.",
        chart: "Entity supply",
      },
      {
        title: "Custody",
        description:
          "Coins associated with exchanges, funds, miners, and other observable custody groups.",
        chart: "Custody balances",
      },
    ],
  },
  {
    title: "Liquidity",
    description:
      "How available supply changes as coins move between liquid, illiquid, and exchange venues.",
    chart: "Liquidity overview",
    children: [
      {
        title: "Liquid Supply",
        description:
          "Coins held by entities that tend to spend, trade, or redistribute frequently.",
        chart: "Liquid supply",
      },
      {
        title: "Illiquid Supply",
        description:
          "Coins held by entities with low spending history and stronger accumulation behavior.",
        chart: "Illiquid supply",
      },
      {
        title: "Exchange Flow",
        description:
          "Deposits, withdrawals, and balance changes across known exchange clusters.",
        chart: "Exchange netflow",
      },
    ],
  },
  {
    title: "Risk",
    description:
      "Stress, leverage, drawdown, and valuation conditions that shape market fragility.",
    chart: "Risk overview",
    children: [
      {
        title: "Drawdown",
        description:
          "Distance from prior highs and the depth of cycle retracements over time.",
        chart: "Drawdown",
      },
      {
        title: "Stress",
        description:
          "Periods where losses, volatility, and fee pressure concentrate together.",
        chart: "Network stress",
      },
      {
        title: "Leverage",
        description:
          "Market conditions that indicate amplified exposure and forced positioning risk.",
        chart: "Leverage proxy",
      },
    ],
  },
  {
    title: "Cycles",
    description:
      "How Bitcoin behaves across halvings, adoption waves, liquidity regimes, and market phases.",
    chart: "Cycle overview",
    children: [
      {
        title: "Halvings",
        description:
          "Supply issuance changes and their relationship to market and miner behavior.",
        chart: "Halving cycles",
      },
      {
        title: "Phases",
        description:
          "Bull, bear, recovery, and transition periods described through on-chain behavior.",
        chart: "Cycle phases",
      },
      {
        title: "Comparisons",
        description:
          "Cycle-to-cycle comparisons normalized by time, price, drawdown, or supply behavior.",
        chart: "Cycle comparison",
      },
    ],
  },
  {
    title: "Cohorts",
    description:
      "Groups of market participants organized by age, balance, cost basis, and observed behavior.",
    chart: "Cohort overview",
    children: [
      {
        title: "Short Term",
        description:
          "Recently moved coins and holders more sensitive to price, volatility, and liquidity.",
        chart: "Short-term holder supply",
      },
      {
        title: "Long Term",
        description:
          "Older coins and holders with stronger dormancy, conviction, or lower spend frequency.",
        chart: "Long-term holder supply",
      },
      {
        title: "Cost Basis",
        description:
          "Estimated acquisition prices across cohorts and how they frame profit and loss.",
        chart: "Cohort cost basis",
      },
    ],
  },
];
