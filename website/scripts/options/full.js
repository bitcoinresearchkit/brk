import { createPartialOptions } from "./partial.js";
import { createButtonElement, createAnchorElement } from "../utils/dom.js";
import { pushHistory, resetParams } from "../utils/url.js";
import { readStored, writeToStorage } from "../utils/storage.js";
import { stringToId } from "../utils/format.js";
import { collect, markUsed, logUnused } from "./unused.js";
import { setQr } from "../panes/share.js";
import { getConstant } from "./constants.js";
import { colors } from "../chart/colors.js";

/**
 * @param {BrkClient} brk
 */
export function initOptions(brk) {
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

  const partialOptions = createPartialOptions({
    brk,
  });

  /** @type {Option[]} */
  const list = [];

  /** @type {Map<string, HTMLLIElement>} */
  const liByPath = new Map();

  /** @type {Set<(option: Option) => void>} */
  const selectedListeners = new Set();

  /**
   * @param {Option | undefined} sel
   */
  function updateHighlight(sel) {
    if (!sel) return;
    liByPath.forEach((li) => {
      delete li.dataset.highlight;
    });
    for (let i = 1; i <= sel.path.length; i++) {
      const pathKey = sel.path.slice(0, i).join("/");
      const li = liByPath.get(pathKey);
      if (li) li.dataset.highlight = "";
    }
  }

  const selected = {
    /** @type {Option | undefined} */
    value: undefined,
    /** @param {Option} v */
    set(v) {
      this.value = v;
      updateHighlight(v);
      selectedListeners.forEach((cb) => cb(v));
    },
    /** @param {(option: Option) => void} cb */
    onChange(cb) {
      selectedListeners.add(cb);
      if (this.value) cb(this.value);
      return () => selectedListeners.delete(cb);
    },
  };

  /**
   * @param {string[]} nodePath
   */
  function isOnSelectedPath(nodePath) {
    const selectedPath = selected.value?.path;
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
    /** @type {Map<Unit, Set<number>>} */
    const priceLines = new Map();

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

      // Track baseline base values for auto price lines
      if (blueprint.type === "Baseline") {
        const baseValue = blueprint.options?.baseValue?.price ?? 0;
        if (!priceLines.has(unit)) priceLines.set(unit, new Set());
        priceLines.get(unit)?.add(baseValue);
      }

      // Remove from set if manual price line already exists
      if (blueprint.type === "Line") {
        const path = Object.values(blueprint.metric.by)[0]?.path ?? "";
        if (path.includes("constant_")) {
          priceLines.get(unit)?.delete(parseFloat(blueprint.title));
        }
      }
    }

    // Add price lines at end for remaining values
    for (const [unit, values] of priceLines) {
      for (const baseValue of values) {
        const metric = getConstant(brk.metrics.constants, baseValue);
        markUsed(metric);
        map.get(unit)?.push({
          metric,
          title: `${baseValue}`,
          color: colors.gray,
          unit,
          options: { lineStyle: 4, lastValueVisible: false, crosshairMarkerVisible: false },
        });
      }
    }

    return map;
  }

  /**
   * @param {Option} option
   */
  function selectOption(option) {
    if (selected.value === option) return;
    pushHistory(option.path);
    resetParams(option);
    writeToStorage(LS_SELECTED_KEY, JSON.stringify(option.path));
    selected.set(option);
  }

  /**
   * @param {Object} args
   * @param {Option} args.option
   * @param {string} [args.name]
   */
  function createOptionElement({ option, name }) {
    const title = option.title;
    if (option.kind === "link") {
      const href = option.url();

      if (option.qrcode) {
        return createButtonElement({
          inside: option.name,
          title,
          onClick: () => {
            setQr(option.url());
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
              kind: "link",
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
        });
        li.append(element);
      }
    }
  }

  /** @type {HTMLElement | null} */
  let parentEl = null;

  /**
   * @param {HTMLElement} el
   */
  function setParent(el) {
    if (parentEl) return;
    parentEl = el;
    buildTreeDOM(processedTree, el);
  }

  if (!selected.value) {
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
    setParent,
    createOptionElement,
    selectOption,
  };
}
/** @typedef {ReturnType<typeof initOptions>} Options */
