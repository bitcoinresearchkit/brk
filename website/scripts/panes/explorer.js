import { explorerElement } from "../utils/elements.js";
import { brk } from "../client.js";

/** @type {HTMLDivElement} */
let chain;

/** @type {HTMLDivElement} */
let blocksEl;

/** @type {HTMLDivElement} */
let details;

/** @type {HTMLDivElement} */
let olderSentinel;

/** @type {HTMLDivElement} */
let newerSentinel;

/** @type {Map<BlockHash, BlockInfoV1>} */
const blocksByHash = new Map();

let newestHeight = -1;
let oldestHeight = Infinity;
let loadingLatest = false;
let loadingOlder = false;
let loadingNewer = false;
let reachedTip = false;

/** @type {HTMLDivElement | null} */
let selectedCube = null;

/** @type {number | undefined} */
let pollInterval;

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

export function init() {
  chain = window.document.createElement("div");
  chain.id = "chain";
  explorerElement.append(chain);

  newerSentinel = window.document.createElement("div");
  newerSentinel.classList.add("sentinel");
  chain.append(newerSentinel);

  blocksEl = window.document.createElement("div");
  blocksEl.classList.add("blocks");
  chain.append(blocksEl);

  details = window.document.createElement("div");
  details.id = "block-details";
  explorerElement.append(details);

  olderSentinel = window.document.createElement("div");
  olderSentinel.classList.add("sentinel");
  blocksEl.append(olderSentinel);

  function checkSentinels() {
    const p = chain.getBoundingClientRect();
    const older = olderSentinel.getBoundingClientRect();
    if (older.top < p.bottom + 200 && older.bottom > p.top) {
      loadOlder();
    }
    const newer = newerSentinel.getBoundingClientRect();
    if (newer.bottom > p.top - 200 && newer.top < p.bottom) {
      loadNewer();
    }
  }

  chain.addEventListener("scroll", checkSentinels, { passive: true });

  // Self-contained lifecycle: poll when visible, stop when hidden
  new MutationObserver(() => {
    if (explorerElement.hidden) {
      stopPolling();
    } else {
      startPolling();
    }
  }).observe(explorerElement, {
    attributes: true,
    attributeFilter: ["hidden"],
  });

  document.addEventListener("visibilitychange", () => {
    if (!document.hidden && !explorerElement.hidden) {
      loadLatest();
    }
  });

  loadLatest();
}

/** @returns {Promise<number | null>} */
async function getStartHeight() {
  const path = window.location.pathname.split("/").filter((v) => v);
  if (path[0] !== "block" || !path[1]) return null;
  const value = path[1];
  if (/^\d+$/.test(value)) return Number(value);
  const block = await brk.getBlockV1(value);
  return block.height;
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

    // First load: insert all blocks between sentinels
    if (newestHeight === -1) {
      const cubes = blocks.map((b) => createBlockCube(b));
      for (const cube of cubes) {
        olderSentinel.after(cube);
      }
      newestHeight = blocks[0].height;
      oldestHeight = blocks[blocks.length - 1].height;
      if (startHeight === null) reachedTip = true;
      selectCube(cubes[0]);
      if (!reachedTip) {
        newerSentinel.style.minHeight = chain.clientHeight + "px";
        requestAnimationFrame(() => {
          if (selectedCube) {
            selectedCube.scrollIntoView({
              behavior: "instant",
              block: "center",
            });
          }
        });
      }
      loadingLatest = false;
      return;
    } else {
      // Subsequent polls: append newer blocks to blocksEl
      const newBlocks = blocks.filter((b) => b.height > newestHeight);
      if (newBlocks.length) {
        newBlocks.sort((a, b) => a.height - b.height);
        for (const b of newBlocks) {
          blocksEl.append(createBlockCube(b));
        }
        newestHeight = newBlocks[newBlocks.length - 1].height;
      }
      reachedTip = true;
    }
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
    for (const block of blocks) {
      olderSentinel.after(createBlockCube(block));
    }
    if (blocks.length) {
      oldestHeight = blocks[blocks.length - 1].height;
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
    const blocks = await brk.getBlocksV1FromHeight(newestHeight + 15);
    const newer = blocks.filter((b) => b.height > newestHeight);
    if (newer.length) {
      newer.sort((a, b) => a.height - b.height);
      for (const b of newer) {
        blocksEl.append(createBlockCube(b));
      }
      newestHeight = newer[newer.length - 1].height;
    } else {
      reachedTip = true;
      newerSentinel.style.minHeight = "";
    }
  } catch (e) {
    console.error("explorer loadNewer:", e);
  }
  loadingNewer = false;
}

