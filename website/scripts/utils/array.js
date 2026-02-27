/**
 * Typed Object.entries that preserves key types
 * @template {Record<string, any>} T
 * @param {T} obj
 * @returns {[keyof T & string, T[keyof T & string]][]}
 */
export const entries = (obj) => /** @type {[keyof T & string, T[keyof T & string]][]} */ (Object.entries(obj));

/**
 * Typed Object.fromEntries that preserves key/value types
 * @template {string} K
 * @template V
 * @param {Iterable<readonly [K, V]>} pairs
 * @returns {Record<K, V>}
 */
export const fromEntries = (pairs) => /** @type {Record<K, V>} */ (Object.fromEntries(pairs));

/**
 * Type-safe includes that narrows the value type
 * @template T
 * @param {readonly T[]} arr
 * @param {unknown} value
 * @returns {value is T}
 */
export const includes = (arr, value) => arr.includes(/** @type {T} */ (value));
