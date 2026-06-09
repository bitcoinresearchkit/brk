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

/**
 * @param {HTMLElement} main
 * @param {ScrollBehavior} behavior
 */
function scrollToCurrentHash(main, behavior) {
  const target = getHashTarget(main, window.location.hash);

  if (target) scrollToTarget(target, behavior);
}

/**
 * @param {HTMLElement} main
 * @param {(hash: string) => void} onHashNavigate
 */
export function initHashLinks(main, onHashNavigate) {
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
    onHashNavigate(url.hash);
    scrollToTarget(target, "smooth");

    if (url.hash !== window.location.hash) {
      history.pushState(null, "", url.hash);
    }
  });

  window.addEventListener("popstate", () => {
    if (main.hidden) return;
    scrollToCurrentHash(main, "auto");
  });

  main.addEventListener("pageactive", () => scrollToCurrentHash(main, "auto"));
}
