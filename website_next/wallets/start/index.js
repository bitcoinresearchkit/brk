import { createElement } from "../dom.js";
import { createPersistentVault } from "./persistent.js";
import { createStartStory } from "./story.js";
import { createTemporaryVault } from "./temporary.js";

/**
 * @typedef {"create" | "unlock"} StartMode
 */

/**
 * @typedef {Object} StartOptions
 * @property {StartMode} mode
 * @property {(password: string, button: HTMLButtonElement, status: HTMLElement) => boolean | void | Promise<boolean | void>} onPassword
 * @property {() => void} onEphemeral
 * @property {() => void} [onReset]
 */


/**
 * @param {StartOptions} options
 */
export function createStart(options) {
  const section = createElement("section", "start");
  const modes = document.createElement("div");
  const divider = document.createElement("p");
  const persistent = createPersistentVault({
    mode: options.mode,
    onPassword: options.onPassword,
    onReset: options.onReset,
  });

  divider.append("OR");
  modes.append(
    persistent.element,
    divider,
    createTemporaryVault(options.onEphemeral),
  );
  section.append(createStartStory(), modes);
  queueMicrotask(() => {
    persistent.password.focus({ preventScroll: true });
  });

  return section;
}
