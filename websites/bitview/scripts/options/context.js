import { s, fromBlockCount, fromBitcoin, fromBlockSize } from "./series.js";
import { createPriceLine, createPriceLines, line } from "./constants.js";

/**
 * Create a context object with all dependencies for building partial options
 * @param {Object} args
 * @param {Colors} args.colors
 * @param {BrkClient} args.brk
 * @returns {PartialContext}
 */
export function createContext({ colors, brk }) {
  const constants = brk.tree.constants;

  return {
    colors,
    brk,

    // Series helpers
    s,
    fromBlockCount: (pattern, title, color) =>
      fromBlockCount(colors, pattern, title, color),
    fromBitcoin: (pattern, title, color) =>
      fromBitcoin(colors, pattern, title, color),
    fromBlockSize: (pattern, title, color) =>
      fromBlockSize(colors, pattern, title, color),

    createPriceLine: (args) => createPriceLine({ constants, colors, ...args }),
    createPriceLines: (args) =>
      createPriceLines({ constants, colors, ...args }),
    line: (args) => line({ colors, ...args }),
  };
}
