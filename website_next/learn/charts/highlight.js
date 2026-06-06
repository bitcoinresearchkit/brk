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
  function activateItem(index) {
    setActive(items[index], true);
  }

  /** @param {number} index */
  function clearItem(index) {
    clearState(items[index]);
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
    node.addEventListener("pointerenter", () => {
      scrollToItem(index);
      activateItem(index);
    });
    node.addEventListener("pointerleave", () => clearItem(index));
  }

  function clearNodes() {
    clear();

    for (const nodes of seriesNodes) {
      nodes.length = 0;
    }
  }

  return {
    add,
    clearNodes,
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
}

/** @typedef {(SVGPathElement | SVGCircleElement)[]} SeriesNode */

/**
 * @typedef {Object} SeriesHighlight
 * @property {(node: SVGPathElement | SVGCircleElement, index: number) => void} add
 * @property {() => void} clearNodes
 */
