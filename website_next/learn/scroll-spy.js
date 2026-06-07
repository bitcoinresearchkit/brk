/** @param {HTMLElement} main */
export function initScrollSpy(main) {
  const nav = /** @type {HTMLElement} */ (main.querySelector("nav"));
  const sections = [...main.querySelectorAll("section[id]")];
  const sectionStates = sections.map((section) => ({
    section,
    firstChild: section.querySelector(":scope > section"),
  }));
  const links = new Map(
    [...main.querySelectorAll('nav a[href^="#"]')].map((link) => [
      link.getAttribute("href"),
      link,
    ]),
  );

  /** @type {string | null} */
  let current = null;
  let scheduled = false;

  function getViewportTop() {
    return Number.parseFloat(getComputedStyle(main).getPropertyValue("--offset"));
  }

  /**
   * @param {Element} section
   * @param {Element | null} firstChild
   */
  function getOwnVisibleHeight(section, firstChild) {
    const sectionRect = section.getBoundingClientRect();
    const childRect = firstChild?.getBoundingClientRect();
    const top = Math.max(sectionRect.top, getViewportTop());
    const bottom = Math.min(
      childRect?.top ?? sectionRect.bottom,
      window.innerHeight,
    );

    return Math.max(
      0,
      bottom - top,
    );
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
    /** @type {{ section: Element, firstChild: Element | null } | undefined} */
    let currentState;
    let currentHeight = 0;

    for (const state of sectionStates) {
      const height = getOwnVisibleHeight(state.section, state.firstChild);

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

  function scheduleUpdate() {
    if (scheduled) return;

    scheduled = true;
    requestAnimationFrame(() => {
      scheduled = false;
      update();
    });
  }

  window.addEventListener("scroll", scheduleUpdate, { passive: true });
  main.addEventListener("pageactive", scheduleUpdate);
  scheduleUpdate();
}
