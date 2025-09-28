import { runWhenIdle } from "./idle";
import { POOL_ID_TO_POOL_NAME } from "./generated/pools";
import { createMetricToIndexes } from "./metrics";

const localhost = window.location.hostname === "localhost";

/**
 * @param {Signals} signals
 */
export default function (signals) {
  const owner = signals.getOwner();

  const pools = POOL_ID_TO_POOL_NAME;
  const metricToIndexes = createMetricToIndexes();

  const defaultFrom = -10_000;
  const defaultTo = undefined;

  /**
   * Defaults
   * - from: -10_000
   * - to: undefined
   *
   * @param {Object} [args]
   * @param {number} [args.from]
   * @param {number} [args.to]
   */
  function genFetchedKey(args) {
    return `${args?.from}-${args?.to}`;
  }

  const defaultFetchedKey = genFetchedKey({ from: defaultFrom, to: defaultTo });

  /**
   * @template {number | OHLCTuple} [T=number]
   * @param {Metric} metric
   * @param {Index} index
   */
  function createMetricResource(metric, index) {
    if (localhost && !(metric in metricToIndexes)) {
      throw Error(`${metric} not recognized`);
    }

    return signals.runWithOwner(owner, () => {
      const fetchedRecord = signals.createSignal(
        /** @type {Map<string, {loading: boolean, at: Date | null, vec: Signal<T[] | null>}>} */ (
          new Map()
        ),
      );

      return {
        url: api.genUrl(index, metric, defaultFrom),
        fetched: fetchedRecord,
        /**
         * Defaults
         * - from: -10_000
         * - to: undefined
         *
         * @param {Object} [args]
         * @param {number} [args.from]
         * @param {number} [args.to]
         */
        async fetch(args) {
          const from = args?.from ?? defaultFrom;
          const to = args?.to ?? defaultTo;
          const fetchedKey = genFetchedKey({ from, to });
          if (!fetchedRecord().has(fetchedKey)) {
            fetchedRecord.set((map) => {
              map.set(fetchedKey, {
                loading: false,
                at: null,
                vec: signals.createSignal(/** @type {T[] | null} */ (null), {
                  equals: false,
                }),
              });
              return map;
            });
          }
          const fetched = fetchedRecord().get(fetchedKey);
          if (!fetched) throw Error("Unreachable");
          if (fetched.loading) return fetched.vec();
          if (fetched.at) {
            const diff = new Date().getTime() - fetched.at.getTime();
            const ONE_MINUTE_IN_MS = 60_000;
            if (diff < ONE_MINUTE_IN_MS) return fetched.vec();
          }
          fetched.loading = true;
          const res = /** @type {T[] | null} */ (
            await fetchVec(
              (values) => {
                if (values.length || !fetched.vec()) {
                  fetched.vec.set(values);
                }
              },
              index,
              metric,
              from,
              to,
            )
          );
          fetched.at = new Date();
          fetched.loading = false;
          return res;
        },
      };
    });
  }

  /** @type {Map<string, NonNullable<ReturnType<typeof createMetricResource>>>} */
  const map = new Map();

  const metrics = {
    /**
     * @template {number | OHLCTuple} [T=number]
     * @param {Metric} metric
     * @param {Index} index
     */
    getOrCreate(metric, index) {
      const key = `${metric}/${index}`;
      const found = map.get(key);
      if (found) {
        return found;
      }

      const vec = createMetricResource(index, metric);
      if (!vec) throw Error("vec is undefined");
      map.set(key, /** @type {any} */ (vec));
      return vec;
    },
    genFetchedKey,
    defaultFetchedKey,
  };

  return { metrics, pools };
}
// /** @typedef {ReturnType<typeof createVecsResources>} VecsResources */
// /** @typedef {ReturnType<VecsResources["getOrCreate"]>} VecResource */

const CACHE_NAME = "api";
const API_VECS_PREFIX = "/api/vecs";

/**
 * @template T
 * @param {(value: T) => void} callback
 * @param {string} path
 */
async function fetchApi(callback, path) {
  const url = `${API_VECS_PREFIX}${path}`;

  /** @type {T | null} */
  let cachedJson = null;

  /** @type {Cache | undefined} */
  let cache;
  try {
    cache = await caches.open(CACHE_NAME);
    const cachedResponse = await cache.match(url);
    if (cachedResponse) {
      console.debug(`cache: ${url}`);
      const json = /** @type {T} */ await cachedResponse.json();
      cachedJson = json;
      callback(json);
    }
  } catch {}

  if (navigator.onLine) {
    // TODO: rerun after 10s instead of returning (due to some kind of error)

    /** @type {Response | undefined} */
    let fetchedResponse;
    try {
      fetchedResponse = await fetch(url, {
        signal: AbortSignal.timeout(5000),
      });
      if (!fetchedResponse.ok) {
        throw Error;
      }
    } catch {
      return cachedJson;
    }

    const clonedResponse = fetchedResponse.clone();

    let fetchedJson = /** @type {T | null} */ (null);
    try {
      const f = await fetchedResponse.json();
      fetchedJson = /** @type {T} */ (f);
    } catch (_) {
      return cachedJson;
    }

    if (!fetchedJson) return cachedJson;

    console.debug(`fetch: ${url}`);

    if (Array.isArray(cachedJson) && Array.isArray(fetchedJson)) {
      const previousLength = cachedJson?.length || 0;
      const newLength = fetchedJson.length;

      if (!newLength) {
        return cachedJson;
      }

      if (previousLength && previousLength === newLength) {
        const previousLastValue = Object.values(cachedJson || []).at(-1);
        const newLastValue = Object.values(fetchedJson).at(-1);
        if (
          JSON.stringify(previousLastValue) === JSON.stringify(newLastValue)
        ) {
          return cachedJson;
        }
      }
    }

    callback(fetchedJson);

    runWhenIdle(async function () {
      try {
        await cache?.put(url, clonedResponse);
      } catch (_) {}
    });

    return fetchedJson;
  } else {
    return cachedJson;
  }
}

/**
 * @param {Index} index
 * @param {Metric} metric
 * @param {number} [from]
 * @param {number} [to]
 */
function genPath(index, metric, from, to) {
  let path = `/${serdeIndex.serialize(index)}-to-${metric.replaceAll("_", "-")}?`;

  if (from !== undefined) {
    path += `from=${from}`;
  }
  if (to !== undefined) {
    if (!path.endsWith("?")) {
      path += `&`;
    }
    path += `to=${to}`;
  }
  return path;
}

// /**
//  * @template {number | OHLCTuple} [T=number]
//  * @param {(v: T[]) => void} callback
//  * @param {Index} index
//  * @param {Metric} metric
//  * @param {number} [from]
//  * @param {number} [to]
//  */
// function fetchMetric(callback, index, metric, from, to) {
//   return fetchApi(callback, genPath(index, metric, from, to));
// }

// /**
//  * @param {Index} index
//  * @param {Metric} metric
//  * @param {number} from
//  */
// function genUrl(index, metric, from) {
//   return `${API_VECS_PREFIX}${genPath(index, metric, from)}`;
// }
