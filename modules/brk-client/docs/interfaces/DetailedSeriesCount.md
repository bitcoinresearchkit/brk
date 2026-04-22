[**brk-client**](../README.md)

***

[brk-client](../globals.md) / DetailedSeriesCount

# Interface: DetailedSeriesCount

Defined in: [Developer/brk/modules/brk-client/index.js:414](https://github.com/bitcoinresearchkit/brk/blob/bdc3ba1df62c3c70339afe14f39c8c9a5a094c37/modules/brk-client/index.js#L414)

## Properties

### byDb

> **byDb**: `object`

Defined in: [Developer/brk/modules/brk-client/index.js:419](https://github.com/bitcoinresearchkit/brk/blob/bdc3ba1df62c3c70339afe14f39c8c9a5a094c37/modules/brk-client/index.js#L419)

Per-database breakdown of counts

#### Index Signature

\[`key`: `string`\]: [`SeriesCount`](SeriesCount.md)

***

### distinctSeries

> **distinctSeries**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:415](https://github.com/bitcoinresearchkit/brk/blob/bdc3ba1df62c3c70339afe14f39c8c9a5a094c37/modules/brk-client/index.js#L415)

Number of unique series available (e.g., realized_price, market_cap)

***

### lazyEndpoints

> **lazyEndpoints**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:417](https://github.com/bitcoinresearchkit/brk/blob/bdc3ba1df62c3c70339afe14f39c8c9a5a094c37/modules/brk-client/index.js#L417)

Number of lazy (computed on-the-fly) series-index combinations

***

### storedEndpoints

> **storedEndpoints**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:418](https://github.com/bitcoinresearchkit/brk/blob/bdc3ba1df62c3c70339afe14f39c8c9a5a094c37/modules/brk-client/index.js#L418)

Number of eager (stored on disk) series-index combinations

***

### totalEndpoints

> **totalEndpoints**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:416](https://github.com/bitcoinresearchkit/brk/blob/bdc3ba1df62c3c70339afe14f39c8c9a5a094c37/modules/brk-client/index.js#L416)

Total number of series-index combinations across all timeframes
