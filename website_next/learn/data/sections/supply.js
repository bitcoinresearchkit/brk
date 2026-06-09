import {
  addressBalanceSeries,
  ageSeries,
  classSeries,
  epochSeries,
  exposedSupplySeries,
  exposedSupplyTypeSeries,
  termSeries,
  typeSeries,
  utxoBalanceSeries,
} from "../cohorts.js";
import {
  circulatingSupplySeries,
  supplyProfitabilitySeries,
} from "../supply.js";
import { units } from "../../charts/units.js";

export const supplySection = {
  title: "Supply",
  description:
    "Bitcoin has a fixed issuance schedule. This chart shows how many BTC are in circulation over time, so you can see supply rising toward the 21 million limit.",
  chart: {
    title: "Circulating supply",
    unit: units.btc,
    series: circulatingSupplySeries,
  },
  children: [
    {
      title: "Profitability",
      description:
        "Shows whether coins are in profit or loss based on the price when they last moved on-chain. A coin is in profit when today's price is higher than its last moved price, and in loss when today's price is lower.",
      chart: {
        title: "Profitability",
        unit: units.btc,
        series: supplyProfitabilitySeries,
      },
    },
    {
      title: "Exposed",
      description:
        "Shows BTC held by addresses whose public key is already visible on-chain. This can happen because the address type exposes the key directly, or because coins were spent from that address before.",
      chart: {
        title: "Exposed supply",
        unit: units.btc,
        defaultType: /** @type {const} */ ("line"),
        series: exposedSupplySeries,
      },
      children: [
        {
          title: "Type",
          description:
            "Splits exposed supply by address type. This shows which script formats account for the visible-public-key supply.",
          chart: {
            title: "Exposed supply by type",
            unit: units.btc,
            series: exposedSupplyTypeSeries,
          },
        },
      ],
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
};
