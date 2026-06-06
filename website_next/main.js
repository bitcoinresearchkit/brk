import { createHeader } from "./header/index.js";
import { createRoutePage, isRoute, normalizePath } from "./routes.js";
import { getEventAnchor, isPlainLeftClick } from "./utils/event.js";
import { revealPage, transitionPage } from "./utils/transition.js";

/** @type {HTMLElement | undefined} */
let currentPage;

/** @type {Map<string, HTMLElement>} */
const pageByPath = new Map();

const header = createHeader();
document.body.append(header);

/** @param {string} pathname */
function getPage(pathname) {
  let page = pageByPath.get(pathname);

  if (!page) {
    page = createRoutePage(pathname);
    page.hidden = true;
    page.inert = true;
    pageByPath.set(pathname, page);
    document.body.append(page);
  }

  return page;
}

/** @param {HTMLElement} page */
function activatePage(page) {
  if (currentPage) {
    currentPage.hidden = true;
    currentPage.inert = true;
  }

  page.hidden = false;
  page.inert = false;
  currentPage = page;
  page.dispatchEvent(new Event("pageactive"));
}

function renderPage() {
  const pathname = normalizePath(window.location.pathname);
  activatePage(getPage(pathname));
}

/** @param {string} path */
function navigate(path) {
  if (path === `${window.location.pathname}${window.location.hash}`) return;
  history.pushState(null, "", path);
  void transitionPage(renderPage);
}

document.addEventListener("click", (event) => {
  if (!isPlainLeftClick(event)) return;

  const anchor = getEventAnchor(event);
  if (!anchor) return;

  const url = new URL(anchor.href);
  if (url.origin !== window.location.origin) return;
  if (url.pathname === window.location.pathname && url.hash) return;

  if (!isRoute(url.pathname)) return;

  event.preventDefault();
  navigate(`${url.pathname}${url.hash}`);
});

window.addEventListener("popstate", renderPage);

renderPage();

requestAnimationFrame(() => {
  void revealPage();
});
