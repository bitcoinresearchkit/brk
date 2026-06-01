import { numberToShortUSFormat } from "../../../scripts/utils/format.js";

/**
 * @param {Object} [args]
 * @param {string} [args.valueLabel]
 * @param {string} [args.averageLabel]
 * @returns {HeatmapTooltipFn}
 */
export function defaultTooltip({
  valueLabel = "Value",
  averageLabel = "Avg value",
} = {}) {
  return ({ option, grid, col, row }) => {
    const dateRange = grid.getDateIndexRange(col);
    const yRange = grid.getYRange(row);
    const value = grid.getValue(col, row);
    const yLabel = option.axis?.y?.label ?? "y";
    const formatY = option.axis?.y?.format ?? formatNumber;
    const label = grid.getCount(col, row) > 1 ? averageLabel : valueLabel;

    const from = grid.dates[dateRange.start] ?? "";
    const to = grid.dates[dateRange.end] ?? from;
    const date = from === to ? from : `${from} to ${to}`;

    return [
      date,
      `${capitalize(yLabel)}: ${formatY(yRange.start)} to ${formatY(yRange.end)}`,
      `${label}: ${formatNumber(value)}`,
    ].join("\n");
  };
}

/** @param {number} value */
function formatNumber(value) {
  return numberToShortUSFormat(value);
}

/** @param {string} value */
function capitalize(value) {
  return value ? value[0].toUpperCase() + value.slice(1) : value;
}
