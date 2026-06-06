/** @param {HTMLElement} target */
export function createFullscreenButton(target) {
  const button = document.createElement("button");

  function update() {
    const active = document.fullscreenElement === target;

    button.textContent = active ? "Exit" : "Full";
    button.setAttribute("aria-pressed", active.toString());
  }

  button.type = "button";
  button.dataset.chart = "fullscreen";
  button.addEventListener("click", () => {
    if (document.fullscreenElement === target) {
      void document.exitFullscreen();
    } else {
      void target.requestFullscreen();
    }
  });
  target.addEventListener("fullscreenchange", update);
  update();

  return button;
}
