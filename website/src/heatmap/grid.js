/** @import { HeatmapGrid, HeatmapGridFactory, HeatmapRange } from "./types.js" */

/**
 * Generic date/y binning with average merge semantics.
 *
 * @param {Object} args
 * @param {number} args.yStart
 * @param {number} args.yEnd
 * @param {number} [args.minCellSize]
 * @param {number} [args.maxCols]
 * @param {number} [args.nativeRows]
 * @returns {HeatmapGridFactory}
 */
export function createAverageGrid({
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
      const maxByCol = new Float64Array(cols);
      const cumulativeMaxByCol = new Float64Array(cols);
      let cumulativeMaxDirty = true;
      const ySpan = yEnd - yStart;

      /** @param {number} dateIndex */
      function toCol(dateIndex) {
        if (dateIndex < 0 || dateIndex >= dates.length) return undefined;
        return clamp(
          Math.floor((dateIndex * cols) / dates.length),
          0,
          cols - 1,
        );
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
       * @param {number} col
       * @param {number} y
       * @param {number} value
       */
      function addValue(col, y, value) {
        if (!Number.isFinite(value)) return undefined;
        const row = toRow(y);
        if (row === undefined) return undefined;
        const index = row * cols + col;
        sums[index] += value;
        counts[index] += 1;
        maxByCol[col] = Math.max(maxByCol[col], sums[index] / counts[index]);
        cumulativeMaxDirty = true;
        return col;
      }

      /** @type {HeatmapGrid} */
      const grid = {
        dates,
        cols,
        rows,
        add(dateIndex, points) {
          const col = toCol(dateIndex);
          if (col === undefined) return undefined;
          let dirty = false;
          if (points.kind === "implicit") {
            for (let i = 0; i < points.values.length; i++) {
              dirty =
                addValue(
                  col,
                  points.yStart + i * points.yStep,
                  points.values[i],
                ) !== undefined || dirty;
            }
          } else {
            const length = Math.min(points.y.length, points.values.length);
            for (let i = 0; i < length; i++) {
              dirty =
                addValue(col, points.y[i], points.values[i]) !== undefined ||
                dirty;
            }
          }
          return dirty ? col : undefined;
        },
        getValue(col, row) {
          if (col < 0 || col >= cols || row < 0 || row >= rows) {
            return Number.NaN;
          }
          const index = row * cols + col;
          return counts[index] ? sums[index] / counts[index] : Number.NaN;
        },
        getMaxValue(col = cols - 1) {
          if (cumulativeMaxDirty) {
            let max = 0;
            for (let c = 0; c < cols; c++) {
              max = Math.max(max, maxByCol[c]);
              cumulativeMaxByCol[c] = max;
            }
            cumulativeMaxDirty = false;
          }
          return cumulativeMaxByCol[clamp(col, 0, cols - 1)] ?? 0;
        },
        getDateIndexRange(col) {
          if (col < 0 || col >= cols || dates.length === 0) {
            return emptyRange();
          }
          const start = Math.ceil((col * dates.length) / cols);
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

/**
 * @param {number} value
 * @param {number} min
 * @param {number} max
 */
function clamp(value, min, max) {
  return Math.min(max, Math.max(min, value));
}

/** @returns {HeatmapRange} */
function emptyRange() {
  return { start: Number.NaN, end: Number.NaN };
}
