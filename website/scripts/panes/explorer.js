import { explorerElement } from "../utils/elements.js";
import { brk } from "../client.js";

/** @type {HTMLDivElement} */
let chain;

/** @type {HTMLDivElement} */
let sentinel;

let newestHeight = -1;
let oldestHeight = Infinity;
let loading = false;

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

  sentinel = window.document.createElement("div");
  sentinel.classList.add("sentinel");
  chain.append(sentinel);

  // Infinite scroll: load older blocks when sentinel becomes visible
  new IntersectionObserver((entries) => {
    if (entries[0].isIntersecting) {
      loadOlder();
    }
  }).observe(sentinel);

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

async function loadLatest() {
  if (loading) return;
  loading = true;
  try {
    const blocks = await brk.getBlocksV1();

    // First load: insert all blocks before sentinel
    if (newestHeight === -1) {
      for (const block of blocks) {
        sentinel.after(createBlockCube(block));
      }
      newestHeight = blocks[0].height;
      oldestHeight = blocks[blocks.length - 1].height;
    } else {
      // Subsequent polls: prepend only new blocks
      const newBlocks = blocks.filter((b) => b.height > newestHeight);
      if (newBlocks.length) {
        // sentinel.after(createBlockCube(block));
        sentinel.after(...newBlocks.map((b) => createBlockCube(b)));
        newestHeight = newBlocks[0].height;
      }
    }
  } catch (e) {
    console.error("explorer poll:", e);
  }
  loading = false;
}

async function loadOlder() {
  if (loading || oldestHeight <= 0) return;
  loading = true;
  try {
    const blocks = await brk.getBlocksV1FromHeight(oldestHeight - 1);
    for (const block of blocks) {
      sentinel.after(createBlockCube(block));
    }
    if (blocks.length) {
      oldestHeight = blocks[blocks.length - 1].height;
    }
  } catch (e) {
    console.error("explorer loadOlder:", e);
  }
  loading = false;
}

/** @param {BlockInfoV1} block */
function createBlockCube(block) {
  const { cubeElement, leftFaceElement, rightFaceElement, topFaceElement } =
    createCube();

  // cubeElement.style.setProperty("--face-color", `var(--${color})`);

  const heightElement = window.document.createElement("p");
  const height = block.height.toString();
  const prefixLength = 7 - height.length;
  const spanPrefix = window.document.createElement("span");
  spanPrefix.style.opacity = "0.5";
  spanPrefix.style.userSelect = "none";
  heightElement.append(spanPrefix);
  spanPrefix.innerHTML = "#" + "0".repeat(prefixLength);
  const spanHeight = window.document.createElement("span");
  heightElement.append(spanHeight);
  spanHeight.innerHTML = height;
  rightFaceElement.append(heightElement);

  const feesElement = window.document.createElement("div");
  feesElement.classList.add("fees");
  leftFaceElement.append(feesElement);
  const extras = block.extras;
  const medianFee = extras ? extras.medianFee : 0;
  const feeRange = extras ? extras.feeRange : [0, 0, 0, 0, 0, 0, 0];
  const averageFeeElement = window.document.createElement("p");
  feesElement.append(averageFeeElement);
  averageFeeElement.innerHTML = `~${Number(medianFee).toFixed(2)}`;
  const feeRangeElement = window.document.createElement("p");
  feesElement.append(feeRangeElement);
  const minFeeElement = window.document.createElement("span");
  minFeeElement.innerHTML = `${Number(feeRange[0]).toFixed(2)}`;
  feeRangeElement.append(minFeeElement);
  const dashElement = window.document.createElement("span");
  dashElement.style.opacity = "0.5";
  dashElement.innerHTML = `-`;
  feeRangeElement.append(dashElement);
  const maxFeeElement = window.document.createElement("span");
  maxFeeElement.innerHTML = `${Number(feeRange[6]).toFixed(1)}`;
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
