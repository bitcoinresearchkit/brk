/**
 * @param {{ title: string, children: Section[] }} section
 */
function createSectionId(section) {
  return section.title.toLowerCase().replaceAll(" ", "-");
}

/**
 * @param {{ title: string, children: Section[] }} section
 * @param {number[]} indexes
 */
function createContentsItem(section, indexes) {
  const item = document.createElement("li");
  const anchor = document.createElement("a");
  anchor.href = `#${createSectionId(section)}`;
  anchor.append(section.title);

  if (section.children.length) {
    const list = document.createElement("ol");

    for (const [index, child] of section.children.entries()) {
      list.append(createContentsItem(child, indexes.concat(index + 1)));
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

  for (const [index, section] of sections.entries()) {
    list.append(createContentsItem(section, [index + 1]));
  }

  nav.append(list);
  return nav;
}

/**
 * @typedef {Object} Section
 * @property {string} title
 * @property {Section[]} children
 */
