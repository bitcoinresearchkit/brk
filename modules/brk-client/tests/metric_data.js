/**
 * Tests for MetricData helper methods and date conversion functions.
 * Run: node tests/metric_data.js
 */

import { BrkClient } from "../index.js";

const client = new BrkClient("http://localhost:3110");

console.log("Testing MetricData helpers...\n");

// Fetch a date-based metric
console.log("1. Fetching price data (dateindex):");
const price = await client.metrics.price.usd.split.close.by.dateindex.first(5);
console.log(
  `   Total: ${price.total}, Start: ${price.start}, End: ${price.end}`,
);

// Test indexes()
console.log("\n2. indexes():");
const indexes = price.indexes();
console.log(`   ${JSON.stringify(indexes)}`);
if (indexes.length !== 5) throw new Error("Expected 5 indexes");
if (indexes[0] !== price.start)
  throw new Error("First index should equal start");

// Test dates()
console.log("\n3. dates():");
const dates = price.dates();
console.log(
  `   First: ${dates[0].toISOString()}, Last: ${dates[dates.length - 1].toISOString()}`,
);
if (dates.length !== 5) throw new Error("Expected 5 dates");
// DateIndex 0 = Jan 3, 2009 (genesis)
if (
  dates[0].getFullYear() !== 2009 ||
  dates[0].getMonth() !== 0 ||
  dates[0].getDate() !== 3
) {
  throw new Error(
    `Expected genesis date (2009-01-03), got ${dates[0].toISOString()}`,
  );
}

// Test toIndexMap()
console.log("\n4. toIndexMap():");
const indexMap = price.toIndexMap();
console.log(
  `   Size: ${indexMap.size}, First value: ${indexMap.get(price.start)}`,
);
if (indexMap.size !== 5) throw new Error("Expected map size 5");
if (indexMap.get(price.start) !== price.data[0])
  throw new Error("First value mismatch");

// Test toDateMap()
console.log("\n5. toDateMap():");
const dateMap = price.toDateMap();
console.log(`   Size: ${dateMap.size}`);
if (dateMap.size !== 5) throw new Error("Expected map size 5");

// Test indexEntries()
console.log("\n6. indexEntries():");
const indexEntries = price.indexEntries();
console.log(`   First: [${indexEntries[0][0]}, ${indexEntries[0][1]}]`);
if (indexEntries[0][0] !== price.start)
  throw new Error("First entry index mismatch");
if (indexEntries[0][1] !== price.data[0])
  throw new Error("First entry value mismatch");

// Test dateEntries()
console.log("\n7. dateEntries():");
const dateEntries = price.dateEntries();
console.log(
  `   First: [${dateEntries[0][0].toISOString()}, ${dateEntries[0][1]}]`,
);
if (dateEntries[0][1] !== price.data[0])
  throw new Error("First entry value mismatch");

// Test iter()
console.log("\n8. iter():");
const iterResult = [...price.iter()];
console.log(
  `   Length: ${iterResult.length}, First: [${iterResult[0][0]}, ${iterResult[0][1]}]`,
);
if (iterResult.length !== 5) throw new Error("Expected 5 items");

// Test iterDates()
console.log("\n9. iterDates():");
const iterDatesResult = [...price.iterDates()];
console.log(
  `   Length: ${iterDatesResult.length}, First date: ${iterDatesResult[0][0].toISOString()}`,
);
if (iterDatesResult.length !== 5) throw new Error("Expected 5 items");

// Test Symbol.iterator (for...of)
console.log("\n10. for...of iteration:");
let count = 0;
for (const [idx, val] of price.iter()) {
  count++;
}
console.log(`   Iterated ${count} items`);
if (count !== 5) throw new Error("Expected 5 iterations");

// Test with non-date-based index (height)
console.log("\n11. Testing height-based metric:");
const heightMetric =
  await client.metrics.price.usd.split.close.by.height.last(3);
console.log(
  `   Total: ${heightMetric.total}, Start: ${heightMetric.start}, End: ${heightMetric.end}`,
);
const heightIndexes = heightMetric.indexes();
console.log(`   Indexes: ${JSON.stringify(heightIndexes)}`);
if (heightIndexes[0] !== heightMetric.start)
  throw new Error("Height index mismatch");

// Test different date indexes
console.log("\n12. Testing monthindex:");
const monthMetric =
  await client.metrics.price.usd.split.close.by.monthindex.first(3);
