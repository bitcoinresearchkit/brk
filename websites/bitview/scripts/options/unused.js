/** Track unused metrics (dev only) */

import { localhost } from "../utils/env.js";

/** @type {Set<MetricAccessor<any>> | null} */
export const unused = localhost ? new Set() : null;

/**
 * Walk and collect MetricAccessors
 * @param {TreeNode | null | undefined} node
 * @param {Set<MetricAccessor<unknown>>} set
 */
function walk(node, set) {
  if (node && "by" in node) {
    set.add(/** @type {MetricAccessor<unknown>} */ (node));
  } else if (node && typeof node === "object") {
    for (const value of Object.values(node)) {
      walk(/** @type {TreeNode | null | undefined} */ (value), set);
    }
  }
}

/**
 * Collect all MetricAccessors from tree
 * @param {TreeNode} tree
 */
export function collect(tree) {
  if (unused) walk(tree, unused);
}

/**
 * Mark a metric as used
 * @param {MetricAccessor<any>} metric
 */
export function markUsed(metric) {
  unused?.delete(metric);
}

/** Log unused metrics to console */
export function logUnused() {
  if (!unused?.size) return;
  const paths = [...unused].map((m) => Object.values(m.by)[0].path);
  console.warn("Unused metrics:", paths);
}
