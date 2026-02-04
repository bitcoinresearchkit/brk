import { localhost } from "../utils/env.js";
import { serdeChartableIndex } from "../utils/serde.js";

/** @type {Map<AnyMetricPattern, string[]> | null} */
export const unused = localhost ? new Map() : null;

/**
 * Check if a metric pattern has at least one chartable index
 * @param {AnyMetricPattern} node
 * @returns {boolean}
 */
function hasChartableIndex(node) {
  const indexes = node.indexes();
  return indexes.some((idx) => serdeChartableIndex.serialize(idx) !== null);
}

/**
 * @param {TreeNode | null | undefined} node
 * @param {Map<AnyMetricPattern, string[]>} map
 * @param {string[]} path
 */
function walk(node, map, path) {
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
        //   kn === "time" ||
        //   kn === "height" ||
        kn === "constants" ||
        kn === "blockhash" ||
        kn === "date" ||
        //   kn === "oracle" ||
        kn === "split" ||
        //   kn === "ohlc" ||
        kn === "outpoint" ||
        kn === "positions" ||
        //   kn === "outputtype" ||
        kn === "heighttopool" ||
        kn === "txid" ||
        kn.startsWith("timestamp") ||
        kn.startsWith("satdays") ||
        kn.startsWith("satblocks") ||
        //   kn.endsWith("state") ||
        //   kn.endsWith("cents") ||
        kn.endsWith("index") ||
        kn.endsWith("indexes")
        //   kn.endsWith("raw") ||
        //   kn.endsWith("bytes") ||
        //   (kn.startsWith("_") && kn.endsWith("start"))
      )
        continue;
      walk(/** @type {TreeNode | null | undefined} */ (value), map, [
        ...path,
        key,
      ]);
    }
  }
}

/**
 * Collect all AnyMetricPatterns from tree
 * @param {TreeNode} tree
 */
export function collect(tree) {
  if (unused) walk(tree, unused, []);
}

/**
 * Mark a metric as used
 * @param {AnyMetricPattern} metric
 */
export function markUsed(metric) {
  unused?.delete(metric);
}

/** Log unused metrics to console */
export function logUnused() {
  if (!unused?.size) return;

  /** @type {Record<string, any>} */
  const tree = {};

  for (const path of unused.values()) {
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

  console.log("Unused metrics:", { count: unused.size, tree });
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
      // Price patterns in top pane have dollars/sats sub-metrics
      const metric = /** @type {any} */ (s.metric);
      if (isTop && metric?.dollars && metric?.sats) {
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
    // Group with children
    if ("tree" in node && node.tree) {
      return {
        name: node.name,
        children: node.tree.map(processNode),
      };
    }
    // Chart option
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
    // URL option
    if ("url" in node) {
      return { name: node.name, url: true };
    }
    // Other options (explorer, table, simulation)
    return { name: node.name };
  }

  return options.map(processNode);
}

/**
 * Log the options tree structure to console (localhost only)
 * @param {PartialOptionsTree} options
 */
export function logTreeStructure(options) {
  if (!localhost) return;
  const structure = extractTreeStructure(options);
  console.log("Options tree structure:", JSON.stringify(structure, null, 2));
}
