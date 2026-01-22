import { webSockets } from "./utils/ws.js";
import * as formatters from "./utils/format.js";
import { onFirstIntersection, getElementById, isHidden } from "./utils/dom.js";
import { BrkClient } from "./modules/brk-client/index.js";
import { initOptions } from "./options/full.js";
import {
  init as initChart,
  setOption as setChartOption,
} from "./panes/chart.js";
import { initSearch } from "./panes/search.js";
import { next } from "./utils/timing.js";
import { replaceHistory } from "./utils/url.js";
import { removeStored, writeToStorage } from "./utils/storage.js";
import {
  asideElement,
  asideLabelElement,
  bodyElement,
  chartElement,
  frameSelectorsElement,
  mainElement,
  navElement,
  navLabelElement,
  searchElement,
  style,
} from "./utils/elements.js";

function initFrameSelectors() {
  const children = Array.from(frameSelectorsElement.children);

  /** @type {HTMLElement | undefined} */
  let focusedFrame = undefined;

  for (let i = 0; i < children.length; i++) {
    const element = children[i];

    switch (element.tagName) {
      case "LABEL": {
        element.addEventListener("click", () => {
          const inputId = element.getAttribute("for");

          if (!inputId) {
            console.log(element, element.getAttribute("for"));
            throw "Input id in label not found";
          }

          const input = window.document.getElementById(inputId);

          if (!input || !("value" in input)) {
            throw "Not input or no value";
          }

          const frame = window.document.getElementById(
            /** @type {string} */ (input.value),
          );

          if (!frame) {
            console.log(input.value);
            throw "Frame element doesn't exist";
          }

          if (frame === focusedFrame) {
            return;
          }

          frame.hidden = false;
          if (focusedFrame) {
            focusedFrame.hidden = true;
          }
          focusedFrame = frame;
        });
        break;
      }
    }
  }

  asideLabelElement.click();

  // When going from mobile view to desktop view, if selected frame was open, go to the nav frame
  new IntersectionObserver((entries) => {
    for (let i = 0; i < entries.length; i++) {
      if (
        !entries[i].isIntersecting &&
        entries[i].target === asideLabelElement &&
        focusedFrame == asideElement
      ) {
        navLabelElement.click();
      }
    }
  }).observe(asideLabelElement);

  function setAsideParent() {
    const { clientWidth } = window.document.documentElement;
    const MEDIUM_WIDTH = 768;
    if (clientWidth >= MEDIUM_WIDTH) {
      asideElement.parentElement !== bodyElement &&
        bodyElement.append(asideElement);
    } else {
      asideElement.parentElement !== mainElement &&
        mainElement.append(asideElement);
    }
  }

  setAsideParent();

  window.addEventListener("resize", setAsideParent);
}
initFrameSelectors();

const brk = new BrkClient("https://next.bitview.space");
// const brk = new BrkClient("/");

console.log(`VERSION = ${brk.VERSION}`);

webSockets.kraken1dCandle.onLatest((latest) => {
  console.log("close:", latest.close);
  window.document.title = `${latest.close.toLocaleString("en-us")} | ${window.location.host}`;
});

const options = initOptions(brk);

window.addEventListener("popstate", (_event) => {
  const path = window.document.location.pathname.split("/").filter((v) => v);
  let folder = options.tree;

  while (path.length) {
    const id = path.shift();
    const res = folder.find((v) => id === formatters.stringToId(v.name));
    if (!res) throw "Option not found";
    if (path.length >= 1) {
      if (!("tree" in res)) {
        throw "Unreachable";
      }
      folder = res.tree;
    } else {
      if ("tree" in res) {
        throw "Unreachable";
      }
      options.selected.set(res);
    }
  }
});

