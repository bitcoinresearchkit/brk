import { formatValue } from "./format.js";
import { createSvgElement } from "./svg.js";
import { VIEWBOX_WIDTH } from "./viewbox.js";

/** @typedef {import("./highlight.js").SeriesHighlight} SeriesHighlight */
/** @typedef {import("./legend.js").Readout} Readout */

const dateFormat = new Intl.DateTimeFormat("en-US", {
  day: "2-digit",
  month: "2-digit",
  year: "numeric",
});

/**
 * @param {number} value
 * @param {number} min
 * @param {number} max
 */
function clamp(value, min, max) {
  return Math.min(Math.max(value, min), max);
}

/**
 * @param {ScrubberSeries} series
 * @param {number} ratio
 */
function getPointAtRatio(series, ratio) {
  return series.points[Math.round(ratio * (series.points.length - 1))];
}

/**
 * @param {HTMLTimeElement} time
 * @param {Date} date
 */
function updateTime(time, date) {
  time.textContent = dateFormat.format(date);
  time.dateTime = date.toISOString().slice(0, 10);
}

/**
 * @param {Readout} readout
 * @param {ReturnType<typeof getPointAtRatio>[]} points
 */
function updateReadout(readout, points) {
  updateTime(readout.time, points[0].date);

  readout.rows.forEach(({ value }, index) => {
    value.textContent = formatValue(points[index].value);
  });
}

/**
 * @param {SVGSVGElement} svg
 * @param {Readout} readout
 * @param {SeriesHighlight} highlight
 */
export function createScrubber(svg, readout, highlight) {
  const group = createSvgElement("g");
  const guide = createSvgElement("line");
  /** @type {ScrubberSeries[]} */
  let series = [];
  /** @type {SVGCircleElement[]} */
  let markers = [];
  let height = 0;
  let stepCount = 0;

  group.dataset.scrubber = "root";
  guide.dataset.scrubber = "guide";
  group.append(guide);
  svg.append(group);

  /**
   * @param {number} ratio
   * @param {boolean} [scrubbing]
   */
  function update(ratio, scrubbing = true) {
    if (!series.length) return;

    const nextRatio = clamp(ratio, 0, 1);
    const points = series.map((item) => getPointAtRatio(item, nextRatio));
    const x = points[0].x.toFixed(2);

    svg.dataset.index = Math.round(nextRatio * stepCount).toString();
    guide.setAttribute("x1", x);
    guide.setAttribute("x2", x);
    guide.setAttribute("y1", "0");
    guide.setAttribute("y2", height.toString());
    updateReadout(readout, points);

    markers.forEach((marker, index) => {
      const point = points[index];

      marker.setAttribute("cx", point.x.toFixed(2));
      marker.setAttribute("cy", point.y.toFixed(2));
    });

    if (scrubbing) {
      svg.dataset.scrubbing = "true";
    } else {
      delete svg.dataset.scrubbing;
    }
  }

  function hide() {
    update(1, false);
  }

  function clear() {
    series = [];
    markers = [];
    highlight.clearPreview();
    group.replaceChildren(guide);
    delete svg.dataset.index;
    delete svg.dataset.scrubbing;
  }

  /**
   * @param {ScrubberSeries[]} nextSeries
   * @param {number} nextHeight
   */
  function setSeries(nextSeries, nextHeight) {
    series = nextSeries;
    height = nextHeight;
    stepCount = Math.max(...series.map(({ points }) => points.length - 1));
    markers = series.map(({ color }, index) => {
      const marker = createSvgElement("circle");

      marker.dataset.series = index.toString();
      marker.dataset.scrubber = "marker";
      marker.style.setProperty("--color", color);
      marker.setAttribute("r", "3");
      highlight.addNode(marker, index);

      return marker;
    });

    group.replaceChildren(guide, ...markers);
    update(1, false);
  }

  /** @param {PointerEvent} event */
  function updateFromPointer(event) {
    const { left, width } = svg.getBoundingClientRect();
    const x = ((event.clientX - left) / width) * VIEWBOX_WIDTH;
    const index = Number(
      /** @type {SVGElement} */ (event.target).dataset.series,
    );

    if (Number.isInteger(index)) highlight.preview(index);
    else highlight.clearPreview();
    update(x / VIEWBOX_WIDTH);
  }

  svg.addEventListener("pointermove", updateFromPointer);
  svg.addEventListener("pointerleave", () => {
    highlight.clearPreview();
    hide();
  });
  svg.addEventListener("focus", () => update(1));
  svg.addEventListener("blur", () => {
    highlight.clearPreview();
    hide();
  });
  svg.addEventListener("keydown", (event) => {
    const current = Number(svg.dataset.index || stepCount);

    if (event.key === "ArrowLeft") {
      event.preventDefault();
      update((current - 1) / stepCount);
    }

    if (event.key === "ArrowRight") {
      event.preventDefault();
      update((current + 1) / stepCount);
    }
  });

  return { clear, setSeries };
}

/**
 * @typedef {Object} ScrubberSeries
 * @property {string} color
 * @property {{ date: Date, value: number, x: number, y: number }[]} points
 */
