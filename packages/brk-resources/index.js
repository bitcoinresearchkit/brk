/**
 * @import { Signal, Signals } from "../brk-signals/index";
 */

/**
 * @typedef {ReturnType<createResources>} BRKResources
 * @typedef {ReturnType<BRKResources["metrics"]["getOrCreate"]>} BRKMetricResource
 */

/**
 * @param {BRKClient} client
 * @param {Signals} signals
 */
export function createResources(client, signals) {
  const owner = signals.getOwner();

  const defaultFrom = -10_000;
  const defaultTo = undefined;

  /**
   * @param {Object} [args]
   * @param {number} [args.from]
   * @param {number} [args.to]
   */
  function genKey(args) {
    return `${args?.from ?? defaultFrom}-${args?.to ?? ""}`;
  }

  /**
   * @template T
   * @param {Metric} metric
   * @param {Index} index
   */
  function createMetricResource(metric, index) {
    if (client.hasMetric(metric)) {
      throw Error(`${metric} is invalid`);
    }

    return signals.runWithOwner(owner, () => {
      const fetchedRecord = signals.createSignal(
        /** @type {Map<string, {loading: boolean, at: Date | null, data: Signal<T[] | null>}>} */ (
          new Map()
        ),
      );

      return {
        url: client.genMetricURL(metric, index, defaultFrom),
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
          const fetchedKey = genKey({ from, to });
          if (!fetchedRecord().has(fetchedKey)) {
            fetchedRecord.set((map) => {
              map.set(fetchedKey, {
                loading: false,
                at: null,
                data: signals.createSignal(/** @type {T[] | null} */ (null), {
                  equals: false,
                }),
              });
              return map;
            });
          }
          const fetched = fetchedRecord().get(fetchedKey);
          if (!fetched) throw Error("Unreachable");
          if (fetched.loading) return fetched.data();
          if (fetched.at) {
            const diff = new Date().getTime() - fetched.at.getTime();
            const ONE_MINUTE_IN_MS = 60_000;
            if (diff < ONE_MINUTE_IN_MS) return fetched.data();
          }
          fetched.loading = true;
          const res = /** @type {T[] | null} */ (
            await client.fetchMetric(
              (data) => {
                if (data.length || !fetched.data()) {
                  fetched.data.set(data);
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
     * @template T
     * @param {Metric} metric
     * @param {Index} index
     */
    getOrCreate(metric, index) {
      const key = `${metric}/${index}`;
      const found = map.get(key);
      if (found) {
        return found;
      }

      const resource = createMetricResource(metric, index);
      if (!resource) throw Error("metric is undefined");
      map.set(key, /** @type {any} */ (resource));
      return resource;
    },
    genKey,
  };

  return { metrics };
}
