/**
 * @template T
 * @typedef {Object} Resource
 * @property {Signal<T | null>} data
 * @property {Signal<boolean>} loading
 * @property {Signal<Error | null>} error
 * @property {(...args: any[]) => Promise<T | null>} fetch
 */

/**
 * @template T
 * @typedef {Object} RangeState
 * @property {Signal<MetricData<T> | null>} response
 * @property {Signal<boolean>} loading
 */
/** @typedef {RangeState<unknown>} AnyRangeState */

/**
 * @template T
 * @typedef {Object} MetricResource
 * @property {string} path
 * @property {(from?: number, to?: number) => RangeState<T>} range
 * @property {(from?: number, to?: number) => Promise<MetricData<T> | null>} fetch
 */
/** @typedef {MetricResource<unknown>} AnyMetricResource */

/**
 * @typedef {{ createResource: typeof createResource, useMetricEndpoint: typeof useMetricEndpoint }} Resources
 */

import signals from "./signals.js";

/**
 * Create a generic reactive resource wrapper for any async fetcher
 * @template T
 * @template {any[]} Args
 * @param {(...args: Args) => Promise<T>} fetcher
 * @returns {Resource<T>}
 */
function createResource(fetcher) {
  const owner = signals.getOwner();
  return signals.runWithOwner(owner, () => {
    const data = signals.createSignal(/** @type {T | null} */ (null));
    const loading = signals.createSignal(false);
    const error = signals.createSignal(/** @type {Error | null} */ (null));

    return {
      data,
      loading,
      error,
      /**
       * @param {Args} args
       */
      async fetch(...args) {
        loading.set(true);
        error.set(null);
        try {
          const result = await fetcher(...args);
          data.set(() => result);
          return result;
        } catch (e) {
          error.set(e instanceof Error ? e : new Error(String(e)));
          return null;
        } finally {
          loading.set(false);
        }
      },
    };
  });
}

/**
 * Create a reactive resource wrapper for a MetricEndpoint with multi-range support
 * @template T
 * @param {MetricEndpoint<T>} endpoint
 * @returns {MetricResource<T>}
 */
function useMetricEndpoint(endpoint) {
  const owner = signals.getOwner();
  return signals.runWithOwner(owner, () => {
    /** @type {Map<string, RangeState<T>>} */
    const ranges = new Map();

    /**
     * Get or create range state
     * @param {number} [from=-10000]
     * @param {number} [to]
     * @returns {RangeState<T>}
     */
    function range(from = -10000, to) {
      const key = `${from}-${to ?? ""}`;
      const existing = ranges.get(key);
      if (existing) return existing;

      /** @type {RangeState<T>} */
      const state = {
        response: signals.createSignal(
          /** @type {MetricData<T> | null} */ (null),
        ),
        loading: signals.createSignal(false),
      };
      ranges.set(key, state);
      return state;
    }

    return {
      path: endpoint.path,
      range,
      /**
       * Fetch data for a range
       * @param {number} [start=-10000]
       * @param {number} [end]
       */
      async fetch(start = -10000, end) {
        const r = range(start, end);
        r.loading.set(true);
        try {
          const result = await endpoint
            .slice(start, end)
            .fetch(r.response.set);
          return result;
        } finally {
          r.loading.set(false);
        }
      },
    };
  });
}

export const resources = { createResource, useMetricEndpoint };
