/**
 * @param {HTMLElement[]} items
 * @param {HTMLElement} menu
 */
export function createSeriesHighlight(items, menu) {
  const seriesNodes = /** @type {SeriesNode[]} */ (items.map(() => []));
  const noSeries = -1;
  let selectedSeries = noSeries;
  let previewedSeries = noSeries;

  /** @param {number} index */
  function scrollToItem(index) {
    const margin = Number.parseFloat(getComputedStyle(menu).paddingLeft);
    const itemRect = items[index].getBoundingClientRect();
    const menuRect = menu.getBoundingClientRect();

    if (itemRect.left < menuRect.left + margin) {
      menu.scrollBy({
        left: itemRect.left - menuRect.left - margin,
        behavior: "smooth",
      });
    } else if (itemRect.right > menuRect.right - margin) {
      menu.scrollBy({
        left: itemRect.right - menuRect.right + margin,
        behavior: "smooth",
      });
    }
  }

  /** @param {number} index */
  function highlightSeries(index) {
    for (const [itemIndex, item] of items.entries()) {
      setActive(item, itemIndex === index);
    }

    seriesNodes.forEach((nodes, nodeIndex) => {
      for (const node of nodes) {
        setActive(node, nodeIndex === index);
      }
    });
  }

  function clearHighlight() {
    for (const item of items) clearElementState(item);

    for (const nodes of seriesNodes) {
      for (const node of nodes) clearElementState(node);
    }
  }

  function restoreSelectedHighlight() {
    if (selectedSeries === noSeries) {
      clearHighlight();
    } else {
      highlightSeries(selectedSeries);
    }
  }

  function clearInteractionHighlight() {
    clearPreview();
    restoreSelectedHighlight();
  }

  /** @param {number} index */
  function selectSeries(index) {
    selectedSeries = index;

    items.forEach((item, itemIndex) => {
      item.setAttribute(
        "aria-pressed",
        (itemIndex === selectedSeries).toString(),
      );
    });

    restoreSelectedHighlight();
  }

  /** @param {number} index */
  function previewSeries(index) {
    if (index === previewedSeries) return;

    clearPreview();
    scrollToItem(index);
    items[index].dataset.preview = "";
    previewedSeries = index;
  }

  function clearPreview() {
    if (previewedSeries === noSeries) return;

    delete items[previewedSeries].dataset.preview;
    previewedSeries = noSeries;
  }

  items.forEach((item, index) => {
    item.setAttribute("aria-pressed", "false");
    item.addEventListener("pointerenter", () => highlightSeries(index));
    item.addEventListener("pointerleave", clearInteractionHighlight);
    item.addEventListener("focus", () => highlightSeries(index));
    item.addEventListener("blur", clearInteractionHighlight);
    item.addEventListener("click", () => {
      selectSeries(selectedSeries === index ? noSeries : index);
    });
  });

  /**
   * @param {SVGPathElement | SVGCircleElement} node
   * @param {number} index
   */
  function addNode(node, index) {
    if (selectedSeries !== noSeries) setActive(node, index === selectedSeries);
    seriesNodes[index].push(node);
  }

  function clearNodes() {
    clearInteractionHighlight();

    for (const nodes of seriesNodes) {
      nodes.length = 0;
    }
  }

  return {
    addNode,
    clearPreview,
    clearNodes,
    preview: previewSeries,
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
function clearElementState(element) {
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
