const thresholds = [0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1];

/** @param {HTMLElement} main */
export function initScrollSpy(main) {
  const nav = /** @type {HTMLElement} */ (main.querySelector("nav"));
  const sections = [...main.querySelectorAll("section[id]")];
  const sectionStates = sections.map((section) => ({
    section,
    children: [...section.querySelectorAll(":scope > section")],
    intersecting: false,
  }));
  const stateBySection = new Map(
    sectionStates.map((state) => [state.section, state]),
  );
  const links = new Map(
    [...main.querySelectorAll('nav a[href^="#"]')].map((link) => [
      link.getAttribute("href"),
      link,
    ]),
  );

  /** @type {string | null} */
  let current = null;

  /** @param {Element} section */
  function getVisibleHeight(section) {
    const rect = section.getBoundingClientRect();
    return Math.max(
      0,
      Math.min(rect.bottom, window.innerHeight) - Math.max(rect.top, 0),
    );
  }

  /** @param {{ section: Element, children: Element[] }} state */
  function getOwnVisibleHeight(state) {
    let height = getVisibleHeight(state.section);

    for (const child of state.children) {
      height -= getVisibleHeight(child);
    }

    return Math.max(0, height);
  }

  /** @param {string} hash */
  function getLink(hash) {
    return /** @type {HTMLAnchorElement} */ (links.get(hash));
  }

  /** @param {HTMLElement} link */
  function scrollLinkIntoNav(link) {
    const style = getComputedStyle(nav);
    const top = Number.parseFloat(style.paddingTop);
    const bottom = Number.parseFloat(style.paddingBottom);
    const navRect = nav.getBoundingClientRect();
    const linkRect = link.getBoundingClientRect();

    if (linkRect.top < navRect.top + top) {
      nav.scrollBy({ top: linkRect.top - navRect.top - top });
    }

    if (linkRect.bottom > navRect.bottom - bottom) {
      nav.scrollBy({ top: linkRect.bottom - navRect.bottom + bottom });
    }
  }

  /** @param {string} hash */
  function setCurrentHash(hash) {
    if (hash === current) return;

    if (current) getLink(current).removeAttribute("aria-current");

    const link = getLink(hash);
    link.setAttribute("aria-current", "location");
    scrollLinkIntoNav(link);

    history.replaceState(null, "", hash);
    current = hash;
  }

  function getCurrentSection() {
    /** @type {{ section: Element, children: Element[] } | undefined} */
    let currentState;
    let currentHeight = 0;

    for (const state of sectionStates) {
      if (!state.intersecting) continue;

      const height = getOwnVisibleHeight(state);

      if (height > currentHeight) {
        currentState = state;
        currentHeight = height;
      }
    }

    return currentState?.section;
  }

  function update() {
    if (main.hidden) return;

    const section = getCurrentSection();
    if (section) setCurrentHash(`#${section.id}`);
  }

  const observer = new IntersectionObserver(
    (entries) => {
      for (const entry of entries) {
        const state = /** @type {{ intersecting: boolean }} */ (
          stateBySection.get(entry.target)
        );
        state.intersecting = entry.isIntersecting;
      }

      update();
    },
    {
      threshold: thresholds,
    },
  );

  for (const section of sections) observer.observe(section);
}
