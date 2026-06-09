import {
  ageSeries,
  balanceSeries,
  changeSeries,
  classSeries,
  epochSeries,
  growthRateSeries,
  spendingRateAgeSeries,
  spendingRateBalanceSeries,
  spendingRateClassSeries,
  spendingRateEpochSeries,
  spendingRateSeries,
  spendingRateTermSeries,
  spendingRateTypeSeries,
  spentSeries,
  termSeries,
  totalSeries,
  typeSeries,
} from "../utxo-set.js";
import { units } from "../../charts/units.js";

const line = /** @type {const} */ ("line");

export const utxoSetSection = {
  title: "UTXO Set",
  description:
    "The UTXO set is the collection of all spendable bitcoin outputs that exist right now. Each UTXO is a separate coin fragment created by a transaction and later consumed when it is spent. Counting UTXOs shows how Bitcoin is split into pieces, which is different from counting how much BTC those pieces contain.",
  chart: {
    title: "UTXO set",
    unit: units.utxos,
    defaultType: line,
    series: totalSeries,
  },
  children: [
    {
      title: "Activity",
      description:
        "Shows how the UTXO set changes as transactions create new outputs and consume old ones. These charts focus on movement and turnover, not the current composition of the set.",
      children: [
        {
          title: "Change",
          description:
            "Shows the rolling net change in the UTXO set. The count rises when transactions create more spendable outputs than they consume, and falls when spending consolidates many old outputs into fewer new ones.",
          chart: {
            title: "UTXO set change",
            unit: units.utxos,
            defaultType: line,
            series: changeSeries,
          },
        },
        {
          title: "Growth Rate",
          description:
            "Shows the rolling percentage change of the UTXO set. It measures the same net expansion or contraction as Change, but normalizes it by the size of the set so different periods are easier to compare.",
          chart: {
            title: "UTXO set growth rate",
            unit: units.percent,
            defaultType: line,
            series: growthRateSeries,
          },
        },
        {
          title: "Spent",
          description:
            "Counts how many UTXOs were spent during each rolling window. This measures how much of the existing set was consumed as transaction inputs, regardless of the BTC value inside those outputs.",
          chart: {
            title: "Spent UTXOs",
            unit: units.utxos,
            defaultType: line,
            series: spentSeries,
          },
        },
        {
          title: "Spending Rate",
          description:
            "Shows how quickly the UTXO set is being consumed. Instead of counting spent outputs directly, it expresses spending as a rate, which makes busy and quiet periods easier to compare.",
          chart: {
            title: "UTXO set spending rate",
            unit: units.percent,
            defaultType: line,
            series: spendingRateSeries,
          },
          children: [
            {
              title: "Term",
              description:
                "Splits spending rate between short-term and long-term holder cohorts. This shows whether recent or dormant UTXOs are being consumed faster.",
              chart: {
                title: "UTXO spending rate by term",
                unit: units.percent,
                defaultType: line,
                series: spendingRateTermSeries,
              },
            },
            {
              title: "Age",
              description:
                "Groups spending rate by how long UTXOs have stayed unspent. This shows which age bands are turning over fastest.",
              chart: {
                title: "UTXO spending rate by age",
                unit: units.percent,
                defaultType: line,
                series: spendingRateAgeSeries,
              },
            },
            {
              title: "Balance",
              description:
                "Groups spending rate by the BTC amount held in each UTXO. This shows whether small or large outputs are being consumed faster.",
              chart: {
                title: "UTXO spending rate by balance",
                unit: units.percent,
                defaultType: line,
                series: spendingRateBalanceSeries,
              },
            },
            {
              title: "Type",
              description:
                "Groups spending rate by output type. This shows how quickly UTXOs from each script format are being consumed.",
              chart: {
                title: "UTXO spending rate by type",
                unit: units.percent,
                defaultType: line,
                series: spendingRateTypeSeries,
              },
            },
            {
              title: "Epoch",
              description:
                "Groups spending rate by the halving epoch when coins were mined. This shows which issuance periods are turning over fastest.",
              chart: {
                title: "UTXO spending rate by epoch",
                unit: units.percent,
                defaultType: line,
                series: spendingRateEpochSeries,
              },
            },
            {
              title: "Class",
              description:
                "Groups spending rate by the calendar year when coins were mined. This shows which issuance years are turning over fastest.",
              chart: {
                title: "UTXO spending rate by class",
                unit: units.percent,
                defaultType: line,
                series: spendingRateClassSeries,
              },
            },
          ],
        },
      ],
    },
    {
      title: "Distribution",
      description:
        "Shows how the current UTXO set is split across different groups. These charts describe the composition of existing spendable outputs, not how quickly they are changing.",
      children: [
        {
          title: "Term",
          description:
            "Splits the UTXO set between short-term and long-term holder cohorts. This counts pieces, not BTC, so it shows whether recent and dormant supply is made of many small outputs or fewer larger ones.",
          chart: {
            title: "UTXO set by term",
            unit: units.utxos,
            series: termSeries,
          },
        },
        {
          title: "Age",
          description:
            "Groups UTXOs by how long they have stayed unspent. A young UTXO was created recently, while an old UTXO has survived many blocks without being consumed in a transaction.",
          chart: {
            title: "UTXO set by age",
            unit: units.utxos,
            series: ageSeries,
          },
        },
        {
          title: "Balance",
          description:
            "Groups UTXOs by the BTC amount held in each output. This shows the size distribution of spendable pieces, from tiny fragments to very large outputs.",
          chart: {
            title: "UTXO set by balance",
            unit: units.utxos,
            series: balanceSeries,
          },
        },
        {
          title: "Type",
          description:
            "Groups UTXOs by output type. The type is the script format that defines how the output can be spent, such as legacy, SegWit, or Taproot.",
          chart: {
            title: "UTXO set by type",
            unit: units.utxos,
            series: typeSeries,
          },
        },
        {
          title: "Epoch",
          description:
            "Groups UTXOs by the halving epoch when their coins were mined. This shows how many currently spendable pieces trace back to each issuance period.",
          chart: {
            title: "UTXO set by epoch",
            unit: units.utxos,
            series: epochSeries,
          },
        },
        {
          title: "Class",
          description:
            "Groups UTXOs by the calendar year when their coins were mined. This shows how the current set of spendable pieces is distributed across issuance years.",
          chart: {
            title: "UTXO set by class",
            unit: units.utxos,
            series: classSeries,
          },
        },
      ],
    },
  ],
};
