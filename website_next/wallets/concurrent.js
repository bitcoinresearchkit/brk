/**
 * @template Item, Result
 * @param {readonly Item[]} items
 * @param {number} limit
 * @param {(item: Item) => Promise<Result>} fn
 * @returns {Promise<Result[]>}
 */
export async function mapConcurrent(items, limit, fn) {
  const results = /** @type {Result[]} */ ([]);
  let next = 0;
  const workerCount = Math.min(limit, items.length);
  const workers = Array.from({ length: workerCount }, async () => {
    while (next < items.length) {
      const index = next;

      next += 1;
      results[index] = await fn(items[index]);
    }
  });

  await Promise.all(workers);

  return results;
}
