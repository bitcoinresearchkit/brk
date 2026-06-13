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
  const id = createPathId(sectionPath);

  anchor.href = `#${id}`;
  anchor.append(section.title);

  if (children.length) {
    const details = document.createElement("details");
    const summary = document.createElement("summary");
    const list = document.createElement("ol");

    for (const child of children) {
      list.append(createContentsItem(child, sectionPath));
    }

    summary.append(anchor);
    details.append(summary, list);
    item.append(details);
  } else {
    item.append(anchor);
  }

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
