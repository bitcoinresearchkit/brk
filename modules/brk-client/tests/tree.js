/**
 * Comprehensive test that fetches all endpoints in the tree.
 */

import { BrkClient } from "../index.js";

/**
 * Recursively collect all metric patterns from the tree.
 * @param {Record<string, any>} obj
 * @param {string} path
 * @returns {Array<{path: string, metric: Record<string, any>, indexes: string[]}>}
 */
function getAllMetrics(obj, path = "") {
  const metrics = [];

  for (const key of Object.keys(obj)) {
    if (key.startsWith("_")) continue;

    const attr = obj[key];
    if (!attr || typeof attr !== "object") continue;

    const currentPath = path ? `${path}.${key}` : key;

    // Check if this is a metric pattern (has 'by' property with index getters)
    if (attr.by && typeof attr.by === "object") {
      const indexes = Object.keys(attr.by).filter(
        (k) => !k.startsWith("_") && typeof attr.by[k] === "object",
      );
      if (indexes.length > 0) {
        metrics.push({ path: currentPath, metric: attr, indexes });
      }
    }

    // Recurse into nested tree nodes
    if (typeof attr === "object" && !Array.isArray(attr)) {
      metrics.push(...getAllMetrics(attr, currentPath));
    }
  }

  return metrics;
}

async function testAllEndpoints() {
  const client = new BrkClient({ baseUrl: "http://localhost:3110", timeout: 15000 });

  const metrics = getAllMetrics(client.metrics);
  console.log(`\nFound ${metrics.length} metrics`);

  let success = 0;

  for (const { path, metric, indexes } of metrics) {
    for (const idxName of indexes) {
      try {
        const endpoint = metric.by[idxName];
        const res = await endpoint.last(1);
        const count = res.data.length;
        if (count !== 1) {
          console.log(
            `FAIL: ${path}.by.${idxName} -> expected 1, got ${count}`,
          );
          return;
        }
        success++;
        console.log(`OK: ${path}.by.${idxName} -> ${count} items`);
      } catch (e) {
        console.log(
          `FAIL: ${path}.by.${idxName} -> ${e instanceof Error ? e.message : e}`,
        );
        return;
      }
    }
  }

  console.log(`\n=== Results ===`);
  console.log(`Success: ${success}`);
}

testAllEndpoints();
