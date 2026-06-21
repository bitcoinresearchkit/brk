import { createElement } from "../../dom.js";

const FILL_MS = 2_000;
const DRAIN_MS = 600;
const LABEL = "Reset vault";

/**
 * @param {number} value
 */
function clampProgress(value) {
  return Math.max(0, Math.min(1, value));
}

/**
 * @param {HTMLButtonElement} button
 * @param {() => void} onReset
 */
function bindHold(button, onReset) {
  /** @type {number | undefined} */
  let frame;
  let holding = false;
  let progress = 0;
  let previous = 0;

  function render() {
    button.style.setProperty("--reset-progress", String(progress));
    button.style.setProperty("--reset-progress-width", `${progress * 100}%`);
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
      onReset();
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
 * @param {() => void} onReset
 */
export function createResetButton(onReset) {
  const button = createElement("button", "reset");
  const label = document.createElement("span");

  button.type = "button";
  button.dataset.label = LABEL;
  button.title = "Hold to reset";
  label.append(LABEL);
  button.append(label);
  bindHold(button, onReset);

  return button;
}
