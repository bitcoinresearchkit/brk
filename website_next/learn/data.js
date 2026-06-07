import {
  capitalizationSeries,
  marketCapAddressBalanceSeries,
  marketCapAgeSeries,
  marketCapClassSeries,
  marketCapEpochSeries,
  marketCapProfitabilitySeries,
  marketCapSeries,
  marketCapTermSeries,
  marketCapTypeSeries,
  marketCapUtxoBalanceSeries,
  realizedCapAddressBalanceSeries,
  realizedCapAgeSeries,
  realizedCapClassSeries,
  realizedCapEpochSeries,
  realizedCapProfitabilitySeries,
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
import { units } from "./charts/units.js";

const lineType = /** @type {const} */ ("line");

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
    title: "Introduction",
    numbered: false,
    description:
      "Bitcoin can be measured from many angles, but a single number rarely explains much on its own. This page introduces core Bitcoin concepts through data that changes over time. Each chart is meant to answer a simple question: what is being measured, how has it changed, and how does it compare across different groups? The goal is to make the system easier to read, from the supply itself to the way coins move, age, concentrate, and gain value.",
  },
  {
    title: "Supply",
    description:
      "Bitcoin has a fixed issuance schedule. This chart shows how many BTC are in circulation over time, so you can see supply rising toward the 21 million limit.",
    chart: {
      title: "Circulating supply",
      unit: units.btc,
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
          "Shows whether coins are in profit or loss based on the price when they last moved on-chain. A coin is in profit when today's price is higher than its last moved price, and in loss when today's price is lower.",
        chart: {
          title: "Profitability",
          unit: units.btc,
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
          "Splits supply between coins that moved recently and coins that have stayed still longer. This helps separate more active supply from long-term holder supply.",
        chart: {
          title: "Supply by term",
          unit: units.btc,
          series: termSeries,
        },
      },
      {
        title: "Age",
        description:
          "Groups coins by how long they have stayed still since their last on-chain movement. Older coins are usually more dormant, while younger coins have moved more recently.",
        chart: {
          title: "Supply by age",
          unit: units.btc,
          series: ageSeries,
        },
      },
      {
        title: "UTXO Balance",
        description:
          "Groups supply by the size of each unspent output. A UTXO is a spendable piece of bitcoin created by a transaction, so this shows the size distribution of coin fragments.",
        chart: {
          title: "Supply by UTXO balance",
          unit: units.btc,
          series: utxoBalanceSeries,
        },
      },
      {
        title: "Address Balance",
        description:
          "Groups supply by the total BTC held at each address. An address is not the same as a person or entity, but this still helps show how balances are distributed on-chain.",
        chart: {
          title: "Supply by address balance",
          unit: units.btc,
          series: addressBalanceSeries,
        },
      },
      {
        title: "Type",
        description:
          "Groups supply by Bitcoin output type. The output type is the script format that defines how coins can be spent.",
        chart: {
          title: "Supply by type",
          unit: units.btc,
          series: typeSeries,
        },
      },
      {
        title: "Epoch",
        description:
          "Groups supply by the halving epoch when coins were mined. A halving epoch is a period between two subsidy halvings, when the amount of new BTC paid to miners changes.",
        chart: {
          title: "Supply by epoch",
          unit: units.btc,
          series: epochSeries,
        },
      },
      {
        title: "Class",
        description:
          "Groups supply by the calendar year when coins were mined. This shows how much of today's supply comes from each issuance year.",
        chart: {
          title: "Supply by class",
          unit: units.btc,
          series: classSeries,
        },
      },
    ],
  },
  {
    title: "Capitalization",
    description:
      "Shows ways to value Bitcoin in US dollars. Market cap uses today's price, while realized cap uses the price when coins last moved on-chain.",
    chart: {
      title: "Capitalization",
      unit: units.usd,
      defaultType: lineType,
      series: capitalizationSeries,
    },
    children: [
      {
        title: "Market Cap",
        description:
          "Market cap is circulating supply multiplied by the current bitcoin price. It answers: what is all circulating BTC worth at today's market price?",
        chart: {
          title: "Market cap",
          unit: units.usd,
          series: marketCapSeries,
        },
        children: [
          {
            title: "Profitability",
            description:
              "Splits market cap between coins that are currently in profit and coins that are currently in loss. This shows how much current market value sits above or below each coin's last moved price.",
            chart: {
              title: "Market cap by profitability",
              unit: units.usd,
              series: marketCapProfitabilitySeries,
            },
          },
          {
            title: "Term",
            description:
              "Splits market cap between coins that moved recently and coins that have stayed still longer. This shows how much current market value sits with active supply versus long-term holder supply.",
            chart: {
              title: "Market cap by term",
              unit: units.usd,
              series: marketCapTermSeries,
            },
          },
          {
            title: "Age",
            description:
              "Groups market cap by how long coins have stayed still since their last on-chain movement. It shows which age bands hold the most current market value.",
            chart: {
              title: "Market cap by age",
              unit: units.usd,
              series: marketCapAgeSeries,
            },
          },
          {
            title: "UTXO Balance",
            description:
              "Groups market cap by the size of each unspent output. This shows how current market value is distributed across small and large spendable coin fragments.",
            chart: {
              title: "Market cap by UTXO balance",
              unit: units.usd,
              series: marketCapUtxoBalanceSeries,
            },
          },
          {
            title: "Address Balance",
            description:
              "Groups market cap by the total BTC held at each address. Addresses are not people or entities, but this still helps show how current market value is distributed across address balances.",
            chart: {
              title: "Market cap by address balance",
              unit: units.usd,
              series: marketCapAddressBalanceSeries,
            },
          },
          {
            title: "Type",
            description:
              "Groups market cap by Bitcoin output type. This shows how much current market value is held in each script format.",
            chart: {
              title: "Market cap by type",
              unit: units.usd,
              series: marketCapTypeSeries,
            },
          },
          {
            title: "Epoch",
            description:
              "Groups market cap by the halving epoch when coins were mined. This shows the current value of coins created during each issuance period.",
            chart: {
              title: "Market cap by epoch",
              unit: units.usd,
              series: marketCapEpochSeries,
            },
          },
          {
            title: "Class",
            description:
              "Groups market cap by the calendar year when coins were mined. This shows the current value of supply created in each year.",
            chart: {
              title: "Market cap by class",
              unit: units.usd,
              series: marketCapClassSeries,
            },
          },
        ],
      },
      {
        title: "Realized Cap",
        description:
          "Realized cap values each coin at the price when it last moved on-chain. It is often used as a rough view of the market's aggregate cost basis.",
        chart: {
          title: "Realized cap",
          unit: units.usd,
          series: realizedCapSeries,
        },
        children: [
          {
            title: "Profitability",
            description:
              "Splits realized cap between coins that are currently in profit and coins that are currently in loss. This shows how the market's cost basis is distributed across coins above or below their last moved price.",
            chart: {
              title: "Realized cap by profitability",
              unit: units.usd,
              series: realizedCapProfitabilitySeries,
            },
          },
          {
            title: "Term",
            description:
              "Splits realized cap between coins that moved recently and coins that have stayed still longer. This shows where the market's cost basis sits across active and long-term holder supply.",
            chart: {
              title: "Realized cap by term",
              unit: units.usd,
              series: realizedCapTermSeries,
            },
          },
          {
            title: "Age",
            description:
              "Groups realized cap by how long coins have stayed still since their last on-chain movement. This shows which coin ages carry the largest share of the market's cost basis.",
            chart: {
              title: "Realized cap by age",
              unit: units.usd,
              series: realizedCapAgeSeries,
            },
          },
          {
            title: "UTXO Balance",
            description:
              "Groups realized cap by the size of each unspent output. This shows how cost basis is distributed across small and large spendable coin fragments.",
            chart: {
              title: "Realized cap by UTXO balance",
              unit: units.usd,
              series: realizedCapUtxoBalanceSeries,
            },
          },
          {
            title: "Address Balance",
            description:
              "Groups realized cap by the total BTC held at each address. Addresses are not people or entities, but this still helps show how cost basis is distributed across address balances.",
            chart: {
              title: "Realized cap by address balance",
              unit: units.usd,
              series: realizedCapAddressBalanceSeries,
            },
          },
          {
            title: "Type",
            description:
              "Groups realized cap by Bitcoin output type. This shows how much cost basis is held in each script format.",
            chart: {
              title: "Realized cap by type",
              unit: units.usd,
              series: realizedCapTypeSeries,
            },
          },
          {
            title: "Epoch",
            description:
              "Groups realized cap by the halving epoch when coins were mined. This shows the cost basis of coins created during each issuance period.",
            chart: {
              title: "Realized cap by epoch",
              unit: units.usd,
              series: realizedCapEpochSeries,
            },
          },
          {
            title: "Class",
            description:
              "Groups realized cap by the calendar year when coins were mined. This shows the cost basis of supply created in each year.",
            chart: {
              title: "Realized cap by class",
              unit: units.usd,
              series: realizedCapClassSeries,
            },
          },
        ],
      },
    ],
  },
];
