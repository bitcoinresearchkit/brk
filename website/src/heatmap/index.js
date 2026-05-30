/** @import { HeatmapOption } from "../../scripts/options/types.js" */
/** @import { HeatmapGrid, HeatmapPoints } from "./types.js" */

import { createHeader, createSelect } from "../../scripts/utils/dom.js";
import { heatmapElement } from "../../scripts/utils/elements.js";
import { debounce, next } from "../../scripts/utils/timing.js";
import { dark, onChange as onThemeChange } from "../../scripts/utils/theme.js";
import { createRenderer } from "./renderer.js";
import { dateRange, GENESIS_DATE, todayISODate, toISODate } from "./time.js";

/**
 * @typedef {Object} RangeChoice
 * @property {string} label
 * @property {string} date
 */

const MAX_PARALLEL_FETCHES = 8;

/** @type {ReturnType<typeof createRenderer> | undefined} */
let renderer;
/** @type {HTMLCanvasElement | undefined} */
let canvas;
/** @type {HTMLHeadingElement | undefined} */
let headingElement;
/** @type {HTMLElement | undefined} */
let statusElement;
/** @type {HeatmapOption | undefined} */
let currentOption;
/** @type {HeatmapGrid | undefined} */
let currentGrid;
/** @type {string[]} */
let currentDates = [];
/** @type {Map<string, number>} */
let currentDateIndex = new Map();
/** @type {Map<string, HeatmapPoints>} */
let pointsByDate = new Map();
/** @type {AbortController | undefined} */
let abortController;
let loadGeneration = 0;
let initialized = false;
let from = GENESIS_DATE;
let to = todayISODate();

/**
 * Initializes the heatmap pane once for the app lifetime.
 */
export function init() {
  if (initialized) return;
  initialized = true;

  const header = createHeader();
  headingElement = header.headingElement;
  const { headerElement } = header;
  heatmapElement.append(headerElement);
  heatmapElement.append(createRangeControls());

  canvas = document.createElement("canvas");
  heatmapElement.append(canvas);
  renderer = createRenderer(canvas);

  canvas.addEventListener("mousemove", updateTooltip);
  canvas.addEventListener("mouseleave", () => canvas?.removeAttribute("title"));
  onThemeChange(paint);

  void next().then(resizeAndRebuild);

  new ResizeObserver(
    debounce(() => {
      resizeAndRebuild();
    }, 250),
  ).observe(heatmapElement);
}

/** @param {HeatmapOption} option */
export function setOption(option) {
  init();
  if (currentOption !== option) {
    currentOption = option;
    pointsByDate = new Map();
    if (headingElement) headingElement.textContent = option.title;
    if (canvas) canvas.removeAttribute("title");
  }
  loadRange();
}

function resizeAndRebuild() {
  if (!canvas || !renderer) return;
  const { width, height } = canvas.getBoundingClientRect();
  if (renderer.resize(width, height)) rebuildGrid();
}

function loadRange() {
  if (!currentOption) return;

  abortController?.abort();
  const generation = ++loadGeneration;
  const option = currentOption;
  const controller = new AbortController();
  abortController = controller;
  currentDates = dateRange(from, to);
  currentDateIndex = createDateIndex(currentDates);

  rebuildGrid();

  const missing = currentDates.filter((date) => !pointsByDate.has(date));
  let completed = currentDates.length - missing.length;
  let failed = 0;
  updateStatus(completed, currentDates.length, failed);

  if (!missing.length) return;

  let cursor = 0;
  const workers = Array.from({
    length: Math.min(MAX_PARALLEL_FETCHES, missing.length),
  }).map(async () => {
    let index = nextMissingIndex();
    while (index !== undefined) {
      const date = missing[index];
      try {
        const points = await option.points.fetch(date, controller.signal);
        if (isCurrentLoad(option, controller, generation)) {
          pointsByDate.set(date, points);
          addDateToGrid(date, points);
        }
      } catch (error) {
        if (controller.signal.aborted) return;
        failed += 1;
        console.error(`Failed to fetch heatmap points for ${date}`, error);
      } finally {
        if (isCurrentLoad(option, controller, generation)) {
          completed += 1;
          updateStatus(completed, currentDates.length, failed);
        }
      }
      index = nextMissingIndex();
    }
  });

  void Promise.all(workers).then(() => {
    if (isCurrentLoad(option, controller, generation)) {
      updateStatus(completed, currentDates.length, failed);
    }
  });

  function nextMissingIndex() {
    if (cursor >= missing.length) return undefined;
    const index = cursor;
    cursor += 1;
    return index;
  }
}

/**
 * @param {HeatmapOption} option
 * @param {AbortController} controller
 * @param {number} generation
 */
function isCurrentLoad(option, controller, generation) {
  return (
    currentOption === option &&
    abortController === controller &&
    loadGeneration === generation &&
    !controller.signal.aborted
  );
}

function rebuildGrid() {
  if (
    !currentOption ||
    !renderer ||
    renderer.width < 1 ||
    renderer.height < 1 ||
    !currentDates.length
  ) {
    currentGrid = undefined;
    return;
  }

  currentGrid = currentOption.cells.create({
    dates: currentDates,
    width: renderer.width,
    height: renderer.height,
  });

  for (let i = 0; i < currentDates.length; i++) {
    const points = pointsByDate.get(currentDates[i]);
    if (points) currentGrid.add(i, points);
  }

  paint();
}

