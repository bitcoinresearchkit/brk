import {
  searchInput,
  searchLabelElement,
  searchResultsElement,
} from "../utils/elements.js";
import ufuzzy from "../modules/leeoniya-ufuzzy/1.0.19/dist/uFuzzy.mjs";

/**
 * @param {Options} options
 */
export function initSearch(options) {
  console.log("search: init");

  const haystack = options.list.map((option) => option.title);

  const RESULTS_PER_PAGE = 100;

  /**
   * @param {uFuzzy.SearchResult} searchResult
   * @param {number} pageIndex
   */
  function computeResultPage(searchResult, pageIndex) {
    /** @type {{ option: Option, title: string }[]} */
    let list = [];

    let [indexes, _info, order] = searchResult || [null, null, null];

    const minIndex = pageIndex * RESULTS_PER_PAGE;

    if (indexes?.length) {
      const maxIndex = Math.min(
        (order || indexes).length - 1,
        minIndex + RESULTS_PER_PAGE - 1,
      );

      list = Array(maxIndex - minIndex + 1);

      for (let i = minIndex; i <= maxIndex; i++) {
        let index = indexes[i];

        const title = haystack[index];

        list[i % 100] = {
          option: options.list[index],
          title,
        };
      }
    }

    return list;
  }

  /** @type {uFuzzy.Options} */
  const config = {
    intraIns: Infinity,
    intraChars: `[a-z\d' ]`,
  };

  const fuzzyMultiInsert = /** @type {uFuzzy} */ (
    ufuzzy({
      intraIns: 1,
    })
  );
  const fuzzyMultiInsertFuzzier = /** @type {uFuzzy} */ (ufuzzy(config));
  const fuzzySingleError = /** @type {uFuzzy} */ (
    ufuzzy({
      intraMode: 1,
      ...config,
    })
  );
  const fuzzySingleErrorFuzzier = /** @type {uFuzzy} */ (
    ufuzzy({
      intraMode: 1,
      ...config,
    })
  );

  function inputEvent() {
    const needle = /** @type {string} */ (searchInput.value);

    searchResultsElement.scrollTo({
      top: 0,
    });

    if (!needle) {
      searchResultsElement.innerHTML = "";
      return;
    }

    const outOfOrder = 5;
    const infoThresh = 5_000;

    let result = fuzzyMultiInsert?.search(
      haystack,
      needle,
      undefined,
      infoThresh,
    );

    if (!result?.[0]?.length || !result?.[1]) {
      result = fuzzyMultiInsert?.search(
        haystack,
        needle,
        outOfOrder,
        infoThresh,
      );
    }

    if (!result?.[0]?.length || !result?.[1]) {
      result = fuzzySingleError?.search(
        haystack,
        needle,
        outOfOrder,
        infoThresh,
      );
    }

    if (!result?.[0]?.length || !result?.[1]) {
      result = fuzzySingleErrorFuzzier?.search(
        haystack,
        needle,
        outOfOrder,
        infoThresh,
      );
    }

    if (!result?.[0]?.length || !result?.[1]) {
      result = fuzzyMultiInsertFuzzier?.search(
        haystack,
        needle,
        undefined,
        infoThresh,
      );
    }

    if (!result?.[0]?.length || !result?.[1]) {
      result = fuzzyMultiInsertFuzzier?.search(
        haystack,
        needle,
        outOfOrder,
        infoThresh,
      );
    }

    searchResultsElement.innerHTML = "";

    const list = computeResultPage(result, 0);

    list.forEach(({ option, title }) => {
      const li = window.document.createElement("li");
      searchResultsElement.appendChild(li);

      const element = options.createOptionElement({
        option,
        name: title,
      });

      if (element) {
        li.append(element);
      }
    });
  }

  if (searchInput.value) {
    inputEvent();
  }

  searchInput.addEventListener("input", inputEvent);
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