const monthDates = monthMetric.dates();
console.log(`   First month: ${monthDates[0].toISOString()}`);
// MonthIndex 0 = Jan 1, 2009
if (
  monthDates[0].getFullYear() !== 2009 ||
  monthDates[0].getMonth() !== 0 ||
  monthDates[0].getDate() !== 1
) {
  throw new Error(`Expected 2009-01-01, got ${monthDates[0].toISOString()}`);
}

// Test indexToDate directly
console.log("\n13. Testing indexToDate():");
const genesis = client.indexToDate("dateindex", 0);
if (
  genesis.getFullYear() !== 2009 ||
  genesis.getMonth() !== 0 ||
  genesis.getDate() !== 3
) {
  throw new Error(`Expected genesis 2009-01-03, got ${genesis.toISOString()}`);
}
const dayOne = client.indexToDate("dateindex", 1);
if (
  dayOne.getFullYear() !== 2009 ||
  dayOne.getMonth() !== 0 ||
  dayOne.getDate() !== 9
) {
  throw new Error(`Expected day one 2009-01-09, got ${dayOne.toISOString()}`);
}
console.log(`   dateindex 0: ${genesis.toISOString()}`);
console.log(`   dateindex 1: ${dayOne.toISOString()}`);

// Test weekindex
const week0 = client.indexToDate("weekindex", 0);
const week1 = client.indexToDate("weekindex", 1);
if (week0.getTime() !== genesis.getTime())
  throw new Error("weekindex 0 should equal genesis");
console.log(`   weekindex 0: ${week0.toISOString()}`);
console.log(`   weekindex 1: ${week1.toISOString()}`);

// Test yearindex
const year0 = client.indexToDate("yearindex", 0);
const year1 = client.indexToDate("yearindex", 1);
if (
  year0.getFullYear() !== 2009 ||
  year0.getMonth() !== 0 ||
  year0.getDate() !== 1
) {
  throw new Error(`Expected 2009-01-01, got ${year0.toISOString()}`);
}
if (year1.getFullYear() !== 2010) throw new Error("yearindex 1 should be 2010");
console.log(`   yearindex 0: ${year0.toISOString()}`);
console.log(`   yearindex 1: ${year1.toISOString()}`);

// Test quarterindex
const q0 = client.indexToDate("quarterindex", 0);
const q1 = client.indexToDate("quarterindex", 1);
if (q0.getMonth() !== 0) throw new Error("Q0 should be January");
if (q1.getMonth() !== 3) throw new Error("Q1 should be April");
console.log(`   quarterindex 0: ${q0.toISOString()}`);
console.log(`   quarterindex 1: ${q1.toISOString()}`);

// Test semesterindex
const s0 = client.indexToDate("semesterindex", 0);
const s1 = client.indexToDate("semesterindex", 1);
if (s0.getMonth() !== 0) throw new Error("S0 should be January");
if (s1.getMonth() !== 6) throw new Error("S1 should be July");
console.log(`   semesterindex 0: ${s0.toISOString()}`);
console.log(`   semesterindex 1: ${s1.toISOString()}`);

// Test decadeindex
const d0 = client.indexToDate("decadeindex", 0);
const d1 = client.indexToDate("decadeindex", 1);
if (d0.getFullYear() !== 2009) throw new Error("decadeindex 0 should be 2009");
if (d1.getFullYear() !== 2019) throw new Error("decadeindex 1 should be 2019");
console.log(`   decadeindex 0: ${d0.toISOString()}`);
console.log(`   decadeindex 1: ${d1.toISOString()}`);

// Test isDateIndex
console.log("\n14. Testing isDateIndex():");
const dateIndexes = /** @type {const} */ ([
  "dateindex",
  "weekindex",
  "monthindex",
  "yearindex",
  "quarterindex",
  "semesterindex",
  "decadeindex",
]);
const nonDateIndexes = /** @type {const} */ (["height", "txindex"]);
for (const idx of dateIndexes) {
  if (!client.isDateIndex(idx))
    throw new Error(`${idx} should be a date index`);
}
for (const idx of nonDateIndexes) {
  if (client.isDateIndex(idx))
    throw new Error(`${idx} should not be a date index`);
}
console.log(`   Date indexes: ${dateIndexes.join(", ")} ✓`);
console.log(`   Non-date indexes: ${nonDateIndexes.join(", ")} ✓`);

console.log("\nAll MetricData tests passed!");
