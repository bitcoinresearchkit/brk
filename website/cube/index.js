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

  cube.append(
    face("glass", "bottom"),
    face("glass", "rear-right"),
    face("glass", "rear-left"),
    face("liquid", "bottom"),
    face("liquid", "rear-right"),
    face("liquid", "rear-left"),
    face("liquid", "right"),
    face("liquid", "left"),
    face("liquid", "top"),
    face("glass", "right"),
    face("glass", "left"),
    face("glass", "top"),
  );

  return cube;
}
