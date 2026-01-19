/**
 * @param {number} ms
 */
export function sleep(ms) {
  return new Promise((resolve) => {
    setTimeout(resolve, ms);
  });
}

export function next() {
  return sleep(0);
}

/**
 *
 * @template {(...args: any[]) => any} F
 * @param {F} callback
 * @param {number} [wait]
 */
export function throttle(callback, wait = 1000) {
  /** @type {number | null} */
  let timeoutId = null;
  /** @type {Parameters<F>} */
  let latestArgs;

  return (/** @type {Parameters<F>} */ ...args) => {
    latestArgs = args;

    if (!timeoutId) {
      // Otherwise it optimizes away timeoutId in Chrome and FF
      timeoutId = timeoutId;
      timeoutId = setTimeout(() => {
        callback(...latestArgs); // Execute with latest args
        timeoutId = null;
      }, wait);
    }
  };
}

/**
 * @template {(...args: any[]) => any} F
 * @param {F} callback
 * @param {number} [wait]
 */
export function debounce(callback, wait = 1000) {
  /** @type {number | null} */
  let timeoutId = null;

  return (/** @type {Parameters<F>} */ ...args) => {
    if (timeoutId) {
      clearTimeout(timeoutId);
    }
    timeoutId = setTimeout(() => {
      callback(...args);
      timeoutId = null;
    }, wait);
  };
}
