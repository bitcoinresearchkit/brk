import { index as serdeIndex } from "./serde";
import { runWhenIdle } from "./scheduling";

/**
 * @param {Signals} signals
 * @param {Utilities} utils
 * @param {Env} env
 * @param {VecIdToIndexes} vecIdToIndexes
 */
export function createVecsResources(signals, utils, env, vecIdToIndexes) {
  const owner = signals.getOwner();

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
   * @param {Index} index
   * @param {VecId} id
   */
  function createVecResource(index, id) {
    if (env.localhost && !(id in vecIdToIndexes)) {
      throw Error(`${id} not recognized`);
    }

    return signals.runWithOwner(owner, () => {
      /** @typedef {T extends number ? SingleValueData : CandlestickData} Value */

      const fetchedRecord = signals.createSignal(
        /** @type {Map<string, {loading: boolean, at: Date | null, vec: Signal<T[] | null>}>} */ (
          new Map()
        ),
      );

      return {
        url: api.genUrl(index, id, defaultFrom),
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
            await api.fetchVec(
              (values) => {
                if (values.length || !fetched.vec()) {
                  fetched.vec.set(values);
                }
              },
              index,
              id,
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

  /** @type {Map<string, NonNullable<ReturnType<typeof createVecResource>>>} */
  const map = new Map();

  const vecs = {
    /**
     * @template {number | OHLCTuple} [T=number]
     * @param {Index} index
     * @param {VecId} id
     */
    getOrCreate(index, id) {
      const key = `${index},${id}`;
      const found = map.get(key);
      if (found) {
        return found;
      }

      const vec = createVecResource(index, id);
      if (!vec) throw Error("vec is undefined");
      map.set(key, /** @type {any} */ (vec));
      return vec;
    },
    genFetchedKey,
    defaultFetchedKey,
  };

  return vecs;
}
/** @typedef {ReturnType<typeof createVecsResources>} VecsResources */
/** @typedef {ReturnType<VecsResources["getOrCreate"]>} VecResource */

const CACHE_NAME = "api";
const API_VECS_PREFIX = "/api/vecs";

/**
 * @template T
 * @param {(value: T) => void} callback
 * @param {string} path
 * @param {boolean} [mustBeArray]
 */
async function fetchApi(callback, path, mustBeArray) {
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
      fetchedJson = /** @type {T} */ (
        mustBeArray && !Array.isArray(f) ? [f] : f
      );
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
 * @param {VecId} vecId
 * @param {number} [from]
 * @param {number} [to]
 */
function genPath(index, vecId, from, to) {
  let path = `/${serdeIndex.serialize(index)}-to-${vecId.replaceAll("_", "-")}?`;

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

export const api = {
  /**
   * @param {Index} index
   * @param {VecId} vecId
   * @param {number} from
   */
  genUrl(index, vecId, from) {
    return `${API_VECS_PREFIX}${genPath(index, vecId, from)}`;
  },
  /**
   * @template {number | OHLCTuple} [T=number]
   * @param {(v: T[]) => void} callback
   * @param {Index} index
   * @param {VecId} vecId
   * @param {number} [from]
   * @param {number} [to]
   */
  fetchVec(callback, index, vecId, from, to) {
    return fetchApi(callback, genPath(index, vecId, from, to), true);
  },
  /**
   * @template {number | OHLCTuple} [T=number]
   * @param {(v: T) => void} callback
   * @param {Index} index
   * @param {VecId} vecId
   */
  fetchLast(callback, index, vecId) {
    return fetchApi(callback, genPath(index, vecId, -1));
  },
};
