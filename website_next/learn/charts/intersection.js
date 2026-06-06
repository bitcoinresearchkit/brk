/**
 * @param {Element} element
 * @param {() => void} callback
 */
export function onFirstIntersection(element, callback) {
  const observer = new IntersectionObserver((entries) => {
    if (!entries[0].isIntersecting) return;

    observer.disconnect();
    callback();
  });

  observer.observe(element);
}
