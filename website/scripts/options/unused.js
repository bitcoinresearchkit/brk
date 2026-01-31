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
        kn === "mvrv" ||
        kn === "time" ||
        kn === "height" ||
        kn === "constants" ||
        kn === "blockhash" ||
        kn === "oracle" ||
        kn === "split" ||
        kn === "ohlc" ||
        kn === "outpoint" ||
        kn === "positions" ||
        kn === "outputtype" ||
        kn === "heighttopool" ||
        kn === "txid" ||
        kn.startsWith("satblocks") ||
        kn.startsWith("satdays") ||
        kn.endsWith("state") ||
        kn.endsWith("cents") ||
        kn.endsWith("index") ||
        kn.endsWith("indexes") ||
        kn.endsWith("raw") ||
        kn.endsWith("bytes") ||
        (kn.startsWith("_") && kn.endsWith("start"))
      )
        continue;
      // if (
      // kn === "mvrv" ||
      // kn.endsWith("index") ||
      // kn.endsWith("indexes") ||
      // kn.endsWith("start") ||
      // kn.endsWith("hash") ||
      // kn.endsWith("data") ||
      // kn.endsWith("constants")
      // )
      //   return;
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