/**
 * @param {string} date
 * @param {HeatmapPoints} points
 */
function addDateToGrid(date, points) {
  if (!currentGrid) return;
  const dateIndex = currentDateIndex.get(date);
  if (dateIndex === undefined) return;
  const dirtyCol = currentGrid.add(dateIndex, points);
  if (dirtyCol !== undefined) paint([dirtyCol]);
}

/** @param {Iterable<number>} [dirty] */
function paint(dirty) {
  if (!renderer || !currentGrid || !currentOption) return;
  const grid = currentGrid;
  const option = currentOption;
  renderer.paint(
    grid.cols,
    grid.rows,
    (col, row) =>
      option.color(grid.getValue(col, row), { dark, grid, col, row }),
    dirty,
  );
}

/** @param {MouseEvent} event */
function updateTooltip(event) {
  if (!canvas || !currentGrid || !currentOption?.tooltip) return;
  const rect = canvas.getBoundingClientRect();
  const col = Math.floor(((event.clientX - rect.left) * currentGrid.cols) / rect.width);
  const row = Math.floor(((event.clientY - rect.top) * currentGrid.rows) / rect.height);
  if (col < 0 || col >= currentGrid.cols || row < 0 || row >= currentGrid.rows) {
    canvas.removeAttribute("title");
    return;
  }
  canvas.title = currentOption.tooltip({ grid: currentGrid, col, row });
}

/**
 * @param {readonly string[]} dates
 * @returns {Map<string, number>}
 */
function createDateIndex(dates) {
  const map = new Map();
  for (let i = 0; i < dates.length; i++) {
    map.set(dates[i], i);
  }
  return map;
}

/**
 * @param {number} completed
 * @param {number} total
 * @param {number} failed
 */
function updateStatus(completed, total, failed) {
  if (!statusElement) return;
  if (completed >= total) {
    statusElement.textContent = failed ? `${failed} failed` : "";
  } else {
    statusElement.textContent = failed
      ? `${completed}/${total} · ${failed} failed`
      : `${completed}/${total}`;
  }
}

function createRangeControls() {
  const fieldset = document.createElement("fieldset");
  fieldset.classList.add("heatmap-controls");

  statusElement = document.createElement("small");
  statusElement.classList.add("heatmap-status");

  const currentYear = new Date().getUTCFullYear();
  const fromChoices = createFromChoices(currentYear);
  const toChoices = createToChoices(currentYear);
  let fromChoice = fromChoices[0];
  let toChoice = toChoices[0];

  const fromSelect = createSelect({
    id: "heatmap-from",
    choices: fromChoices,
    initialValue: fromChoice,
    onChange(choice) {
      fromChoice = choice;
      setRange(fromChoice.date, toChoice.date);
    },
    toKey: rangeChoiceKey,
    toLabel: rangeChoiceLabel,
  });
  const toSelect = createSelect({
    id: "heatmap-to",
    choices: toChoices,
    initialValue: toChoice,
    onChange(choice) {
      toChoice = choice;
      setRange(fromChoice.date, toChoice.date);
    },
    toKey: rangeChoiceKey,
    toLabel: rangeChoiceLabel,
  });

  const fromLabel = createControlField("from", fromSelect);
  const toLabel = createControlField("to", toSelect);

  fieldset.append(fromLabel, toLabel, statusElement);

  return fieldset;
}

/**
 * @param {number} currentYear
 * @returns {RangeChoice[]}
 */
function createFromChoices(currentYear) {
  const choices = [{ label: "genesis", date: GENESIS_DATE }];
  for (let year = 2009; year <= currentYear; year++) {
    choices.push({ label: String(year), date: yearStartISODate(year) });
  }
  return choices;
}

/**
 * @param {number} currentYear
 * @returns {RangeChoice[]}
 */
function createToChoices(currentYear) {
  const choices = [{ label: "today", date: todayISODate() }];
  for (let year = currentYear; year >= 2009; year--) {
    choices.push({ label: String(year), date: yearEndISODate(year) });
  }
  return choices;
}

/**
 * @param {RangeChoice} choice
 */
function rangeChoiceKey(choice) {
  return choice.label;
}

/**
 * @param {RangeChoice} choice
 */
function rangeChoiceLabel(choice) {
  return choice.label;
}

/**
 * @param {string} text
 * @param {HTMLElement} control
 */
function createControlField(text, control) {
  const label = document.createElement("div");
  label.classList.add("heatmap-control");
  const span = document.createElement("span");
  span.textContent = text;
  const select = control.querySelector("select");
  if (select) select.ariaLabel = text;
  label.append(span, control);
  return label;
}

/**
 * @param {string} nextFrom
 * @param {string} nextTo
 */
function setRange(nextFrom, nextTo) {
  if (nextFrom > nextTo) {
    from = nextTo;
    to = nextFrom;
  } else {
    from = nextFrom;
    to = nextTo;
  }
  loadRange();
}

/** @param {number} year */
function yearStartISODate(year) {
  return toISODate(new Date(Date.UTC(year, 0, 1)));
}

/** @param {number} year */
function yearEndISODate(year) {
  return toISODate(
    new Date(
      Math.min(
        Date.UTC(year, 11, 31),
        Date.parse(`${todayISODate()}T00:00:00Z`),
      ),
    ),
  );
}
