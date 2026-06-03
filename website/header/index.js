import { createCube } from "../cube/index.js";

const header = document.createElement("header");

const home = document.createElement("a");
home.href = "/";
home.ariaLabel = "bitview home";
home.append(createCube(), "bitview");

const nav = document.createElement("nav");
nav.setAttribute("aria-label", "Primary");
nav.innerHTML = `
    <ul>
      <li><a href="/explore" aria-current="page">Explore</a></li>
      <li><a href="/learn">Learn</a></li>
      <li><a href="/build">Build</a></li>
    </ul>
`;

header.append(home, nav);
document.body.append(header);
