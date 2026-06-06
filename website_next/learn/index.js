import { createContents } from "./contents/index.js";
import { sections } from "./data.js";
import { createChart as createDataChart } from "./charts/index.js";
import { initHashLinks } from "./hash-links.js";
import { initScrollSpy } from "./scroll-spy.js";
import { createId } from "../utils/id.js";

/** @param {Section["chart"]} chart */
function createFigure(chart) {
  if (typeof chart !== "string") return createDataChart(chart);

  const figure = document.createElement("figure");
  const placeholder = document.createElement("div");
  const caption = document.createElement("figcaption");

  placeholder.append(chart);
  caption.append(chart);
  figure.append(placeholder, caption);

  return figure;
}

/**
 * @param {Section} section
 * @param {number} [level]
 */
function createSection(section, level = 1) {
  const element = document.createElement("section");
  const heading = document.createElement(level === 1 ? "h1" : "h2");
  const anchor = document.createElement("a");
  const description = document.createElement("p");
  const children = section.children ?? [];
  const id = createId(section.title);

  element.id = id;
  anchor.href = `#${id}`;
  anchor.append(section.title);
  heading.append(anchor);
  description.append(section.description);
  element.append(heading, description, createFigure(section.chart));

  for (const child of children) {
    element.append(createSection(child, level + 1));
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
 * @property {string | import("./charts/index.js").Chart} chart
 * @property {Section[]} [children]
 */
