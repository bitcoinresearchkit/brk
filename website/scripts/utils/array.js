/**
 * Typed Object.entries that preserves key types
 * @template {Record<string, any>} T
 * @param {T} obj
 * @returns {[keyof T & string, T[keyof T & string]][]}
 */
export const entries = (obj) => /** @type {[keyof T & string, T[keyof T & string]][]} */ (Object.entries(obj));

/**
 * Type-safe includes that narrows the value type
 * @template T
 * @param {readonly T[]} arr
 * @param {unknown} value
 * @returns {value is T}
 */
export const includes = (arr, value) => arr.includes(/** @type {T} */ (value));
