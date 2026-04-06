import { explorerElement } from "../utils/elements.js";
import { brk } from "../client.js";

const LOOKAHEAD = 15;
const TX_PAGE_SIZE = 25;

/** @type {HTMLDivElement} */ let chain;
/** @type {HTMLDivElement} */ let blocksEl;
/** @type {HTMLDivElement} */ let details;
/** @type {HTMLDivElement | null} */ let selectedCube = null;
/** @type {number | undefined} */ let pollInterval;
/** @type {IntersectionObserver} */ let olderObserver;

/** @type {Map<BlockHash, BlockInfoV1>} */
const blocksByHash = new Map();

let newestHeight = -1;
let oldestHeight = Infinity;
let loadingLatest = false;
let loadingOlder = false;
let loadingNewer = false;
let reachedTip = false;

/** @type {HTMLSpanElement} */ let heightPrefix;
/** @type {HTMLSpanElement} */ let heightNum;
/** @type {{ row: HTMLDivElement, valueEl: HTMLSpanElement }[]} */ let detailRows;
/** @type {HTMLDivElement} */ let txList;
/** @type {HTMLDivElement} */ let txSection;
/** @type {IntersectionObserver} */ let txObserver;

/** @typedef {{ first: HTMLButtonElement, prev: HTMLButtonElement, label: HTMLSpanElement, next: HTMLButtonElement, last: HTMLButtonElement }} TxNav */
/** @type {TxNav[]} */ let txNavs = [];
/** @type {BlockInfoV1 | null} */ let txBlock = null;
let txPage = -1;
let txTotalPages = 0;
let txLoading = false;

export function init() {
  chain = document.createElement("div");
  chain.id = "chain";
  explorerElement.append(chain);

  blocksEl = document.createElement("div");
  blocksEl.classList.add("blocks");
  chain.append(blocksEl);

  details = document.createElement("div");
  details.id = "block-details";
  explorerElement.append(details);

  initDetails();

  olderObserver = new IntersectionObserver(
    (entries) => {
      if (entries[0].isIntersecting) loadOlder();
    },
    { root: chain },
  );

  chain.addEventListener(
    "scroll",
    () => {
      const nearStart =
        (chain.scrollHeight > chain.clientHeight && chain.scrollTop <= 50) ||
        (chain.scrollWidth > chain.clientWidth && chain.scrollLeft <= 50);
      if (nearStart && !reachedTip && !loadingNewer) loadNewer();
    },
    { passive: true },
  );

  new MutationObserver(() => {
    if (explorerElement.hidden) stopPolling();
    else startPolling();
  }).observe(explorerElement, {
    attributes: true,
    attributeFilter: ["hidden"],
  });

  document.addEventListener("visibilitychange", () => {
    if (!document.hidden && !explorerElement.hidden) loadLatest();
  });

  loadLatest();
}

function startPolling() {
  stopPolling();
  loadLatest();
  pollInterval = setInterval(loadLatest, 15_000);
}

function stopPolling() {
  if (pollInterval !== undefined) {
    clearInterval(pollInterval);
    pollInterval = undefined;
  }
}

function observeOldestEdge() {
  olderObserver.disconnect();
  const oldest = blocksEl.firstElementChild;
  if (oldest) olderObserver.observe(oldest);
}

/** @param {BlockInfoV1[]} blocks */
function appendNewerBlocks(blocks) {
  if (!blocks.length) return false;
  for (const b of [...blocks].reverse()) {
    if (b.height > newestHeight) {
      blocksEl.append(createBlockCube(b));
    } else {
      blocksByHash.set(b.id, b);
    }
  }
  newestHeight = Math.max(newestHeight, blocks[0].height);
  return true;
}

/** @returns {Promise<number | null>} */
async function getStartHeight() {
  const path = window.location.pathname.split("/").filter((v) => v);
  if (path[0] !== "block" || !path[1]) return null;
  const value = path[1];
  if (/^\d+$/.test(value)) return Number(value);
  return (await brk.getBlockV1(value)).height;
}

