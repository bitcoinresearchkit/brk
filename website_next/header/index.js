import { createCube } from "../cube/index.js";
import { primaryRoutes } from "../routes.js";

export function createHeader() {
  const header = document.createElement("header");

  const home = document.createElement("a");
  const cube = document.createElement("span");

  home.href = "/";
  home.ariaLabel = "bitview home";
  cube.append(createCube());
  home.append(cube, "bitview");

  const nav = document.createElement("nav");
  const list = document.createElement("ul");
  nav.setAttribute("aria-label", "Primary");

  for (const { pathname, label } of primaryRoutes) {
    const item = document.createElement("li");
    const anchor = document.createElement("a");

    anchor.href = pathname;
    anchor.append(label);
    item.append(anchor);
    list.append(item);
  }

  nav.append(list);
  header.append(home, nav);
  return header;
}
