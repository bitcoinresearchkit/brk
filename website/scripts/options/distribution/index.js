/**
 * Cohort module - exports all cohort-related functionality
 *
 * Folder builders compose sections from building blocks:
 * - holdings.js: Supply, UTXO Count, Address Count
 * - valuation.js: Realized Cap, Market Cap, MVRV
 * - prices.js: Realized Price, ratios
 * - cost-basis.js: Cost basis percentiles
 * - profitability.js: Unrealized/Realized P&L, Invested Capital
 * - activity.js: SOPR, Volume, Lifespan
 */

import {
  formatCohortTitle,
  satsBtcUsd,
  satsBtcUsdFullTree,
} from "../shared.js";
import {
  ROLLING_WINDOWS,
  line,
  baseline,
  percentRatio,
  sumsTreeBaseline,
  rollingPercentRatioTree,
} from "../series.js";
import { Unit } from "../../utils/units.js";
import { colors } from "../../utils/colors.js";

// Section builders
import {
  createHoldingsSection,
  createHoldingsSectionAll,
  createHoldingsSectionAddress,
  createHoldingsSectionAddressAmount,
  createHoldingsSectionWithProfitLoss,
  createHoldingsSectionWithRelative,
  createHoldingsSectionWithOwnSupply,
  createGroupedHoldingsSection,
  createGroupedHoldingsSectionAddress,
  createGroupedHoldingsSectionAddressAmount,
  createGroupedHoldingsSectionWithRelative,
  createGroupedHoldingsSectionWithOwnSupply,
} from "./holdings.js";
import {
  createValuationSection,
  createValuationSectionFull,
  createGroupedValuationSection,
  createGroupedValuationSectionWithOwnMarketCap,
} from "./valuation.js";
import {
  createPricesSectionFull,
  createPricesSectionBasic,
  createGroupedPricesSection,
  createGroupedPricesSectionFull,
} from "./prices.js";
import {
  createCostBasisSectionWithPercentiles,
  createGroupedCostBasisSectionWithPercentiles,
} from "./cost-basis.js";
import {
  createProfitabilitySection,
  createProfitabilitySectionAll,
  createProfitabilitySectionFull,
  createProfitabilitySectionWithProfitLoss,
  createProfitabilitySectionWithInvestedCapitalPct,
  createProfitabilitySectionLongTerm,
  createGroupedProfitabilitySection,
  createGroupedProfitabilitySectionWithProfitLoss,
  createGroupedProfitabilitySectionWithNupl,
  createGroupedProfitabilitySectionWithInvestedCapitalPct,
} from "./profitability.js";
import {
  createActivitySection,
  createActivitySectionWithAdjusted,
  createActivitySectionWithActivity,
  createGroupedActivitySection,
  createGroupedActivitySectionWithActivity,
  createActivitySectionMinimal,
  createGroupedActivitySectionMinimal,
} from "./activity.js";

// Re-export data builder
export { buildCohortData } from "./data.js";

// ============================================================================
// Single Cohort Folder Builders
// ============================================================================

/**
 * All folder: for the special "All" cohort
 * @param {CohortAll} cohort
 * @returns {PartialOptionsGroup}
 */
export function createCohortFolderAll(cohort) {
  const title = formatCohortTitle(cohort.title);
  return {
    name: cohort.name || "all",
    tree: [
      ...createHoldingsSectionAll({ cohort, title }),
      createValuationSectionFull({ cohort, title }),
      createPricesSectionFull({ cohort, title }),
      createCostBasisSectionWithPercentiles({ cohort, title }),
      createProfitabilitySectionAll({ cohort, title }),
      createActivitySectionWithAdjusted({ cohort, title }),
    ],
  };
}

/**
 * Full folder: adjustedSopr + percentiles + RelToMarketCap
 * @param {CohortFull} cohort
 * @returns {PartialOptionsGroup}
 */
export function createCohortFolderFull(cohort) {
  const title = formatCohortTitle(cohort.title);
  return {
    name: cohort.name || "all",
    tree: [
      ...createHoldingsSectionWithRelative({ cohort, title }),
      createValuationSectionFull({ cohort, title }),
      createPricesSectionFull({ cohort, title }),
      createCostBasisSectionWithPercentiles({ cohort, title }),
      createProfitabilitySectionFull({ cohort, title }),
      createActivitySectionWithAdjusted({ cohort, title }),
    ],
  };
}