async function loadLatest() {
  if (loadingLatest) return;
  if (newestHeight !== -1 && !reachedTip) return;
  loadingLatest = true;
  try {
    const startHeight = newestHeight === -1 ? await getStartHeight() : null;
    const blocks =
      startHeight !== null
        ? await brk.getBlocksV1FromHeight(startHeight)
        : await brk.getBlocksV1();

    if (newestHeight === -1) {
      for (const b of blocks) blocksEl.prepend(createBlockCube(b));
      newestHeight = blocks[0].height;
      oldestHeight = blocks[blocks.length - 1].height;
      if (startHeight === null) reachedTip = true;
      selectCube(/** @type {HTMLDivElement} */ (blocksEl.lastElementChild));
      loadingLatest = false;
      observeOldestEdge();
      if (!reachedTip) await loadNewer();
      return;
    }

    appendNewerBlocks(blocks);
    reachedTip = true;
  } catch (e) {
    console.error("explorer poll:", e);
  }
  loadingLatest = false;
}

async function loadOlder() {
  if (loadingOlder || oldestHeight <= 0) return;
  loadingOlder = true;
  try {
    const blocks = await brk.getBlocksV1FromHeight(oldestHeight - 1);
    for (const block of blocks) blocksEl.prepend(createBlockCube(block));
    if (blocks.length) {
      oldestHeight = blocks[blocks.length - 1].height;
      observeOldestEdge();
    }
  } catch (e) {
    console.error("explorer loadOlder:", e);
  }
  loadingOlder = false;
}

async function loadNewer() {
  if (loadingNewer || newestHeight === -1 || reachedTip) return;
  loadingNewer = true;
  try {
    const anchor = blocksEl.lastElementChild;
    const anchorRect = anchor?.getBoundingClientRect();

    const blocks = await brk.getBlocksV1FromHeight(newestHeight + LOOKAHEAD);
    if (appendNewerBlocks(blocks)) {
      if (anchor && anchorRect) {
        const r = anchor.getBoundingClientRect();
        chain.scrollTop += r.top - anchorRect.top;
        chain.scrollLeft += r.left - anchorRect.left;
      }
    } else {
      reachedTip = true;
    }
  } catch (e) {
    console.error("explorer loadNewer:", e);
  }
  loadingNewer = false;
}

/** @param {HTMLDivElement} cube */
function selectCube(cube) {
  if (selectedCube) selectedCube.classList.remove("selected");
  selectedCube = cube;
  if (cube) {
    cube.classList.add("selected");
    const hash = cube.dataset.hash;
    if (hash) updateDetails(blocksByHash.get(hash));
  }
}

/** @typedef {[string, (b: BlockInfoV1) => string | null]} RowDef */

