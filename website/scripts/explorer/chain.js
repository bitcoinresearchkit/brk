import { brk } from "../utils/client.js";
import { createHeightElement, formatFeeRate } from "./render.js";

const LOOKAHEAD = 15;

/** @type {HTMLDivElement} */ let chainEl;
/** @type {HTMLDivElement} */ let blocksEl;
/** @type {HTMLDivElement | null} */ let selectedCube = null;
/** @type {IntersectionObserver} */ let olderObserver;
/** @type {(block: BlockInfoV1) => void} */ let onSelect = () => {};
/** @type {(cube: HTMLDivElement) => void} */ let onCubeClick = () => {};

/** @type {Map<BlockHash, BlockInfoV1>} */
const blocksByHash = new Map();

let newestHeight = -1;
let oldestHeight = Infinity;
let loadingOlder = false;
let loadingNewer = false;
let reachedTip = false;

/**
 * @param {HTMLElement} parent
 * @param {{ onSelect: (block: BlockInfoV1) => void, onCubeClick: (cube: HTMLDivElement) => void }} callbacks
 */
export function initChain(parent, callbacks) {
  onSelect = callbacks.onSelect;
  onCubeClick = callbacks.onCubeClick;

  chainEl = document.createElement("div");
  chainEl.id = "chain";
  parent.append(chainEl);

  blocksEl = document.createElement("div");
  blocksEl.classList.add("blocks");
  chainEl.append(blocksEl);

  olderObserver = new IntersectionObserver(
    (entries) => {
      if (entries[0].isIntersecting) loadOlder();
    },
    { root: chainEl },
  );

  chainEl.addEventListener(
    "scroll",
    () => {
      const nearStart =
        (chainEl.scrollHeight > chainEl.clientHeight &&
          chainEl.scrollTop <= 50) ||
        (chainEl.scrollWidth > chainEl.clientWidth &&
          chainEl.scrollLeft <= 50);
      if (nearStart && !reachedTip && !loadingNewer) loadNewer();
    },
    { passive: true },
  );
}

/** @param {string} hash */
export function getBlock(hash) {
  return blocksByHash.get(hash);
}

/** @param {string} hash */
export function findCube(hash) {
  return /** @type {HTMLDivElement | null} */ (
    blocksEl.querySelector(`[data-hash="${hash}"]`)
  );
}

export function lastCube() {
  return /** @type {HTMLDivElement | null} */ (blocksEl.lastElementChild);
}

/** @param {HTMLDivElement} cube @param {{ scroll?: boolean }} [opts] */
export function selectCube(cube, { scroll = false } = {}) {
  const changed = cube !== selectedCube;
  if (changed) {
    if (selectedCube) selectedCube.classList.remove("selected");
    selectedCube = cube;
    cube.classList.add("selected");
  }
  if (scroll) cube.scrollIntoView({ behavior: "smooth" });
  const hash = cube.dataset.hash;
  if (hash) {
    const block = blocksByHash.get(hash);
    if (block) onSelect(block);
  }
}

export function clear() {
  newestHeight = -1;
  oldestHeight = Infinity;
  loadingOlder = false;
  loadingNewer = false;
  reachedTip = false;
  selectedCube = null;
  blocksEl.innerHTML = "";
  olderObserver.disconnect();
}

function observeOldestEdge() {
  olderObserver.disconnect();
  const oldest = blocksEl.firstElementChild;
  if (oldest) olderObserver.observe(oldest);
}

/** @param {BlockInfoV1[]} blocks */
function appendNewerBlocks(blocks) {
  if (!blocks.length) return false;
  const anchor = blocksEl.lastElementChild;
  const anchorRect = anchor?.getBoundingClientRect();
  for (let i = blocks.length - 1; i >= 0; i--) {
    const b = blocks[i];
    if (b.height > newestHeight) {
      blocksEl.append(createBlockCube(b));
    } else {
      blocksByHash.set(b.id, b);
    }
  }
  newestHeight = Math.max(newestHeight, blocks[0].height);
  if (anchor && anchorRect) {
    const r = anchor.getBoundingClientRect();
    chainEl.scrollTop += r.top - anchorRect.top;
    chainEl.scrollLeft += r.left - anchorRect.left;
  }
  return true;
}

/** @param {number | null} [height] */
export async function loadInitial(height) {
  const blocks =
    height != null
      ? await brk.getBlocksV1FromHeight(height)
      : await brk.getBlocksV1();

  for (const b of blocks) blocksEl.prepend(createBlockCube(b));
  newestHeight = blocks[0].height;
  oldestHeight = blocks[blocks.length - 1].height;
  reachedTip = height == null;
  observeOldestEdge();
  if (!reachedTip) await loadNewer();
}

export async function poll() {
  if (newestHeight === -1 || !reachedTip) return;
  try {
    const blocks = await brk.getBlocksV1();
    appendNewerBlocks(blocks);
  } catch (e) {
    console.error("explorer poll:", e);
  }
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
    const blocks = await brk.getBlocksV1FromHeight(newestHeight + LOOKAHEAD);
    if (!appendNewerBlocks(blocks)) reachedTip = true;
  } catch (e) {
    console.error("explorer loadNewer:", e);
  }
  loadingNewer = false;
}

/** @param {BlockInfoV1} block */
function createBlockCube(block) {
  const { cubeElement, leftFaceElement, rightFaceElement, topFaceElement } =
    createCube();

  cubeElement.dataset.hash = block.id;
  blocksByHash.set(block.id, block);
  cubeElement.addEventListener("click", () => onCubeClick(cubeElement));

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
