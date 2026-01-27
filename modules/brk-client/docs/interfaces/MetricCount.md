[**brk-client**](../README.md)

***

[brk-client](../globals.md) / MetricCount

# Interface: MetricCount

Defined in: [Developer/brk/modules/brk-client/index.js:373](https://github.com/bitcoinresearchkit/brk/blob/3d3652470714070e3e6df47b3af6f06512541475/modules/brk-client/index.js#L373)

## Properties

### distinctMetrics

> **distinctMetrics**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:374](https://github.com/bitcoinresearchkit/brk/blob/3d3652470714070e3e6df47b3af6f06512541475/modules/brk-client/index.js#L374)

Number of unique metrics available (e.g., realized_price, market_cap)

***

### lazyEndpoints

> **lazyEndpoints**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:376](https://github.com/bitcoinresearchkit/brk/blob/3d3652470714070e3e6df47b3af6f06512541475/modules/brk-client/index.js#L376)

Number of lazy (computed on-the-fly) metric-index combinations

***

### storedEndpoints

> **storedEndpoints**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:377](https://github.com/bitcoinresearchkit/brk/blob/3d3652470714070e3e6df47b3af6f06512541475/modules/brk-client/index.js#L377)

Number of eager (stored on disk) metric-index combinations

***

### totalEndpoints

> **totalEndpoints**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:375](https://github.com/bitcoinresearchkit/brk/blob/3d3652470714070e3e6df47b3af6f06512541475/modules/brk-client/index.js#L375)

Total number of metric-index combinations across all timeframes
