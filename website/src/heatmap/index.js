import { createHeader } from "../../scripts/utils/dom.js";
import { heatmapElement } from "../../scripts/utils/elements.js";
import { debounce, next } from "../../scripts/utils/timing.js";
import { demoSource } from "./demo.js";
import { createRenderer } from "./renderer.js";

/**
 * Initializes the heatmap pane once for the app lifetime.
 *
 * @param {HeatmapOption} option
 */
export async function init(option) {
  const { headerElement, headingElement } = createHeader();
  headingElement.innerHTML = option.title;
  heatmapElement.append(headerElement);

  const canvas = document.createElement("canvas");
  heatmapElement.append(canvas);
  await next();

  let renderer = createRenderer(canvas);
  let source = demoSource;

  function render() {
    renderer.paint(source.cols, source.rows, source.getColor);
  }

  const { width, height } = canvas.getBoundingClientRect();
  if (renderer.resize(width, height)) {
    render();
  }

  new ResizeObserver(
    debounce(() => {
      const { width, height } = canvas.getBoundingClientRect();
      if (renderer.resize(width, height)) {
        render();
      }
    }, 1000),
  ).observe(heatmapElement);
}
