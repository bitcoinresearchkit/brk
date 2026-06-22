import { createElement } from "../dom.js";

const FILL_MS = 2_000;
const DRAIN_MS = 600;

/**
 * @param {number} value
 */
function clampProgress(value) {
  return Math.max(0, Math.min(1, value));
}

/**
 * @param {HTMLButtonElement} button
 * @param {() => void} onHold
 */
function bindHold(button, onHold) {
  /** @type {number | undefined} */
  let frame;
  let holding = false;
  let progress = 0;
  let previous = 0;

  function render() {
    button.style.setProperty("--hold-progress", String(progress));
    button.style.setProperty("--hold-progress-width", `${progress * 100}%`);
    button.classList.toggle("active", progress > 0);
  }

  function stop() {
    if (frame === undefined) return;

    cancelAnimationFrame(frame);
    frame = undefined;
  }

  /**
   * @param {number} now
   */
  function tick(now) {
    const elapsed = now - previous;
    const rate = elapsed / (holding ? FILL_MS : DRAIN_MS);

    previous = now;
    progress = clampProgress(progress + (holding ? rate : -rate));
    render();

    if (holding && progress === 1) {
      stop();
      holding = false;
      progress = 0;
      button.classList.remove("holding");
      render();
      onHold();
      return;
    }

    if (!holding && progress === 0) {
      stop();
      return;
    }

    frame = requestAnimationFrame(tick);
  }

  function run() {
    if (frame !== undefined) return;

    previous = performance.now();
    frame = requestAnimationFrame(tick);
  }

  function release() {
    if (!holding) return;

    holding = false;
    button.classList.remove("holding");
    run();
  }

  function hold() {
    stop();

    holding = true;
    button.classList.add("holding");
    run();
  }

  render();

  button.addEventListener("pointerdown", (event) => {
    if (event.button !== 0) return;

    button.setPointerCapture(event.pointerId);
    hold();
  });
  button.addEventListener("pointerup", release);
  button.addEventListener("pointercancel", release);
  button.addEventListener("lostpointercapture", release);
  button.addEventListener("keydown", (event) => {
    if (event.repeat || (event.key !== " " && event.key !== "Enter")) return;

    event.preventDefault();
    hold();
  });
  button.addEventListener("keyup", (event) => {
    if (event.key === " " || event.key === "Enter") {
      release();
    }
  });
  button.addEventListener("blur", release);
}

/**
 * @param {Object} options
 * @param {string} options.label
 * @param {string} options.title
 * @param {string} [options.className]
 * @param {() => void} options.onHold
 */
export function createHoldButton(options) {
  const button = createElement("button", "hold");
  const label = document.createElement("span");

  if (options.className) {
    button.classList.add(options.className);
  }
  button.type = "button";
  button.dataset.label = options.label;
  button.title = options.title;
  label.append(options.label);
  button.append(label);
  bindHold(button, options.onHold);

  return button;
}
