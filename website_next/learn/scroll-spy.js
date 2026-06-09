/** @param {HTMLElement} main */
export function initScrollSpy(main) {
  const nav = /** @type {HTMLElement} */ (main.querySelector("nav"));
  const sections = [...main.querySelectorAll("section[id]")];
  const links = new Map(
    [...main.querySelectorAll('nav a[href^="#"]')].map((link) => [
      link.getAttribute("href"),
      link,
    ]),
  );

  /** @type {string | null} */
  let current = null;
  /** @type {string | null} */
  let navigatingTo = null;
  let alignNavToTop = true;
  let scheduled = false;

  function getViewportTop() {
    return Number.parseFloat(getComputedStyle(main).scrollPaddingTop);
  }

  /** @param {string} hash */
  function getLink(hash) {
    return /** @type {HTMLAnchorElement} */ (links.get(hash));
  }

  /**
   * @param {HTMLElement} link
   * @param {ScrollBehavior} behavior
   */
  function scrollLinkIntoNav(link, behavior) {
    const style = getComputedStyle(nav);
    const top = Number.parseFloat(style.paddingTop);
    const bottom = Number.parseFloat(style.paddingBottom);
    const navRect = nav.getBoundingClientRect();
    const linkRect = link.getBoundingClientRect();

    if (linkRect.top < navRect.top + top) {
      nav.scrollBy({
        top: linkRect.top - navRect.top - top,
        behavior,
      });
    } else if (linkRect.bottom > navRect.bottom - bottom) {
      nav.scrollBy({
        top: linkRect.bottom - navRect.bottom + bottom,
        behavior,
      });
    }
  }

  /**
   * @param {HTMLElement} link
   * @param {ScrollBehavior} behavior
   */
  function scrollLinkToNavTop(link, behavior) {
    const top = Number.parseFloat(getComputedStyle(nav).paddingTop);
    const navRect = nav.getBoundingClientRect();
    const linkRect = link.getBoundingClientRect();

    nav.scrollBy({
      top: linkRect.top - navRect.top - top,
      behavior,
    });
  }

  function stopHashNavigation() {
    navigatingTo = null;
  }

  /** @param {string} hash */
  function selectHash(hash) {
    if (hash === current) return;

    if (current) getLink(current).removeAttribute("aria-current");

    const link = getLink(hash);
    link.setAttribute("aria-current", "location");
    current = hash;
  }

  /** @param {string} hash */
  function syncHash(hash) {
    if (hash === current) return;

    selectHash(hash);
    const link = getLink(hash);
    if (alignNavToTop) {
      scrollLinkToNavTop(link, "auto");
      alignNavToTop = false;
    } else {
      scrollLinkIntoNav(link, "auto");
    }
    history.replaceState(null, "", hash);
  }

  /** @param {string} hash */
  function navigateToHash(hash) {
    navigatingTo = hash;
    selectHash(hash);
    scrollLinkIntoNav(getLink(hash), "smooth");
  }

  function getCurrentSection() {
    let currentSection = sections[0];
    const viewportTop = getViewportTop();

    for (const section of sections) {
      if (section.getBoundingClientRect().top > viewportTop) break;

      currentSection = section;
    }

    return currentSection;
  }

  function update() {
    if (main.hidden) return;

    const section = getCurrentSection();
    if (!section) return;

    const hash = `#${section.id}`;
    if (navigatingTo) {
      if (hash === navigatingTo) navigatingTo = null;
      return;
    }

    syncHash(hash);
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
  window.addEventListener("scrollend", () => {
    stopHashNavigation();
    scheduleUpdate();
  }, { passive: true });
  main.addEventListener("pageactive", () => {
    stopHashNavigation();
    alignNavToTop = true;
    scheduleUpdate();
  });
  scheduleUpdate();
  return navigateToHash;
}
