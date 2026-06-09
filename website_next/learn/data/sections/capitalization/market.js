import {
  marketCapAddressBalanceSeries,
  marketCapAgeSeries,
  marketCapClassSeries,
  marketCapEpochSeries,
  marketCapProfitabilitySeries,
  marketCapSeries,
  marketCapTermSeries,
  marketCapTypeSeries,
  marketCapUtxoBalanceSeries,
} from "../../capitalization.js";
import { units } from "../../../charts/units.js";

export const marketCapSection = {
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
};
