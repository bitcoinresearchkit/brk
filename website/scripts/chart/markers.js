import { throttle } from "../utils/timing.js";

/**
 * @param {Object} args
 * @param {IChartApi} args.chart
 * @param {Accessor<Set<AnySeries>>} args.seriesList
 * @param {Colors} args.colors
 * @param {(value: number) => string} args.formatValue
 */
export function createMinMaxMarkers({ chart, seriesList, colors, formatValue }) {
  /** @type {Set<AnySeries>} */
  const prevMarkerSeries = new Set();

  function update() {
    const timeScale = chart.timeScale();
    const width = timeScale.width();
    const range = timeScale.getVisibleRange();
    if (!range) return;

    const tLeft = timeScale.coordinateToTime(30);
    const tRight = timeScale.coordinateToTime(width - 30);
    const t0 = /** @type {number} */ (tLeft ?? range.from);
    const t1 = /** @type {number} */ (tRight ?? range.to);
    const color = colors.gray();

    /** @type {Map<number, { minV: number, minT: Time, minS: AnySeries, maxV: number, maxT: Time, maxS: AnySeries }>} */
    const byPane = new Map();

    for (const series of seriesList()) {
      if (!series.active() || !series.hasData()) continue;

      const data = series.getData();
      const len = data.length;
      if (!len) continue;

      // Binary search for start
      let lo = 0, hi = len;
      while (lo < hi) {
        const mid = (lo + hi) >>> 1;
        if (/** @type {number} */ (data[mid].time) < t0) lo = mid + 1;
        else hi = mid;
      }
      if (lo >= len) continue;

      const paneIndex = series.paneIndex;
      let pane = byPane.get(paneIndex);
      if (!pane) {
        pane = {
          minV: Infinity,
          minT: /** @type {Time} */ (0),
          minS: series,
          maxV: -Infinity,
          maxT: /** @type {Time} */ (0),
          maxS: series,
        };
        byPane.set(paneIndex, pane);
      }

      for (let i = lo; i < len; i++) {
        const pt = data[i];
        if (/** @type {number} */ (pt.time) > t1) break;
        const v = pt.low ?? pt.value;
        const h = pt.high ?? pt.value;
        if (v && v < pane.minV) {
          pane.minV = v;
          pane.minT = pt.time;
          pane.minS = series;
        }
        if (h && h > pane.maxV) {
          pane.maxV = h;
          pane.maxT = pt.time;
          pane.maxS = series;
        }
      }
    }

    // Set new markers
    /** @type {Set<AnySeries>} */
    const used = new Set();
    for (const { minV, minT, minS, maxV, maxT, maxS } of byPane.values()) {
      if (!Number.isFinite(minV) || !Number.isFinite(maxV) || minT === maxT)
        continue;

      const minM = /** @type {TimeSeriesMarker} */ ({
        time: minT,
        position: "belowBar",
        shape: "arrowUp",
        color,
        size: 0,
        text: formatValue(minV),
      });
      const maxM = /** @type {TimeSeriesMarker} */ ({
        time: maxT,
        position: "aboveBar",
        shape: "arrowDown",
        color,
        size: 0,
        text: formatValue(maxV),
      });

      used.add(minS);
      used.add(maxS);
      if (minS === maxS) {
        minS.setMarkers([minM, maxM]);
      } else {
        minS.setMarkers([minM]);
        maxS.setMarkers([maxM]);
      }
    }

    // Clear stale
    for (const s of prevMarkerSeries) {
      if (!used.has(s)) s.clearMarkers();
    }
    prevMarkerSeries.clear();
    for (const s of used) prevMarkerSeries.add(s);
  }

  function clear() {
    for (const s of prevMarkerSeries) s.clearMarkers();
    prevMarkerSeries.clear();
  }

  return {
    update,
    scheduleUpdate: throttle(update, 100),
    clear,
  };
}
