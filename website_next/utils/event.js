/**
 * @param {Event} event
 * @param {string} selector
 */
export function getEventTarget(event, selector) {
  return /** @type {HTMLElement | null} */ (
    /** @type {HTMLElement} */ (event.target).closest(selector)
  );
}

/** @param {Event} event */
export function getEventAnchor(event) {
  return /** @type {HTMLAnchorElement | null} */ (
    getEventTarget(event, "a[href]")
  );
}
