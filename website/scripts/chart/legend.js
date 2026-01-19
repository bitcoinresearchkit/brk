import { createLabeledInput, createSpanName } from "../utils/dom.js";
import { stringToId } from "../utils/format.js";

/** @param {string} color */
const tameColor = (color) => `${color.slice(0, -1)} / 50%)`;

/**
 * @param {Signals} signals
 */
export function createLegend(signals) {
  const element = window.document.createElement("legend");

  const hovered = signals.createSignal(/** @type {AnySeries | null} */ (null));

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

      const { input, label } = createLabeledInput({
        inputId: stringToId(`legend-${series.id}`),
        inputName: stringToId(`selected-${series.id}`),
        inputValue: "value",
        title: "Click to toggle",
        inputChecked: series.active(),
        onClick: () => {
          series.active.set(input.checked);
        },
        type: "checkbox",
      });

      const spanMain = window.document.createElement("span");
      spanMain.classList.add("main");
      label.append(spanMain);

      const spanName = createSpanName(name);
      spanMain.append(spanName);

      div.append(label);
      label.addEventListener("mouseover", () => {
        const h = hovered();
        if (!h || h !== series) {
          hovered.set(series);
        }
      });
      label.addEventListener("mouseleave", () => {
        hovered.set(null);
      });

      const shouldHighlight = () => !hovered() || hovered() === series;

      const spanColors = window.document.createElement("span");
      spanColors.classList.add("colors");
      spanMain.prepend(spanColors);
      colors.forEach((color) => {
        const spanColor = window.document.createElement("span");
        spanColors.append(spanColor);

        signals.createEffect(
          () => ({
            color: color(),
            shouldHighlight: shouldHighlight(),
          }),
          ({ color, shouldHighlight }) => {
            if (shouldHighlight) {
              spanColor.style.backgroundColor = color;
            } else {
              spanColor.style.backgroundColor = tameColor(color);
            }
          },
        );
      });

      const initialColors = /** @type {Record<string, any>} */ ({});
      const darkenedColors = /** @type {Record<string, any>} */ ({});

      const seriesOptions = series.getOptions();
      if (!seriesOptions) return;

      Object.entries(seriesOptions).forEach(([k, v]) => {
        if (k.toLowerCase().includes("color") && typeof v === "string") {
          if (!v.startsWith("oklch")) return;
          initialColors[k] = v;
          darkenedColors[k] = tameColor(v);
        } else if (k === "lastValueVisible" && v) {
          initialColors[k] = true;
          darkenedColors[k] = false;
        }
      });

      signals.createEffect(shouldHighlight, (shouldHighlight) => {
        if (shouldHighlight) {
          series.applyOptions(initialColors);
        } else {
          series.applyOptions(darkenedColors);
        }
      });

      const anchor = window.document.createElement("a");

      signals.createEffect(series.url, (url) => {
        if (url) {
          anchor.href = url;
          anchor.target = "_blank";
          anchor.rel = "noopener noreferrer";
          anchor.title = "Click to view data";
          div.append(anchor);
        }
      });
    },
    /**
     * @param {number} start
     */
    removeFrom(start) {
      legends.splice(start).forEach((child) => child.remove());
    },
  };
}
