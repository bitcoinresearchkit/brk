import { explorerElement } from "../utils/elements.js";
import { brk } from "../utils/client.js";
import { createMapCache } from "../utils/cache.js";
import {
  initChain,
  loadInitial,
  poll,
  selectCube,
  findCube,
  lastCube,
  clear as clearChain,
} from "./chain.js";
import {
  initBlockDetails,
  update as updateBlock,
  show as showBlock,
  hide as hideBlock,
} from "./block.js";
import { showTxFromData } from "./tx.js";
import { showAddrDetail } from "./address.js";

/** @returns {string[]} */
function pathSegments() {
  return window.location.pathname.split("/").filter((v) => v);
}

/** @type {HTMLDivElement} */ let secondaryPanel;
/** @type {number | undefined} */ let pollInterval;
/** @type {Transaction | null} */ let pendingTx = null;
let navController = new AbortController();
const txCache = createMapCache(50);
const addrCache = createMapCache(50);

function navigate() {
  navController.abort();
  navController = new AbortController();
  return navController.signal;
}

function showBlockPanel() {
  showBlock();
  secondaryPanel.hidden = true;
}

function showSecondaryPanel() {
  hideBlock();
  secondaryPanel.hidden = false;
}

/** @param {MouseEvent} e */
function handleLinkClick(e) {
  const a = /** @type {HTMLAnchorElement | null} */ (
    /** @type {HTMLElement} */ (e.target).closest("a[href]")
  );
  if (!a) return;
  const m = a.pathname.match(/^\/(block|tx|address)\/(.+)/);
  if (!m) return;
  e.preventDefault();
  if (m[1] === "block") {
    navigateToBlock(m[2]);
  } else if (m[1] === "tx") {
    history.pushState(null, "", a.href);
    navigateToTx(m[2]);
  } else {
    history.pushState(null, "", a.href);
    navigateToAddr(m[2]);
  }
}

export function init() {
  initChain(explorerElement, {
    onSelect: (block) => {
      updateBlock(block);
      showBlockPanel();
    },
    onCubeClick: (cube) => {
      const hash = cube.dataset.hash;
      if (hash) history.pushState(null, "", `/block/${hash}`);
      selectCube(cube);
    },
  });

  initBlockDetails(explorerElement, handleLinkClick);

  secondaryPanel = document.createElement("div");
  secondaryPanel.id = "tx-details";
  secondaryPanel.hidden = true;
  explorerElement.append(secondaryPanel);
  secondaryPanel.addEventListener("click", handleLinkClick);

  new MutationObserver(() => {
    if (explorerElement.hidden) stopPolling();
    else startPolling();
  }).observe(explorerElement, {
    attributes: true,
    attributeFilter: ["hidden"],
  });

  document.addEventListener("visibilitychange", () => {
    if (!document.hidden && !explorerElement.hidden) poll();
  });

  window.addEventListener("popstate", () => {
    const [kind, value] = pathSegments();
    if (kind === "block" && value) navigateToBlock(value, false);
    else if (kind === "tx" && value) navigateToTx(value);
    else if (kind === "address" && value) navigateToAddr(value);
    else showBlockPanel();
  });

  load();
}

function startPolling() {
  stopPolling();
  poll();
  pollInterval = setInterval(poll, 15_000);
}

function stopPolling() {
  if (pollInterval !== undefined) {
    clearInterval(pollInterval);
    pollInterval = undefined;
  }
}

async function load() {
  try {
    const height = await resolveStartHeight();
    await loadInitial(height);
    route();
  } catch (e) {
    console.error("explorer load:", e);
  }
}

/** @param {AbortSignal} [signal] @returns {Promise<number | null>} */
async function resolveStartHeight(signal) {
  const [kind, value] = pathSegments();
  if (!value) return null;
  if (kind === "block") {
    if (/^\d+$/.test(value)) return Number(value);
    return (await brk.getBlockV1(value, { signal })).height;
  }
  if (kind === "tx") {
    const tx = txCache.get(value) ?? (await brk.getTx(value, { signal }));
    txCache.set(value, tx);
    pendingTx = tx;
    return tx.status?.blockHeight ?? null;
  }
  return null;
}

function route() {
  const [kind, value] = pathSegments();
  if (pendingTx) {
    const hash = pendingTx.status?.blockHash;
    const cube = hash ? findCube(hash) : null;
    if (cube) selectCube(cube);
    showTxFromData(pendingTx, secondaryPanel);
    showSecondaryPanel();
    pendingTx = null;
  } else if (kind === "address" && value) {
    const cube = lastCube();
    if (cube) selectCube(cube);
    navigateToAddr(value);
  } else {
    const cube = lastCube();
    if (cube) selectCube(cube);
  }
}

/** @param {string} hash @param {boolean} [pushUrl] */
async function navigateToBlock(hash, pushUrl = true) {
  if (pushUrl) history.pushState(null, "", `/block/${hash}`);
  const cube = findCube(hash);
  if (cube) {
    selectCube(cube, { scroll: true });
  } else {
    const signal = navigate();
    try {
      clearChain();
      await loadInitial(await resolveStartHeight(signal));
      if (signal.aborted) return;
      route();
    } catch (e) {
      if (!signal.aborted) console.error("explorer block:", e);
    }
  }
}

/** @param {string} txid */
async function navigateToTx(txid) {
  const cached = txCache.get(txid);
  if (cached) {
    navigate();
    showTxAndSelectBlock(cached);
    return;
  }
  const signal = navigate();
  try {
    const tx = await brk.getTx(txid, {
      signal,
      onUpdate: (tx) => {
        txCache.set(txid, tx);
        if (!signal.aborted) showTxAndSelectBlock(tx);
      },
    });
    txCache.set(txid, tx);
  } catch (e) {
    if (!signal.aborted) console.error("explorer tx:", e);
  }
}

/** @param {Transaction} tx */
function showTxAndSelectBlock(tx) {
  if (tx.status?.blockHash) {
    const cube = findCube(tx.status.blockHash);
    if (cube) {
      selectCube(cube, { scroll: true });
      showTxFromData(tx, secondaryPanel);
      showSecondaryPanel();
      return;
    }
    pendingTx = tx;
    clearChain();
    loadInitial(tx.status.blockHeight ?? null).then(() => {
      if (!navController.signal.aborted) route();
    });
    return;
  }
  showTxFromData(tx, secondaryPanel);
  showSecondaryPanel();
}

/** @param {string} address */
function navigateToAddr(address) {
  const signal = navigate();
  showAddrDetail(address, secondaryPanel, { signal, cache: addrCache });
  showSecondaryPanel();
}
