let _latest = /** @type {number | null} */ (null);

/** @type {Set<(price: number) => void>} */
const listeners = new Set();

/** @param {(price: number) => void} callback */
export function onPrice(callback) {
  listeners.add(callback);
  if (_latest !== null) callback(_latest);
  return () => listeners.delete(callback);
}

export function latestPrice() {
  return _latest;
}

/** @param {BitviewClient} bitview */
export function initPrice(bitview) {
  let pending = false;
  let delay = 1_000;
  let timer = 0;

  async function poll() {
    if (pending || document.hidden) return;
    clearTimeout(timer);
    pending = true;
    try {
      const price = await bitview.getLivePrice({ memCache: false });
      delay = 1_000;
      if (price !== _latest) {
        _latest = price;
        listeners.forEach((cb) => cb(price));
      }
    } catch (e) {
      delay = Math.min(delay * 2, 30_000);
      console.error("price poll:", e);
    } finally {
      pending = false;
      if (!document.hidden) timer = window.setTimeout(poll, delay);
    }
  }

  poll();
  document.addEventListener("visibilitychange", () => {
    if (document.hidden) clearTimeout(timer);
    else poll();
  });
}
