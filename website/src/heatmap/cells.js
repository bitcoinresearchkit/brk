/** @import { HeatmapCells, HeatmapGrid, HeatmapRange } from "./types.js" */

/**
 * @param {number} value
 * @param {number} min
 * @param {number} max
 */
function clamp(value, min, max) {
  return Math.min(max, Math.max(min, value));
}

/**
 * Generic date/y binning with average merge semantics.
 *
 * @param {Object} args
 * @param {number} args.yStart
 * @param {number} args.yEnd
 * @param {number} [args.minCellSize]
 * @param {number} [args.maxCols]
 * @param {number} [args.nativeRows]
 * @returns {HeatmapCells}
 */
export function createAverageCells({
  yStart,
  yEnd,
  minCellSize = 1,
  maxCols = Number.POSITIVE_INFINITY,
  nativeRows = Number.POSITIVE_INFINITY,
}) {
  return {
    create({ dates, width, height }) {
      const cols = Math.max(
        1,
        Math.min(
          dates.length || 1,
          maxCols,
          Math.floor(width / minCellSize) || 1,
        ),
      );
      const rows = Math.max(
        1,
        Math.min(nativeRows, Math.floor(height / minCellSize) || 1),
      );
      const sums = new Float64Array(cols * rows);
      const counts = new Uint32Array(cols * rows);
      const ySpan = yEnd - yStart;

      /** @param {number} dateIndex */
      function toCol(dateIndex) {
        if (dateIndex < 0 || dateIndex >= dates.length) return undefined;
        return clamp(Math.floor((dateIndex * cols) / dates.length), 0, cols - 1);
      }

      /** @param {number} y */
      function toRow(y) {
        if (!Number.isFinite(y) || !Number.isFinite(ySpan) || ySpan <= 0) {
          return undefined;
        }
        const t = (y - yStart) / ySpan;
        if (t < 0 || t > 1) return undefined;
        return rows - 1 - clamp(Math.floor(t * rows), 0, rows - 1);
      }

      /**
       * @param {number} dateIndex
       * @param {number} y
       * @param {number} value
       */
      function addValue(dateIndex, y, value) {
        if (!Number.isFinite(value)) return undefined;
        const col = toCol(dateIndex);
        const row = toRow(y);
        if (col === undefined || row === undefined) return undefined;
        const index = row * cols + col;
        sums[index] += value;
        counts[index] += 1;
        return col;
      }

      /** @type {HeatmapGrid} */
      const grid = {
        dates,
        cols,
        rows,
        add(dateIndex, points) {
          let dirty;
          if (points.kind === "implicit") {
            for (let i = 0; i < points.values.length; i++) {
              const col = addValue(
                dateIndex,
                points.yStart + i * points.yStep,
                points.values[i],
              );
              dirty = col ?? dirty;
            }
          } else {
            const length = Math.min(points.y.length, points.values.length);
            for (let i = 0; i < length; i++) {
              const col = addValue(dateIndex, points.y[i], points.values[i]);
              dirty = col ?? dirty;
            }
          }
          return dirty;
        },
        getValue(col, row) {
          if (col < 0 || col >= cols || row < 0 || row >= rows) {
            return Number.NaN;
          }
          const index = row * cols + col;
          return counts[index] ? sums[index] / counts[index] : Number.NaN;
        },
        getDateIndexRange(col) {
          if (col < 0 || col >= cols || dates.length === 0) {
            return emptyRange();
          }
          const start = Math.floor((col * dates.length) / cols);
          const end = Math.floor(((col + 1) * dates.length - 1) / cols);
          return { start, end: clamp(end, start, dates.length - 1) };
        },
        getYRange(row) {
          if (row < 0 || row >= rows || ySpan <= 0) return emptyRange();
          const start = yStart + ((rows - row - 1) / rows) * ySpan;
          const end = yStart + ((rows - row) / rows) * ySpan;
          return { start, end };
        },
      };

      return grid;
    },
  };
}

/** @returns {HeatmapRange} */
function emptyRange() {
  return { start: Number.NaN, end: Number.NaN };
}
