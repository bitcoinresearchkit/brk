import { readStored, removeStored, writeToStorage } from "./storage.js";

const preferredColorSchemeMatchMedia = window.matchMedia(
  "(prefers-color-scheme: dark)",
);
const stored = readStored("theme");
const initial = stored ? stored === "dark" : preferredColorSchemeMatchMedia.matches;

export let dark = initial;

/** @type {Set<() => void>} */
const callbacks = new Set();

/** @param {() => void} callback */
export function onChange(callback) {
  callbacks.add(callback);
  return () => callbacks.delete(callback);
}

/** @param {boolean} value */
export function setDark(value) {
  if (dark === value) return;
  dark = value;
  apply(value);
  callbacks.forEach((cb) => cb());
}

/** @param {boolean} isDark */
function apply(isDark) {
  document.documentElement.style.colorScheme = isDark ? "dark" : "light";
}
apply(initial);

preferredColorSchemeMatchMedia.addEventListener("change", ({ matches }) => {
  if (!readStored("theme")) {
    setDark(matches);
  }
});

function invert() {
  const newValue = !dark;
  setDark(newValue);
  if (newValue === preferredColorSchemeMatchMedia.matches) {
    removeStored("theme");
  } else {
    writeToStorage("theme", newValue ? "dark" : "light");
  }
}

document.getElementById("invert-button")?.addEventListener("click", invert);
