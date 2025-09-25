/**
 * @param {VoidFunction} callback
 * @param {number} [timeout = 1]
 */
export function runWhenIdle(callback, timeout = 1) {
  if ("requestIdleCallback" in window) {
    requestIdleCallback(callback);
  } else {
    setTimeout(callback, timeout);
  }
}
