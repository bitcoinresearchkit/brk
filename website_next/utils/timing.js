/** @param {number} ms */
export function wait(ms) {
  return new Promise((resolve) => {
    setTimeout(resolve, ms);
  });
}

/** @param {string} name */
export function readCssDuration(name) {
  const value = getComputedStyle(document.documentElement)
    .getPropertyValue(name)
    .trim();

  return Number.parseFloat(value) * (value.endsWith("ms") ? 1 : 1000);
}
