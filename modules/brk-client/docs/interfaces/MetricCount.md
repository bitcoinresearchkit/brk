[**brk-client**](../README.md)

***

[brk-client](../globals.md) / MetricCount

# Interface: MetricCount

Defined in: [Developer/brk/modules/brk-client/index.js:430](https://github.com/bitcoinresearchkit/brk/blob/ba60b7e4f64e81cffbd4781566a0df0728014881/modules/brk-client/index.js#L430)

## Properties

### distinctMetrics

> **distinctMetrics**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:431](https://github.com/bitcoinresearchkit/brk/blob/ba60b7e4f64e81cffbd4781566a0df0728014881/modules/brk-client/index.js#L431)

Number of unique metrics available (e.g., realized_price, market_cap)

***

### lazyEndpoints

> **lazyEndpoints**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:433](https://github.com/bitcoinresearchkit/brk/blob/ba60b7e4f64e81cffbd4781566a0df0728014881/modules/brk-client/index.js#L433)

Number of lazy (computed on-the-fly) metric-index combinations

***

### storedEndpoints

> **storedEndpoints**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:434](https://github.com/bitcoinresearchkit/brk/blob/ba60b7e4f64e81cffbd4781566a0df0728014881/modules/brk-client/index.js#L434)

Number of eager (stored on disk) metric-index combinations

***

### totalEndpoints

> **totalEndpoints**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:432](https://github.com/bitcoinresearchkit/brk/blob/ba60b7e4f64e81cffbd4781566a0df0728014881/modules/brk-client/index.js#L432)

Total number of metric-index combinations across all timeframes
