import { createContents } from "./contents/index.js";
import { sections } from "./data.js";
import { initScrollSpy } from "./scroll-spy.js";
import { createId } from "../utils/id.js";

/** @param {string} label */
function createChart(label) {
  const figure = document.createElement("figure");
  const chart = document.createElement("div");
  const caption = document.createElement("figcaption");

  chart.append(label);
  caption.append(label);
  figure.append(chart, caption);

  return figure;
}

/**
 * @param {Section} section
 * @param {number} [level]
 */
function createSection(section, level = 1) {
  const element = document.createElement("section");
  const title = document.createElement(level === 1 ? "h1" : "h2");
  const anchor = document.createElement("a");
  const description = document.createElement("p");
  const id = createId(section.title);

  element.id = id;
  anchor.href = `#${id}`;
  anchor.append(section.title);
  title.append(anchor);
  description.append(section.description);
  element.append(title, description, createChart(section.chart));

  for (const child of section.children) {
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
  initScrollSpy(main);
  return main;
}

/**
 * @typedef {Object} Section
 * @property {string} title
 * @property {string} description
 * @property {string} chart
 * @property {Section[]} children
 */
