[**brk-client**](../README.md)

***

[brk-client](../globals.md) / MetricCount

# Interface: MetricCount

Defined in: [Developer/brk/modules/brk-client/index.js:392](https://github.com/bitcoinresearchkit/brk/blob/36bc1fb4912b070acbe47217b1be5b7c1096f80f/modules/brk-client/index.js#L392)

## Properties

### distinctMetrics

> **distinctMetrics**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:393](https://github.com/bitcoinresearchkit/brk/blob/36bc1fb4912b070acbe47217b1be5b7c1096f80f/modules/brk-client/index.js#L393)

Number of unique metrics available (e.g., realized_price, market_cap)

***

### lazyEndpoints

> **lazyEndpoints**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:395](https://github.com/bitcoinresearchkit/brk/blob/36bc1fb4912b070acbe47217b1be5b7c1096f80f/modules/brk-client/index.js#L395)

Number of lazy (computed on-the-fly) metric-index combinations

***

### storedEndpoints

> **storedEndpoints**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:396](https://github.com/bitcoinresearchkit/brk/blob/36bc1fb4912b070acbe47217b1be5b7c1096f80f/modules/brk-client/index.js#L396)

Number of eager (stored on disk) metric-index combinations

***

### totalEndpoints

> **totalEndpoints**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:394](https://github.com/bitcoinresearchkit/brk/blob/36bc1fb4912b070acbe47217b1be5b7c1096f80f/modules/brk-client/index.js#L394)

Total number of metric-index combinations across all timeframes