/**
 * Adjusted folder: adjustedSopr only, no percentiles
 * @param {CohortWithAdjusted} cohort
 * @returns {PartialOptionsGroup}
 */
export function createCohortFolderWithAdjusted(cohort) {
  const title = formatCohortTitle(cohort.title);
  return {
    name: cohort.name || "all",
    tree: [
      ...createHoldingsSectionWithOwnSupply({ cohort, title }),
      createValuationSection({ cohort, title }),
      createPricesSectionBasic({ cohort, title }),
      createProfitabilitySectionWithInvestedCapitalPct({ cohort, title }),
      createActivitySectionWithActivity({ cohort, title }),
    ],
  };
}

/**
 * Folder for cohorts with nupl + percentiles
 * @param {CohortWithNuplPercentiles} cohort
 * @returns {PartialOptionsGroup}
 */
export function createCohortFolderWithNupl(cohort) {
  const title = formatCohortTitle(cohort.title);
  return {
    name: cohort.name || "all",
    tree: [
      ...createHoldingsSectionWithRelative({ cohort, title }),
      createValuationSectionFull({ cohort, title }),
      createPricesSectionFull({ cohort, title }),
      createCostBasisSectionWithPercentiles({ cohort, title }),
      createProfitabilitySection({ cohort, title }),
      createActivitySection({ cohort, title }),
    ],
  };
}

/**
 * LongTerm folder: has own market cap + NUPL + peak regret + P/L ratio
 * @param {CohortLongTerm} cohort
 * @returns {PartialOptionsGroup}
 */
export function createCohortFolderLongTerm(cohort) {
  const title = formatCohortTitle(cohort.title);
  return {
    name: cohort.name || "all",
    tree: [
      ...createHoldingsSectionWithRelative({ cohort, title }),
      createValuationSectionFull({ cohort, title }),
      createPricesSectionFull({ cohort, title }),
      createCostBasisSectionWithPercentiles({ cohort, title }),
      createProfitabilitySectionLongTerm({ cohort, title }),
      createActivitySection({ cohort, title }),
    ],
  };
}

/**
 * Age range folder: no nupl
 * @param {CohortAgeRange} cohort
 * @returns {PartialOptionsGroup}
 */
export function createCohortFolderAgeRange(cohort) {
  const title = formatCohortTitle(cohort.title);
  return {
    name: cohort.name || "all",
    tree: [
      ...createHoldingsSectionWithOwnSupply({ cohort, title }),
      createValuationSection({ cohort, title }),
      createPricesSectionBasic({ cohort, title }),
      createProfitabilitySectionWithInvestedCapitalPct({ cohort, title }),
      createActivitySectionWithActivity({ cohort, title }),
    ],
  };
}

/**
 * Age range folder with matured supply
 * @param {CohortAgeRangeWithMatured} cohort
 * @returns {PartialOptionsGroup}
 */
export function createCohortFolderAgeRangeWithMatured(cohort) {
  const folder = createCohortFolderAgeRange(cohort);
  const title = formatCohortTitle(cohort.title);
  folder.tree.push({
    name: "Matured",
    tree: satsBtcUsdFullTree({
      pattern: cohort.matured,
      title: title("Matured Supply"),
    }),
  });
  return folder;
}

/**
 * Basic folder WITH RelToMarketCap
 * @param {CohortBasicWithMarketCap} cohort
 * @returns {PartialOptionsGroup}
 */
export function createCohortFolderBasicWithMarketCap(cohort) {
  const title = formatCohortTitle(cohort.title);
  return {
    name: cohort.name || "all",
    tree: [
      ...createHoldingsSection({ cohort, title }),
      createValuationSection({ cohort, title }),
      createPricesSectionBasic({ cohort, title }),
      createProfitabilitySection({ cohort, title }),
      createActivitySectionMinimal({ cohort, title }),
    ],
  };
}


/**
 * Address folder: like basic but with address count
 * @param {CohortAddr} cohort
 * @returns {PartialOptionsGroup}
 */
export function createCohortFolderAddress(cohort) {
  const title = formatCohortTitle(cohort.title);
  return {
    name: cohort.name || "all",
    tree: [
      ...createHoldingsSectionAddress({ cohort, title }),
      createValuationSection({ cohort, title }),
      createPricesSectionBasic({ cohort, title }),
      createProfitabilitySectionWithProfitLoss({ cohort, title }),
      createActivitySectionMinimal({ cohort, title }),
    ],
  };
}

