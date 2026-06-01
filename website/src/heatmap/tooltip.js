/** @import { HeatmapTooltipFn } from "./types.js" */

import { numberToShortUSFormat } from "../../scripts/utils/format.js";

/** @satisfies {HeatmapTooltipFn} */
export const defaultTooltip = ({ option, grid, col, row }) => {
  const dateRange = grid.getDateIndexRange(col);
  const yRange = grid.getYRange(row);
  const value = grid.getValue(col, row);
  const yLabel = option.axis?.y?.label ?? "y";
  const formatY = option.axis?.y?.format ?? formatNumber;

  const from = grid.dates[dateRange.start] ?? "";
  const to = grid.dates[dateRange.end] ?? from;
  const date = from === to ? from : `${from} to ${to}`;

  return [
    date,
    `${yLabel} ${formatY(yRange.start)} to ${formatY(yRange.end)}`,
    `value ${formatNumber(value)}`,
  ].join("\n");
};

/** @param {number} value */
function formatNumber(value) {
  return numberToShortUSFormat(value);
}
