const lifecycleByElement = new WeakMap();
const observer = new IntersectionObserver(
  (entries) => {
    for (const entry of entries) {
      const lifecycle = lifecycleByElement.get(entry.target);
      lifecycle?.[entry.isIntersecting ? "show" : "hide"]();
    }
  },
  {
    rootMargin: "400px 0px",
  },
);

/**
 * @param {Element} element
 * @param {{ show: () => void, hide: () => void }} lifecycle
 */
export function onChartVisibility(element, lifecycle) {
  lifecycleByElement.set(element, lifecycle);
  observer.observe(element);
}
