import { createColors } from "./core/colors";
import { createWebSockets } from "./core/ws";
import * as formatters from "./core/format";
import modules from "./lazy";
import { onFirstIntersection, getElementById, isHidden } from "./core/dom";
import { next } from "./core/timing";
import { replaceHistory } from "./core/url";
import { removeStored, writeToStorage } from "./core/storage";
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
  searchElement,
  searchInput,
  searchResultsElement,
  simulationElement,
  style,
  tableElement,
} from "./core/elements";

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

Promise.all([
  modules.signals(),
  modules.brkClient(),
  modules.brkResources(),
  modules.options(),
]).then(([signals, { BrkClient, VERSION }, { createResources }, { initOptions }]) =>
  signals.createRoot(() => {
    const brk = new BrkClient("/");
    const resources = createResources(brk, signals);
    const owner = signals.getOwner();

    console.log(`VERSION = ${VERSION}`);

    function initDark() {
      const preferredColorSchemeMatchMedia = window.matchMedia(
        "(prefers-color-scheme: dark)",
      );
      const dark = signals.createSignal(preferredColorSchemeMatchMedia.matches);
      preferredColorSchemeMatchMedia.addEventListener(
        "change",
        ({ matches }) => {
          dark.set(matches);
        },
      );
      return dark;
    }
    const dark = initDark();

    const qrcode = signals.createSignal(/** @type {string | null} */ (null));

    // function createLastHeightResource() {
    //   const lastHeight = signals.createSignal(0);
    //   function fetchLastHeight() {
    //     utils.api.fetchLast(
    //       (h) => {
    //         lastHeight.set(h);
    //       },
    //       /** @satisfies {Height} */ (5),
    //       "height",
    //     );
    //   }
    //   fetchLastHeight();
    //   setInterval(fetchLastHeight, 10_000);
    //   return lastHeight;
    // }
    // const lastHeight = createLastHeightResource();

    const webSockets = createWebSockets(signals);

    const colors = createColors(dark);

    const options = initOptions({
      colors,
      signals,
      brk,
      qrcode,
    });

    window.addEventListener("popstate", (event) => {
      const path = window.document.location.pathname
        .split("/")
        .filter((v) => v);
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

        const owner = signals.getOwner();

        const chartOption = signals.createSignal(
          /** @type {ChartOption | null} */ (null),
        );
        const simOption = signals.createSignal(
          /** @type {SimulationOption | null} */ (null),
        );

        let previousElement = /** @type {HTMLElement | undefined} */ (
          undefined
        );
        let firstTimeLoadingChart = true;
        let firstTimeLoadingTable = true;
        let firstTimeLoadingSimulation = true;
        let firstTimeLoadingExplorer = true;

        signals.createEffect(options.selected, (option) => {
          /** @type {HTMLElement} */
          let element;

          switch (option.kind) {
            case "explorer": {
              element = explorerElement;

              if (firstTimeLoadingExplorer) {
                const chartPkg = modules.chart();
                import("./panes/explorer.js").then(({ init }) =>
                  chartPkg.then(({ createChartElement }) =>
                    signals.runWithOwner(owner, () =>
                      init({
                        colors,
                        createChartElement,
                        option: /** @type {Accessor<ChartOption>} */ (
                          chartOption
                        ),
                        signals,
                        webSockets,
                        resources,
                        brk,
                      }),
                    ),
                  ),
                );
              }
              firstTimeLoadingExplorer = false;

              break;
            }
            case "chart": {
              element = chartElement;

              chartOption.set(option);

              if (firstTimeLoadingChart) {
                const chartPkg = modules.chart();
                import("./panes/chart/index.js").then(
                  ({ init: initChartsElement }) =>
                    chartPkg.then(({ createChartElement }) =>
                      signals.runWithOwner(owner, () =>
                        initChartsElement({
                          colors,
                          createChartElement,
                          option: /** @type {Accessor<ChartOption>} */ (
                            chartOption
                          ),
                          signals,
                          webSockets,
                          resources,
                          brk,
                        }),
                      ),
                    ),
                );
              }
              firstTimeLoadingChart = false;

              break;
            }
            case "table": {
              element = tableElement;

              if (firstTimeLoadingTable) {
                import("./panes/table.js").then(({ init }) =>
                  signals.runWithOwner(owner, () =>
                    init({
                      signals,
                      resources,
                      option,
                      brk,
                    }),
                  ),
                );
              }
              firstTimeLoadingTable = false;

              break;
            }
            case "simulation": {
              element = simulationElement;

              simOption.set(option);

              if (firstTimeLoadingSimulation) {
                const chart = modules.chart();
                import("./panes/simulation.js").then(({ init }) =>
                  chart.then(({ createChartElement }) =>
                    signals.runWithOwner(owner, () =>
                      init({
                        colors,
                        createChartElement,
                        signals,
                        resources,
                      }),
                    ),
                  ),
                );
              }
              firstTimeLoadingSimulation = false;

              break;
            }
            case "url": {
              return;
            }
          }

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

      function createMobileSwitchEffect() {
        let firstRun = true;
        signals.createEffect(options.selected, () => {
          if (!firstRun && !isHidden(asideLabelElement)) {
            asideLabelElement.click();
          }
          firstRun = false;
        });
      }
      createMobileSwitchEffect();

      onFirstIntersection(asideElement, () =>
        signals.runWithOwner(owner, initSelectedFrame),
      );
    }
    initSelected();

    onFirstIntersection(navElement, async () => {
      options.parent.set(navElement);

      const option = options.selected();
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

      let i = 0;
      while (path.length > 1) {
        const name = path.shift();
        if (!name) throw "Unreachable";
        /** @type {HTMLDetailsElement[]} */
        let detailsList = [];
        while (!detailsList.length) {
          detailsList = Array.from(
            ul.querySelectorAll(":scope > li > details"),
          );
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
        .find(
          (a) => a.getAttribute("href") == window.document.location.pathname,
        )
        ?.scrollIntoView({
          behavior: "instant",
          block: "center",
        });
    });

    onFirstIntersection(searchElement, () => {
      console.log("search: init");

      const haystack = options.list.map((option) => option.title);

      const RESULTS_PER_PAGE = 100;

      modules.ufuzzy().then((ufuzzy) => {
        /**
         * @param {uFuzzy.SearchResult} searchResult
         * @param {number} pageIndex
         */
        function computeResultPage(searchResult, pageIndex) {
          /** @type {{ option: Option, title: string }[]} */
          let list = [];

          let [indexes, info, order] = searchResult || [null, null, null];

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

        /** @type {VoidFunction | undefined} */
        let dispose;

        function inputEvent() {
          signals.createRoot((_dispose) => {
            const needle = /** @type {string} */ (searchInput.value);

            dispose?.();

            dispose = _dispose;

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
                frame: "search",
                name: title,
                qrcode,
              });

              if (element) {
                li.append(element);
              }
            });
          });
        }

        if (searchInput.value) {
          inputEvent();
        }

        searchInput.addEventListener("input", inputEvent);
      });
    });

    function initShare() {
      const shareDiv = getElementById("share-div");
      const shareContentDiv = getElementById("share-content-div");

      shareDiv.addEventListener("click", () => {
        qrcode.set(null);
      });

      shareContentDiv.addEventListener("click", (event) => {
        event.stopPropagation();
        event.preventDefault();
      });

      modules.leanQr().then(({ generate }) =>
        signals.runWithOwner(owner, () => {
          const imgQrcode = /** @type {HTMLImageElement} */ (
            getElementById("share-img")
          );

          const anchor = /** @type {HTMLAnchorElement} */ (
            getElementById("share-anchor")
          );

          signals.createEffect(qrcode, (qrcode) => {
            if (!qrcode) {
              shareDiv.hidden = true;
              return;
            }

            const href = qrcode;
            anchor.href = href;
            anchor.innerText =
              (href.startsWith("http")
                ? href.split("//").at(-1)
                : href.split(":").at(-1)) || "";

            imgQrcode.src =
              generate(/** @type {any} */ (href))?.toDataURL({
                // @ts-ignore
                padX: 0,
                padY: 0,
              }) || "";

            shareDiv.hidden = false;
          });
        }),
      );
    }
    initShare();

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
  }),
);
