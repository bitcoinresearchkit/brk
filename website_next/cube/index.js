/**
 * @param {{ fill?: number }} [options]
 */
export function createCube({ fill = 0.5 } = {}) {
  const cube = document.createElement("span");
  cube.className = "cube";
  cube.setAttribute("aria-hidden", "true");
  cube.style.setProperty("--fill", String(fill));

  /** @param {...string} names */
  const face = (...names) => {
    const element = document.createElement("span");
    element.className = `face ${names.join(" ")}`;
    return element;
  };

  const faces = /** @type {const} */ ([
    ["glass", "front", "bottom"],
    ["glass", "rear", "right"],
    ["glass", "rear", "left"],
    ["liquid", "front", "bottom"],
    ["liquid", "rear", "right"],
    ["liquid", "rear", "left"],
    ["liquid", "front", "right"],
    ["liquid", "front", "left"],
    ["liquid", "front", "top"],
    ["glass", "front", "right"],
    ["glass", "front", "left"],
    ["glass", "front", "top"],
  ]);

  cube.append(...faces.map((names) => face(...names)));

  return cube;
}
