import { createContents } from "./contents/index.js";
import { sections } from "./data/index.js";
import { createChart } from "./charts/index.js";
import { initSectionDetails } from "./details.js";
import { initHashLinks } from "./hash-links.js";
import { initScrollSpy } from "./scroll-spy.js";
import { createPathId } from "./path.js";

/**
 * @param {LearnSection} section
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
  /** @type {HTMLElement[]} */
  const content = [description];

  element.id = id;
  if (section.numbered === false) element.dataset.numbered = "false";
  anchor.href = `#${id}`;
  anchor.append(section.title);
  heading.append(anchor);
  description.append(section.description);
  if (section.chart) content.push(createChart(section.chart, id));

  for (const child of children) {
    content.push(createSection(child, sectionPath));
  }

  if (section.numbered === false) {
    element.append(heading, ...content);
  } else {
    const details = document.createElement("details");
    const summary = document.createElement("summary");

    summary.append(heading);
    details.append(summary, ...content);
    element.append(details);
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
  const details = initSectionDetails(main);
  const navigateToHash = initScrollSpy(main);
  initHashLinks(main, navigateToHash, details);
  return main;
}
