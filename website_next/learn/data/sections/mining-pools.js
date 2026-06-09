import {
  createMajorPoolBlocksMinedSeries,
  createMajorPoolDominanceSeries,
  createMajorPoolRewardsSeries,
  createMinorPoolBlocksMinedSeries,
  createMinorPoolDominanceSeries,
  majorPools,
  majorPoolBlocksMinedSeries,
  majorPoolDominanceSeries,
  majorPoolRewardsSeries,
  minorPools,
} from "../mining-pools.js";
import { units } from "../../charts/units.js";

const line = /** @type {const} */ ("line");

/** @param {string} name */
function createPoolDescription(name) {
  return `${name} is tracked from pool attribution in mined blocks. These charts show share of blocks, blocks found, and rewards where available. Pool attribution is useful for understanding block production, but it is not the same as knowing who owns the underlying mining hardware.`;
}

/**
 * @param {(typeof majorPools)[number]} item
 */
function createMajorPoolSection({ name, pool }) {
  return {
    title: name,
    description: createPoolDescription(name),
    children: [
      {
        title: "Dominance",
        description:
          "Dominance is the pool's share of mined blocks over each rolling window. It is estimated from blocks attributed to the pool, so it is best read as block-production share.",
        chart: {
          title: `${name} dominance`,
          unit: units.percent,
          defaultType: line,
          series: createMajorPoolDominanceSeries(pool),
        },
      },
      {
        title: "Blocks Mined",
        description:
          "Counts how many blocks were attributed to the pool in each rolling window. This is the raw activity behind the dominance percentage.",
        chart: {
          title: `${name} blocks mined`,
          unit: units.blocks,
          defaultType: line,
          series: createMajorPoolBlocksMinedSeries(pool),
        },
      },
      {
        title: "Rewards",
        description:
          "Sums the BTC earned by blocks attributed to the pool. Rewards include both the block subsidy and transaction fees.",
        chart: {
          title: `${name} rewards`,
          unit: units.btc,
          defaultType: line,
          series: createMajorPoolRewardsSeries(pool),
        },
      },
    ],
  };
}

/**
 * @param {(typeof minorPools)[number]} item
 */
function createMinorPoolSection({ name, pool }) {
  return {
    title: name,
    description: createPoolDescription(name),
    children: [
      {
        title: "Dominance",
        description:
          "Shows the pool's all-time share of mined blocks. Minor pools expose a smaller historical metric set, so rolling dominance is not shown here.",
        chart: {
          title: `${name} dominance`,
          unit: units.percent,
          defaultType: line,
          series: createMinorPoolDominanceSeries(pool),
        },
      },
      {
        title: "Blocks Mined",
        description:
          "Counts how many blocks were attributed to the pool in each rolling window. For minor pools, this is usually the most useful activity view.",
        chart: {
          title: `${name} blocks mined`,
          unit: units.blocks,
          defaultType: line,
          series: createMinorPoolBlocksMinedSeries(pool),
        },
      },
    ],
  };
}

export const miningPoolsSection = {
  title: "Mining Pools",
  description:
    "Mining pools coordinate miners so they can find blocks more steadily and split rewards. Pool charts show which pools are producing blocks, how their share changes, and how much BTC is paid to pools that are large enough to track in detail.",
  children: [
    {
      title: "Major",
      description:
        "Major pools have enough historical activity to track dominance, blocks mined, and rewards. This makes them useful for studying mining concentration and how block production changes over time.",
      children: [
        {
          title: "Dominance",
          description:
            "Compares the rolling monthly block-production share of all major pools. This is the clearest overview of mining-pool concentration.",
          chart: {
            title: "Major pool dominance",
            unit: units.percent,
            series: majorPoolDominanceSeries,
          },
        },
        {
          title: "Blocks Mined",
          description:
            "Compares rolling monthly blocks mined by major pools. This shows the raw block counts behind dominance percentages.",
          chart: {
            title: "Major pool blocks mined",
            unit: units.blocks,
            series: majorPoolBlocksMinedSeries,
          },
        },
        {
          title: "Rewards",
          description:
            "Compares rolling monthly BTC rewards earned by major pools. Rewards include both subsidy and transaction fees.",
          chart: {
            title: "Major pool rewards",
            unit: units.btc,
            series: majorPoolRewardsSeries,
          },
        },
        ...majorPools.map(createMajorPoolSection),
      ],
    },
    {
      title: "Minor",
      description:
        "Minor pools are smaller or less persistent pools. They matter because the long tail shows how broad or narrow block production is beyond the largest names, even when each pool is individually small.",
      children: minorPools.map(createMinorPoolSection),
    },
  ],
};
