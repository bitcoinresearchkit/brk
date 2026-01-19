import { createSeriesMarkers } from "../modules/lightweight-charts/5.1.0/dist/lightweight-charts.standalone.production.mjs";
import { throttle } from "../utils/timing.js";

/**
 * @param {Object} args
 * @param {IChartApi} args.chart
 * @param {Accessor<Set<AnySeries>>} args.seriesList
 * @param {Colors} args.colors
 * @param {(value: number) => string} args.formatValue
 */
export function createMinMaxMarkers({ chart, seriesList, colors, formatValue }) {
  /** @type {WeakMap<ISeries, SeriesMarkersPlugin>} */
  const pluginCache = new WeakMap();

  /** @param {ISeries} iseries */
  function getOrCreatePlugin(iseries) {
    let plugin = pluginCache.get(iseries);
    if (!plugin) {
      plugin = createSeriesMarkers(iseries, [], { autoScale: false });
      pluginCache.set(iseries, plugin);
    }
    return plugin;
  }

  /** @type {Set<ISeries>} */
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

    /** @type {Map<number, { minV: number, minT: Time, minS: ISeries, maxV: number, maxT: Time, maxS: ISeries }>} */
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
      const iseries = series.inner();
      let pane = byPane.get(paneIndex);
      if (!pane) {
        pane = {
          minV: Infinity,
          minT: /** @type {Time} */ (0),
          minS: iseries,
          maxV: -Infinity,
          maxT: /** @type {Time} */ (0),
          maxS: iseries,
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
          pane.minS = iseries;
        }
        if (h && h > pane.maxV) {
          pane.maxV = h;
          pane.maxT = pt.time;
          pane.maxS = iseries;
        }
      }
    }

    // Set new markers
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
        getOrCreatePlugin(minS).setMarkers([minM, maxM]);
      } else {
        getOrCreatePlugin(minS).setMarkers([minM]);
        getOrCreatePlugin(maxS).setMarkers([maxM]);
      }
    }

    // Clear stale
    for (const s of prevMarkerSeries) {
      if (!used.has(s)) getOrCreatePlugin(s).setMarkers([]);
    }
    prevMarkerSeries.clear();
    for (const s of used) prevMarkerSeries.add(s);
  }

  function clear() {
    for (const s of prevMarkerSeries) getOrCreatePlugin(s).setMarkers([]);
    prevMarkerSeries.clear();
  }

  return {
    update,
    scheduleUpdate: throttle(update, 100),
    clear,
  };
}
