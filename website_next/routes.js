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
export function isRoute(pathname) {
  return pathname in routes;
}

/** @param {string} pathname */
export function normalizePath(pathname) {
  return isRoute(pathname) ? pathname : "/";
}

/** @param {string} pathname */
export function createRoutePage(pathname) {
  return routes[pathname]();
}