/**
 * Folder for cohorts WITHOUT relative section
 * @param {CohortWithoutRelative} cohort
 * @returns {PartialOptionsGroup}
 */
export function createCohortFolderWithoutRelative(cohort) {
  const title = formatCohortTitle(cohort.title);
  return {
    name: cohort.name || "all",
    tree: [
      ...createHoldingsSectionWithProfitLoss({ cohort, title }),
      createValuationSection({ cohort, title }),
      createPricesSectionBasic({ cohort, title }),
      createProfitabilitySectionWithProfitLoss({ cohort, title }),
      createActivitySectionMinimal({ cohort, title }),
    ],
  };
}

/**
 * Address amount cohort folder: has NUPL + addrCount
 * @param {AddrCohortObject} cohort
 * @returns {PartialOptionsGroup}
 */
export function createAddressCohortFolder(cohort) {
  const title = formatCohortTitle(cohort.title);
  return {
    name: cohort.name || "all",
    tree: [
      ...createHoldingsSectionAddressAmount({ cohort, title }),
      createValuationSection({ cohort, title }),
      createPricesSectionBasic({ cohort, title }),
      createProfitabilitySection({ cohort, title }),
      createActivitySectionMinimal({ cohort, title }),
    ],
  };
}

// ============================================================================
// Grouped Cohort Folder Builders
// ============================================================================

/**
 * @param {CohortGroupWithAdjusted} args
 * @returns {PartialOptionsGroup}
 */
export function createGroupedCohortFolderWithAdjusted({
  name,
  title: groupTitle,
  list,
  all,
}) {
  const title = formatCohortTitle(groupTitle);
  return {
    name: name || "all",
    tree: [
      ...createGroupedHoldingsSectionWithOwnSupply({ list, all, title }),
      createGroupedValuationSection({ list, all, title }),
      createGroupedPricesSection({ list, all, title }),
      createGroupedProfitabilitySectionWithInvestedCapitalPct({
        list,
        all,
        title,
      }),
      createGroupedActivitySectionWithActivity({ list, all, title }),
    ],
  };
}

/**
 * @param {CohortGroupWithNuplPercentiles} args
 * @returns {PartialOptionsGroup}
 */
export function createGroupedCohortFolderWithNupl({
  name,
  title: groupTitle,
  list,
  all,
}) {
  const title = formatCohortTitle(groupTitle);
  return {
    name: name || "all",
    tree: [
      ...createGroupedHoldingsSectionWithRelative({ list, all, title }),
      createGroupedValuationSectionWithOwnMarketCap({ list, all, title }),
      createGroupedPricesSectionFull({ list, all, title }),
      createGroupedCostBasisSectionWithPercentiles({ list, all, title }),
      createGroupedProfitabilitySectionWithNupl({ list, all, title }),
      createGroupedActivitySection({ list, all, title }),
    ],
  };
}

/**
 * @param {CohortGroupAgeRange} args
 * @returns {PartialOptionsGroup}
 */
export function createGroupedCohortFolderAgeRange({
  name,
  title: groupTitle,
  list,
  all,
}) {
  const title = formatCohortTitle(groupTitle);
  return {
    name: name || "all",
    tree: [
      ...createGroupedHoldingsSectionWithOwnSupply({ list, all, title }),
      createGroupedValuationSection({ list, all, title }),
      createGroupedPricesSection({ list, all, title }),
      createGroupedProfitabilitySectionWithInvestedCapitalPct({
        list,
        all,
        title,
      }),
      createGroupedActivitySectionWithActivity({ list, all, title }),
    ],
  };
}

/**
 * @param {{ name: string, title: string, list: readonly CohortAgeRangeWithMatured[], all: CohortAll }} args
 * @returns {PartialOptionsGroup}
 */