/** @param {HTMLDivElement} cube */
function selectCube(cube) {
  if (selectedCube) {
    selectedCube.classList.remove("selected");
  }
  selectedCube = cube;
  if (cube) {
    cube.classList.add("selected");
    const hash = cube.dataset.hash;
    if (hash) {
      renderDetails(blocksByHash.get(hash));
    }
  }
}

/** @param {BlockInfoV1 | undefined} block */
function renderDetails(block) {
  details.innerHTML = "";
  if (!block) return;

  const title = window.document.createElement("h1");
  title.textContent = "Block ";
  const titleCode = window.document.createElement("code");
  titleCode.append(createHeightElement(block.height));
  title.append(titleCode);
  details.append(title);

  const extras = block.extras;

  /** @type {[string, string][]} */
  const rows = [
    ["Hash", block.id],
    ["Previous Hash", block.previousblockhash],
    ["Merkle Root", block.merkleRoot],
    ["Timestamp", new Date(block.timestamp * 1000).toUTCString()],
    ["Median Time", new Date(block.mediantime * 1000).toUTCString()],
    ["Version", `0x${block.version.toString(16)}`],
    ["Bits", block.bits.toString(16)],
    ["Nonce", block.nonce.toLocaleString()],
    ["Difficulty", Number(block.difficulty).toLocaleString()],
    ["Size", `${(block.size / 1_000_000).toFixed(2)} MB`],
    ["Weight", `${(block.weight / 1_000_000).toFixed(2)} MWU`],
    ["Transactions", block.txCount.toLocaleString()],
  ];

  if (extras) {
    rows.push(
      ["Price", `$${extras.price.toLocaleString()}`],
      ["Pool", extras.pool.name],
      ["Pool ID", extras.pool.id.toString()],
      ["Pool Slug", extras.pool.slug],
      ["Miner Names", extras.pool.minerNames?.join(", ") || "N/A"],
      ["Reward", `${(extras.reward / 1e8).toFixed(8)} BTC`],
      ["Total Fees", `${(extras.totalFees / 1e8).toFixed(8)} BTC`],
      ["Median Fee Rate", `${formatFeeRate(extras.medianFee)} sat/vB`],
      ["Avg Fee Rate", `${formatFeeRate(extras.avgFeeRate)} sat/vB`],
      ["Avg Fee", `${extras.avgFee.toLocaleString()} sat`],
      ["Median Fee", `${extras.medianFeeAmt.toLocaleString()} sat`],
      [
        "Fee Range",
        extras.feeRange.map((f) => formatFeeRate(f)).join(", ") + " sat/vB",
      ],
      [
        "Fee Percentiles",
        extras.feePercentiles.map((f) => f.toLocaleString()).join(", ") +
          " sat",
      ],
      ["Avg Tx Size", `${extras.avgTxSize.toLocaleString()} B`],
      ["Virtual Size", `${extras.virtualSize.toLocaleString()} vB`],
      ["Inputs", extras.totalInputs.toLocaleString()],
      ["Outputs", extras.totalOutputs.toLocaleString()],
      ["Total Input Amount", `${(extras.totalInputAmt / 1e8).toFixed(8)} BTC`],
      [
        "Total Output Amount",
        `${(extras.totalOutputAmt / 1e8).toFixed(8)} BTC`,
      ],
      ["UTXO Set Change", extras.utxoSetChange.toLocaleString()],
      ["UTXO Set Size", extras.utxoSetSize.toLocaleString()],
      ["SegWit Txs", extras.segwitTotalTxs.toLocaleString()],
      ["SegWit Size", `${extras.segwitTotalSize.toLocaleString()} B`],
      ["SegWit Weight", `${extras.segwitTotalWeight.toLocaleString()} WU`],
      ["Coinbase Address", extras.coinbaseAddress || "N/A"],
      ["Coinbase Addresses", extras.coinbaseAddresses.join(", ") || "N/A"],
      ["Coinbase Raw", extras.coinbaseRaw],
      ["Coinbase Signature", extras.coinbaseSignature],
      ["Coinbase Signature ASCII", extras.coinbaseSignatureAscii],
      ["Header", extras.header],
    );
  }

  for (const [label, value] of rows) {
    const row = window.document.createElement("div");
    row.classList.add("row");
    const labelElement = window.document.createElement("span");
    labelElement.classList.add("label");
    labelElement.textContent = label;
    const valueElement = window.document.createElement("span");
    valueElement.classList.add("value");
    valueElement.textContent = value;
    row.append(labelElement, valueElement);
    details.append(row);
  }
}

