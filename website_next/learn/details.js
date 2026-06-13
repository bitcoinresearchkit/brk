const storagePrefix = "bitview:learn-section";

/** @param {string} id */
function getStorageKey(id) {
  return `${storagePrefix}:${id}`;
}

/** @param {HTMLElement} section */
function getDetails(section) {
  return /** @type {HTMLDetailsElement} */ (
    section.querySelector(":scope > details")
  );
}

/** @param {HTMLElement} section */
function getHash(section) {
  return `#${section.id}`;
}

/**
 * @param {HTMLElement} main
 * @param {HTMLElement} section
 */
function getNavDetails(main, section) {
  const anchor = main.querySelector(`nav a[href="${getHash(section)}"]`);
  const summary = anchor?.parentElement;

  return summary?.localName === "summary" ? summary.parentElement : null;
}

/**
 * @param {HTMLElement} main
 * @param {HTMLElement} section
 */
function syncNavSection(main, section) {
  const details = getNavDetails(main, section);

  if (details) details.toggleAttribute("open", getDetails(section).open);
}

/**
 * @param {HTMLElement} main
 * @param {HTMLElement} section
 */
function openSection(main, section) {
  let current = section;

  while (main.contains(current)) {
    const details = current.querySelector(":scope > details");
    if (details) {
      /** @type {HTMLDetailsElement} */ (details).open = true;
      syncNavSection(main, current);
    }

    const parent = current.parentElement?.closest("section[id]");
    if (!parent) break;
    current = /** @type {HTMLElement} */ (parent);
  }
}

/** @param {HTMLElement} main */
export function initSectionDetails(main) {
  const sections = [
    ...main.querySelectorAll("section[id]:not([data-numbered='false'])"),
  ].map((section) => /** @type {HTMLElement} */ (section));

  for (const section of sections) {
    const saved = localStorage.getItem(getStorageKey(section.id));

    getDetails(section).open = saved === "1";
    syncNavSection(main, section);
  }

  main.addEventListener("toggle", (event) => {
    const details = /** @type {HTMLDetailsElement} */ (event.target);
    const section = /** @type {HTMLElement} */ (details.parentElement);
    if (!section.matches("article section[id]")) return;

    localStorage.setItem(getStorageKey(section.id), details.open ? "1" : "0");
    syncNavSection(main, section);
    main.dispatchEvent(new Event("sectiontoggle"));
  }, { capture: true });

  /** @param {string} hash */
  function openHash(hash) {
    const target = document.getElementById(hash.slice(1));

    if (target && main.contains(target)) openSection(main, target);
  }

  /** @param {string} hash */
  function toggleHash(hash) {
    const target = document.getElementById(hash.slice(1));
    if (!target || !main.contains(target)) return false;

    const section = /** @type {HTMLElement} */ (target);
    const details = getDetails(section);

    if (details.open) {
      details.open = false;
      syncNavSection(main, section);
      return false;
    }

    openSection(main, section);
    return true;
  }

  if (window.location.hash) openHash(window.location.hash);

  return {
    openHash,
    toggleHash,
  };
}
