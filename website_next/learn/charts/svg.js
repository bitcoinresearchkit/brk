const SVG_NS = "http://www.w3.org/2000/svg";

/**
 * @template {keyof SVGElementTagNameMap} Name
 * @param {Name} name
 * @returns {SVGElementTagNameMap[Name]}
 */
export function createSvgElement(name) {
  return document.createElementNS(SVG_NS, name);
}
