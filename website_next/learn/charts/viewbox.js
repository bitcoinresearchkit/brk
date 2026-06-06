export const VIEWBOX_WIDTH = 640;
export const FALLBACK_VIEWBOX_HEIGHT = 220;

/** @param {SVGSVGElement} svg */
export function getViewBoxHeight(svg) {
  const { width, height } = svg.getBoundingClientRect();

  return width && height
    ? (VIEWBOX_WIDTH * height) / width
    : FALLBACK_VIEWBOX_HEIGHT;
}
