import { brk } from "../utils/client.js";
import { createMapCache } from "../utils/cache.js";
import { latestPrice } from "../utils/price.js";
import { formatBtc, renderRows, renderTx, TX_PAGE_SIZE } from "./render.js";

/** @type {MapCache<Transaction[]>} */
const addrTxCache = createMapCache(200);

/**
 * @param {string} address
 * @param {HTMLDivElement} el
 * @param {{ signal: AbortSignal, cache: MapCache<AddrStats> }} options
 */
export async function showAddrDetail(address, el, { signal, cache }) {
  el.hidden = false;
  el.scrollTop = 0;
  el.innerHTML = "";

  try {
    const cached = cache.get(address);
    const stats = cached ?? (await brk.getAddress(address, { signal }));
    if (!cached) cache.set(address, stats);
    if (signal.aborted) return;
    const chain = stats.chainStats;

    const title = document.createElement("h1");
    title.textContent = "Address";
    el.append(title);

    const balance = chain.fundedTxoSum - chain.spentTxoSum;
    const mempool = stats.mempoolStats;
    const pending = mempool ? mempool.fundedTxoSum - mempool.spentTxoSum : 0;
    const pendingUtxos = mempool
      ? mempool.fundedTxoCount - mempool.spentTxoCount
      : 0;
    const confirmedUtxos = chain.fundedTxoCount - chain.spentTxoCount;
    const price = latestPrice();
    const fmtUsd = (/** @type {number} */ sats) =>
      price
        ? ` $${((sats / 1e8) * price).toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 })}`
        : "";

    renderRows(
      [
        ["Address", address],
        ["Confirmed Balance", `${formatBtc(balance)} BTC${fmtUsd(balance)}`],
        [
          "Pending",
          `${pending >= 0 ? "+" : ""}${formatBtc(pending)} BTC${fmtUsd(pending)}`,
        ],
        ["Confirmed UTXOs", confirmedUtxos.toLocaleString()],
        ["Pending UTXOs", pendingUtxos.toLocaleString()],
        ["Total Received", `${formatBtc(chain.fundedTxoSum)} BTC`],
        ["Tx Count", chain.txCount.toLocaleString()],
        [
          "Type",
          /** @type {any} */ ((stats).addrType ?? "unknown")
            .replace(/^v\d+_/, "")
            .toUpperCase(),
        ],
        [
          "Avg Cost Basis",
          chain.realizedPrice
            ? `$${Number(chain.realizedPrice).toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 })}`
            : "N/A",
        ],
      ],
      el,
    );

    const section = document.createElement("div");
    section.classList.add("transactions");
    const heading = document.createElement("h2");
    heading.textContent = "Transactions";
    section.append(heading);
    el.append(section);

    let loading = false;
    let pageIndex = 0;
    /** @type {string | undefined} */
    let afterTxid;

    const observer = new IntersectionObserver((entries) => {
      if (entries[0].isIntersecting && !loading && pageIndex * TX_PAGE_SIZE < chain.txCount)
        loadMore();
    });

    async function loadMore() {
      loading = true;
      const key = `${address}:${pageIndex}`;
      try {
        const cached = addrTxCache.get(key);
        const txs = cached ?? await brk.getAddressTxs(address, afterTxid, { signal });
        if (!cached) addrTxCache.set(key, txs);
        for (const tx of txs) section.append(renderTx(tx));
        pageIndex++;
        if (txs.length) {
          afterTxid = txs[txs.length - 1].txid;
          observer.disconnect();
          const last = section.lastElementChild;
          if (last) observer.observe(last);
        }
      } catch (e) {
        console.error("explorer addr txs:", e);
        pageIndex = chain.txCount; // stop loading
      }
      loading = false;
    }

    await loadMore();
  } catch (e) {
    console.error("explorer addr:", e);
    el.textContent = "Address not found";
  }
}
