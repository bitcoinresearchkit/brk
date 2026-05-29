import { INFERNO_LUT } from "./lut.js";

const COLS = 100;
const ROWS = 100;

export const demoSource = {
  cols: COLS,
  rows: ROWS,
  getColor,
};

/**
 * @param {number} col
 * @param {number} row
 */
function getColor(col, row) {
  const x = col / (COLS - 1);
  const y = row / (ROWS - 1);
  const ridge = Math.exp(-((y - (0.75 - x * 0.45)) ** 2) / 0.01);
  const blob = Math.exp(
    -(((x - 0.72) ** 2) / 0.018 + ((y - 0.28) ** 2) / 0.028),
  );
  const floor = x * 0.18 + (1 - y) * 0.12;
  const i = Math.min(
    255,
    Math.max(0, ((ridge * 0.65 + blob * 0.45 + floor) * 255) | 0),
  );
  return INFERNO_LUT[i];
}
