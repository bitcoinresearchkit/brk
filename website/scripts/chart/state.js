import {
  readParam,
  readNumberParam,
  writeParam,
} from "../utils/url.js";

/**
 * @typedef {{ from: number | null, to: number | null }} Range
 */

const INDEX_KEY = "chart-index";
const RANGES_KEY = "chart-ranges";

/**
 * @param {Signals} signals
 */
export function createChartState(signals) {
  /** @type {Record<string, Range>} */
  let ranges = {};
  try {
    const stored = localStorage.getItem(RANGES_KEY);
    if (stored) ranges = JSON.parse(stored);
  } catch {}

  const saveRanges = () => {
    try {
      localStorage.setItem(RANGES_KEY, JSON.stringify(ranges));
    } catch {}
  };

  // Read index: URL > localStorage > default
  /** @type {ChartableIndexName} */
  const defaultIndex = "date";
  const urlIndex = readParam("index");
  /** @type {ChartableIndexName} */
  let initialIndex = defaultIndex;
  if (urlIndex) {
    initialIndex = /** @type {ChartableIndexName} */ (urlIndex);
  } else {
    try {
      const stored = localStorage.getItem(INDEX_KEY);
      if (stored) initialIndex = /** @type {ChartableIndexName} */ (stored);
    } catch {}
  }

  // Read range: URL > localStorage (per index)
  const urlFrom = readNumberParam("from");
  const urlTo = readNumberParam("to");
  const storedRange = ranges[initialIndex] ?? { from: null, to: null };
  const initialRange = {
    from: urlFrom ?? storedRange.from,
    to: urlTo ?? storedRange.to,
  };
  // Save URL range to localStorage if present
  if (urlFrom !== null || urlTo !== null) {
    ranges[initialIndex] = initialRange;
    saveRanges();
  }

  const index = signals.createSignal(/** @type {ChartableIndexName} */ (initialIndex));
  const currentRange = signals.createSignal(initialRange);

  // Save index changes to localStorage + URL
  signals.createEffect(index, (value) => {
    try {
      localStorage.setItem(INDEX_KEY, value);
    } catch {}
    writeParam("index", value !== defaultIndex ? value : null);
  });

  // When index changes, switch to that index's saved range
  signals.createEffect(index, (i) => {
    const range = ranges[i] ?? { from: null, to: null };
    currentRange.set(range);
    // Update URL with new range
    writeParam("from", range.from !== null ? String(range.from) : null);
    writeParam("to", range.to !== null ? String(range.to) : null);
  });

  return {
    index,
    /** @type {Accessor<Range>} */
    range: currentRange,
    /**
     * @param {Range} value
     */
    setRange(value) {
      const i = index();
      ranges[i] = value;
      currentRange.set(value);
      saveRanges();
      writeParam("from", value.from !== null ? String(value.from) : null);
      writeParam("to", value.to !== null ? String(value.to) : null);
    },
  };
}