/** @type {RowDef[]} */
const ROW_DEFS = [
  ["Hash", (b) => b.id],
  ["Previous Hash", (b) => b.previousblockhash],
  ["Merkle Root", (b) => b.merkleRoot],
  ["Timestamp", (b) => new Date(b.timestamp * 1000).toUTCString()],
  ["Median Time", (b) => new Date(b.mediantime * 1000).toUTCString()],
  ["Version", (b) => `0x${b.version.toString(16)}`],
  ["Bits", (b) => b.bits.toString(16)],
  ["Nonce", (b) => b.nonce.toLocaleString()],
  ["Difficulty", (b) => Number(b.difficulty).toLocaleString()],
  ["Size", (b) => `${(b.size / 1_000_000).toFixed(2)} MB`],
  ["Weight", (b) => `${(b.weight / 1_000_000).toFixed(2)} MWU`],
  ["Transactions", (b) => b.txCount.toLocaleString()],
  ["Price", (b) => (b.extras ? `$${b.extras.price.toLocaleString()}` : null)],
  ["Pool", (b) => b.extras?.pool.name ?? null],
  ["Pool ID", (b) => b.extras?.pool.id.toString() ?? null],
  ["Pool Slug", (b) => b.extras?.pool.slug ?? null],
  ["Miner Names", (b) => b.extras?.pool.minerNames?.join(", ") || null],
  ["Reward", (b) => (b.extras ? `${(b.extras.reward / 1e8).toFixed(8)} BTC` : null)],
  ["Total Fees", (b) => (b.extras ? `${(b.extras.totalFees / 1e8).toFixed(8)} BTC` : null)],
  ["Median Fee Rate", (b) => (b.extras ? `${formatFeeRate(b.extras.medianFee)} sat/vB` : null)],
  ["Avg Fee Rate", (b) => (b.extras ? `${formatFeeRate(b.extras.avgFeeRate)} sat/vB` : null)],
  ["Avg Fee", (b) => (b.extras ? `${b.extras.avgFee.toLocaleString()} sat` : null)],
  ["Median Fee", (b) => (b.extras ? `${b.extras.medianFeeAmt.toLocaleString()} sat` : null)],
  ["Fee Range", (b) => (b.extras ? b.extras.feeRange.map((f) => formatFeeRate(f)).join(", ") + " sat/vB" : null)],
  ["Fee Percentiles", (b) => (b.extras ? b.extras.feePercentiles.map((f) => f.toLocaleString()).join(", ") + " sat" : null)],
  ["Avg Tx Size", (b) => (b.extras ? `${b.extras.avgTxSize.toLocaleString()} B` : null)],
  ["Virtual Size", (b) => (b.extras ? `${b.extras.virtualSize.toLocaleString()} vB` : null)],
  ["Inputs", (b) => b.extras?.totalInputs.toLocaleString() ?? null],
  ["Outputs", (b) => b.extras?.totalOutputs.toLocaleString() ?? null],
  ["Total Input Amount", (b) => (b.extras ? `${(b.extras.totalInputAmt / 1e8).toFixed(8)} BTC` : null)],
  ["Total Output Amount", (b) => (b.extras ? `${(b.extras.totalOutputAmt / 1e8).toFixed(8)} BTC` : null)],
  ["UTXO Set Change", (b) => b.extras?.utxoSetChange.toLocaleString() ?? null],
  ["UTXO Set Size", (b) => b.extras?.utxoSetSize.toLocaleString() ?? null],
  ["SegWit Txs", (b) => b.extras?.segwitTotalTxs.toLocaleString() ?? null],
  ["SegWit Size", (b) => (b.extras ? `${b.extras.segwitTotalSize.toLocaleString()} B` : null)],
  ["SegWit Weight", (b) => (b.extras ? `${b.extras.segwitTotalWeight.toLocaleString()} WU` : null)],
  ["Coinbase Address", (b) => b.extras?.coinbaseAddress || null],
  ["Coinbase Addresses", (b) => b.extras?.coinbaseAddresses.join(", ") || null],
  ["Coinbase Raw", (b) => b.extras?.coinbaseRaw ?? null],
  ["Coinbase Signature", (b) => b.extras?.coinbaseSignature ?? null],
  ["Coinbase Signature ASCII", (b) => b.extras?.coinbaseSignatureAscii ?? null],
  ["Header", (b) => b.extras?.header ?? null],
];

function initDetails() {
  const title = document.createElement("h1");
  title.textContent = "Block ";
  const code = document.createElement("code");
  const container = document.createElement("span");
  heightPrefix = document.createElement("span");
  heightPrefix.style.opacity = "0.5";
  heightPrefix.style.userSelect = "none";
  heightNum = document.createElement("span");
  container.append(heightPrefix, heightNum);
  code.append(container);
  title.append(code);
  details.append(title);

  detailRows = ROW_DEFS.map(([label]) => {
    const row = document.createElement("div");
    row.classList.add("row");
    const labelEl = document.createElement("span");
    labelEl.classList.add("label");
    labelEl.textContent = label;
    const valueEl = document.createElement("span");
    valueEl.classList.add("value");
    row.append(labelEl, valueEl);
    details.append(row);
    return { row, valueEl };
  });

  txSection = document.createElement("div");
  txSection.classList.add("transactions");
  details.append(txSection);

  const txHeader = document.createElement("div");
  txHeader.classList.add("tx-header");
  const heading = document.createElement("h2");
  heading.textContent = "Transactions";
  txHeader.append(heading, createTxNav());
  txSection.append(txHeader);

  txList = document.createElement("div");
  txList.classList.add("tx-list");
  txSection.append(txList, createTxNav());

  txObserver = new IntersectionObserver((entries) => {
    if (entries[0].isIntersecting && txPage === -1) loadTxPage(0);
  });
  txObserver.observe(txSection);
}

/** @returns {HTMLDivElement} */
function createTxNav() {
  const nav = document.createElement("div");
  nav.classList.add("pagination");
  const first = document.createElement("button");
  first.textContent = "\u00AB";
  const prev = document.createElement("button");
  prev.textContent = "\u2190";
  const label = document.createElement("span");
  const next = document.createElement("button");
  next.textContent = "\u2192";
  const last = document.createElement("button");
  last.textContent = "\u00BB";
  nav.append(first, prev, label, next, last);
  first.addEventListener("click", () => loadTxPage(0));
  prev.addEventListener("click", () => loadTxPage(txPage - 1));
  next.addEventListener("click", () => loadTxPage(txPage + 1));
  last.addEventListener("click", () => loadTxPage(txTotalPages - 1));
  txNavs.push({ first, prev, label, next, last });
  return nav;
}

