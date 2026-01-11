/**
 * Comprehensive test that fetches all endpoints in the tree.
 */

import { BrkClient } from "../index.js";

/**
 * Recursively collect all metric patterns from the tree.
 * @param {object} obj
 * @param {string} path
 * @returns {Array<{path: string, metric: object, indexes: string[]}>}
 */
function getAllMetrics(obj, path = "") {
  const metrics = [];

  for (const key of Object.keys(obj)) {
    if (key.startsWith("_")) continue;

    const attr = obj[key];
    if (!attr || typeof attr !== "object") continue;

    const currentPath = path ? `${path}.${key}` : key;

    // Check if this is a metric pattern (has 'by' property with index methods)
    if (attr.by && typeof attr.by === "object") {
      const indexes = Object.keys(attr.by).filter(
        (k) => !k.startsWith("_") && typeof attr.by[k] === "function",
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
  const client = new BrkClient("http://localhost:3110");

  const metrics = getAllMetrics(client.tree);
  console.log(`\nFound ${metrics.length} metrics`);

  let success = 0;
  let failed = 0;
  const errors = [];

  for (const { path, metric, indexes } of metrics) {
    for (const idxName of indexes) {
      try {
        const endpoint = metric.by[idxName]();
        const res = await endpoint.range(-3);
        const count = res.data.length;
        if (count !== 3) {
          failed++;
          const errorMsg = `FAIL: ${path}.by.${idxName}() -> expected 3, got ${count}`;
          errors.push(errorMsg);
          console.log(errorMsg);
        } else {
          success++;
          console.log(`OK: ${path}.by.${idxName}() -> ${count} items`);
        }
      } catch (e) {
        failed++;
        const errorMsg = `FAIL: ${path}.by.${idxName}() -> ${e.message}`;
        errors.push(errorMsg);
        console.log(errorMsg);
      }
    }
  }

  console.log(`\n=== Results ===`);
  console.log(`Success: ${success}`);
  console.log(`Failed: ${failed}`);

  if (errors.length > 0) {
    console.log(`\nErrors:`);
    errors.slice(0, 10).forEach((err) => console.log(`  ${err}`));
    if (errors.length > 10) {
      console.log(`  ... and ${errors.length - 10} more`);
    }
  }

  if (failed > 0) {
    process.exit(1);
  }
}

testAllEndpoints();
