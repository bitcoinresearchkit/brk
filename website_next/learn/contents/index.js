import { createId } from "../../utils/id.js";

/** @param {Section} section */
function createContentsItem(section) {
  const item = document.createElement("li");
  const anchor = document.createElement("a");
  const children = section.children ?? [];

  anchor.href = `#${createId(section.title)}`;
  anchor.append(section.title);

  if (children.length) {
    const list = document.createElement("ol");

    for (const child of children) {
      list.append(createContentsItem(child));
    }
    item.append(list);
  }

  item.prepend(anchor);
  return item;
}

/** @param {Section[]} sections */
export function createContents(sections) {
  const nav = document.createElement("nav");
  const list = document.createElement("ol");

  nav.setAttribute("aria-label", "Learn contents");

  for (const section of sections) {
    list.append(createContentsItem(section));
  }

  nav.append(list);
  return nav;
}

/**
 * @typedef {Object} Section
 * @property {string} title
 * @property {Section[]} [children]
 */
