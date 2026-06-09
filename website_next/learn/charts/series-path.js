import { createSvgElement } from "./svg.js";

/**
 * @param {Object} args
 * @param {SVGGElement} args.group
 * @param {SeriesHighlight} args.highlight
 * @param {number} args.index
 * @param {string} args.chart
 * @param {string} args.color
 * @param {string} args.d
 */
export function appendSeriesPath(args) {
  const path = createSvgElement("path");

  path.dataset.chart = args.chart;
  path.dataset.series = args.index.toString();
  path.style.setProperty("--color", args.color);
  path.setAttribute("d", args.d);
  args.highlight.addNode(path, args.index);
  args.group.append(path);

  return path;
}
