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
 * @param {number[]} indexes
 */
function createSectionId(title, indexes) {
  return `${indexes.join("-")}-${title.toLowerCase().replaceAll(" ", "-")}`;
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

  element.id = createSectionId(section.title, indexes);
  anchor.href = `#${element.id}`;
  anchor.append(section.title);
  title.append(anchor);
  description.append(section.description);
  element.append(title, description, createChart(section.chart));

  for (const [index, child] of section.children.entries()) {
    element.append(createSection(child, indexes.concat(index + 1)));
  }

  return element;
}

/**
 * @param {{ title: string, children: Section[] }} section
 * @param {number[]} indexes
 */
function createContentsItem(section, indexes) {
  const item = document.createElement("li");
  const anchor = document.createElement("a");
  anchor.href = `#${createSectionId(section.title, indexes)}`;
  anchor.append(section.title);
  item.append(anchor);

  if (section.children.length) {
    const list = document.createElement("ol");
    for (const [index, child] of section.children.entries()) {
      list.append(createContentsItem(child, indexes.concat(index + 1)));
    }
    item.append(list);
  }

  return item;
}

function createContents() {
  const nav = document.createElement("nav");
  const list = document.createElement("ol");

  nav.setAttribute("aria-label", "Learn contents");

  for (const [index, section] of sections.entries()) {
    list.append(createContentsItem(section, [index + 1]));
  }

  nav.append(list);
  return nav;
}

/** @param {HTMLElement} main */
function initScrollSpy(main) {
  const sections = [...main.querySelectorAll("section[id]")];
  const visible = new Set();
  const links = new Map(
    [...main.querySelectorAll('nav a[href^="#"]')].map((link) => [
      link.getAttribute("href"),
      link,
    ]),
  );

  /** @type {string | null} */
  let current = null;

  function update() {
    const section = sections.find((section) => visible.has(section.id));
    if (!section) return;

    const hash = `#${section.id}`;
    if (hash === current) return;

    links.get(current)?.removeAttribute("aria-current");
    links.get(hash)?.setAttribute("aria-current", "location");
    history.replaceState(null, "", `/learn${hash}`);
    current = hash;
  }

  const observer = new IntersectionObserver((entries) => {
    for (const entry of entries) {
      if (entry.isIntersecting) {
        visible.add(entry.target.id);
      } else {
        visible.delete(entry.target.id);
      }
    }
    update();
  });

  for (const section of sections) {
    observer.observe(section);
  }
}

export function createLearnPage() {
  const main = document.createElement("main");
  main.className = "learn";
  const article = document.createElement("article");

  for (const [index, section] of sections.entries()) {
    article.append(createSection(section, [index + 1]));
  }

  main.append(article, createContents());
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
