import { initPrice, onPrice } from "./utils/price.js";
import { brk } from "./utils/client.js";
import { onFirstIntersection, getElementById, isHidden } from "./utils/dom.js";
import { initOptions } from "./options/full.js";
import {
  init as initChart,
  setOption as setChartOption,
} from "./panes/chart.js";
import { init as initExplorer } from "./explorer/index.js";
import { init as initSearch } from "./panes/search.js";
import { idle } from "./utils/timing.js";
import { readStored, removeStored, writeToStorage } from "./utils/storage.js";
import {
  asideElement,
  asideLabelElement,
  bodyElement,
  chartElement,
  explorerElement,
  frameSelectorsElement,
  mainElement,
  navElement,
  navLabelElement,
  pinButtonElement,
  searchElement,
  style,
} from "./utils/elements.js";

const DESKTOP_QUERY = window.matchMedia("(min-width: 768px)");
let sidebarPinned = readStored("sidebar-pinned") !== "false";

function updateLayout() {
  const layout = DESKTOP_QUERY.matches && sidebarPinned ? "desktop" : "mobile";
  document.documentElement.dataset.layout = layout;
  if (layout === "desktop") {
    asideElement.parentElement !== bodyElement &&
      bodyElement.append(asideElement);
  } else {
    asideElement.parentElement !== mainElement &&
      mainElement.append(asideElement);
  }
}

DESKTOP_QUERY.addEventListener("change", updateLayout);
updateLayout();

pinButtonElement.addEventListener("click", () => {
  sidebarPinned = !sidebarPinned;
  writeToStorage("sidebar-pinned", String(sidebarPinned));
  pinButtonElement.textContent = sidebarPinned ? "Unpin" : "Pin";
  pinButtonElement.title = sidebarPinned ? "Unpin sidebar" : "Pin sidebar";
  updateLayout();
  if (!sidebarPinned) asideLabelElement.click();
});

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
}
initFrameSelectors();

initPrice(brk);

onPrice((price) => {
  console.log("close:", price);
  window.document.title = `${price.toLocaleString("en-us")} | ${window.location.host}`;
});

const options = initOptions();

window.addEventListener("popstate", () => options.resolveUrl());

function initSelected() {
  let firstRun = true;
  function initSelectedFrame() {
    if (!firstRun) throw Error("Unreachable");
    firstRun = false;

    let previousElement = /** @type {HTMLElement | undefined} */ (undefined);
    let firstTimeLoadingChart = true;
    let firstTimeLoadingExplorer = true;

    options.selected.onChange((option) => {
      /** @type {HTMLElement | undefined} */
      let element;

      switch (option.kind) {
        case "explorer": {
          element = explorerElement;

          if (firstTimeLoadingExplorer) {
            initExplorer();
          }
          firstTimeLoadingExplorer = false;

          break;
        }
        case "chart": {
          element = chartElement;

          if (firstTimeLoadingChart) {
            initChart();
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

idle(() => options.setParent(navElement));

onFirstIntersection(navElement, () => {
  options.setParent(navElement);

  navElement
    .querySelector(`a[href="${window.document.location.pathname}"]`)
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
