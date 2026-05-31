/** @import { HeatmapTooltipFn } from "./types.js" */

import { numberToShortUSFormat } from "../../scripts/utils/format.js";

/** @satisfies {HeatmapTooltipFn} */
export const defaultTooltip = ({ grid, col, row }) => {
  const dateRange = grid.getDateIndexRange(col);
  const yRange = grid.getYRange(row);
  const value = grid.getValue(col, row);

  const from = grid.dates[dateRange.start] ?? "";
  const to = grid.dates[dateRange.end] ?? from;
  const date = from === to ? from : `${from} to ${to}`;

  return [
    date,
    `y ${formatNumber(yRange.start)} to ${formatNumber(yRange.end)}`,
    `value ${formatNumber(value)}`,
  ].join("\n");
};

/** @param {number} value */
function formatNumber(value) {
  return numberToShortUSFormat(value);
}
