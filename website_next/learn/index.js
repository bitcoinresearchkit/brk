import { createContents } from "./contents/index.js";
import { sections } from "./data.js";

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
 * @param {string} title
 */
function createSectionId(title) {
  return title.toLowerCase().replaceAll(" ", "-");
}

/**
 * @param {Section} section
 * @param {number[]} indexes
 */
function createSection(section, indexes) {
  const element = document.createElement("section");
  const title = document.createElement(indexes.length === 1 ? "h1" : "h2");
  const anchor = document.createElement("a");
  const description = document.createElement("p");
  const id = createSectionId(section.title);

  element.id = id;
  anchor.href = `#${id}`;
  anchor.append(section.title);
  title.append(anchor);
  description.append(section.description);
  element.append(title, description, createChart(section.chart));

  for (const [index, child] of section.children.entries()) {
    element.append(createSection(child, indexes.concat(index + 1)));
  }

  return element;
}

/** @param {HTMLElement} main */
function initScrollSpy(main) {
  const headings = [...main.querySelectorAll("article h1, article h2")];
  const visibleHeadings = new Set();
  const links = new Map(
    [...main.querySelectorAll('nav a[href^="#"]')].map((link) => [
      link.getAttribute("href"),
      link,
    ]),
  );

  /** @type {string | null} */
  let current = null;

  /** @param {Element} heading */
  function getHash(heading) {
    const section = /** @type {HTMLElement} */ (
      heading.closest("section[id]")
    );
    return `#${section.id}`;
  }

  /** @param {string} hash */
  function getLink(hash) {
    return /** @type {HTMLAnchorElement} */ (links.get(hash));
  }

  /** @param {string} hash */
  function setCurrent(hash) {
    if (hash === current) return;

    if (current) getLink(current).removeAttribute("aria-current");
    getLink(hash).setAttribute("aria-current", "location");
    history.replaceState(null, "", hash);
    current = hash;
  }

  function update() {
    if (main.hidden) return;

    const heading = headings.findLast((heading) =>
      visibleHeadings.has(heading),
    );
    if (heading) setCurrent(getHash(heading));
  }

  const observer = new IntersectionObserver(
    (entries) => {
      for (const entry of entries) {
        if (entry.isIntersecting) {
          visibleHeadings.add(entry.target);
        } else {
          visibleHeadings.delete(entry.target);
        }
      }

      update();
    },
    { rootMargin: "0px 0px -80% 0px" },
  );

  for (const heading of headings) observer.observe(heading);
}

export function createLearnPage() {
  const main = document.createElement("main");
  main.className = "learn";
  const article = document.createElement("article");

  for (const [index, section] of sections.entries()) {
    article.append(createSection(section, [index + 1]));
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
