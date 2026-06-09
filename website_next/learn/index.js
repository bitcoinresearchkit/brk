import { createContents } from "./contents/index.js";
import { sections } from "./data/index.js";
import { createChart as createDataChart } from "./charts/index.js";
import { initHashLinks } from "./hash-links.js";
import { initScrollSpy } from "./scroll-spy.js";
import { createPathId } from "./path.js";

/**
 * @param {Section} section
 * @param {readonly string[]} [path]
 */
function createSection(section, path = []) {
  const element = document.createElement("section");
  const level = path.length + 1;
  const sectionPath = [...path, section.title];
  const heading = document.createElement(`h${Math.min(level, 6)}`);
  const anchor = document.createElement("a");
  const description = document.createElement("p");
  const children = section.children ?? [];
  const id = createPathId(sectionPath);

  element.id = id;
  if (section.numbered === false) element.dataset.numbered = "false";
  anchor.href = `#${id}`;
  anchor.append(section.title);
  heading.append(anchor);
  description.append(section.description);
  element.append(heading, description);
  if (section.chart) element.append(createDataChart(section.chart));

  for (const child of children) {
    element.append(createSection(child, sectionPath));
  }

  return element;
}

export function createLearnPage() {
  const main = document.createElement("main");
  main.className = "learn";
  const article = document.createElement("article");

  for (const section of sections) {
    article.append(createSection(section));
  }

  main.append(createContents(sections), article);
  initHashLinks(main);
  initScrollSpy(main);
  return main;
}

/**
 * @typedef {Object} Section
 * @property {string} title
 * @property {string} description
 * @property {import("./charts/index.js").Chart} [chart]
 * @property {boolean} [numbered]
 * @property {Section[]} [children]
 */
