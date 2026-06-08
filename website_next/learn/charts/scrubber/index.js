import { formatValue } from "../format.js";
import { clamp } from "../math.js";
import { createSvgElement } from "../svg.js";
import { VIEWBOX_WIDTH } from "../viewbox.js";

/** @typedef {import("../highlight.js").SeriesHighlight} SeriesHighlight */
/** @typedef {import("../legend/index.js").Readout} Readout */

const dateFormat = new Intl.DateTimeFormat("en-US", {
  day: "2-digit",
  month: "2-digit",
  year: "numeric",
});

const markerRadiusPx = 4;

/** @param {number} width */
function getMarkerRadiusInViewBox(width) {
  return width ? (markerRadiusPx * VIEWBOX_WIDTH) / width : markerRadiusPx;
}

/**
 * @param {ScrubberSeries} series
 * @param {number} step
 */
function getPointAtStep(series, step) {
  return series.points[step];
}

/**
 * @param {ReturnType<typeof getPointAtStep>[]} points
 * @param {number} y
 */
function getClosestPointIndex(points, y) {
  let closestIndex = 0;
  let closestDistance = Infinity;

  for (const [index, point] of points.entries()) {
    const distance = Math.abs(point.y - y);

    if (distance < closestDistance) {
      closestIndex = index;
      closestDistance = distance;
    }
  }

  return closestIndex;
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
 * @param {ReturnType<typeof getPointAtStep>[]} points
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
  const shade = createSvgElement("rect");
  const guide = createSvgElement("line");
  /** @type {ScrubberSeries[]} */
  let series = [];
  /** @type {SVGCircleElement[]} */
  let markers = [];
  let height = 0;
  let stepCount = 0;
  let currentStep = -1;
  let currentPoints = getPointsAtStep(0);
  let rect = svg.getBoundingClientRect();

  group.dataset.scrubber = "root";
  shade.dataset.scrubber = "shade";
  guide.dataset.scrubber = "guide";
  group.append(shade, guide);
  svg.append(group);

  function measure() {
    rect = svg.getBoundingClientRect();
  }

  /** @param {number} step */
  function getPointsAtStep(step) {
    return series.map((item) => getPointAtStep(item, step));
  }

  /**
   * @param {number} ratio
   * @param {number} [y]
   * @param {boolean} [scrubbing]
   */
  function update(ratio, y, scrubbing = true) {
    if (!series.length) return;

    const nextStep = Math.round(clamp(ratio, 0, 1) * stepCount);

    if (nextStep !== currentStep) {
      currentStep = nextStep;
      currentPoints = getPointsAtStep(nextStep);

      const x = currentPoints[0].x;
      const xText = x.toFixed(2);

      svg.dataset.index = nextStep.toString();
      shade.setAttribute("x", xText);
      shade.setAttribute("y", "0");
      shade.setAttribute("width", (VIEWBOX_WIDTH - x).toFixed(2));
      shade.setAttribute("height", height.toString());
      guide.setAttribute("x1", xText);
      guide.setAttribute("x2", xText);
      guide.setAttribute("y1", "0");
      guide.setAttribute("y2", height.toString());
      updateReadout(readout, currentPoints);

      markers.forEach((marker, index) => {
        const point = currentPoints[index];

        marker.setAttribute("cx", point.x.toFixed(2));
        marker.setAttribute("cy", point.y.toFixed(2));
      });
    }

    if (scrubbing) {
      svg.dataset.scrubbing = "true";
    } else {
      delete svg.dataset.scrubbing;
    }

    if (y !== undefined) {
      highlight.preview(getClosestPointIndex(currentPoints, y));
    }
  }

  function hide() {
    update(1, undefined, false);
  }

  function clear() {
    series = [];
    markers = [];
    currentStep = -1;
    currentPoints = [];
    highlight.clearPreview();
    group.replaceChildren(shade, guide);
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
    currentStep = -1;
    stepCount = Math.max(...series.map(({ points }) => points.length - 1));
    measure();
    const radius = getMarkerRadiusInViewBox(rect.width);
    markers = series.map(({ color }, index) => {
      const marker = createSvgElement("circle");

      marker.dataset.series = index.toString();
      marker.dataset.scrubber = "marker";
      marker.style.setProperty("--color", color);
      marker.setAttribute("r", radius.toString());
      highlight.addNode(marker, index);

      return marker;
    });

    group.replaceChildren(shade, guide, ...markers);
    update(1, undefined, false);
  }

  /** @param {PointerEvent} event */
  function updateFromPointer(event) {
    const x = ((event.clientX - rect.left) / rect.width) * VIEWBOX_WIDTH;
    const y = ((event.clientY - rect.top) / rect.height) * height;

    update(x / VIEWBOX_WIDTH, y);
  }

  svg.addEventListener("pointerenter", measure);
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
