import { createPartialOptions } from "./partial.js";
import { createButtonElement, createAnchorElement } from "../utils/dom.js";
import { pushHistory, resetParams } from "../utils/url.js";
import { readStored, writeToStorage } from "../utils/storage.js";
import { stringToId } from "../utils/format.js";
import { collect, markUsed, logUnused, extractTreeStructure } from "./unused.js";
import { localhost } from "../utils/env.js";
import { setQr } from "../panes/share.js";
import { getConstant } from "./constants.js";
import { colors } from "../chart/colors.js";
import { Unit } from "../utils/units.js";

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

  const partialOptions = createPartialOptions({
    brk,
  });

  // Log tree structure for analysis (localhost only)
  if (localhost) {
    console.log(extractTreeStructure(partialOptions));
  }

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
   * @param {(AnyFetchedSeriesBlueprint | FetchedPriceSeriesBlueprint)[]} [arr]
   */
  function arrayToMap(arr) {
    /** @type {Map<Unit, AnyFetchedSeriesBlueprint[]>} */
    const map = new Map();
    /** @type {Map<Unit, Set<number>>} */
    const priceLines = new Map();

    if (!arr) return map;

    // Cache arrays for common units outside loop
    /** @type {AnyFetchedSeriesBlueprint[] | undefined} */
    let usdArr;
    /** @type {AnyFetchedSeriesBlueprint[] | undefined} */
    let satsArr;

    for (let i = 0; i < arr.length; i++) {
      const blueprint = arr[i];

      // Check for undefined metric
      if (!blueprint.metric) {
        throw new Error(`Blueprint has undefined metric: ${blueprint.title}`);
      }

      // Check for price pattern blueprint (has dollars/sats sub-metrics)
      // Use unknown cast for safe property access check
      const maybePriceMetric = /** @type {{ dollars?: AnyMetricPattern, sats?: AnyMetricPattern }} */ (
        /** @type {unknown} */ (blueprint.metric)
      );
      if (maybePriceMetric.dollars?.by && maybePriceMetric.sats?.by) {
        const { dollars, sats } = maybePriceMetric;
        markUsed(dollars);
        if (!usdArr) map.set(Unit.usd, (usdArr = []));
        usdArr.push({ ...blueprint, metric: dollars, unit: Unit.usd });

        markUsed(sats);
        if (!satsArr) map.set(Unit.sats, (satsArr = []));
        satsArr.push({ ...blueprint, metric: sats, unit: Unit.sats });
        continue;
      }

      // After continue, we know this is a regular metric blueprint
      const regularBlueprint = /** @type {AnyFetchedSeriesBlueprint} */ (blueprint);
      const metric = regularBlueprint.metric;
      const unit = regularBlueprint.unit;
      if (!unit) continue;

      markUsed(metric);
      let unitArr = map.get(unit);
      if (!unitArr) map.set(unit, (unitArr = []));
      unitArr.push(regularBlueprint);

      // Track baseline base values for auto price lines
      const type = regularBlueprint.type;
      if (type === "Baseline") {
        let priceSet = priceLines.get(unit);
        if (!priceSet) priceLines.set(unit, (priceSet = new Set()));
        priceSet.add(regularBlueprint.options?.baseValue?.price ?? 0);
      } else if (!type || type === "Line") {
        // Check if manual price line - avoid Object.values() array allocation
        const by = metric.by;
        for (const k in by) {
          if (by[/** @type {Index} */ (k)]?.path?.includes("constant_")) {
            priceLines.get(unit)?.delete(parseFloat(regularBlueprint.title));
          }
          break;
        }
      }
    }

    // Add price lines at end for remaining values
    for (const [unit, values] of priceLines) {
      const arr = map.get(unit);
      if (!arr) continue;
      for (const baseValue of values) {
        const metric = getConstant(brk.metrics.constants, baseValue);
        markUsed(metric);
        arr.push({
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

  // Pre-compute path strings for faster comparison
  const urlPathStr = urlPath?.join("/");
  const savedPathStr = savedPath?.join("/");

  /**
   * @param {PartialOptionsTree} partialTree
   * @param {string[]} parentPath
   * @param {string} parentPathStr
   * @returns {{ nodes: ProcessedNode[], count: number }}
   */
  function processPartialTree(partialTree, parentPath = [], parentPathStr = "") {
    /** @type {ProcessedNode[]} */
    const nodes = [];
    let totalCount = 0;

    for (let i = 0; i < partialTree.length; i++) {
      const anyPartial = partialTree[i];
      if ("tree" in anyPartial) {
        const serName = stringToId(anyPartial.name);
        const pathStr = parentPathStr ? `${parentPathStr}/${serName}` : serName;
        const path = parentPath.concat(serName);
        const { nodes: children, count } = processPartialTree(anyPartial.tree, path, pathStr);

        // Skip groups with no children
        if (count === 0) continue;

        totalCount += count;
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
        const serName = stringToId(name);
        const pathStr = parentPathStr ? `${parentPathStr}/${serName}` : serName;
        const path = parentPath.concat(serName);

        // Transform partial to full option
        if ("kind" in anyPartial && anyPartial.kind === "explorer") {
          option.kind = anyPartial.kind;
          option.path = path;
          option.name = name;
        } else if ("kind" in anyPartial && anyPartial.kind === "table") {
          option.kind = anyPartial.kind;
          option.path = path;
          option.name = name;
        } else if ("kind" in anyPartial && anyPartial.kind === "simulation") {
          option.kind = anyPartial.kind;
          option.path = path;
          option.name = name;
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
          const title = option.title || name;
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
        totalCount++;

        // Check if this matches URL or saved path (string comparison is faster)
        if (urlPathStr && pathStr === urlPathStr) {
          selected.set(option);
        } else if (savedPathStr && pathStr === savedPathStr) {
          savedOption = option;
        }

        nodes.push({
          type: "option",
          option,
          path,
        });
      }
    }

    return { nodes, count: totalCount };
  }

  const { nodes: processedTree } = processPartialTree(partialOptions);
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
