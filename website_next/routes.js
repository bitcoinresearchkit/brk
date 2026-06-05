import { createBuildPage } from "./build/index.js";
import { createExplorePage } from "./explore/index.js";
import { createHomePage } from "./home/index.js";
import { createLearnPage } from "./learn/index.js";

const pages = [
  { pathname: "/", createPage: createHomePage },
  { pathname: "/explore", label: "Explore", createPage: createExplorePage },
  { pathname: "/learn", label: "Learn", createPage: createLearnPage },
  { pathname: "/build", label: "Build", createPage: createBuildPage },
];

/** @type {Record<string, () => HTMLElement>} */
const routes = Object.fromEntries(
  pages.map(({ pathname, createPage }) => [pathname, createPage]),
);

export const primaryRoutes = pages.flatMap(({ pathname, label }) =>
  label ? [{ pathname, label }] : [],
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
