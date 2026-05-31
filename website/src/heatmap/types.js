/**
 * @typedef {Object} HeatmapImplicitPoints
 * @property {"implicit"} kind
 * @property {number} yStart
 * @property {number} yStep
 * @property {ArrayLike<number>} values
 *
 * @typedef {Object} HeatmapExplicitPoints
 * @property {"explicit"} kind
 * @property {ArrayLike<number>} y
 * @property {ArrayLike<number>} values
 *
 * @typedef {HeatmapImplicitPoints | HeatmapExplicitPoints} HeatmapPoints
 *
 * @typedef {Object} HeatmapPointSource
 * @property {(date: string, signal: AbortSignal, onPoints?: (points: HeatmapPoints) => void) => Promise<HeatmapPoints>} fetch
 *
 * @typedef {Object} HeatmapRange
 * @property {number} start
 * @property {number} end
 *
 * @typedef {Object} HeatmapGrid
 * @property {readonly string[]} dates
 * @property {number} cols
 * @property {number} rows
 * @property {(dateIndex: number, points: HeatmapPoints) => number | undefined} add
 * @property {(col: number, row: number) => number} getValue
 * @property {(col?: number) => number} getMaxValue
 * @property {(col: number) => HeatmapRange} getDateIndexRange
 * @property {(row: number) => HeatmapRange} getYRange
 *
 * @typedef {Object} HeatmapGridFactory
 * @property {(args: { dates: readonly string[], width: number, height: number }) => HeatmapGrid} create
 *
 * @typedef {(value: number, context: { dark: boolean, grid: HeatmapGrid, col: number, row: number }) => number} HeatmapColorFn
 * @typedef {(context: { grid: HeatmapGrid, col: number, row: number }) => string} HeatmapTooltipFn
 */

export {};
