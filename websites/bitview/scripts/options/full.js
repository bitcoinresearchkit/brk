import { createPartialOptions } from "./partial.js";
import {
  createButtonElement,
  createAnchorElement,
  insertElementAtIndex,
} from "../utils/dom.js";
import { pushHistory, resetParams } from "../utils/url.js";
import { readStored, writeToStorage } from "../utils/storage.js";
import { stringToId } from "../utils/format.js";
import { collect, markUsed, logUnused } from "./unused.js";

/**
 * @param {Object} args
 * @param {Colors} args.colors
 * @param {Signals} args.signals
 * @param {BrkClient} args.brk
 * @param {Signal<string | null>} args.qrcode
 */
export function initOptions({ colors, signals, brk, qrcode }) {
  collect(brk.tree);

  const LS_SELECTED_KEY = `selected_path`;

  const urlPath_ = window.document.location.pathname
    .split("/")
    .filter((v) => v);
  const urlPath = urlPath_.length ? urlPath_ : undefined;
  const savedPath = /** @type {string[]} */ (
    JSON.parse(readStored(LS_SELECTED_KEY) || "[]") || []
  ).filter((v) => v);
  console.log(savedPath);

  /** @type {Signal<Option>} */
  const selected = signals.createSignal(/** @type {any} */ (undefined));

  const partialOptions = createPartialOptions({
    colors,
    brk,
  });

  /** @type {Option[]} */
  const list = [];

  const parent = signals.createSignal(/** @type {HTMLElement | null} */ (null));

  /**
   * @param {AnyFetchedSeriesBlueprint[]} [arr]
   */
  function arrayToMap(arr = []) {
    /** @type {Map<Unit, AnyFetchedSeriesBlueprint[]>} */
    const map = new Map();
    for (const blueprint of arr || []) {
      if (!blueprint.metric) {
        throw new Error(
          `Blueprint missing metric: ${JSON.stringify(blueprint)}`,
        );
      }
      if (!blueprint.unit) {
        throw new Error(
          `Blueprint missing unit: ${blueprint.title}`,
        );
      }
      markUsed(blueprint.metric);
      const unit = blueprint.unit;
      if (!map.has(unit)) {
        map.set(unit, []);
      }
      map.get(unit)?.push(blueprint);
    }
    return map;
  }

  /**
   * @param {Option} option
   */
  function selectOption(option) {
    pushHistory(option.path);
    resetParams(option);
    writeToStorage(LS_SELECTED_KEY, JSON.stringify(option.path));
    selected.set(option);
  }

  /**
   * @param {Object} args
   * @param {Option} args.option
   * @param {Signal<string | null>} args.qrcode
   * @param {string} [args.name]
   */
  function createOptionElement({ option, name, qrcode }) {
    const title = option.title;
    if (option.kind === "url") {
      const href = option.url();

      if (option.qrcode) {
        return createButtonElement({
          inside: option.name,
          title,
          onClick: () => {
            qrcode.set(option.url);
          },
        });
      } else {
        return createAnchorElement({
          href,
          blank: true,
          text: option.name,
          title,
        });
      }
    } else {
      return createAnchorElement({
        href: `/${option.path.join("/")}`,
        title,
        text: name || option.name,
        onClick: () => {
          selectOption(option);
        },
      });
    }
  }

  /** @type {Option | undefined} */
  let savedOption;

  /**
   * @param {PartialOptionsTree} partialTree
   * @param {Accessor<HTMLElement | null>} parent
   * @param {string[] | undefined} parentPath
   * @returns {Accessor<number>}
   */
  function recursiveProcessPartialTree(
    partialTree,
    parent,
    parentPath = [],
    depth = 0,
  ) {
    /** @type {Accessor<number>[]} */
    const listForSum = [];

    const ul = signals.createMemo(
      // @ts_ignore
      (_previous) => {
        const previous = /** @type {HTMLUListElement | null} */ (_previous);
        previous?.remove();

        const _parent = parent();
        if (_parent) {
          if ("open" in _parent && !_parent.open) {
            throw "Set accesor to null instead";
          }

          const ul = window.document.createElement("ul");
          _parent.append(ul);
          return ul;
        } else {
          return null;
        }
      },
      null,
    );

    partialTree.forEach((anyPartial, partialIndex) => {
      const renderLi = signals.createSignal(true);

      const li = signals.createMemo((_previous) => {
        const previous = _previous;
        previous?.remove();

        const _ul = ul();

        if (renderLi() && _ul) {
          const li = window.document.createElement("li");
          insertElementAtIndex(_ul, li, partialIndex);
          return li;
        } else {
          return null;
        }
      }, /** @type {HTMLLIElement | null} */ (null));

      if ("tree" in anyPartial) {
        /** @type {Omit<OptionsGroup, keyof PartialOptionsGroup>} */
        const groupAddons = {};

        Object.assign(anyPartial, groupAddons);

        const passedDetails = signals.createSignal(
          /** @type {HTMLDivElement | HTMLDetailsElement | null} */ (null),
        );

        const serName = stringToId(anyPartial.name);
        const path = [...parentPath, serName];
        const childOptionsCount = recursiveProcessPartialTree(
          anyPartial.tree,
          passedDetails,
          path,
          depth + 1,
        );

        listForSum.push(childOptionsCount);

        signals.createEffect(li, (li) => {
          if (!li) {
            passedDetails.set(null);
            return;
          }

          signals.createEffect(selected, (selected) => {
            if (
              path.length <= selected.path.length &&
              path.every((v, i) => selected.path.at(i) === v)
            ) {
              li.dataset.highlight = "";
            } else {
              delete li.dataset.highlight;
            }
          });

          const details = window.document.createElement("details");
          details.dataset.name = serName;
          li.appendChild(details);

          const summary = window.document.createElement("summary");
          details.append(summary);
          summary.append(anyPartial.name);

          const supCount = window.document.createElement("sup");
          summary.append(supCount);

          signals.createEffect(childOptionsCount, (childOptionsCount) => {
            supCount.innerHTML = childOptionsCount.toLocaleString("en-us");
          });

          details.addEventListener("toggle", () => {
            const open = details.open;

            if (open) {
              passedDetails.set(details);
            } else {
              passedDetails.set(null);
            }
          });
        });

        function createRenderLiEffect() {
          signals.createEffect(childOptionsCount, (count) => {
            renderLi.set(!!count);
          });
        }
        createRenderLiEffect();
      } else {
        const option = /** @type {Option} */ (anyPartial);

        const name = option.name;
        const path = [...parentPath, stringToId(option.name)];

        if ("kind" in anyPartial && anyPartial.kind === "explorer") {
          Object.assign(
            option,
            /** @satisfies {ExplorerOption} */ ({
              kind: anyPartial.kind,
              path,
              name,
              title: option.title,
            }),
          );
        } else if ("kind" in anyPartial && anyPartial.kind === "table") {
          Object.assign(
            option,
            /** @satisfies {TableOption} */ ({
              kind: anyPartial.kind,
              path,
              name,
              title: option.title,
            }),
          );
        } else if ("kind" in anyPartial && anyPartial.kind === "simulation") {
          Object.assign(
            option,
            /** @satisfies {SimulationOption} */ ({
              kind: anyPartial.kind,
              path,
              name,
              title: anyPartial.title,
            }),
          );
        } else if ("url" in anyPartial) {
          Object.assign(
            option,
            /** @satisfies {UrlOption} */ ({
              kind: "url",
              path,
              name,
              title: name,
              qrcode: !!anyPartial.qrcode,
              url: anyPartial.url,
            }),
          );
        } else {
          const title = option.title || option.name;
          Object.assign(
            option,
            /** @satisfies {ChartOption} */ ({
              kind: "chart",
              name,
              title,
              path,
              top: arrayToMap(anyPartial.top),
              bottom: arrayToMap(anyPartial.bottom),
            }),
          );
        }

        list.push(option);

        if (urlPath) {
          const sameAsURLPath =
            urlPath.length === path.length &&
            urlPath.every((val, i) => val === path[i]);
          if (sameAsURLPath) {
            selected.set(option);
          }
        } else if (savedPath) {
          const sameAsSavedPath =
            savedPath.length === path.length &&
            savedPath.every((val, i) => val === path[i]);
          if (sameAsSavedPath) {
            savedOption = option;
          }
        }

        signals.createEffect(li, (li) => {
          if (!li) {
            return;
          }

          signals.createEffect(selected, (selected) => {
            if (selected === option) {
              li.dataset.highlight = "";
            } else {
              delete li.dataset.highlight;
            }
          });

          const element = createOptionElement({
            option,
            qrcode,
          });

          li.append(element);
        });

        listForSum.push(() => 1);
      }
    });

    return signals.createMemo(() =>
      listForSum.reduce((acc, s) => acc + s(), 0),
    );
  }
  recursiveProcessPartialTree(partialOptions, parent);
  logUnused();

  if (!selected()) {
    const option =
      savedOption || list.find((option) => option.kind === "chart");
    if (option) {
      selected.set(option);
    }
  }

  return {
    selected,
    list,
    tree: /** @type {OptionsTree} */ (partialOptions),
    parent,
    createOptionElement,
    selectOption,
  };
}
/** @typedef {ReturnType<typeof initOptions>} Options */
