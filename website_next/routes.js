import { createBuildPage } from "./build/index.js";
import { createExplorePage } from "./explore/index.js";
import { createHomePage } from "./home/index.js";
import { createLearnPage } from "./learn/index.js";

/** @type {Record<string, () => HTMLElement>} */
const routes = {
  "/": createHomePage,
  "/explore": createExplorePage,
  "/learn": createLearnPage,
  "/build": createBuildPage,
};

/** @param {string} pathname */
function canonicalPath(pathname) {
  return pathname !== "/" && pathname.endsWith("/")
    ? pathname.slice(0, -1)
    : pathname;
}

/** @param {string} pathname */
export function resolvePath(pathname) {
  const path = canonicalPath(pathname);

  return path in routes ? path : undefined;
}

/** @param {string} pathname */
export function normalizePath(pathname) {
  return resolvePath(pathname) ?? "/";
}

/** @param {string} pathname */
export function createRoutePage(pathname) {
  return routes[pathname]();
}
