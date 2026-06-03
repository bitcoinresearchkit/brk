import "./header/index.js";
import { createBuildPage } from "./build/index.js";
import { createExplorePage } from "./explore/index.js";
import { createHomePage } from "./home/index.js";
import { createLearnPage } from "./learn/index.js";

/** @type {Record<string, () => HTMLElement>} */
const routes = {
  "/": createHomePage,
  "/build": createBuildPage,
  "/explore": createExplorePage,
  "/learn": createLearnPage,
};

/** @type {HTMLElement | undefined} */
let currentPage;

/** @type {Map<string, HTMLElement>} */
const pageByPath = new Map();

/** @param {string} pathname */
function normalizePath(pathname) {
  return pathname in routes ? pathname : "/";
}

/** @param {string} pathname */
function updateCurrentLink(pathname) {
  const currentPath = normalizePath(pathname);

  for (const link of document.querySelectorAll("body > header > nav a")) {
    const linkPath = new URL(/** @type {HTMLAnchorElement} */ (link).href)
      .pathname;

    if (linkPath === currentPath) {
      link.setAttribute("aria-current", "page");
    } else {
      link.removeAttribute("aria-current");
    }
  }
}

/** @param {string} pathname */
function getPage(pathname) {
  let page = pageByPath.get(pathname);

  if (!page) {
    page = routes[pathname]();
    page.inert = true;
    pageByPath.set(pathname, page);
    document.body.append(page);
  }

  return page;
}

/** @param {HTMLElement} page */
function activatePage(page) {
  if (currentPage) {
    currentPage.inert = true;
    delete currentPage.dataset.active;
  }

  page.inert = false;
  page.dataset.active = "";
  currentPage = page;
}

function renderPage() {
  const pathname = normalizePath(window.location.pathname);
  activatePage(getPage(pathname));
  updateCurrentLink(pathname);
}

/** @param {string} pathname */
function navigate(pathname) {
  if (pathname !== window.location.pathname) {
    history.pushState(null, "", pathname);
  }

  renderPage();
}

document.addEventListener("click", (event) => {
  if (event.metaKey || event.ctrlKey || event.shiftKey || event.button !== 0) {
    return;
  }

  const anchor = /** @type {HTMLAnchorElement | null} */ (
    /** @type {HTMLElement} */ (event.target).closest("a[href]")
  );
  if (!anchor) return;

  const url = new URL(anchor.href);
  if (url.origin !== window.location.origin) return;
  if (url.pathname === window.location.pathname && url.hash) return;

  if (!(url.pathname in routes)) return;

  event.preventDefault();
  navigate(url.pathname);
});

window.addEventListener("popstate", renderPage);

renderPage();

requestAnimationFrame(() => {
  setTimeout(() => {
    delete document.documentElement.dataset.loading;
  }, 150);
});
