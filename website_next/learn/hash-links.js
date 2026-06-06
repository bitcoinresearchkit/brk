import { getEventAnchor, isPlainLeftClick } from "../utils/event.js";

/**
 * @param {HTMLElement} main
 * @param {string} hash
 */
function getHashTarget(main, hash) {
  const target = document.getElementById(hash.slice(1));

  return target && main.contains(target) ? target : null;
}

/**
 * @param {Element} target
 * @param {ScrollBehavior} behavior
 */
function scrollToTarget(target, behavior) {
  target.scrollIntoView({ behavior, block: "start" });
}

/** @param {HTMLElement} main */
export function initHashLinks(main) {
  const initialHash = window.location.hash;

  main.addEventListener("click", (event) => {
    if (!isPlainLeftClick(event)) return;

    const anchor = getEventAnchor(event);
    if (!anchor) return;

    const url = new URL(anchor.href);
    if (url.origin !== window.location.origin) return;
    if (url.pathname !== window.location.pathname || !url.hash) return;

    const target = getHashTarget(main, url.hash);
    if (!target) return;

    event.preventDefault();
    scrollToTarget(target, "smooth");

    if (url.hash !== window.location.hash) {
      history.pushState(null, "", url.hash);
    }
  });

  window.addEventListener("popstate", () => {
    if (main.hidden) return;
    const target = getHashTarget(main, window.location.hash);
    if (target) scrollToTarget(target, "auto");
  });

  requestAnimationFrame(() => {
    const target = getHashTarget(main, initialHash);
    if (target) scrollToTarget(target, "auto");
  });
}
