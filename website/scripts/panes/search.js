import {
  searchInput,
  searchLabelElement,
  searchResultsElement,
} from "../utils/elements.js";
import { QuickMatch } from "../modules/quickmatch-js/0.3.1/src/index.js";

/**
 * @param {Options} options
 */
export function initSearch(options) {
  console.log("search: init");

  const haystack = options.list.map((option) => option.title.toLowerCase());
  const titleToOption = new Map(
    options.list.map((option) => [option.title.toLowerCase(), option]),
  );

  const matcher = new QuickMatch(haystack);

  function inputEvent() {
    const needle = /** @type {string} */ (searchInput.value).trim();

    searchResultsElement.scrollTo({ top: 0 });
    searchResultsElement.innerHTML = "";

    if (needle.length < 3) {
      const li = window.document.createElement("li");
      li.textContent = 'e.g. "BTC"';
      li.style.color = "var(--off-color)";
      searchResultsElement.appendChild(li);
      return;
    }

    const matches = matcher.matches(needle);

    if (!matches.length) {
      const li = window.document.createElement("li");
      li.textContent = "No results";
      li.style.color = "var(--off-color)";
      searchResultsElement.appendChild(li);
      return;
    }

    matches.forEach((title) => {
      const option = titleToOption.get(title);
      if (!option) return;

      const li = window.document.createElement("li");
      searchResultsElement.appendChild(li);

      const element = options.createOptionElement({
        option,
        name: option.title,
      });

      if (element) {
        li.append(element);
      }
    });
  }

  inputEvent();

  searchInput.addEventListener("input", inputEvent);
  const len = searchInput.value.length;
  searchInput.setSelectionRange(len, len);
}

document.addEventListener("keydown", (e) => {
  const el = document.activeElement;
  const isTextInput =
    el?.tagName === "INPUT" &&
    /** @type {HTMLInputElement} */ (el).type === "text";
  if (e.key === "/" && !isTextInput) {
    e.preventDefault();
    searchLabelElement.click();
    searchInput.focus();
  }
});
