/** @param {string} value */
export function createId(value) {
  return value.toLowerCase().replaceAll(" ", "-");
}
