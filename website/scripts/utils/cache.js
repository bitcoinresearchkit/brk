/**
 * @template V
 * @param {number} [maxSize]
 */
export function createMapCache(maxSize = 100) {
  /** @type {Map<string, V>} */
  const map = new Map();

  return {
    /** @param {string} key @returns {V | undefined} */
    get(key) {
      return map.get(key);
    },
    /** @param {string} key @returns {boolean} */
    has(key) {
      return map.has(key);
    },
    /** @param {string} key @param {V} value */
    set(key, value) {
      if (map.size >= maxSize && !map.has(key)) {
        const first = map.keys().next().value;
        if (first !== undefined) map.delete(first);
      }
      map.set(key, value);
    },
  };
}

/**
 * @template V
 * @typedef {{ get: (key: string) => V | undefined, has: (key: string) => boolean, set: (key: string, value: V) => void }} MapCache
 */
