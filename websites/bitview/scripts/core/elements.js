/**
 * @param {string} id
 */
function getElementById(id) {
  const element = window.document.getElementById(id);
  if (!element) throw `Element with id = "${id}" should exist`;
  return element;
}

export default {
  head: window.document.getElementsByTagName("head")[0],
  body: window.document.body,
  main: getElementById("main"),
  aside: getElementById("aside"),
  asideLabel: getElementById("aside-selector-label"),
  navLabel: getElementById(`nav-selector-label`),
  searchLabel: getElementById(`search-selector-label`),
  search: getElementById("search"),
  nav: getElementById("nav"),
  searchInput: /** @type {HTMLInputElement} */ (getElementById("search-input")),
  searchResults: getElementById("search-results"),
  selectors: getElementById("frame-selectors"),
  style: getComputedStyle(window.document.documentElement),
  charts: getElementById("charts"),
  table: getElementById("table"),
  explorer: getElementById("explorer"),
  simulation: getElementById("simulation"),
};
