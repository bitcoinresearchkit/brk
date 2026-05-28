import { createHeader } from "../../scripts/utils/dom.js";
import { heatmapElement } from "../../scripts/utils/elements.js";
import { debounce, next } from "../../scripts/utils/timing.js";

/**
 * @param {HeatmapOption} option
 */
export async function init(option) {
  const { headerElement, headingElement } = createHeader();
  headingElement.innerHTML = option.title;
  heatmapElement.append(headerElement);

  const canvas = document.createElement("canvas");
  heatmapElement.append(canvas);
  await next();

  let renderer = createRenderer(canvas);

  function randomColor() {
    // Inferno-ish: just random for now
    const r = (Math.random() * 255) | 0;
    const g = (Math.random() * 100) | 0;
    const b = (Math.random() * 200) | 0;
    return 0xff000000 | (b << 16) | (g << 8) | r; // ABGR
  }

  /**
   * @param {number} col
   * @param {number} row
   */
  function getColor(col, row) {
    return randomColor();
  }

  function render() {
    renderer.paint(100, 100, getColor);
  }

  const { width, height } = canvas.getBoundingClientRect();
  if (renderer.resize(width, height)) {
    render();
  }

  new ResizeObserver(
    debounce(() => {
      const { width, height } = canvas.getBoundingClientRect();
      if (width && height && renderer.resize(width, height)) {
        render();
      }
    }, 1000),
  ).observe(heatmapElement);
}

/** @param {HTMLCanvasElement} */
export function createRenderer(canvas) {
  const context = canvas.getContext("2d");
  if (!context) throw "Expected context from canvas";
  let width = 0;
  let height = 0;
  let imageData = new ImageData(1, 1);
  let buffer = new Uint32Array();

  return {
    /**
     * @param {number} w
     * @param {number} h
     * @returns {boolean} wether the canvas was actually resized (true) or not (false)
     */
    resize(w, h) {
      if (w == width && h == height) return false;
      const bound = canvas.getBoundingClientRect();
      width = Math.floor(Math.min(w, bound.width));
      height = Math.floor(Math.min(h, bound.height));
      canvas.width = width;
      canvas.height = height;
      imageData = context.createImageData(width, height);
      buffer = new Uint32Array(imageData.data.buffer);
      return true;
    },

    get width() {
      return width;
    },
    get height() {
      return height;
    },

    /**
     * Full repaint: iterate all cells, colorize, one blit.
     * @param {number} cols
     * @param {number} rows
     * @param {(col: number, row: number) => number} getColor - returns ABGR uint32
     */
    paint(cols, rows, getColor) {
      const colX = new Int32Array(cols + 1);
      for (let c = 0; c <= cols; c++) colX[c] = ((c * width) / cols + 0.5) | 0;
      const rowY = new Int32Array(rows + 1);
      for (let r = 0; r <= rows; r++) rowY[r] = ((r * height) / rows + 0.5) | 0;

      for (let c = 0; c < cols; c++) {
        const x0 = colX[c];
        const x1 = colX[c + 1];
        for (let r = 0; r < rows; r++) {
          const color = getColor(c, r);
          const y0 = rowY[r];
          const y1 = rowY[r + 1];
          for (let y = y0; y < y1; y++) {
            buffer.fill(color, y * width + x0, y * width + x1);
          }
        }
      }
      context.putImageData(imageData, 0, 0);
    },

    /**
     * Incremental repaint: only dirty columns, blit each separately.
     * @param {number} cols
     * @param {number} rows
     * @param {Iterable<number>} dirty
     * @param {(col: number, row: number) => number} getColor
     */
    paintCols(cols, rows, dirty, getColor) {
      const colW = width / cols;
      const rowH = height / rows;
      for (const c of dirty) {
        const x0 = (c * colW) | 0;
        const x1 = Math.min(width, ((c + 1) * colW) | 0);
        for (let r = 0; r < rows; r++) {
          const color = getColor(c, r);
          const y0 = (r * rowH) | 0;
          const y1 = Math.min(height, ((r + 1) * rowH) | 0);
          for (let y = y0; y < y1; y++) {
            buffer.fill(color, y * width + x0, y * width + x1);
          }
        }
        context.putImageData(imageData, 0, 0, x0, 0, x1 - x0, height);
      }
    },
  };
}
