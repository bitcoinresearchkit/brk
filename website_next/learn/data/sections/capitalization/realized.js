import {
  realizedCapAddressBalanceSeries,
  realizedCapAgeSeries,
  realizedCapClassSeries,
  realizedCapEpochSeries,
  realizedCapProfitabilitySeries,
  realizedCapSeries,
  realizedCapTermSeries,
  realizedCapTypeSeries,
  realizedCapUtxoBalanceSeries,
} from "../../capitalization.js";
import { units } from "../../../charts/units.js";

export const realizedCapSection = {
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
};
