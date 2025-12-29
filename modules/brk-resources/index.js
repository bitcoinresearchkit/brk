/**
 * @import { Signal, Signals } from "../brk-signals/index";
 * @import { MetricNode } from "../brk-client/index";
 */

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
 * @property {Signal<T[] | null>} data
 * @property {Signal<boolean>} loading
 */

/**
 * @template T
 * @typedef {Object} MetricResource
 * @property {string} path
 * @property {(from?: number, to?: number) => RangeState<T>} range
 * @property {(from?: number, to?: number) => Promise<T[] | null>} fetch
 */

/**
 * @typedef {ReturnType<typeof createResources>} Resources
 */

/**
 * @param {Signals} signals
 */
export function createResources(signals) {
  const owner = signals.getOwner();

  /**
   * Create a generic reactive resource wrapper for any async fetcher
   * @template T
   * @template {any[]} Args
   * @param {(...args: Args) => Promise<T>} fetcher
   * @returns {Resource<T>}
   */
  function createResource(fetcher) {
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
            data.set(result);
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
   * Create a reactive resource wrapper for a MetricNode with multi-range support
   * @template T
   * @param {MetricNode<T>} node
   * @returns {MetricResource<T>}
   */
  function useMetricNode(node) {
    return signals.runWithOwner(owner, () => {
      /** @type {Map<string, RangeState<T>>} */
      const ranges = new Map();

      /**
       * Get or create range state
       * @param {number} [from=-10000]
       * @param {number} [to]
       */
      function range(from = -10000, to) {
        const key = `${from}-${to ?? ""}`;
        if (!ranges.has(key)) {
          ranges.set(key, {
            data: signals.createSignal(/** @type {T[] | null} */ (null)),
            loading: signals.createSignal(false),
          });
        }
        return /** @type {RangeState<T>} */ (ranges.get(key));
      }

      return {
        path: node._path,
        range,
        /**
         * Fetch data for a range
         * @param {number} [from=-10000]
         * @param {number} [to]
         */
        async fetch(from = -10000, to) {
          const r = range(from, to);
          r.loading.set(true);
          try {
            const result = await node.getRange(from, to, r.data.set);
            return result;
          } finally {
            r.loading.set(false);
          }
        },
      };
    });
  }

  return { createResource, useMetricNode };
}
