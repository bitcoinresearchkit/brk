import { createCube } from "../cube/index.js";

export function createHeader() {
  const header = document.createElement("header");

  const home = document.createElement("a");
  const cube = document.createElement("span");

  home.href = "/";
  home.ariaLabel = "bitview home";
  cube.append(createCube());
  home.append(cube, "bitview");

  header.append(home);
  return header;
}
