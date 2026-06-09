import { createPathId } from "../path.js";

/**
 * @param {LearnSection} section
 * @param {readonly string[]} path
 */
function createContentsItem(section, path) {
  const item = document.createElement("li");
  const anchor = document.createElement("a");
  const children = section.children ?? [];
  const sectionPath = [...path, section.title];

  if (section.numbered === false) item.dataset.numbered = "false";
  anchor.href = `#${createPathId(sectionPath)}`;
  anchor.append(section.title);

  if (children.length) {
    const list = document.createElement("ol");

    for (const child of children) {
      list.append(createContentsItem(child, sectionPath));
    }
    item.append(list);
  }

  item.prepend(anchor);
  return item;
}

/** @param {LearnSection[]} sections */
export function createContents(sections) {
  const nav = document.createElement("nav");
  const list = document.createElement("ol");

  nav.setAttribute("aria-label", "Learn contents");

  for (const section of sections) {
    list.append(createContentsItem(section, []));
  }

  nav.append(list);
  return nav;
}
