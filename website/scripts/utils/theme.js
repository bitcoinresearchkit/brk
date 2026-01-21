import signals from "../signals.js";
import { readStored, removeStored, writeToStorage } from "./storage.js";

const preferredColorSchemeMatchMedia = window.matchMedia(
  "(prefers-color-scheme: dark)",
);
const stored = readStored("theme");
const initial = stored ? stored === "dark" : preferredColorSchemeMatchMedia.matches;

export const dark = signals.createSignal(initial);

/** @param {boolean} isDark */
function apply(isDark) {
  document.documentElement.style.colorScheme = isDark ? "dark" : "light";
}
apply(initial);

preferredColorSchemeMatchMedia.addEventListener("change", ({ matches }) => {
  if (!readStored("theme")) {
    dark.set(matches);
    apply(matches);
  }
});

function invert() {
  const newValue = !dark();
  dark.set(newValue);
  apply(newValue);
  if (newValue === preferredColorSchemeMatchMedia.matches) {
    removeStored("theme");
  } else {
    writeToStorage("theme", newValue ? "dark" : "light");
  }
}

document.getElementById("invert-button")?.addEventListener("click", invert);
