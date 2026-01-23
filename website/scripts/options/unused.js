import { localhost } from "../utils/env.js";

/** @type {Map<AnyMetricPattern, string[]> | null} */
export const unused = localhost ? new Map() : null;

/**
 * @param {TreeNode | null | undefined} node
 * @param {Map<AnyMetricPattern, string[]>} map
 * @param {string[]} path
 */
function walk(node, map, path) {
  if (node && "by" in node) {
    map.set(/** @type {AnyMetricPattern} */ (node), path);
  } else if (node && typeof node === "object") {
    for (const [key, value] of Object.entries(node)) {
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
