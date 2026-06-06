import { createBuildPage } from "./build/index.js";
import { createExplorePage } from "./explore/index.js";
import { createHomePage } from "./home/index.js";
import { createLearnPage } from "./learn/index.js";

const pages = [
  { pathname: "/", createPage: createHomePage },
  { pathname: "/explore", createPage: createExplorePage },
  { pathname: "/learn", createPage: createLearnPage },
  { pathname: "/build", createPage: createBuildPage },
];

/** @type {Record<string, () => HTMLElement>} */
const routes = Object.fromEntries(
  pages.map(({ pathname, createPage }) => [pathname, createPage]),
);

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
