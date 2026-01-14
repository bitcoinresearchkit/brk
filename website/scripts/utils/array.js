/**
 * @param {number} start
 * @param {number} end
 */
export function range(start, end) {
  const range = [];
  while (start <= end) {
    range.push(start);
    start += 1;
  }
  return range;
}

/**
 * @template T
 * @param {T[]} array
 */
export function randomFromArray(array) {
  return array[Math.floor(Math.random() * array.length)];
}
