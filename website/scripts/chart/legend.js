import { createLabeledInput, createSpanName } from "../utils/dom.js";
import { stringToId } from "../utils/format.js";

export function createLegend() {
  const element = window.document.createElement("legend");

  /** @type {AnySeries | null} */
  let hoveredSeries = null;
  /** @type {Map<AnySeries, { span: HTMLSpanElement, color: Color }[]>} */
  const seriesColorSpans = new Map();

  /** @param {AnySeries | null} series */
  function setHovered(series) {
    if (hoveredSeries === series) return;
    hoveredSeries = series;
    for (const [entrySeries, colorSpans] of seriesColorSpans) {
      const shouldHighlight = !hoveredSeries || hoveredSeries === entrySeries;
      shouldHighlight ? entrySeries.highlight() : entrySeries.tame();
      for (const { span, color } of colorSpans) {
        span.style.backgroundColor = color.highlight(shouldHighlight);
      }
    }
  }

  /** @type {HTMLElement[]} */
  const legends = [];

  return {
    element,
    /**
     * @param {Object} args
     * @param {AnySeries} args.series
     * @param {string} args.name
     * @param {number} args.order
     * @param {Color[]} args.colors
     */
    addOrReplace({ series, name, colors, order }) {
      const div = window.document.createElement("div");

      const prev = legends[order];
      if (prev) {
        prev.replaceWith(div);
      } else {
        const elementAtOrder = Array.from(element.children).at(order);
        if (elementAtOrder) {
          elementAtOrder.before(div);
        } else {
          element.append(div);
        }
      }
      legends[order] = div;

      const { label } = createLabeledInput({
        inputId: stringToId(`legend-${series.id}`),
        inputName: stringToId(`selected-${series.id}`),
        inputValue: "value",
        title: "Click to toggle",
        inputChecked: series.active.value,
        onClick: () => {
          series.setActive(!series.active.value);
        },
        type: "checkbox",
      });

      const spanMain = window.document.createElement("span");
      spanMain.classList.add("main");
      label.append(spanMain);

      const spanName = createSpanName(name);
      spanMain.append(spanName);

      div.append(label);
      label.addEventListener("mouseover", () => setHovered(series));
      label.addEventListener("mouseleave", () => setHovered(null));

      const spanColors = window.document.createElement("span");
      spanColors.classList.add("colors");
      spanMain.prepend(spanColors);
      /** @type {{ span: HTMLSpanElement, color: Color }[]} */
      const colorSpans = [];
      colors.forEach((color) => {
        const spanColor = window.document.createElement("span");
        spanColor.style.backgroundColor = color.highlight(true);
        spanColors.append(spanColor);
        colorSpans.push({ span: spanColor, color });
      });
      seriesColorSpans.set(series, colorSpans);

      if (series.url) {
        const anchor = window.document.createElement("a");
        anchor.href = series.url;
        anchor.target = "_blank";
        anchor.rel = "noopener noreferrer";
        anchor.title = "Open the metric data in a new tab";
        div.append(anchor);
      }
    },
    /**
     * @param {number} start
     */
    removeFrom(start) {
      legends.splice(start).forEach((child) => child.remove());
    },
  };
}
