import {
  activeSeries,
  balanceSeries,
  bidirectionalSeries,
  changeSeries,
  fundedSeries,
  growthRateSeries,
  newSeries,
  reactivatedSeries,
  receivingSeries,
  reuseSeries,
  sendingSeries,
  stateSeries,
  typeSeries,
} from "../address-count.js";
import { units } from "../../charts/units.js";

const line = /** @type {const} */ ("line");

export const addressCountSection = {
  title: "Address Count",
  description:
    "Address count measures Bitcoin addresses, not people or entities. A funded address currently has a non-zero balance, while empty addresses have received or spent coins before but no longer hold BTC. These charts show how the address set grows, turns over, and distributes across balances and script types.",
  chart: {
    title: "Funded addresses",
    unit: units.addresses,
    defaultType: line,
    series: fundedSeries,
  },
  children: [
    {
      title: "Activity",
      description:
        "Shows how addresses appear and participate in transactions over time. These charts focus on address movement and usage, not the current distribution of address balances.",
      children: [
        {
          title: "New",
          description:
            "Counts addresses that appear for the first time during each rolling window. A new address is not necessarily a new user, but it does show fresh address creation on-chain.",
          chart: {
            title: "New addresses",
            unit: units.addresses,
            defaultType: line,
            series: newSeries,
          },
        },
        {
          title: "Change",
          description:
            "Shows the rolling net change in funded address count. The count rises when more addresses receive a non-zero balance, and falls when more addresses are emptied.",
          chart: {
            title: "Funded address count change",
            unit: units.addresses,
            defaultType: line,
            series: changeSeries,
          },
        },
        {
          title: "Growth Rate",
          description:
            "Shows the rolling percentage change of funded address count. It measures the same expansion or contraction as Change, but normalizes it by the size of the funded address set.",
          chart: {
            title: "Funded address growth rate",
            unit: units.percent,
            defaultType: line,
            series: growthRateSeries,
          },
        },
        {
          title: "Active",
          description:
            "Counts addresses that are active during each rolling window. Active addresses can send, receive, do both, or return after inactivity.",
          chart: {
            title: "Active addresses",
            unit: units.addresses,
            defaultType: line,
            series: activeSeries,
          },
          children: [
            {
              title: "Sending",
              description:
                "Counts addresses that spend from at least one output during each rolling window. This shows address-side transaction participation from senders.",
              chart: {
                title: "Sending addresses",
                unit: units.addresses,
                defaultType: line,
                series: sendingSeries,
              },
            },
            {
              title: "Receiving",
              description:
                "Counts addresses that receive at least one output during each rolling window. This shows address-side transaction participation from recipients.",
              chart: {
                title: "Receiving addresses",
                unit: units.addresses,
                defaultType: line,
                series: receivingSeries,
              },
            },
            {
              title: "Bidirectional",
              description:
                "Counts addresses that both send and receive during each rolling window. This can highlight addresses with more two-sided transaction behavior.",
              chart: {
                title: "Bidirectional addresses",
                unit: units.addresses,
                defaultType: line,
                series: bidirectionalSeries,
              },
            },
            {
              title: "Reactivated",
              description:
                "Counts addresses that become active again after a quiet period. This helps show when older address activity returns.",
              chart: {
                title: "Reactivated addresses",
                unit: units.addresses,
                defaultType: line,
                series: reactivatedSeries,
              },
            },
          ],
        },
      ],
    },
    {
      title: "Distribution",
      description:
        "Shows how addresses are split across states, balances, and address types. These charts describe the current address set rather than how addresses are moving.",
      children: [
        {
          title: "State",
          description:
            "Splits addresses into funded, empty, and total counts. Funded addresses currently hold BTC; empty addresses have no current balance; total includes both.",
          chart: {
            title: "Address count by state",
            unit: units.addresses,
            defaultType: line,
            series: stateSeries,
          },
        },
        {
          title: "Balance",
          description:
            "Groups funded addresses by the BTC amount held at each address. Addresses are not people or entities, but this still shows how address balances are distributed on-chain.",
          chart: {
            title: "Address count by balance",
            unit: units.addresses,
            series: balanceSeries,
          },
        },
        {
          title: "Type",
          description:
            "Groups funded addresses by address type. The type reflects the script format used by the address, such as legacy, SegWit, or Taproot.",
          chart: {
            title: "Funded address count by type",
            unit: units.addresses,
            series: typeSeries,
          },
        },
      ],
    },
    {
      title: "Reuse",
      description:
        "Shows address patterns that can reduce privacy or reveal public-key information. These counts are address-level signals, not direct counts of people.",
      chart: {
        title: "Address count by reuse",
        unit: units.addresses,
        defaultType: line,
        series: reuseSeries,
      },
    },
  ],
};
