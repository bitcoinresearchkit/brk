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

const navLinks = [...header.querySelectorAll("nav a")];

/** @param {string} pathname */
function updateCurrentLink(pathname) {
  const currentPath = normalizePath(pathname);

  for (const link of navLinks) {
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
}

function renderPage() {
  const pathname = normalizePath(window.location.pathname);
  activatePage(getPage(pathname));
  updateCurrentLink(pathname);
}

/** @param {string} pathname */
function navigate(pathname) {
  if (pathname === window.location.pathname) return;
  history.pushState(null, "", pathname);
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
  navigate(url.pathname);
});

window.addEventListener("popstate", renderPage);

renderPage();

requestAnimationFrame(() => {
  void revealPage();
});
