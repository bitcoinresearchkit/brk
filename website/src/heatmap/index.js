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
/** @type {Map<string, HeatmapPoints>} */
let pointsByDate = new Map();
/** @type {AbortController | undefined} */
let abortController;
const dirtyCols = new Set();
let loadGeneration = 0;
let paintScheduled = false;
let initialized = false;
let from = yearStartISODate(new Date().getUTCFullYear());
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

  /** @type {{ date: string, dateIndex: number }[]} */
  const missing = [];
  for (let dateIndex = 0; dateIndex < currentDates.length; dateIndex++) {
    const date = currentDates[dateIndex];
    if (!pointsByDate.has(date)) missing.push({ date, dateIndex });
  }
  let completed = currentDates.length - missing.length;
  let failed = 0;
  updateStatus(completed, currentDates.length, failed);

  if (!missing.length) {
    rebuildGrid();
    abortController = undefined;
    return;
  }

  let cursor = 0;
  let needsRebuild = false;
  const workers = Array.from({
    length: Math.min(MAX_PARALLEL_FETCHES, missing.length),
  }).map(async () => {
    let index = nextMissingIndex();
    while (index !== undefined) {
      const entry = missing[index];
      try {
        const points = await option.points.fetch(
          entry.date,
          controller.signal,
          (points) => {
            if (isCurrentLoad(option, controller, generation)) {
              setPoints(entry, points);
            }
          },
        );
        if (isCurrentLoad(option, controller, generation)) {
          setPoints(entry, points);
        }
      } catch (error) {
        if (controller.signal.aborted) return;
        failed += 1;
        console.error(`Failed to fetch heatmap points for ${entry.date}`, error);
      } finally {
        if (isCurrentLoad(option, controller, generation)) {
          completed += 1;
          updateStatus(completed, currentDates.length, failed);
        }
      }
      index = nextMissingIndex();
    }
  });

  rebuildGrid();

  void Promise.all(workers).then(() => {
    if (isCurrentLoad(option, controller, generation)) {
      updateStatus(completed, currentDates.length, failed);
      if (needsRebuild) {
        rebuildGrid();
      } else {
        paint();
      }
    }
  });

  function nextMissingIndex() {
    if (cursor >= missing.length) return undefined;
    const index = cursor;
    cursor += 1;
    return index;
  }

  /**
   * @param {{ date: string, dateIndex: number }} entry
   * @param {HeatmapPoints} points
   */
  function setPoints(entry, points) {
    const previous = pointsByDate.get(entry.date);
    if (previous && samePoints(previous, points)) return;
    pointsByDate.set(entry.date, points);
    if (previous) {
      needsRebuild = true;
    } else {
      addDateToGrid(entry.dateIndex, points);
    }
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

  currentGrid = currentOption.grid.create({
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
 * @param {number} dateIndex
 * @param {HeatmapPoints} points
 */
function addDateToGrid(dateIndex, points) {
  if (!currentGrid) return;
  const dirtyCol = currentGrid.add(dateIndex, points);
  if (dirtyCol !== undefined) schedulePaint(dirtyCol);
}

/**
 * @param {HeatmapPoints} a
 * @param {HeatmapPoints} b
 */
function samePoints(a, b) {
  if (a === b) return true;
  if (a.kind !== b.kind || a.values !== b.values) return false;
  if (a.kind === "implicit" && b.kind === "implicit") {
    return a.yStart === b.yStart && a.yStep === b.yStep;
  }
  if (a.kind === "explicit" && b.kind === "explicit") return a.y === b.y;
  return false;
}

/** @param {number} col */
function schedulePaint(col) {
  dirtyCols.add(col);
  if (paintScheduled) return;
  paintScheduled = true;
  requestAnimationFrame(() => {
    paintScheduled = false;
    if (!dirtyCols.size) return;
    const cols = Array.from(dirtyCols);
    dirtyCols.clear();
    paint(cols);
  });
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

  statusElement = document.createElement("small");

  const currentYear = new Date().getUTCFullYear();
  const fromChoices = createFromChoices(currentYear);
  const toChoices = createToChoices(currentYear);
  let fromChoice = fromChoices.at(-1) ?? fromChoices[0];
  let toChoice = toChoices[0];

  const fromSelect = createSelect({
    id: "heatmap-from",
    label: "from",
    choices: fromChoices,
    initialValue: fromChoice,
    onChange(choice) {
      fromChoice = choice;
      if (fromChoice.date > toChoice.date) {
        toChoice = findSameLabelChoice(toChoices, fromChoice, toChoices[0]);
        toSelect.set(toChoice);
      }
      setRange(fromChoice.date, toChoice.date);
    },
    toKey: rangeChoiceLabel,
    toLabel: rangeChoiceLabel,
  });
  const toSelect = createSelect({
    id: "heatmap-to",
    label: "to",
    choices: toChoices,
    initialValue: toChoice,
    onChange(choice) {
      toChoice = choice;
      if (fromChoice.date > toChoice.date) {
        fromChoice = findSameLabelChoice(fromChoices, toChoice, fromChoices[0]);
        fromSelect.set(fromChoice);
      }
      setRange(fromChoice.date, toChoice.date);
    },
    toKey: rangeChoiceLabel,
    toLabel: rangeChoiceLabel,
  });

  fieldset.append(fromSelect.element, toSelect.element, statusElement);

  return fieldset;
}

/**
 * @param {number} currentYear
 * @returns {RangeChoice[]}
 */
function createFromChoices(currentYear) {
  const choices = [{ label: "genesis", date: GENESIS_DATE }];
  for (let year = 2009; year <= currentYear; year++) {
    choices.push({
      label: String(year),
      date: year === 2009 ? GENESIS_DATE : yearStartISODate(year),
    });
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
function rangeChoiceLabel(choice) {
  return choice.label;
}

/**
 * @param {readonly RangeChoice[]} choices
 * @param {RangeChoice} choice
 * @param {RangeChoice} fallback
 */
function findSameLabelChoice(choices, choice, fallback) {
  return choices.find((candidate) => candidate.label === choice.label) ?? fallback;
}

/**
 * @param {string} nextFrom
 * @param {string} nextTo
 */
function setRange(nextFrom, nextTo) {
  from = nextFrom;
  to = nextTo;
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
