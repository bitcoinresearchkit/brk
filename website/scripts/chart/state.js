import { readParam, writeParam } from "../utils/url.js";
import { readStored, writeToStorage } from "../utils/storage.js";

/**
 * @typedef {{ from: number | null, to: number | null }} Range
 */

const RANGES_KEY = "chart-ranges";
const RANGE_SEP = "_";

/**
 * @param {Signals} signals
 */
export function createChartState(signals) {
  const index = signals.createPersistedSignal({
    storageKey: "chart-index",
    urlKey: "index",
    defaultValue: /** @type {ChartableIndexName} */ ("date"),
    serialize: (v) => v,
    deserialize: (s) => /** @type {ChartableIndexName} */ (s),
  });

  // Ranges stored per-index in localStorage only
  /** @type {Record<string, Range>} */
  let ranges = {};
  try {
    const stored = readStored(RANGES_KEY);
    if (stored) ranges = JSON.parse(stored);
  } catch {}

  // Initialize from URL if present
  const urlRange = readParam("range");
  if (urlRange) {
    const [from, to] = urlRange.split(RANGE_SEP).map(Number);
    if (!isNaN(from) && !isNaN(to)) {
      ranges[index()] = { from, to };
      writeToStorage(RANGES_KEY, JSON.stringify(ranges));
    }
  }

  return {
    index,
    /** @returns {Range} */
    range: () => ranges[index()] ?? { from: null, to: null },
    /** @param {Range} value */
    setRange(value) {
      ranges[index()] = value;
      writeToStorage(RANGES_KEY, JSON.stringify(ranges));
      if (value.from !== null && value.to !== null) {
        // Round to 2 decimals for cleaner URLs
        const f = Math.floor(value.from * 100) / 100;
        const t = Math.floor(value.to * 100) / 100;
        writeParam("range", `${f}${RANGE_SEP}${t}`);
      } else {
        writeParam("range", null);
      }
    },
  };
}
