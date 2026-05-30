/**
 * @typedef {Object} HeatmapDataSource
 * @property {(signal: AbortSignal) => Promise<string[]>} list
 * @property {(date: string, signal: AbortSignal) => Promise<unknown>} fetch
 *
 * @typedef {Object} HeatmapCells
 * @property {(args: { dates: string[], width: number, height: number }) => unknown} create
 * @property {(grid: unknown, dateIndex: number, snapshot: unknown) => number | undefined} add
 * @property {(grid: unknown, col: number, row: number) => unknown} getValue
 *
 * @typedef {Object} HeatmapColorContext
 * @property {boolean} dark
 * @property {unknown} grid
 * @property {number} col
 * @property {number} row
 *
 * @typedef {(value: unknown, context: HeatmapColorContext) => number} HeatmapColorFn
 *
 * @typedef {Object} HeatmapTooltipContext
 * @property {unknown} grid
 * @property {number} col
 * @property {number} row
 *
 * @typedef {(context: HeatmapTooltipContext) => string} HeatmapTooltipFn
 */

export {};
