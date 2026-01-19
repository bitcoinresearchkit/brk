import { createPartialOptions } from "./partial.js";
import {
  createButtonElement,
  createAnchorElement,
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
  collect(brk.metrics);

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

  /** @type {Map<string, HTMLLIElement>} */
  const liByPath = new Map();

  /**
   * @param {string[]} nodePath
   */
  function isOnSelectedPath(nodePath) {
    const selectedPath = selected()?.path;
    return (
      selectedPath &&
      nodePath.length <= selectedPath.length &&
      nodePath.every((v, i) => v === selectedPath[i])
    );
  }

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
        throw new Error(`Blueprint missing unit: ${blueprint.title}`);
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

  // ============================================
  // Phase 1: Process partial tree (non-reactive)
  // Transforms options, computes counts, populates list
  // ============================================

  /**
   * @typedef {{ type: "group"; name: string; serName: string; path: string[]; count: number; children: ProcessedNode[] }} ProcessedGroup
   * @typedef {{ type: "option"; option: Option; path: string[] }} ProcessedOption
   * @typedef {ProcessedGroup | ProcessedOption} ProcessedNode
   */

  /**
   * @param {PartialOptionsTree} partialTree
   * @param {string[]} parentPath
   * @returns {ProcessedNode[]}
   */
  function processPartialTree(partialTree, parentPath = []) {
    /** @type {ProcessedNode[]} */
    const nodes = [];

    for (const anyPartial of partialTree) {
      if ("tree" in anyPartial) {
        const serName = stringToId(anyPartial.name);
        const path = [...parentPath, serName];
        const children = processPartialTree(anyPartial.tree, path);

        // Compute count from children
        const count = children.reduce(
          (sum, child) => sum + (child.type === "group" ? child.count : 1),
          0,
        );

        // Skip groups with no children
        if (count === 0) continue;

        nodes.push({
          type: "group",
          name: anyPartial.name,
          serName,
          path,
          count,
          children,
        });
      } else {
        const option = /** @type {Option} */ (anyPartial);
        const name = option.name;
        const path = [...parentPath, stringToId(option.name)];

        // Transform partial to full option
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

        // Check if this matches URL or saved path
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

        nodes.push({
          type: "option",
          option,
          path,
        });
      }
    }

    return nodes;
  }

  const processedTree = processPartialTree(partialOptions);
  logUnused();

  // ============================================
  // Phase 2: Build DOM lazily (imperative)
  // Uses native toggle events for lazy loading
  // ============================================

  /**
   * @param {ProcessedNode[]} nodes
   * @param {HTMLElement} parentEl
   */
  function buildTreeDOM(nodes, parentEl) {
    const ul = window.document.createElement("ul");
    parentEl.append(ul);

    for (const node of nodes) {
      const li = window.document.createElement("li");
      ul.append(li);

      const pathKey = node.path.join("/");
      liByPath.set(pathKey, li);

      if (isOnSelectedPath(node.path)) {
        li.dataset.highlight = "";
      }

      if (node.type === "group") {
        const details = window.document.createElement("details");
        details.dataset.name = node.serName;
        li.appendChild(details);

        const summary = window.document.createElement("summary");
        details.append(summary);
        summary.append(node.name);

        const supCount = window.document.createElement("sup");
        supCount.innerHTML = node.count.toLocaleString("en-us");
        summary.append(supCount);

        let built = false;
        details.addEventListener("toggle", () => {
          if (details.open && !built) {
            built = true;
            buildTreeDOM(node.children, details);
          }
        });
      } else {
        const element = createOptionElement({
          option: node.option,
          qrcode,
        });
        li.append(element);
      }
    }
  }

  // Single effect to kick off DOM building when parent is set
  signals.createEffect(
    () => parent(),
    (_parent) => {
      if (!_parent) return;
      buildTreeDOM(processedTree, _parent);
    },
  );

  // Single effect for highlighting on selection change
  signals.createEffect(
    () => selected(),
    (selected) => {
      if (!selected) return;

      // Clear all existing highlights
      liByPath.forEach((li) => {
        delete li.dataset.highlight;
      });

      // Highlight selected option and parent groups
      for (let i = 1; i <= selected.path.length; i++) {
        const pathKey = selected.path.slice(0, i).join("/");
        const li = liByPath.get(pathKey);
        if (li) li.dataset.highlight = "";
      }
    },
  );

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
