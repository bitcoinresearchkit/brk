import { explorerElement } from "../utils/elements.js";
import { brk } from "../utils/client.js";
import { createMapCache } from "../utils/cache.js";
import {
  initChain,
  loadInitial,
  poll,
  selectCube,
  deselectCube,
  findCube,
  clear as clearChain,
} from "./chain.js";
import {
  initBlockDetails,
  update as updateBlock,
  show as showBlock,
  hide as hideBlock,
} from "./block.js";
import {
  initTxDetails,
  update as updateTx,
  clear as clearTx,
  show as showTx,
  hide as hideTx,
} from "./tx.js";
import {
  initAddrDetails,
  update as updateAddr,
  show as showAddr,
  hide as hideAddr,
} from "./address.js";

/** @returns {string[]} */
function pathSegments() {
  return window.location.pathname.split("/").filter((v) => v);
}

/** @type {number | undefined} */ let pollInterval;
let navController = new AbortController();
const txCache = createMapCache(50);

function navigate() {
  navController.abort();
  navController = new AbortController();
  return navController.signal;
}

function showPanel(/** @type {"block" | "tx" | "addr"} */ which) {
  which === "block" ? showBlock() : hideBlock();
  which === "tx" ? showTx() : hideTx();
  which === "addr" ? showAddr() : hideAddr();
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
      showPanel("block");
    },
    onCubeClick: (cube) => {
      navigate();
      const hash = cube.dataset.hash;
      if (hash) history.pushState(null, "", `/block/${hash}`);
      selectCube(cube);
    },
  });

  initBlockDetails(explorerElement, handleLinkClick);
  initTxDetails(explorerElement, handleLinkClick);
  initAddrDetails(explorerElement, handleLinkClick);

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
    else showPanel("block");
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
    const [kind, value] = pathSegments();

    if (kind === "tx" && value) {
      const tx = txCache.get(value) ?? (await brk.getTx(value));
      txCache.set(value, tx);
      const startHash = await loadInitial(tx.status?.blockHeight ?? null);
      const cube = tx.status?.blockHash ? findCube(tx.status.blockHash) : findCube(startHash);
      if (cube) selectCube(cube, { silent: true });
      updateTx(tx);
      showPanel("tx");
      return;
    }

    if (kind === "address" && value) {
      const startHash = await loadInitial(null);
      const cube = findCube(startHash);
      if (cube) selectCube(cube, { silent: true });
      navigateToAddr(value);
      return;
    }

    const height =
      kind === "block" && value
        ? /^\d+$/.test(value)
          ? Number(value)
          : (await brk.getBlockV1(value)).height
        : null;
    const startHash = await loadInitial(height);
    const cube = findCube(startHash);
    if (cube) selectCube(cube, { scroll: "instant" });
  } catch (e) {
    console.error("explorer load:", e);
  }
}

/** @param {string} hash @param {boolean} [pushUrl] */
async function navigateToBlock(hash, pushUrl = true) {
  if (pushUrl) history.pushState(null, "", `/block/${hash}`);
  const existing = findCube(hash);
  if (existing) {
    navigate();
    selectCube(existing, { scroll: "smooth" });
    return;
  }
  const signal = navigate();
  try {
    clearChain();
    const height = /^\d+$/.test(hash)
      ? Number(hash)
      : (await brk.getBlockV1(hash, { signal })).height;
    if (signal.aborted) return;
    const startHash = await loadInitial(height);
    if (signal.aborted) return;
    const cube = findCube(hash) ?? findCube(startHash);
    if (cube) selectCube(cube);
  } catch (e) {
    if (!signal.aborted) console.error("explorer block:", e);
  }
}

/** @param {string} txid */
async function navigateToTx(txid) {
  const signal = navigate();
  clearTx();
  showPanel("tx");
  try {
    const tx = txCache.get(txid) ?? (await brk.getTx(txid, { signal }));
    if (signal.aborted) return;
    txCache.set(txid, tx);

    if (tx.status?.blockHash) {
      let cube = findCube(tx.status.blockHash);
      if (!cube) {
        clearChain();
        const startHash = await loadInitial(tx.status.blockHeight ?? null);
        if (signal.aborted) return;
        cube = findCube(tx.status.blockHash) ?? findCube(startHash);
      }
      if (cube) selectCube(cube, { scroll: "smooth", silent: true });
    }

    updateTx(tx);
  } catch (e) {
    if (!signal.aborted) console.error("explorer tx:", e);
  }
}

/** @param {string} address */
function navigateToAddr(address) {
  navigate();
  deselectCube();
  updateAddr(address, navController.signal);
  showPanel("addr");
}
