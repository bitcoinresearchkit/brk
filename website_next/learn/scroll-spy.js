/** @param {HTMLElement} main */
export function initScrollSpy(main) {
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
