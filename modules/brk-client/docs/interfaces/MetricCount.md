[**brk-client**](../README.md)

***

[brk-client](../globals.md) / MetricCount

# Interface: MetricCount

Defined in: [Developer/brk/modules/brk-client/index.js:392](https://github.com/bitcoinresearchkit/brk/blob/0433e3b25662fded3395ef89ebe1c68d82b918b1/modules/brk-client/index.js#L392)

## Properties

### distinctMetrics

> **distinctMetrics**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:393](https://github.com/bitcoinresearchkit/brk/blob/0433e3b25662fded3395ef89ebe1c68d82b918b1/modules/brk-client/index.js#L393)

Number of unique metrics available (e.g., realized_price, market_cap)

***

### lazyEndpoints

> **lazyEndpoints**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:395](https://github.com/bitcoinresearchkit/brk/blob/0433e3b25662fded3395ef89ebe1c68d82b918b1/modules/brk-client/index.js#L395)

Number of lazy (computed on-the-fly) metric-index combinations

***

### storedEndpoints

> **storedEndpoints**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:396](https://github.com/bitcoinresearchkit/brk/blob/0433e3b25662fded3395ef89ebe1c68d82b918b1/modules/brk-client/index.js#L396)

Number of eager (stored on disk) metric-index combinations

***

### totalEndpoints

> **totalEndpoints**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:394](https://github.com/bitcoinresearchkit/brk/blob/0433e3b25662fded3395ef89ebe1c68d82b918b1/modules/brk-client/index.js#L394)

Total number of metric-index combinations across all timeframes
