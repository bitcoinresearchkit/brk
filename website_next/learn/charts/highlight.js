/**
 * @param {HTMLElement[]} items
 * @param {HTMLElement} menu
 */
export function createSeriesHighlight(items, menu) {
  const seriesNodes = /** @type {SeriesNode[]} */ (items.map(() => []));
  /** @type {number | undefined} */
  let previewIndex;

  /** @param {number} index */
  function scrollToItem(index) {
    const itemRect = items[index].getBoundingClientRect();
    const menuRect = menu.getBoundingClientRect();

    if (itemRect.left < menuRect.left) {
      menu.scrollBy({
        left: itemRect.left - menuRect.left,
        behavior: "smooth",
      });
    } else if (itemRect.right > menuRect.right) {
      menu.scrollBy({
        left: itemRect.right - menuRect.right,
        behavior: "smooth",
      });
    }
  }

  /** @param {number} index */
  function activate(index) {
    for (const [itemIndex, item] of items.entries()) {
      setActive(item, itemIndex === index);
    }

    seriesNodes.forEach((nodes, nodeIndex) => {
      for (const node of nodes) {
        setActive(node, nodeIndex === index);
      }
    });
  }

  function clear() {
    for (const item of items) clearState(item);

    for (const nodes of seriesNodes) {
      for (const node of nodes) clearState(node);
    }

    previewIndex = undefined;
  }

  /** @param {number} index */
  function previewItem(index) {
    if (index === previewIndex) return;

    clearPreview();
    scrollToItem(index);
    items[index].dataset.preview = "";
    previewIndex = index;
  }

  function clearPreview() {
    if (previewIndex === undefined) return;

    delete items[previewIndex].dataset.preview;
    previewIndex = undefined;
  }

  items.forEach((item, index) => {
    item.addEventListener("pointerenter", () => activate(index));
    item.addEventListener("pointerleave", clear);
    item.addEventListener("focus", () => activate(index));
    item.addEventListener("blur", clear);
  });

  /**
   * @param {SVGPathElement | SVGCircleElement} node
   * @param {number} index
   */
  function addNode(node, index) {
    seriesNodes[index].push(node);
  }

  function clearNodes() {
    clear();

    for (const nodes of seriesNodes) {
      nodes.length = 0;
    }
  }

  return {
    addNode,
    clearPreview,
    clearNodes,
    preview: previewItem,
  };
}

/**
 * @param {HTMLElement | SVGElement} element
 * @param {boolean} active
 */
function setActive(element, active) {
  if (active) {
    element.dataset.active = "";
    delete element.dataset.muted;
  } else {
    delete element.dataset.active;
    element.dataset.muted = "";
  }
}

/** @param {HTMLElement | SVGElement} element */
function clearState(element) {
  delete element.dataset.active;
  delete element.dataset.muted;
  delete element.dataset.preview;
}

/** @typedef {(SVGPathElement | SVGCircleElement)[]} SeriesNode */

/**
 * @typedef {Object} SeriesHighlight
 * @property {(node: SVGPathElement | SVGCircleElement, index: number) => void} addNode
 * @property {() => void} clearPreview
 * @property {() => void} clearNodes
 * @property {(index: number) => void} preview
 */
