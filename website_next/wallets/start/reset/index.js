import { createHoldButton } from "../../hold/index.js";

/**
 * @param {() => void} onReset
 */
export function createResetButton(onReset) {
  return createHoldButton({
    label: "Reset vault",
    title: "Hold to reset",
    className: "reset",
    onHold: onReset,
  });
}
