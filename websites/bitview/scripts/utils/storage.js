/**
 * @param {string} key
 */
export function readStoredNumber(key) {
  const saved = readStored(key);
  if (saved) {
    return Number(saved);
  }
  return null;
}

/**
 * @param {string} key
 */
export function readStoredBool(key) {
  const saved = readStored(key);
  if (saved) {
    return saved === "true" || saved === "1";
  }
  return null;
}

/**
 * @param {string} key
 */
export function readStored(key) {
  try {
    return localStorage.getItem(key);
  } catch (_) {
    return null;
  }
}

/**
 * @param {string} key
 * @param {string | boolean | null | undefined} value
 */
export function writeToStorage(key, value) {
  try {
    value !== undefined && value !== null
      ? localStorage.setItem(key, String(value))
      : localStorage.removeItem(key);
  } catch (_) {}
}

/**
 * @param {string} key
 */
export function removeStored(key) {
  writeToStorage(key, undefined);
}