/** @param {number} page */
function updateTxNavs(page) {
  const atFirst = page <= 0;
  const atLast = page >= txTotalPages - 1;
  for (const n of txNavs) {
    n.label.textContent = `${page + 1} / ${txTotalPages}`;
    n.first.disabled = atFirst;
    n.prev.disabled = atFirst;
    n.next.disabled = atLast;
    n.last.disabled = atLast;
  }
}

/** @param {BlockInfoV1 | undefined} block */
function updateDetails(block) {
  if (!block) return;
  details.scrollTop = 0;

  const str = block.height.toString();
  heightPrefix.textContent = "#" + "0".repeat(7 - str.length);
  heightNum.textContent = str;

  ROW_DEFS.forEach(([, getter], i) => {
    const value = getter(block);
    const { row, valueEl } = detailRows[i];
    if (value !== null) {
      valueEl.textContent = value;
      row.hidden = false;
    } else {
      row.hidden = true;
    }
  });

  txBlock = block;
  txTotalPages = Math.ceil(block.txCount / TX_PAGE_SIZE);
  txPage = -1;
  updateTxNavs(0);
  txList.innerHTML = "";
  txObserver.disconnect();
  txObserver.observe(txSection);
}

/** @param {number} page */
async function loadTxPage(page) {
  if (txLoading || !txBlock || page < 0 || page >= txTotalPages) return;
  txLoading = true;
  txPage = page;
  updateTxNavs(page);
  try {
    const txs = await brk.getBlockTxsFromIndex(txBlock.id, page * TX_PAGE_SIZE);
    txList.innerHTML = "";
    for (const tx of txs) txList.append(renderTx(tx));
  } catch (e) {
    console.error("explorer txs:", e);
  }
  txLoading = false;
}

/** @param {Transaction} tx */
function renderTx(tx) {
  const el = document.createElement("div");
  el.classList.add("tx");

  const head = document.createElement("div");
  head.classList.add("tx-head");
  const txidEl = document.createElement("span");
  txidEl.classList.add("txid");
  txidEl.textContent = tx.txid;
  head.append(txidEl);
  if (tx.status?.blockTime) {
    const time = document.createElement("span");
    time.classList.add("tx-time");
    time.textContent = new Date(tx.status.blockTime * 1000).toLocaleString();
    head.append(time);
  }
  el.append(head);

  const body = document.createElement("div");
  body.classList.add("tx-body");

  const inputs = document.createElement("div");
  inputs.classList.add("tx-inputs");
  for (const vin of tx.vin) {
    const row = document.createElement("div");
    row.classList.add("tx-io");
    const addr = document.createElement("span");
    addr.classList.add("addr");
    if (vin.isCoinbase) {
      addr.textContent = "Coinbase";
      addr.classList.add("coinbase");
    } else {
      const a = /** @type {string | undefined} */ (/** @type {any} */ (vin.prevout)?.scriptpubkey_address);
      setAddrContent(a || "Unknown", addr);
    }
    const amt = document.createElement("span");
    amt.classList.add("amount");
    amt.textContent = vin.prevout ? `${formatBtc(vin.prevout.value)} BTC` : "";
    row.append(addr, amt);
    inputs.append(row);
  }

  const outputs = document.createElement("div");
  outputs.classList.add("tx-outputs");
  let totalOut = 0;
  for (const vout of tx.vout) {
    totalOut += vout.value;
    const row = document.createElement("div");
    row.classList.add("tx-io");
    const addr = document.createElement("span");
    addr.classList.add("addr");
    const type = /** @type {string | undefined} */ (/** @type {any} */ (vout).scriptpubkey_type);
    const a = /** @type {string | undefined} */ (/** @type {any} */ (vout).scriptpubkey_address);
    if (type === "op_return") {
      addr.textContent = "OP_RETURN";
      addr.classList.add("op-return");
    } else {
      setAddrContent(a || vout.scriptpubkey, addr);
    }
    const amt = document.createElement("span");
    amt.classList.add("amount");
    amt.textContent = `${formatBtc(vout.value)} BTC`;
    row.append(addr, amt);
    outputs.append(row);
  }

  body.append(inputs, outputs);
  el.append(body);

  const foot = document.createElement("div");
  foot.classList.add("tx-foot");
  const feeInfo = document.createElement("span");
  const vsize = Math.ceil(tx.weight / 4);
  const feeRate = vsize > 0 ? tx.fee / vsize : 0;
  feeInfo.textContent = `${formatFeeRate(feeRate)} sat/vB \u2013 ${tx.fee.toLocaleString()} sats`;
  const total = document.createElement("span");
  total.classList.add("amount", "total");
  total.textContent = `${formatBtc(totalOut)} BTC`;
  foot.append(feeInfo, total);
  el.append(foot);

  return el;
}