/** @param {number} rate */
function formatFeeRate(rate) {
  if (rate >= 100) return Math.round(rate).toLocaleString();
  if (rate >= 10) return rate.toFixed(1);
  return rate.toFixed(2);
}

/** @param {number} height */
function createHeightElement(height) {
  const container = window.document.createElement("span");
  const str = height.toString();
  const spanPrefix = window.document.createElement("span");
  spanPrefix.style.opacity = "0.5";
  spanPrefix.style.userSelect = "none";
  spanPrefix.textContent = "#" + "0".repeat(7 - str.length);
  const spanHeight = window.document.createElement("span");
  spanHeight.textContent = str;
  container.append(spanPrefix, spanHeight);
  return container;
}

/** @param {BlockInfoV1} block */
function createBlockCube(block) {
  const { cubeElement, leftFaceElement, rightFaceElement, topFaceElement } =
    createCube();

  cubeElement.dataset.hash = block.id;
  blocksByHash.set(block.id, block);
  cubeElement.addEventListener("click", () => selectCube(cubeElement));

  const heightElement = window.document.createElement("p");
  heightElement.append(createHeightElement(block.height));
  rightFaceElement.append(heightElement);

  const feesElement = window.document.createElement("div");
  feesElement.classList.add("fees");
  leftFaceElement.append(feesElement);
  const extras = block.extras;
  const medianFee = extras ? extras.medianFee : 0;
  const feeRange = extras ? extras.feeRange : [0, 0, 0, 0, 0, 0, 0];
  const averageFeeElement = window.document.createElement("p");
  feesElement.append(averageFeeElement);
  averageFeeElement.innerHTML = `~${formatFeeRate(medianFee)}`;
  const feeRangeElement = window.document.createElement("p");
  feesElement.append(feeRangeElement);
  const minFeeElement = window.document.createElement("span");
  minFeeElement.innerHTML = formatFeeRate(feeRange[0]);
  feeRangeElement.append(minFeeElement);
  const dashElement = window.document.createElement("span");
  dashElement.style.opacity = "0.5";
  dashElement.innerHTML = `-`;
  feeRangeElement.append(dashElement);
  const maxFeeElement = window.document.createElement("span");
  maxFeeElement.innerHTML = formatFeeRate(feeRange[6]);
  feeRangeElement.append(maxFeeElement);
  const feeUnitElement = window.document.createElement("p");
  feesElement.append(feeUnitElement);
  feeUnitElement.style.opacity = "0.5";
  feeUnitElement.innerHTML = `sat/vB`;

  const spanMiner = window.document.createElement("span");
  spanMiner.innerHTML = extras ? extras.pool.name : "Unknown";
  topFaceElement.append(spanMiner);

  return cubeElement;
}

function createCube() {
  const cubeElement = window.document.createElement("div");
  cubeElement.classList.add("cube");

  const rightFaceElement = window.document.createElement("div");
  rightFaceElement.classList.add("face");
  rightFaceElement.classList.add("right");
  cubeElement.append(rightFaceElement);

  const leftFaceElement = window.document.createElement("div");
  leftFaceElement.classList.add("face");
  leftFaceElement.classList.add("left");
  cubeElement.append(leftFaceElement);

  const topFaceElement = window.document.createElement("div");
  topFaceElement.classList.add("face");
  topFaceElement.classList.add("top");
  cubeElement.append(topFaceElement);

  return {
    cubeElement,
    leftFaceElement,
    rightFaceElement,
    topFaceElement,
  };
}
