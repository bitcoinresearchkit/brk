/**
 * @import { IndexName } from "./generated/metrics"
 * @import { Metric } from './metrics'
 *
 * @typedef {ReturnType<createClient>} BRK
 */

// client.metrics.catalog.a.b.c() -> string (uncompress inside)

import { runWhenIdle } from "./idle";

import { POOL_ID_TO_POOL_NAME } from "./generated/pools";
import { INDEXES } from "./generated/metrics";
import { hasMetric, getIndexesFromMetric } from "./metrics";
import { VERSION } from "./generated/version";

const CACHE_NAME = "__BRK_CLIENT__";

/**
 * @param {string} origin
 */
export function createClient(origin) {
  /**
   * @template T
   * @param {(value: T) => void} callback
   * @param {string} url
   */
  async function fetchJson(callback, url) {
    /** @type {T | null} */
    let cachedJson = null;

    /** @type {Cache | undefined} */
    let cache;
    /** @type {Response | undefined} */
    let cachedResponse;
    try {
      cache = await caches.open(CACHE_NAME);
      cachedResponse = await cache.match(url);
      if (cachedResponse) {
        console.debug(`cache: ${url}`);
        const json = /** @type {T} */ (await cachedResponse.json());
        cachedJson = json;
        callback(json);
      }
    } catch {}

    try {
      if (!navigator.onLine) {
        throw "Offline";
      }

      console.debug(`fetch: ${url}`);

      const fetchedResponse = await fetch(url, {
        signal: AbortSignal.timeout(5000),
      });
      if (!fetchedResponse.ok) {
        throw `Bad response: ${fetchedResponse}`;
      }

      if (
        cachedResponse?.headers.get("ETag") ===
        fetchedResponse.headers.get("ETag")
      ) {
        return cachedJson;
      }

      const clonedResponse = fetchedResponse.clone();

      const fetchedJson = /** @type {T} */ (await fetchedResponse.json());
      if (!fetchedJson) throw `JSON is false`;

      callback(fetchedJson);

      runWhenIdle(async function () {
        try {
          await cache?.put(url, clonedResponse);
        } catch (_) {}
      });

      return fetchedJson;
    } catch (e) {
      console.error(e);
      return cachedJson;
    }
  }

  /**
   * @param {Metric} metric
   * @param {IndexName} index
   * @param {number} [from]
   * @param {number} [to]
   */
  function genMetricURL(metric, index, from, to) {
    let path = `${origin}api/metrics/${metric.replaceAll("_", "-")}/${index}?`;

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

  /**
   * @template T
   * @param {(v: T[]) => void} callback
   * @param {IndexName} index
   * @param {Metric} metric
   * @param {number} [from]
   * @param {number} [to]
   */
  function fetchMetric(callback, index, metric, from, to) {
    return fetchJson(callback, genMetricURL(metric, index, from, to));
  }

  return {
    VERSION,
    POOL_ID_TO_POOL_NAME,
    INDEXES,

    hasMetric,
    getIndexesFromMetric,

    genMetricURL,
    fetchMetric,
  };
}