export function createGroupedCohortFolderAgeRangeWithMatured({
  name,
  title: groupTitle,
  list,
  all,
}) {
  const folder = createGroupedCohortFolderAgeRange({
    name,
    title: groupTitle,
    list,
    all,
  });
  const title = formatCohortTitle(groupTitle);
  folder.tree.push({
    name: "Matured",
    tree: ROLLING_WINDOWS.map((w) => ({
      name: w.name,
      title: title(`Matured Supply (${w.title})`),
      bottom: list.flatMap((cohort) =>
        satsBtcUsd({
          pattern: cohort.matured.sum[w.key],
          name: cohort.name,
          color: cohort.color,
        }),
      ),
    })),
  });
  return folder;
}

/**
 * @param {CohortGroupBasicWithMarketCap} args
 * @returns {PartialOptionsGroup}
 */
export function createGroupedCohortFolderBasicWithMarketCap({
  name,
  title: groupTitle,
  list,
  all,
}) {
  const title = formatCohortTitle(groupTitle);
  return {
    name: name || "all",
    tree: [
      ...createGroupedHoldingsSection({ list, all, title }),
      createGroupedValuationSection({ list, all, title }),
      createGroupedPricesSection({ list, all, title }),
      createGroupedProfitabilitySection({ list, all, title }),
      createGroupedActivitySectionMinimal({ list, all, title }),
    ],
  };
}


/**
 * @param {CohortGroupAddr} args
 * @returns {PartialOptionsGroup}
 */
export function createGroupedCohortFolderAddress({
  name,
  title: groupTitle,
  list,
  all,
}) {
  const title = formatCohortTitle(groupTitle);
  return {
    name: name || "all",
    tree: [
      ...createGroupedHoldingsSectionAddress({ list, all, title }),
      createGroupedValuationSection({ list, all, title }),
      createGroupedPricesSection({ list, all, title }),
      createGroupedProfitabilitySectionWithProfitLoss({
        list,
        all,
        title,
      }),
      createGroupedActivitySectionMinimal({ list, all, title }),
    ],
  };
}

/**
 * @param {AddrCohortGroupObject} args
 * @returns {PartialOptionsGroup}
 */
export function createGroupedAddressCohortFolder({
  name,
  title: groupTitle,
  list,
  all,
}) {
  const title = formatCohortTitle(groupTitle);
  return {
    name: name || "all",
    tree: [
      ...createGroupedHoldingsSectionAddressAmount({ list, all, title }),
      createGroupedValuationSection({ list, all, title }),
      createGroupedPricesSection({ list, all, title }),
      createGroupedProfitabilitySection({ list, all, title }),
      createGroupedActivitySectionMinimal({ list, all, title }),
    ],
  };
}

// ============================================================================
// UTXO Profitability Folder Builders
// ============================================================================

/**
 * @param {{ name: string, color: Color, pattern: RealizedSupplyPattern }} bucket
 * @returns {PartialOptionsGroup}
 */
function singleBucketFolder({ name, color, pattern }) {
  return {
    name,
    tree: [
      {
        name: "Supply",
        tree: [
          {
            name: "Value",
            title: `${name}: Supply`,
            bottom: [
              ...satsBtcUsd({ pattern: pattern.supply.all, name: "Total" }),
              ...satsBtcUsd({
                pattern: pattern.supply.sth,
                name: "STH",
                color: colors.term.short,
              }),
            ],
          },
          {
            name: "Change",
            tree: [
              {
                ...sumsTreeBaseline({
                  windows: pattern.supply.all.delta.absolute,
                  title: `${name}: Supply Change`,
                  unit: Unit.sats,
                }),
                name: "Change",
              },
              {
                ...rollingPercentRatioTree({
                  windows: pattern.supply.all.delta.rate,
                  title: `${name}: Supply Rate`,
                }),
                name: "Growth Rate",
              },
            ],
          },
        ],
      },
      {
        name: "Realized Cap",
        title: `${name}: Realized Cap`,
        bottom: [
          line({
            series: pattern.realizedCap.all,
            name: "Total",
            unit: Unit.usd,
          }),
          line({
            series: pattern.realizedCap.sth,
            name: "STH",
            color: colors.term.short,
            unit: Unit.usd,
          }),
        ],
      },
      {
        name: "NUPL",
        title: `${name}: NUPL`,
        bottom: [
          line({ series: pattern.nupl.ratio, name, color, unit: Unit.ratio }),
        ],
      },
    ],
  };
}

/**
 * @param {{ name: string, color: Color, pattern: RealizedSupplyPattern }[]} list
 * @param {string} titlePrefix
 * @returns {PartialOptionsTree}
 */