function initSelected() {
  let firstRun = true;
  function initSelectedFrame() {
    if (!firstRun) throw Error("Unreachable");
    firstRun = false;

    let previousElement = /** @type {HTMLElement | undefined} */ (undefined);
    let firstTimeLoadingChart = true;

    options.selected.onChange((option) => {
      /** @type {HTMLElement | undefined} */
      let element;

      switch (option.kind) {
        case "chart": {
          element = chartElement;

          if (firstTimeLoadingChart) {
            initChart(brk);
          }
          firstTimeLoadingChart = false;

          setChartOption(option);

          break;
        }
        case "link": {
          return;
        }
      }

      if (!element) throw "Element should be set";

      if (element !== previousElement) {
        if (previousElement) previousElement.hidden = true;
        element.hidden = false;
      }

      if (!previousElement) {
        replaceHistory({ pathname: option.path });
      }

      previousElement = element;
    });
  }

  let firstMobileSwitch = true;
  options.selected.onChange(() => {
    if (!firstMobileSwitch && !isHidden(asideLabelElement)) {
      asideLabelElement.click();
    }
    firstMobileSwitch = false;
  });

  onFirstIntersection(asideElement, initSelectedFrame);
}
initSelected();

onFirstIntersection(navElement, async () => {
  options.setParent(navElement);

  const option = options.selected.value;
  if (!option) throw "Selected should be set by now";
  const path = [...option.path];

  /** @type {HTMLUListElement | null} */
  let ul = /** @type {any} */ (null);
  async function getFirstChild() {
    try {
      ul = /** @type {HTMLUListElement} */ (navElement.firstElementChild);
      await next();
      if (!ul) {
        await getFirstChild();
      }
    } catch (_) {
      await next();
      await getFirstChild();
    }
  }
  await getFirstChild();
  if (!ul) throw Error("Unreachable");

  while (path.length > 1) {
    const name = path.shift();
    if (!name) throw "Unreachable";
    /** @type {HTMLDetailsElement[]} */
    let detailsList = [];
    while (!detailsList.length) {
      detailsList = Array.from(ul.querySelectorAll(":scope > li > details"));
      if (!detailsList.length) {
        await next();
      }
    }
    const details = detailsList.find((s) => s.dataset.name == name);
    if (!details) return;
    details.open = true;
    ul = null;
    while (!ul) {
      const uls = /** @type {HTMLUListElement[]} */ (
        Array.from(details.querySelectorAll(":scope > ul"))
      );
      if (!uls.length) {
        await next();
      } else if (uls.length > 1) {
        throw "Shouldn't be possible";
      } else {
        ul = /** @type {HTMLUListElement} */ (uls.pop());
      }
    }
  }
  /** @type {HTMLAnchorElement[]} */
  let anchors = [];
  while (!anchors.length) {
    anchors = Array.from(ul.querySelectorAll(":scope > li > a"));
    if (!anchors.length) {
      await next();
    }
  }
  anchors
    .find((a) => a.getAttribute("href") == window.document.location.pathname)
    ?.scrollIntoView({
      behavior: "instant",
      block: "center",
    });
});

onFirstIntersection(searchElement, () => {
  initSearch(options);
});

function initDesktopResizeBar() {
  const resizeBar = getElementById("resize-bar");
  let resize = false;
  let startingWidth = 0;
  let startingClientX = 0;

  const barWidthLocalStorageKey = "bar-width";

  /**
   * @param {number | null} width
   */
  function setBarWidth(width) {
    // TODO: Check if should be a signal ??
    try {
      if (typeof width === "number") {
        mainElement.style.width = `${width}px`;
        writeToStorage(barWidthLocalStorageKey, String(width));
      } else {
        mainElement.style.width = style.getPropertyValue(
          "--default-main-width",
        );
        removeStored(barWidthLocalStorageKey);
      }
    } catch (_) {}
  }

  /**
   * @param {MouseEvent} event
   */
  function mouseMoveEvent(event) {
    if (resize) {
      setBarWidth(startingWidth + (event.clientX - startingClientX));
    }
  }

  resizeBar.addEventListener("mousedown", (event) => {
    startingClientX = event.clientX;
    startingWidth = mainElement.clientWidth;
    resize = true;
    window.document.documentElement.dataset.resize = "";
    window.addEventListener("mousemove", mouseMoveEvent);
  });

  resizeBar.addEventListener("dblclick", () => {
    setBarWidth(null);
  });

  const setResizeFalse = () => {
    resize = false;
    delete window.document.documentElement.dataset.resize;
    window.removeEventListener("mousemove", mouseMoveEvent);
  };
  window.addEventListener("mouseup", setResizeFalse);
  window.addEventListener("mouseleave", setResizeFalse);
}
initDesktopResizeBar();