/** @param {number} sats */
function formatBtc(sats) {
  return (sats / 1e8).toFixed(8);
}

/** @param {number} rate */
function formatFeeRate(rate) {
  if (rate >= 100) return Math.round(rate).toLocaleString();
  if (rate >= 10) return rate.toFixed(1);
  return rate.toFixed(2);
}

/** @param {string} text @param {HTMLElement} el */
function setAddrContent(text, el) {
  el.textContent = "";
  if (text.length <= 6) {
    el.textContent = text;
    return;
  }
  const head = document.createElement("span");
  head.classList.add("addr-head");
  head.textContent = text.slice(0, -6);
  const tail = document.createElement("span");
  tail.classList.add("addr-tail");
  tail.textContent = text.slice(-6);
  el.append(head, tail);
}

/** @param {number} height */
function createHeightElement(height) {
  const container = document.createElement("span");
  const str = height.toString();
  const prefix = document.createElement("span");
  prefix.style.opacity = "0.5";
  prefix.style.userSelect = "none";
  prefix.textContent = "#" + "0".repeat(7 - str.length);
  const num = document.createElement("span");
  num.textContent = str;
  container.append(prefix, num);
  return container;
}

/** @param {BlockInfoV1} block */
function createBlockCube(block) {
  const { cubeElement, leftFaceElement, rightFaceElement, topFaceElement } =
    createCube();

  cubeElement.dataset.hash = block.id;
  blocksByHash.set(block.id, block);
  cubeElement.addEventListener("click", () => selectCube(cubeElement));

  const heightEl = document.createElement("p");
  heightEl.append(createHeightElement(block.height));
  rightFaceElement.append(heightEl);

  const feesEl = document.createElement("div");
  feesEl.classList.add("fees");
  leftFaceElement.append(feesEl);
  const extras = block.extras;
  const medianFee = extras ? extras.medianFee : 0;
  const feeRange = extras ? extras.feeRange : [0, 0, 0, 0, 0, 0, 0];
  const avg = document.createElement("p");
  avg.innerHTML = `~${formatFeeRate(medianFee)}`;
  feesEl.append(avg);
  const range = document.createElement("p");
  const min = document.createElement("span");
  min.innerHTML = formatFeeRate(feeRange[0]);
  const dash = document.createElement("span");
  dash.style.opacity = "0.5";
  dash.innerHTML = `-`;
  const max = document.createElement("span");
  max.innerHTML = formatFeeRate(feeRange[6]);
  range.append(min, dash, max);
  feesEl.append(range);
  const unit = document.createElement("p");
  unit.style.opacity = "0.5";
  unit.innerHTML = `sat/vB`;
  feesEl.append(unit);

  const miner = document.createElement("span");
  miner.innerHTML = extras ? extras.pool.name : "Unknown";
  topFaceElement.append(miner);

  return cubeElement;
}

function createCube() {
  const cubeElement = document.createElement("div");
  cubeElement.classList.add("cube");

  const rightFaceElement = document.createElement("div");
  rightFaceElement.classList.add("face", "right");
  cubeElement.append(rightFaceElement);

  const leftFaceElement = document.createElement("div");
  leftFaceElement.classList.add("face", "left");
  cubeElement.append(leftFaceElement);

  const topFaceElement = document.createElement("div");
  topFaceElement.classList.add("face", "top");
  cubeElement.append(topFaceElement);

  return { cubeElement, leftFaceElement, rightFaceElement, topFaceElement };
}
