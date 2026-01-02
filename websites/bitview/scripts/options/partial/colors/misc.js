/** Miscellaneous color mappings for DCA and averages */

/**
 * Moving average period colors
 * Format: [periodId, days, colorName]
 * @type {readonly [string, number, ColorName][]}
 */
export const averageColors = [
  ["1w", 7, "red"],
  ["8d", 8, "orange"],
  ["13d", 13, "amber"],
  ["21d", 21, "yellow"],
  ["1m", 30, "lime"],
  ["34d", 34, "green"],
  ["55d", 55, "emerald"],
  ["89d", 89, "teal"],
  ["144d", 144, "cyan"],
  ["200d", 200, "sky"],
  ["1y", 365, "blue"],
  ["2y", 730, "indigo"],
  ["200w", 1400, "violet"],
  ["4y", 1460, "purple"],
];

/**
 * DCA class colors by year
 * Format: [year, colorName, defaultActive]
 * @type {readonly [number, ColorName, boolean][]}
 */
export const dcaColors = [
  [2015, "pink", false],
  [2016, "red", false],
  [2017, "orange", true],
  [2018, "yellow", true],
  [2019, "green", true],
  [2020, "teal", true],
  [2021, "sky", true],
  [2022, "blue", true],
  [2023, "purple", true],
  [2024, "fuchsia", true],
  [2025, "pink", true],
];
