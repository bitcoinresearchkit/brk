import { createColors } from "./core/colors";
import { createWebSockets } from "./core/ws";
import * as utils from "./core/utils";
import elements from "./core/elements";
import env from "./core/env";
import packages from "./lazy";
import { onFirstIntersection, getElementById } from "./core/dom";

function initFrameSelectors() {
  const children = Array.from(elements.selectors.children);

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

  elements.asideLabel.click();

  // When going from mobile view to desktop view, if selected frame was open, go to the nav frame
  new IntersectionObserver((entries) => {
    for (let i = 0; i < entries.length; i++) {
      if (
        !entries[i].isIntersecting &&
        entries[i].target === elements.asideLabel &&
        focusedFrame == elements.aside
      ) {
        elements.navLabel.click();
      }
    }
  }).observe(elements.asideLabel);

  function setAsideParent() {
    const { clientWidth } = window.document.documentElement;
    const { aside, body, main } = elements;
    const MEDIUM_WIDTH = 768;
    if (clientWidth >= MEDIUM_WIDTH) {
      aside.parentElement !== body && body.append(aside);
    } else {
      aside.parentElement !== main && main.append(aside);
    }
  }

  setAsideParent();

  window.addEventListener("resize", setAsideParent);
}
initFrameSelectors();

Promise.all([
  packages.signals(),
  packages.vecs(),
  packages.pools(),
  packages.options(),
]).then(
  ([
    signals,
    { createVecIdToIndexes, VERSION },
    { createPools },
    { initOptions },
  ]) =>
    signals.createRoot(() => {
      const owner = signals.getOwner();

      console.log(`VERSION = ${VERSION}`);

      const vecIdToIndexes = createVecIdToIndexes();

      if (env.localhost) {
        Object.keys(vecIdToIndexes).forEach((id) => {
          utils.vecidToUnit(/** @type {VecId} */ (id));
        });
      }

      function initDark() {
        const preferredColorSchemeMatchMedia = window.matchMedia(
          "(prefers-color-scheme: dark)",
        );
        const dark = signals.createSignal(
          preferredColorSchemeMatchMedia.matches,
        );
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

      function createLastHeightResource() {
        const lastHeight = signals.createSignal(0);
        function fetchLastHeight() {
          utils.api.fetchLast(
            (h) => {
              lastHeight.set(h);
            },
            /** @satisfies {Height} */ (5),
            "height",
          );
        }
        fetchLastHeight();
        setInterval(fetchLastHeight, 10_000);
        return lastHeight;
      }
      const lastHeight = createLastHeightResource();

      const webSockets = createWebSockets(signals);

      const vecsResources = createVecsResources(
        signals,
        utils,
        env,
        vecIdToIndexes,
      );

      const pools = createPools();

      const colors = createColors(dark);

      const options = initOptions({
        colors,
        env,
        signals,
        utils,
        qrcode,
        vecIdToIndexes,
        pools,
      });

      window.addEventListener("popstate", (event) => {
        const path = window.document.location.pathname
          .split("/")
          .filter((v) => v);
        let folder = options.tree;

        while (path.length) {
          const id = path.shift();
          const res = folder.find((v) => id === utils.stringToId(v.name));
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
                element = elements.explorer;

                if (firstTimeLoadingExplorer) {
                  const chartPkg = packages.chart();
                  import("./panes/explorer.js").then(({ init }) =>
                    chartPkg.then(({ createChartElement }) =>
                      signals.runWithOwner(owner, () =>
                        init({
                          colors,
                          elements,
                          createChartElement,
                          option: /** @type {Accessor<ChartOption>} */ (
                            chartOption
                          ),
                          signals,
                          utils,
                          webSockets,
                          vecsResources,
                          vecIdToIndexes,
                        }),
                      ),
                    ),
                  );
                }
                firstTimeLoadingExplorer = false;

                break;
              }
              case "chart": {
                element = elements.charts;

                chartOption.set(option);

                if (firstTimeLoadingChart) {
                  const chartPkg = packages.chart();
                  import("./panes/chart.js").then(
                    ({ init: initChartsElement }) =>
                      chartPkg.then(({ createChartElement }) =>
                        signals.runWithOwner(owner, () =>
                          initChartsElement({
                            colors,
                            elements,
                            createChartElement,
                            option: /** @type {Accessor<ChartOption>} */ (
                              chartOption
                            ),
                            env,
                            signals,
                            utils,
                            webSockets,
                            vecsResources,
                            vecIdToIndexes,
                            packages,
                          }),
                        ),
                      ),
                  );
                }
                firstTimeLoadingChart = false;

                break;
              }
              case "table": {
                element = elements.table;

                if (firstTimeLoadingTable) {
                  import("./panes/table.js").then(({ init }) =>
                    signals.runWithOwner(owner, () =>
                      init({
                        elements,
                        signals,
                        utils,
                        vecsResources,
                        option,
                        vecIdToIndexes,
                      }),
                    ),
                  );
                }
                firstTimeLoadingTable = false;

                break;
              }
              case "simulation": {
                element = elements.simulation;

                simOption.set(option);

                if (firstTimeLoadingSimulation) {
                  const chart = packages.chart();
                  import("./panes/simulation.js").then(({ init }) =>
                    chart.then(({ createChartElement }) =>
                      signals.runWithOwner(owner, () =>
                        init({
                          colors,
                          elements,
                          createChartElement,
                          signals,
                          utils,
                          vecsResources,
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
              utils.url.replaceHistory({ pathname: option.path });
            }

            previousElement = element;
          });
        }

        function createMobileSwitchEffect() {
          let firstRun = true;
          signals.createEffect(options.selected, () => {
            if (!firstRun && !utils.dom.isHidden(elements.asideLabel)) {
              elements.asideLabel.click();
            }
            firstRun = false;
          });
        }
        createMobileSwitchEffect();

        onFirstIntersection(elements.aside, () =>
          signals.runWithOwner(owner, initSelectedFrame),
        );
      }
      initSelected();

      onFirstIntersection(elements.nav, async () => {
        options.parent.set(elements.nav);

        const option = options.selected();
        if (!option) throw "Selected should be set by now";
        const path = [...option.path];

        /** @type {HTMLUListElement | null} */
        let ul = /** @type {any} */ (null);
        async function getFirstChild() {
          try {
            ul = /** @type {HTMLUListElement} */ (
              elements.nav.firstElementChild
            );
            await utils.next();
            if (!ul) {
              await getFirstChild();
            }
          } catch (_) {
            await utils.next();
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
              await utils.next();
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
              await utils.next();
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
            await utils.next();
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

      onFirstIntersection(elements.search, () => {
        console.log("search: init");

        const haystack = options.list.map((option) => option.title);

        const RESULTS_PER_PAGE = 100;

        packages.ufuzzy().then((ufuzzy) => {
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
          const fuzzyMultiInsertFuzzier = /** @type {uFuzzy} */ (
            ufuzzy(config)
          );
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
              const needle = /** @type {string} */ (elements.searchInput.value);

              dispose?.();

              dispose = _dispose;

              elements.searchResults.scrollTo({
                top: 0,
              });

              if (!needle) {
                elements.searchResults.innerHTML = "";
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

              elements.searchResults.innerHTML = "";

              const list = computeResultPage(result, 0);

              list.forEach(({ option, title }) => {
                const li = window.document.createElement("li");
                elements.searchResults.appendChild(li);

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

          if (elements.searchInput.value) {
            inputEvent();
          }

          elements.searchInput.addEventListener("input", inputEvent);
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

        packages.leanQr().then(({ generate }) =>
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
              elements.main.style.width = `${width}px`;
              utils.storage.write(barWidthLocalStorageKey, String(width));
            } else {
              elements.main.style.width = elements.style.getPropertyValue(
                "--default-main-width",
              );
              utils.storage.remove(barWidthLocalStorageKey);
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
          startingWidth = elements.main.clientWidth;
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