function groupedBucketCharts(list, titlePrefix) {
  return [
    {
      name: "Supply",
      tree: [
        {
          name: "All",
          title: `${titlePrefix}: Supply`,
          bottom: list.flatMap(({ name, color, pattern }) =>
            satsBtcUsd({ pattern: pattern.supply.all, name, color }),
          ),
        },
        {
          name: "STH",
          title: `${titlePrefix}: STH Supply`,
          bottom: list.flatMap(({ name, color, pattern }) =>
            satsBtcUsd({ pattern: pattern.supply.sth, name, color }),
          ),
        },
        {
          name: "Change",
          tree: [
            {
              name: "Change",
              tree: [
                {
                  name: "Compare",
                  title: `${titlePrefix}: Supply Change`,
                  bottom: ROLLING_WINDOWS.flatMap((w) =>
                    list.map(({ name, color, pattern }) =>
                      baseline({
                        series: pattern.supply.all.delta.absolute[w.key],
                        name: `${name} ${w.name}`,
                        color,
                        unit: Unit.sats,
                      }),
                    ),
                  ),
                },
                ...ROLLING_WINDOWS.map((w) => ({
                  name: w.name,
                  title: `${titlePrefix}: Supply Change (${w.title})`,
                  bottom: list.map(({ name, color, pattern }) =>
                    baseline({
                      series: pattern.supply.all.delta.absolute[w.key],
                      name,
                      color,
                      unit: Unit.sats,
                    }),
                  ),
                })),
              ],
            },
            {
              name: "Growth Rate",
              tree: [
                {
                  name: "Compare",
                  title: `${titlePrefix}: Supply Rate`,
                  bottom: ROLLING_WINDOWS.flatMap((w) =>
                    list.flatMap(({ name, color, pattern }) =>
                      percentRatio({
                        pattern: pattern.supply.all.delta.rate[w.key],
                        name: `${name} ${w.name}`,
                        color,
                      }),
                    ),
                  ),
                },
                ...ROLLING_WINDOWS.map((w) => ({
                  name: w.name,
                  title: `${titlePrefix}: Supply Rate (${w.title})`,
                  bottom: list.flatMap(({ name, color, pattern }) =>
                    percentRatio({
                      pattern: pattern.supply.all.delta.rate[w.key],
                      name,
                      color,
                    }),
                  ),
                })),
              ],
            },
          ],
        },
      ],
    },
    {
      name: "Realized Cap",
      tree: [
        {
          name: "All",
          title: `${titlePrefix}: Realized Cap`,
          bottom: list.map(({ name, color, pattern }) =>
            line({
              series: pattern.realizedCap.all,
              name,
              color,
              unit: Unit.usd,
            }),
          ),
        },
        {
          name: "STH",
          title: `${titlePrefix}: STH Realized Cap`,
          bottom: list.map(({ name, color, pattern }) =>
            line({
              series: pattern.realizedCap.sth,
              name,
              color,
              unit: Unit.usd,
            }),
          ),
        },
      ],
    },
    {
      name: "NUPL",
      title: `${titlePrefix}: NUPL`,
      bottom: list.map(({ name, color, pattern }) =>
        line({ series: pattern.nupl.ratio, name, color, unit: Unit.ratio }),
      ),
    },
  ];
}

/**
 * @param {{ range: { name: string, color: Color, pattern: RealizedSupplyPattern }[], profit: { name: string, color: Color, pattern: RealizedSupplyPattern }[], loss: { name: string, color: Color, pattern: RealizedSupplyPattern }[] }} args
 * @returns {PartialOptionsGroup}
 */
export function createUtxoProfitabilitySection({ range, profit, loss }) {
  return {
    name: "UTXO Profitability",
    tree: [
      {
        name: "Range",
        tree: [
          {
            name: "Compare",
            tree: groupedBucketCharts(range, "Profitability Range"),
          },
          ...range.map(singleBucketFolder),
        ],
      },
      {
        name: "In Profit",
        tree: [
          { name: "Compare", tree: groupedBucketCharts(profit, "In Profit") },
          ...profit.map(singleBucketFolder),
        ],
      },
      {
        name: "In Loss",
        tree: [
          { name: "Compare", tree: groupedBucketCharts(loss, "In Loss") },
          ...loss.map(singleBucketFolder),
        ],
      },
    ],
  };
}
