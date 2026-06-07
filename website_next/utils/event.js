/**
 * @param {Event} event
 * @param {string} selector
 */
export function getEventTarget(event, selector) {
  const target = event.target;

  return target instanceof Element ? target.closest(selector) : null;
}

/** @param {Event} event */
export function getEventAnchor(event) {
  return /** @type {HTMLAnchorElement | null} */ (
    getEventTarget(event, "a[href]")
  );
}

/** @param {MouseEvent} event */
export function isPlainLeftClick(event) {
  return (
    event.button === 0 &&
    !event.altKey &&
    !event.metaKey &&
    !event.ctrlKey &&
    !event.shiftKey
  );
}
