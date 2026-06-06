/**
 * @param {Element} element
 * @param {{ show: () => void, hide: () => void }} lifecycle
 */
export function onChartVisibility(element, lifecycle) {
  const observer = new IntersectionObserver(
    (entries) => {
      if (entries[0].isIntersecting) {
        lifecycle.show();
      } else {
        lifecycle.hide();
      }
    },
    {
      rootMargin: "800px 0px",
    },
  );

  observer.observe(element);
}
