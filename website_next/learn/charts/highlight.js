/** @returns {SeriesNode} */
function createSeriesNode() {
  return [];
}

/**
 * @param {HTMLElement[]} items
 */
export function createSeriesHighlight(items) {
  const seriesNodes = items.map(createSeriesNode);

  /** @param {number} index */
  function scrollToItem(index) {
    items[index].scrollIntoView({
      behavior: "smooth",
      block: "nearest",
      inline: "nearest",
    });
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
  }

  /** @param {number} index */
  function previewItem(index) {
    scrollToItem(index);
    items[index].dataset.preview = "";
  }

  /** @param {number} index */
  function clearPreview(index) {
    delete items[index].dataset.preview;
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
  function add(node, index) {
    seriesNodes[index].push(node);
  }

  function clearNodes() {
    clear();

    for (const nodes of seriesNodes) {
      nodes.length = 0;
    }
  }

  return {
    add,
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
 * @property {(node: SVGPathElement | SVGCircleElement, index: number) => void} add
 * @property {(index: number) => void} clearPreview
 * @property {() => void} clearNodes
 * @property {(index: number) => void} preview
 */
