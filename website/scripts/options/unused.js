import { localhost } from "../utils/env.js";
import { INDEX_LABEL } from "../utils/serde.js";

/**
 * Check if a metric pattern has at least one chartable index
 * @param {AnyMetricPattern} node
 * @returns {boolean}
 */
function hasChartableIndex(node) {
  const indexes = node.indexes();
  return indexes.some((idx) => idx in INDEX_LABEL);
}

/**
 * Walk a metrics tree and collect all chartable metric patterns
 * @param {TreeNode | null | undefined} node
 * @param {Map<AnyMetricPattern, string[]>} map
 * @param {string[]} path
 */
function walkMetrics(node, map, path) {
  if (node && "by" in node) {
    const metricNode = /** @type {AnyMetricPattern} */ (node);
    if (!hasChartableIndex(metricNode)) return;
    map.set(metricNode, path);
  } else if (node && typeof node === "object") {
    for (const [key, value] of Object.entries(node)) {
      const kn = key.toLowerCase();
      if (
        key.endsWith("Raw") ||
        key.endsWith("Cents") ||
        key.endsWith("State") ||
        key.endsWith("Start") ||
        kn === "mvrv" ||
        kn === "constants" ||
        kn === "blockhash" ||
        kn === "date" ||
        kn === "split" ||
        kn === "outpoint" ||
        kn === "positions" ||
        kn === "heighttopool" ||
        kn === "txid" ||
        kn.startsWith("timestamp") ||
        kn.startsWith("satdays") ||
        kn.startsWith("satblocks") ||
        kn.endsWith("index") ||
        kn.endsWith("indexes")
      )
        continue;
      walkMetrics(/** @type {TreeNode | null | undefined} */ (value), map, [
        ...path,
        key,
      ]);
    }
  }
}

/**
 * Walk partial options tree and delete referenced metrics from the map
 * @param {PartialOptionsTree} options
 * @param {Map<AnyMetricPattern, string[]>} map
 */
function walkOptions(options, map) {
  for (const node of options) {
    if ("tree" in node && node.tree) {
      walkOptions(node.tree, map);
    } else if ("top" in node || "bottom" in node) {
      const chartNode = /** @type {PartialChartOption} */ (node);
      markUsedBlueprints(map, chartNode.top);
      markUsedBlueprints(map, chartNode.bottom);
    }
  }
}

/**
 * @param {Map<AnyMetricPattern, string[]>} map
 * @param {(AnyFetchedSeriesBlueprint | FetchedPriceSeriesBlueprint)[]} [arr]
 */
function markUsedBlueprints(map, arr) {
  if (!arr) return;
  for (let i = 0; i < arr.length; i++) {
    const metric = arr[i].metric;
    if (!metric) continue;
    const maybePriceMetric =
      /** @type {{ usd?: AnyMetricPattern, sats?: AnyMetricPattern }} */ (
        /** @type {unknown} */ (metric)
      );
    if (maybePriceMetric.usd?.by && maybePriceMetric.sats?.by) {
      map.delete(maybePriceMetric.usd);
      map.delete(maybePriceMetric.sats);
    } else {
      map.delete(/** @type {AnyMetricPattern} */ (metric));
    }
  }
}

/**
 * Log unused metrics to console (localhost only)
 * @param {TreeNode} metricsTree
 * @param {PartialOptionsTree} partialOptions
 */
export function logUnused(metricsTree, partialOptions) {
  if (!localhost) return;

  console.log(extractTreeStructure(partialOptions));

  /** @type {Map<AnyMetricPattern, string[]>} */
  const all = new Map();
  walkMetrics(metricsTree, all, []);
  walkOptions(partialOptions, all);

  if (!all.size) return;

  /** @type {Record<string, any>} */
  const tree = {};
  for (const path of all.values()) {
    let current = tree;
    for (let i = 0; i < path.length; i++) {
      const part = path[i];
      if (i === path.length - 1) {
        current[part] = null;
      } else {
        current[part] = current[part] || {};
        current = current[part];
      }
    }
  }

  console.log("Unused metrics:", { count: all.size, tree });
}

/**
 * Extract tree structure from partial options (names + hierarchy, series grouped by unit)
 * @param {PartialOptionsTree} options
 * @returns {object[]}
 */
export function extractTreeStructure(options) {
  /**
   * Group series by unit
   * @param {(AnyFetchedSeriesBlueprint | FetchedPriceSeriesBlueprint)[]} series
   * @param {boolean} isTop
   * @returns {Record<string, string[]>}
   */
  function groupByUnit(series, isTop) {
    /** @type {Record<string, string[]>} */
    const grouped = {};
    for (const s of series) {
      const metric = /** @type {any} */ (s.metric);
      if (isTop && metric?.usd && metric?.sats) {
        const title = s.title || s.key || "unnamed";
        (grouped["USD"] ??= []).push(title);
        (grouped["sats"] ??= []).push(title);
      } else {
        const unit = /** @type {AnyFetchedSeriesBlueprint} */ (s).unit;
        const unitName = unit?.name || "unknown";
        const title = s.title || s.key || "unnamed";
        (grouped[unitName] ??= []).push(title);
      }
    }
    return grouped;
  }

  /**
   * @param {AnyPartialOption | PartialOptionsGroup} node
   * @returns {object}
   */
  function processNode(node) {
    if ("tree" in node && node.tree) {
      return {
        name: node.name,
        children: node.tree.map(processNode),
      };
    }
    if ("top" in node || "bottom" in node) {
      const chartNode = /** @type {PartialChartOption} */ (node);
      const top = chartNode.top ? groupByUnit(chartNode.top, true) : undefined;
      const bottom = chartNode.bottom
        ? groupByUnit(chartNode.bottom, false)
        : undefined;
      return {
        name: node.name,
        title: chartNode.title,
        ...(top && Object.keys(top).length > 0 ? { top } : {}),
        ...(bottom && Object.keys(bottom).length > 0 ? { bottom } : {}),
      };
    }
    if ("url" in node) {
      return { name: node.name, url: true };
    }
    return { name: node.name };
  }

  return options.map(processNode);
}
