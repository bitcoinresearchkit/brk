[**brk-client**](../README.md)

***

[brk-client](../globals.md) / DetailedSeriesCount

# Interface: DetailedSeriesCount

Defined in: [Developer/brk/modules/brk-client/index.js:283](https://github.com/bitcoinresearchkit/brk/blob/041652d85d1f6ed0371402865bf343ad227ee250/modules/brk-client/index.js#L283)

## Properties

### byDb

> **byDb**: `object`

Defined in: [Developer/brk/modules/brk-client/index.js:288](https://github.com/bitcoinresearchkit/brk/blob/041652d85d1f6ed0371402865bf343ad227ee250/modules/brk-client/index.js#L288)

Per-database breakdown of counts

#### Index Signature

\[`key`: `string`\]: [`SeriesCount`](SeriesCount.md)

***

### distinctSeries

> **distinctSeries**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:284](https://github.com/bitcoinresearchkit/brk/blob/041652d85d1f6ed0371402865bf343ad227ee250/modules/brk-client/index.js#L284)

Number of unique series available (e.g., realized_price, market_cap)

***

### lazyEndpoints

> **lazyEndpoints**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:286](https://github.com/bitcoinresearchkit/brk/blob/041652d85d1f6ed0371402865bf343ad227ee250/modules/brk-client/index.js#L286)

Number of lazy (computed on-the-fly) series-index combinations

***

### storedEndpoints

> **storedEndpoints**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:287](https://github.com/bitcoinresearchkit/brk/blob/041652d85d1f6ed0371402865bf343ad227ee250/modules/brk-client/index.js#L287)

Number of eager (stored on disk) series-index combinations

***

### totalEndpoints

> **totalEndpoints**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:285](https://github.com/bitcoinresearchkit/brk/blob/041652d85d1f6ed0371402865bf343ad227ee250/modules/brk-client/index.js#L285)

Total number of series-index combinations across all timeframes
