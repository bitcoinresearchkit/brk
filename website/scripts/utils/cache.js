/**
 * @template V
 * @param {number} [maxSize]
 */
export function createMapCache(maxSize = 100) {
  /** @type {Map<string, V>} */
  const map = new Map();

  /** @param {string} key @param {V} value */
  const set = (key, value) => {
    if (map.size >= maxSize && !map.has(key)) {
      const first = map.keys().next().value;
      if (first !== undefined) map.delete(first);
    }
    map.set(key, value);
  };

  return {
    /** @param {string} key @returns {V | undefined} */
    get: (key) => map.get(key),
    /** @param {string} key @returns {boolean} */
    has: (key) => map.has(key),
    set,
    /** @param {string} key @param {() => Promise<V>} fetcher @returns {Promise<V>} */
    async fetch(key, fetcher) {
      const hit = map.get(key);
      if (hit !== undefined) return hit;
      const value = await fetcher();
      set(key, value);
      return value;
    },
  };
}

/**
 * @template V
 * @typedef {ReturnType<typeof createMapCache<V>>} MapCache
 */
