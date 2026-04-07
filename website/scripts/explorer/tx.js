import { formatBtc, formatFeeRate, renderRows, renderTx } from "./render.js";

/**
 * @param {Transaction} tx
 * @param {HTMLDivElement} el
 */
export function showTxFromData(tx, el) {
  el.hidden = false;
  el.scrollTop = 0;
  el.innerHTML = "";

  const title = document.createElement("h1");
  title.textContent = "Transaction";
  el.append(title);

  const vsize = Math.ceil(tx.weight / 4);
  const feeRate = vsize > 0 ? tx.fee / vsize : 0;
  const totalIn = tx.vin.reduce((s, v) => s + (v.prevout?.value ?? 0), 0);
  const totalOut = tx.vout.reduce((s, v) => s + v.value, 0);

  renderRows(
    [
      ["TXID", tx.txid],
      [
        "Status",
        tx.status?.confirmed
          ? `Confirmed (block ${tx.status.blockHeight?.toLocaleString()})`
          : "Unconfirmed",
        tx.status?.blockHash ? `/block/${tx.status.blockHash}` : null,
      ],
      [
        "Timestamp",
        tx.status?.blockTime
          ? new Date(tx.status.blockTime * 1000).toUTCString()
          : "Pending",
      ],
      ["Size", `${tx.size.toLocaleString()} B`],
      ["Virtual Size", `${vsize.toLocaleString()} vB`],
      ["Weight", `${tx.weight.toLocaleString()} WU`],
      ["Fee", `${tx.fee.toLocaleString()} sat`],
      ["Fee Rate", `${formatFeeRate(feeRate)} sat/vB`],
      ["Inputs", `${tx.vin.length}`],
      ["Outputs", `${tx.vout.length}`],
      ["Total Input", `${formatBtc(totalIn)} BTC`],
      ["Total Output", `${formatBtc(totalOut)} BTC`],
      ["Version", `${tx.version}`],
      ["Locktime", `${tx.locktime}`],
    ],
    el,
  );

  const section = document.createElement("div");
  section.classList.add("transactions");
  const heading = document.createElement("h2");
  heading.textContent = "Inputs & Outputs";
  section.append(heading, renderTx(tx));
  el.append(section);
}
